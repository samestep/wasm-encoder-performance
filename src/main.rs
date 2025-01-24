use std::{collections::HashMap, fs::File, io::Write, mem::take};

use regex::Regex;

fn main() -> anyhow::Result<()> {
    let variants = include_str!("variants.txt");
    let encodings = include_str!("encodings.txt");
    let mut encoding_params = HashMap::<&str, Vec<&str>>::new();
    let mut encoding_bodies = HashMap::<&str, Vec<&str>>::new();
    let mut name = None::<&str>;
    let mut params = None::<Vec<&str>>;
    let mut encoding = Vec::<&str>::new();
    let re_pattern = Regex::new(r"^(\w+)(.*) => \{$")?;
    let re_params = Regex::new(r"^\((.*)\)$")?;
    for line in encodings.lines() {
        if let Some(caps) = re_pattern.captures(line) {
            name = Some(caps.get(1).unwrap().as_str());
            if let Some(caps) = re_params.captures(caps.get(2).unwrap().as_str()) {
                params = Some(caps.get(1).unwrap().as_str().split(", ").collect());
            }
        } else if line.starts_with("    ") {
            encoding.push(line);
        } else if line == "}" {
            let name = name.take().unwrap();
            if let Some(names) = params.take() {
                encoding_params.insert(name, names);
            }
            encoding_bodies.insert(name, take(&mut encoding));
        } else {
            assert!(line.is_empty() || line.starts_with("//"));
        }
    }
    let mut out = File::create("generated.rs")?;
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
                    write!(out, ", {param_name}: {}", retype(name, param_name, ty))?;
                }
                writeln!(out, ") -> &mut Self {{")?;
                name
            } else if let Some(caps) = re_struct.captures(line) {
                let name = caps.get(1).unwrap().as_str();
                let snake = snakify(name);
                writeln!(out, "    /// Encode [`Instruction::{name}`].")?;
                write!(out, "    pub fn {snake}(&mut self")?;
                for field in caps.get(2).unwrap().as_str().split(", ") {
                    let (param, ty) = field.split_once(": ").unwrap();
                    let param_name = param.strip_prefix("ref ").unwrap_or(param);
                    write!(out, ", {param_name}: {}", retype(name, param_name, ty))?;
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

fn retype(instruction: &str, param: &str, ty: &'static str) -> &'static str {
    match instruction {
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
            "ordering" => "Ordering",
            _ => panic!("{param}"),
        },
        "ArrayInitData" | "ArrayInitElem" | "ArrayNewFixed" | "ArrayNewData" | "ArrayNewElem" => {
            match param {
                "array_data_index" => "DataIdx",
                "array_elem_index" => "ElemIdx",
                "array_size" => "u32",
                "array_type_index" => "TypeIdx",
                _ => panic!("{param}"),
            }
        }
        "BrOnCast" | "BrOnCastFail" => match param {
            "from_ref_type" => "RefType",
            "relative_depth" => "LabelIdx",
            "to_ref_type" => "RefType",
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
            "ordering" => "Ordering",
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
            "ordering" => "Ordering",
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
                "ordering" => "Ordering",
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
            "ty" => "BlockType",
            _ => panic!("{param}"),
        },
        _ => ty,
    }
}
