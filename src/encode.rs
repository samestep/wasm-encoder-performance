use wasm_encoder::Function;

use crate::sink::InstructionSink;

pub struct Fun {
    bytes: Vec<u8>,
}

impl Default for Fun {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Function> for Fun {
    fn from(f: Function) -> Self {
        Self {
            bytes: f.into_raw_body(),
        }
    }
}

impl Fun {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn sink(&mut self) -> InstructionSink<'_> {
        InstructionSink::new(&mut self.bytes)
    }

    pub fn into_raw_body(self) -> Vec<u8> {
        self.bytes
    }
}
