use std::{
    collections::HashMap,
    fs::{create_dir, remove_dir_all, write, File},
    io::Write,
    mem::take,
    path::Path,
};

use regex::Regex;
use serde::Serialize;

fn main() -> anyhow::Result<()> {
    let variants = include_str!("variants.txt");
    let encodings = include_str!("encodings.txt");
    let sg_dir = Path::new("ast-grep");
    let _ = remove_dir_all(sg_dir);
    create_dir(sg_dir)?;
    let mut sg = File::create("ast-grep.sh")?;
    writeln!(sg, "#!/usr/bin/env bash")?;
    writeln!(sg, "set -x")?;
    let mut encoding_params = HashMap::<&str, Vec<&str>>::new();
    let mut encoding_bodies = HashMap::<&str, Vec<&str>>::new();
    let mut out = File::create("generated.rs")?;
    let mut name = None::<&str>;
    let mut params = None::<Vec<&str>>;
    let mut encoding = Vec::<&str>::new();
    let re_pattern = Regex::new(r"^((\w+)(.*)) => \{$")?;
    let re_tuple = Regex::new(r"^\((.*)\)$")?;
    let re_struct = Regex::new(r"^ \{ (.*) \}$")?;
    writeln!(
        out,
        "fn encode_instruction(instruction: &Instruction, bytes: &mut Vec<u8>) {{"
    )?;
    writeln!(out, "    let mut sink = InstructionSink::new(bytes);")?;
    writeln!(out, "    match *instruction {{")?;
    for line in encodings.lines() {
        if line.is_empty() {
            writeln!(out)?;
        } else if line.starts_with("//") {
            writeln!(out, "        {line}")?;
        } else if let Some(caps) = re_pattern.captures(line) {
            let pattern = caps.get(1).unwrap().as_str();
            name = Some(caps.get(2).unwrap().as_str());
            let snake = snakify(name.unwrap());
            let pat_args = caps.get(3).unwrap().as_str();
            let mut pat = format!("$FUNC.instruction(&Instruction::{}", name.unwrap());
            if let Some(caps) = re_tuple.captures(pat_args) {
                params = Some(split(caps.get(1).unwrap().as_str()));
                pat.push('(');
                let mut first = true;
                for param in params.as_ref().unwrap() {
                    if !first {
                        pat.push_str(", ");
                    }
                    first = false;
                    pat.push('$');
                    pat.push_str(&param.to_uppercase());
                }
                pat.push(')');
            } else if let Some(caps) = re_struct.captures(pat_args) {
                params = Some(split(caps.get(1).unwrap().as_str()));
                pat.push_str(" { ");
                for param in params.as_ref().unwrap() {
                    pat.push_str(param);
                    pat.push_str(": $");
                    pat.push_str(&param.to_uppercase());
                    pat.push_str(", ");
                }
                pat.push('}');
            } else {
                assert!(pat_args.is_empty());
                params = Some(Vec::new());
            }
            pat.push(')');
            let mut fix = format!("$FUNC.instructions().{snake}(");
            write!(out, "        Instruction::{pattern} => sink.{snake}(")?;
            let args = params.as_ref().unwrap();
            if args.is_empty() {
            } else {
                let ordered = if args.len() == 1 {
                    args.to_vec()
                } else {
                    reorder(name.unwrap())
                };
                assert_eq!(sorted(args), sorted(&ordered));
                let mut first = true;
                for arg in ordered {
                    if !first {
                        write!(out, ", ")?;
                        fix.push_str(", ");
                    }
                    first = false;
                    if let Some(ty) = retype(name.unwrap(), arg) {
                        if ty.starts_with("impl ") {
                            match name.unwrap() {
                                "BrTable" => write!(out, "ls.iter().copied().map(LabelIdx)")?,
                                "Resume" | "ResumeThrow" => {
                                    write!(out, "resume_table.iter().cloned()")?
                                }
                                "TryTable" => write!(out, "catches.iter().cloned()")?,
                                instruction => panic!("{instruction}"),
                            }
                            fix.push('$');
                            fix.push_str(&arg.to_uppercase());
                        } else {
                            write!(out, "{ty}({arg})")?;
                            fix.push_str(ty);
                            fix.push_str("($");
                            fix.push_str(&arg.to_uppercase());
                            fix.push(')');
                        }
                    } else {
                        write!(out, "{arg}")?;
                        fix.push('$');
                        fix.push_str(&arg.to_uppercase());
                    }
                }
            }
            writeln!(out, "),")?;
            fix.push(')');
            write(
                sg_dir.join(format!("{snake}.json")),
                serde_json::to_string_pretty(&Rewrite {
                    id: &snake,
                    language: "Rust",
                    rule: Rule { pattern: &pat },
                    fix: &fix,
                })?,
            )?;
            writeln!(
                sg,
                "ast-grep scan --update-all --rule ast-grep/{snake}.json"
            )?;
        } else if line.starts_with("    ") {
            encoding.push(line);
        } else if line == "}" {
            let name = name.take().unwrap();
            encoding_params.insert(name, params.take().unwrap());
            encoding_bodies.insert(name, take(&mut encoding));
        } else {
            panic!("{line}");
        }
    }
    writeln!(out, "        _ => unimplemented!(),")?;
    writeln!(out, "    }};")?;
    writeln!(out, "}}")?;
    writeln!(out)?;
    writeln!(out, "impl<'a> InstructionSink<'a> {{")?;
    let re_unit = Regex::new(r"^(\w+)$")?;
    let re_tuple = Regex::new(r"^(\w+)\((.*)\)$")?;
    let re_struct = Regex::new(r"^(\w+) \{ (.*) \}$")?;
    for line in variants.lines() {
        if line.is_empty() {
            writeln!(out)?;
        } else if line.starts_with("//") {
            writeln!(out, "    {line}")?;
            writeln!(out)?;
        } else {
            let name = if let Some(caps) = re_unit.captures(line) {
                let name = caps.get(1).unwrap().as_str();
                let snake = snakify(name);
                writeln!(out, "    /// Encode [`Instruction::{name}`].")?;
                writeln!(out, "    pub fn {snake}(&mut self) -> &mut Self {{")?;
                name
            } else if let Some(caps) = re_tuple.captures(line) {
                let name = caps.get(1).unwrap().as_str();
                let snake = snakify(name);
                writeln!(out, "    /// Encode [`Instruction::{name}`].")?;
                write!(out, "    pub fn {snake}(&mut self")?;
                let types = caps.get(2).unwrap().as_str().split(", ");
                for (param, ty) in encoding_params.get(name).unwrap().iter().zip(types) {
                    let param_name = param.strip_prefix("ref ").unwrap_or(param);
                    write!(
                        out,
                        ", {param_name}: {}",
                        retype(name, param_name).unwrap_or(ty)
                    )?;
                }
                writeln!(out, ") -> &mut Self {{")?;
                name
            } else if let Some(caps) = re_struct.captures(line) {
                let name = caps.get(1).unwrap().as_str();
                let snake = snakify(name);
                writeln!(out, "    /// Encode [`Instruction::{name}`].")?;
                write!(out, "    pub fn {snake}(&mut self")?;
                let field_types: HashMap<&str, &str> = caps
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|field| {
                        let (param, ty) = field.split_once(": ").unwrap();
                        (param.strip_prefix("ref ").unwrap_or(param), ty)
                    })
                    .collect();
                for param in reorder(name) {
                    let ty = field_types.get(param).unwrap();
                    write!(out, ", {param}: {}", retype(name, param).unwrap_or(ty))?;
                }
                writeln!(out, ") -> &mut Self {{")?;
                name
            } else {
                panic!("{line}");
            };
            for &stmt in encoding_bodies.get(name).unwrap() {
                match stmt {
                    "    catches.encode(sink);" => {
                        writeln!(out, "        encode_vec(catches, self.sink);")?
                    }
                    "    ls.encode(sink);" => writeln!(out, "        encode_vec(ls, self.sink);")?,
                    "    resume_table.encode(sink);" => {
                        writeln!(out, "        encode_vec(resume_table, self.sink);")?
                    }
                    _ => writeln!(out, "    {}", stmt.replace("sink", "self.sink"))?,
                }
            }
            writeln!(out, "        self")?;
            writeln!(out, "    }}")?;
            writeln!(out)?;
        }
    }
    writeln!(out, "}}")?;
    Ok(())
}

fn sorted<T: Clone + Ord>(xs: &[T]) -> Vec<T> {
    let mut ys = xs.to_vec();
    ys.sort();
    ys
}

fn snakify(name: &str) -> String {
    match name {
        "Else" => "else_".to_string(),
        "If" => "if_".to_string(),
        "Loop" => "loop_".to_string(),
        "Return" => "return_".to_string(),
        "Try" => "try_".to_string(),
        _ => {
            let mut s = String::new();
            let mut first = true;
            for c in name.chars() {
                if c.is_uppercase() && !first {
                    s.push('_');
                }
                s.push(c.to_ascii_lowercase());
                first = false;
            }
            s.replace("and_not", "andnot")
                .replace("ext_add", "extadd")
                .replace("ext_mul", "extmul")
                .replace("p_max", "pmax")
                .replace("p_min", "pmin")
                .replace("q15_mulr", "q15mulr")
        }
    }
}

fn split(s: &str) -> Vec<&str> {
    s.split(", ")
        .map(|x| x.strip_prefix("ref ").unwrap_or(x))
        .collect()
}

fn retype(instruction: &str, param: &str) -> Option<&'static str> {
    let ty = match instruction {
        "DataDrop" => "DataIdx",
        "ElemDrop" => "ElemIdx",
        "Call" | "RefFunc" | "ReturnCall" => "FuncIdx",
        "GlobalGet" | "GlobalSet" => "GlobalIdx",
        "Br" | "BrIf" | "BrOnNull" | "BrOnNonNull" | "Delegate" | "Rethrow" => "LabelIdx",
        "LocalGet" | "LocalSet" | "LocalTee" => "LocalIdx",
        "TableFill" | "TableSet" | "TableGet" | "TableGrow" | "TableSize" | "TableCopy" => {
            "TableIdx"
        }
        "MemoryCopy" | "MemoryDiscard" | "MemoryFill" | "MemoryGrow" | "MemorySize" => "MemIdx",
        "Catch" | "Suspend" | "Throw" => "TagIdx",
        "ArrayCopy" | "ArrayFill" | "ArrayGet" | "ArrayGetS" | "ArrayGetU" | "ArrayNew"
        | "ArrayNewDefault" | "ArraySet" | "CallRef" | "ContBind" | "ContNew" | "ReturnCallRef"
        | "StructNew" | "StructNewDefault" => "TypeIdx",
        "ArrayAtomicGet"
        | "ArrayAtomicGetS"
        | "ArrayAtomicGetU"
        | "ArrayAtomicSet"
        | "ArrayAtomicRmwAdd"
        | "ArrayAtomicRmwSub"
        | "ArrayAtomicRmwAnd"
        | "ArrayAtomicRmwOr"
        | "ArrayAtomicRmwXor"
        | "ArrayAtomicRmwXchg"
        | "ArrayAtomicRmwCmpxchg" => match param {
            "array_type_index" => "TypeIdx",
            "ordering" => return None,
            _ => panic!("{param}"),
        },
        "ArrayInitData" | "ArrayInitElem" | "ArrayNewFixed" | "ArrayNewData" | "ArrayNewElem" => {
            match param {
                "array_data_index" => "DataIdx",
                "array_elem_index" => "ElemIdx",
                "array_size" => return None,
                "array_type_index" => "TypeIdx",
                _ => panic!("{param}"),
            }
        }
        "BrOnCast" | "BrOnCastFail" => match param {
            "from_ref_type" | "to_ref_type" => return None,
            "relative_depth" => "LabelIdx",
            _ => panic!("{param}"),
        },
        "BrTable" => match param {
            "l" => "LabelIdx",
            "ls" => "impl IntoIterator<Item = LabelIdx, IntoIter: ExactSizeIterator>",
            _ => panic!("{param}"),
        },
        "CallIndirect" | "ReturnCallIndirect" => match param {
            "table_index" => "TableIdx",
            "type_index" => "TypeIdx",
            _ => panic!("{param}"),
        },
        "GlobalAtomicGet"
        | "GlobalAtomicSet"
        | "GlobalAtomicRmwAdd"
        | "GlobalAtomicRmwSub"
        | "GlobalAtomicRmwAnd"
        | "GlobalAtomicRmwOr"
        | "GlobalAtomicRmwXor"
        | "GlobalAtomicRmwXchg"
        | "GlobalAtomicRmwCmpxchg" => match param {
            "global_index" => "GlobalIdx",
            "ordering" => return None,
            _ => panic!("{param}"),
        },
        "MemoryInit" => match param {
            "data_index" => "DataIdx",
            "mem" => "MemIdx",
            _ => panic!("{param}"),
        },
        "Resume" | "ResumeThrow" | "Switch" => match param {
            "cont_type_index" => "TypeIdx",
            "resume_table" => "impl IntoIterator<Item = Handle, IntoIter: ExactSizeIterator>",
            "tag_index" => "TagIdx",
            _ => panic!("{param}"),
        },
        "StructAtomicGet"
        | "StructAtomicGetS"
        | "StructAtomicGetU"
        | "StructAtomicSet"
        | "StructAtomicRmwAdd"
        | "StructAtomicRmwSub"
        | "StructAtomicRmwAnd"
        | "StructAtomicRmwOr"
        | "StructAtomicRmwXor"
        | "StructAtomicRmwXchg"
        | "StructAtomicRmwCmpxchg" => match param {
            "field_index" => "FieldIdx",
            "ordering" => return None,
            "struct_type_index" => "TypeIdx",
            _ => panic!("{param}"),
        },
        "StructGet" | "StructGetS" | "StructGetU" | "StructSet" => match param {
            "field_index" => "FieldIdx",
            "struct_type_index" => "TypeIdx",
            _ => panic!("{param}"),
        },
        "TableAtomicGet" | "TableAtomicSet" | "TableAtomicRmwXchg" | "TableAtomicRmwCmpxchg" => {
            match param {
                "ordering" => return None,
                "table_index" => "TableIdx",
                _ => panic!("{param}"),
            }
        }
        "TableInit" => match param {
            "elem_index" => "ElemIdx",
            "table" => "TableIdx",
            _ => panic!("{param}"),
        },
        "TryTable" => match param {
            "catches" => "impl IntoIterator<Item = Catch, IntoIter: ExactSizeIterator>",
            "ty" => return None,
            _ => panic!("{param}"),
        },
        _ => return None,
    };
    Some(ty)
}

fn reorder(instruction: &str) -> Vec<&str> {
    match instruction {
        "ArrayAtomicGet"
        | "ArrayAtomicGetS"
        | "ArrayAtomicGetU"
        | "ArrayAtomicSet"
        | "ArrayAtomicRmwAdd"
        | "ArrayAtomicRmwSub"
        | "ArrayAtomicRmwAnd"
        | "ArrayAtomicRmwOr"
        | "ArrayAtomicRmwXor"
        | "ArrayAtomicRmwXchg"
        | "ArrayAtomicRmwCmpxchg" => vec!["ordering", "array_type_index"],
        "ArrayCopy" => vec!["array_type_index_dst", "array_type_index_src"],
        "ArrayInitData" => vec!["array_type_index", "array_data_index"],
        "ArrayInitElem" => vec!["array_type_index", "array_elem_index"],
        "ArrayNewData" => vec!["array_type_index", "array_data_index"],
        "ArrayNewElem" => vec!["array_type_index", "array_elem_index"],
        "ArrayNewFixed" => vec!["array_type_index", "array_size"],
        "BrOnCast" | "BrOnCastFail" => vec!["relative_depth", "from_ref_type", "to_ref_type"],
        "BrTable" => vec!["ls", "l"],
        "CallIndirect" | "ReturnCallIndirect" => vec!["table_index", "type_index"],
        "ContBind" => vec!["argument_index", "result_index"],
        "GlobalAtomicGet"
        | "GlobalAtomicSet"
        | "GlobalAtomicRmwAdd"
        | "GlobalAtomicRmwSub"
        | "GlobalAtomicRmwAnd"
        | "GlobalAtomicRmwOr"
        | "GlobalAtomicRmwXor"
        | "GlobalAtomicRmwXchg"
        | "GlobalAtomicRmwCmpxchg" => vec!["ordering", "global_index"],
        "MemoryCopy" => vec!["dst_mem", "src_mem"],
        "MemoryInit" => vec!["mem", "data_index"],
        "Resume" => vec!["cont_type_index", "resume_table"],
        "ResumeThrow" => vec!["cont_type_index", "tag_index", "resume_table"],
        "StructAtomicGet"
        | "StructAtomicGetS"
        | "StructAtomicGetU"
        | "StructAtomicSet"
        | "StructAtomicRmwAdd"
        | "StructAtomicRmwSub"
        | "StructAtomicRmwAnd"
        | "StructAtomicRmwOr"
        | "StructAtomicRmwXor"
        | "StructAtomicRmwXchg"
        | "StructAtomicRmwCmpxchg" => vec!["ordering", "struct_type_index", "field_index"],
        "StructGet" | "StructGetS" | "StructGetU" | "StructSet" => {
            vec!["struct_type_index", "field_index"]
        }
        "Switch" => vec!["cont_type_index", "tag_index"],
        "TableAtomicGet" | "TableAtomicSet" | "TableAtomicRmwXchg" | "TableAtomicRmwCmpxchg" => {
            vec!["ordering", "table_index"]
        }
        "TableCopy" => vec!["dst_table", "src_table"],
        "TableInit" => vec!["table", "elem_index"],
        "TryTable" => vec!["ty", "catches"],
        "V128Load8Lane" | "V128Load16Lane" | "V128Load32Lane" | "V128Load64Lane"
        | "V128Store8Lane" | "V128Store16Lane" | "V128Store32Lane" | "V128Store64Lane" => {
            vec!["memarg", "lane"]
        }
        _ => panic!("{instruction}"),
    }
}

#[derive(Serialize)]
struct Rewrite<'a> {
    id: &'a str,
    language: &'static str,
    rule: Rule<'a>,
    fix: &'a str,
}

#[derive(Serialize)]
struct Rule<'a> {
    pattern: &'a str,
}
