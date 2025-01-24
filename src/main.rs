use std::{fs::File, io::Write};

use regex::Regex;

fn main() -> anyhow::Result<()> {
    let variants = include_str!("variants.txt");
    let encodings = include_str!("encodings.txt");
    let mut out = File::create("generated.rs")?;
    let re_variant = Regex::new(r"^(\w+)")?;
    writeln!(out, "// Instruction variants")?;
    writeln!(out)?;
    for line in variants.lines() {
        if line.is_empty() {
            writeln!(out)?;
        } else if line.starts_with("//") {
            writeln!(out, "{line}")?;
            writeln!(out)?;
        } else {
            let name = &re_variant.captures(line).unwrap()[1];
            writeln!(out, "/// Standalone type for [`Instruction::{name}`].")?;
            writeln!(out, "#[derive(Clone, Debug)]")?;
            write!(out, "pub struct {line}")?;
            if line.ends_with("}") {
                writeln!(out)?;
            } else {
                writeln!(out, ";")?;
            }
            writeln!(out)?;
        }
    }
    writeln!(out, "// Encodings")?;
    writeln!(out)?;
    let re_pattern = Regex::new(r"^((\w+).*) => \{$")?;
    for line in encodings.lines() {
        if line.is_empty() {
            writeln!(out)?;
        } else if line.starts_with("//") {
            writeln!(out, "{line}")?;
            writeln!(out)?;
        } else if line.starts_with("    ") {
            writeln!(out, "    {line}")?;
        } else if line == "}" {
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
            writeln!(out)?;
        } else {
            let caps = re_pattern.captures(line).unwrap();
            let pat = &caps[1];
            let name = &caps[2];
            writeln!(out, "impl Encode for {name} {{")?;
            writeln!(out, "    fn encode(&self, sink: &mut Vec<u8>) {{")?;
            writeln!(out, "        let {pat} = *self;")?;
        }
    }
    writeln!(out, "impl Encode for Instruction<'_> {{")?;
    writeln!(out, "    fn encode(&self, sink: &mut Vec<u8>) {{")?;
    writeln!(out, "        match *self {{")?;
    for line in encodings.lines() {
        if line.is_empty() {
            writeln!(out)?;
        } else if line.starts_with("//") {
            writeln!(out, "            {line}")?;
        } else if !(line.starts_with("    ") || line == "}") {
            let caps = re_pattern.captures(line).unwrap();
            let pat = &caps[1];
            writeln!(out, "            Instruction::{pat} => {pat}.encode(sink),")?;
        }
    }
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    Ok(())
}
