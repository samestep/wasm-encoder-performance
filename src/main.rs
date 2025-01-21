use regex::Regex;

fn main() {
    println!("use wasm_encoder::{{BlockType, Catch, Encode, Handle, HeapType, MemArg, Ordering, RefType}};");
    println!();
    println!("pub struct InstructionSink<'a> {{");
    println!("    sink: &'a mut Vec<u8>,");
    println!("}}");
    println!();
    println!("impl<'a> InstructionSink<'a> {{");
    println!("    pub fn new(sink: &'a mut Vec<u8>) -> Self {{");
    println!("        Self {{ sink }}");
    println!("    }}");
    println!();
    let re_instr = Regex::new(r"^Instruction::(\w+)(.*)$").unwrap();
    let re_params = Regex::new(r"^\((.*)\)(.*)$").unwrap();
    let re_fields = Regex::new(r"^ (.*) \} => \{$").unwrap();
    let re_field = Regex::new(r"^    (.*),$").unwrap();
    let re_body = Regex::new(r"^ => (.*),$").unwrap();
    let mut lines = include_str!("code.rs").lines();
    loop {
        let Some(start) = lines.next() else {
            break;
        };
        if start.is_empty() {
            println!();
        } else if start.starts_with("//") {
            println!("    {start}");
        } else if start.starts_with("    ") {
            println!("    {}", selfify(start));
        } else if start == "}" {
            println!("        self");
            println!("    }}");
        } else if let Some(caps) = re_instr.captures(start) {
            let name: &str = &caps[1];
            let rest: &str = &caps[2];
            let snake = snakify(name);
            println!();
            print!("    pub fn {snake}(&mut self");
            let more = if let Some(caps) = re_params.captures(rest) {
                let params: &str = &caps[1];
                for param in params.split(", ") {
                    print_param(name, param);
                }
                caps[2].to_string()
            } else if let Some(curly) = rest.strip_prefix(" {") {
                if curly.is_empty() {
                    loop {
                        match lines.next().unwrap() {
                            "} => {" => break,
                            field_line => {
                                let field = &re_field.captures(field_line).unwrap()[1];
                                print_param(name, field);
                            }
                        }
                    }
                } else {
                    let fields: &str = &re_fields.captures(curly).unwrap()[1];
                    for field in fields.split(", ") {
                        print_param(name, field);
                    }
                }
                "".to_string()
            } else {
                rest.to_string()
            };
            println!(") -> &mut Self {{");
            if let Some(caps) = re_body.captures(&more) {
                println!("        {};", selfify(&caps[1]));
                println!("        self");
                println!("    }}");
            }
        } else {
            panic!("{start}");
        }
    }
    println!("}}");
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

fn print_param(name: &str, param: &str) {
    let (param, ty) = param_type(name, param);
    print!(", {param}: {ty}");
}

fn param_type<'a>(name: &str, mut param: &'a str) -> (&'a str, &'static str) {
    if let Some(rest) = param.strip_prefix("ref ") {
        param = rest;
    }
    let ty = match param {
        "catches" => "&[Catch]",
        "resume_table" => "&[Handle]",
        "ls" => "&[u32]",
        "bt" => "BlockType",
        "heap_type" => "HeapType",
        "m" | "mem" | "memarg" => "MemArg",
        "ordering" => "Ordering",
        "from_ref_type" | "to_ref_type" => "RefType",
        "lanes" => "[u8; 16]",
        "lane" => "u8",
        "argument_index"
        | "array_data_index"
        | "array_elem_index"
        | "array_size"
        | "array_type_index"
        | "array_type_index_dst"
        | "array_type_index_src"
        | "cont_type_index"
        | "data"
        | "data_index"
        | "dst_mem"
        | "dst_table"
        | "elem_index"
        | "f"
        | "field_index"
        | "g"
        | "global_index"
        | "i"
        | "l"
        | "relative_depth"
        | "result_index"
        | "segment"
        | "src_mem"
        | "src_table"
        | "struct_type_index"
        | "t"
        | "table"
        | "table_index"
        | "tag_index"
        | "ty"
        | "type_index" => "u32",
        "x" => match name {
            "F32Const" => "f32",
            "F64Const" => "f64",
            "I32Const" => "i32",
            "I64Const" => "i64",
            "V128Const" => "i128",
            _ => panic!("{name}"),
        },
        _ => panic!("{param}"),
    };
    (param, ty)
}

fn selfify(code: &str) -> String {
    code.replace("sink", "self.sink")
}
