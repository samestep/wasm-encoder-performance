use wasm_encoder::{BlockType, Encode, Function, MemArg};

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

    pub fn into_raw_body(self) -> Vec<u8> {
        self.bytes
    }

    fn encode(&mut self, x: impl Encode) {
        x.encode(&mut self.bytes);
    }

    pub fn if_(&mut self, bt: BlockType) {
        self.bytes.push(0x04);
        self.encode(bt);
    }

    pub fn end(&mut self) {
        self.bytes.push(0x0B);
    }

    pub fn drop(&mut self) {
        self.bytes.push(0x1A);
    }

    pub fn local_get(&mut self, l: u32) {
        self.bytes.push(0x20);
        self.encode(l);
    }

    pub fn local_tee(&mut self, l: u32) {
        self.bytes.push(0x22);
        self.encode(l);
    }

    pub fn global_get(&mut self, g: u32) {
        self.bytes.push(0x23);
        self.encode(g);
    }

    pub fn global_set(&mut self, g: u32) {
        self.bytes.push(0x24);
        self.encode(g);
    }

    pub fn i32_load(&mut self, m: MemArg) {
        self.bytes.push(0x28);
        self.encode(m);
    }

    pub fn f32_load(&mut self, m: MemArg) {
        self.bytes.push(0x2A);
        self.encode(m);
    }

    pub fn f64_load(&mut self, m: MemArg) {
        self.bytes.push(0x2B);
        self.encode(m);
    }

    pub fn i32_store(&mut self, m: MemArg) {
        self.bytes.push(0x36);
        self.encode(m);
    }

    pub fn f32_store(&mut self, m: MemArg) {
        self.bytes.push(0x38);
        self.encode(m);
    }

    pub fn f64_store(&mut self, m: MemArg) {
        self.bytes.push(0x39);
        self.encode(m);
    }

    pub fn memory_size(&mut self, i: u32) {
        self.bytes.push(0x3F);
        self.encode(i);
    }

    pub fn memory_grow(&mut self, i: u32) {
        self.bytes.push(0x40);
        self.encode(i);
    }

    pub fn i32_const(&mut self, x: i32) {
        self.bytes.push(0x41);
        self.encode(x);
    }

    pub fn i32_add(&mut self) {
        self.bytes.push(0x6A);
    }

    pub fn i32_sub(&mut self) {
        self.bytes.push(0x6B);
    }

    pub fn i32_shr_u(&mut self) {
        self.bytes.push(0x76);
    }

    pub fn f32_neg(&mut self) {
        self.bytes.push(0x8C);
    }

    pub fn f32_mul(&mut self) {
        self.bytes.push(0x94);
    }

    pub fn f32_div(&mut self) {
        self.bytes.push(0x95);
    }

    pub fn f64_neg(&mut self) {
        self.bytes.push(0x9A);
    }

    pub fn f64_mul(&mut self) {
        self.bytes.push(0xA2);
    }

    pub fn f64_div(&mut self) {
        self.bytes.push(0xA3);
    }
}
