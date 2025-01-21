// adapted from https://github.com/samestep/floretta/blob/v0.3.0/crates/floretta/src/helper.rs

use wasm_encoder::Function;

use crate::encode::Fun;

pub const OFFSET_TYPES: u32 = 6;
pub const TYPE_CONTROL_STORE: u32 = 0;
pub const TYPE_CONTROL_LOAD: u32 = 1;
pub const TYPE_F32_BIN_FWD: u32 = 2;
pub const TYPE_F64_BIN_FWD: u32 = 3;
pub const TYPE_F32_BIN_BWD: u32 = 4;
pub const TYPE_F64_BIN_BWD: u32 = 5;

pub const OFFSET_FUNCTIONS: u32 = 10;
pub const FUNC_CONTROL_STORE: u32 = 0;
pub const FUNC_CONTROL_LOAD: u32 = 1;
pub const FUNC_F32_MUL_FWD: u32 = 2;
pub const FUNC_F32_DIV_FWD: u32 = 3;
pub const FUNC_F64_MUL_FWD: u32 = 4;
pub const FUNC_F64_DIV_FWD: u32 = 5;
pub const FUNC_F32_MUL_BWD: u32 = 6;
pub const FUNC_F32_DIV_BWD: u32 = 7;
pub const FUNC_F64_MUL_BWD: u32 = 8;
pub const FUNC_F64_DIV_BWD: u32 = 9;

pub const OFFSET_MEMORIES: u32 = 2;
const MEM_32_TAPE: u32 = 0;
const MEM_64_TAPE: u32 = 1;

pub const OFFSET_GLOBALS: u32 = 2;
const GLOBAL_32_TAPE: u32 = 0;
const GLOBAL_64_TAPE: u32 = 1;

pub fn helpers() -> impl Iterator<Item = Fun> {
    [
        func_control_store(),
        func_control_load(),
        func_f32_mul_fwd(),
        func_f32_div_fwd(),
        func_f64_mul_fwd(),
        func_f64_div_fwd(),
        func_f32_mul_bwd(),
        func_f32_div_bwd(),
        func_f64_mul_bwd(),
        func_f64_div_bwd(),
    ]
    .into_iter()
}

struct Tape {
    memory: u32,
    global: u32,
    local: u32,
}

impl Tape {
    fn grow(self, f: &mut Fun, local: u32, bytes: i32) {
        f.global_get(self.global);
        f.local_tee(self.local);
        f.i32_const(bytes + 65535);
        f.i32_add();
        f.i32_const(16);
        f.i32_shr_u();
        f.memory_size(self.memory);
        f.i32_sub();
        f.local_tee(local);
        f.if_(wasm_encoder::BlockType::Empty);
        f.local_get(local);
        f.memory_grow(self.memory);
        f.drop();
        f.end();
        f.local_get(self.local);
        f.i32_const(bytes);
        f.i32_add();
        f.global_set(self.global);
    }

    fn shrink(self, f: &mut Fun, bytes: i32) {
        f.global_get(self.global);
        f.i32_const(bytes);
        f.i32_sub();
        f.local_tee(self.local);
        f.global_set(self.global);
    }
}

fn func_control_store() -> Fun {
    let (k, i, n) = (0, 1, 2);
    let mut f = Function::new([(2, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .grow(&mut f, n, 4);
    f.local_get(i);
    f.local_get(k);
    f.i32_store(wasm_encoder::MemArg {
        offset: 0,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.end();
    f
}

fn func_control_load() -> Fun {
    let (i,) = (0,);
    let mut f = Function::new([(1, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .shrink(&mut f, 4);
    f.local_get(i);
    f.i32_load(wasm_encoder::MemArg {
        offset: 0,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.end();
    f
}

fn func_f32_mul_fwd() -> Fun {
    let (x, y, i, n) = (0, 1, 2, 3);
    let mut f = Function::new([(2, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .grow(&mut f, n, 8);
    f.local_get(i);
    f.local_get(x);
    f.f32_store(wasm_encoder::MemArg {
        offset: 0,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.local_get(i);
    f.local_get(y);
    f.f32_store(wasm_encoder::MemArg {
        offset: 4,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.local_get(x);
    f.local_get(y);
    f.f32_mul();
    f.end();
    f
}

fn func_f32_div_fwd() -> Fun {
    let (x, y, z, i, n) = (0, 1, 2, 3, 4);
    let mut f = Function::new([
        (1, wasm_encoder::ValType::F32),
        (2, wasm_encoder::ValType::I32),
    ])
    .into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .grow(&mut f, n, 8);
    f.local_get(i);
    f.local_get(y);
    f.f32_store(wasm_encoder::MemArg {
        offset: 0,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.local_get(i);
    f.local_get(x);
    f.local_get(y);
    f.f32_div();
    f.local_tee(z);
    f.f32_store(wasm_encoder::MemArg {
        offset: 4,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.local_get(z);
    f.end();
    f
}

fn func_f64_mul_fwd() -> Fun {
    let (x, y, i, n) = (0, 1, 2, 3);
    let mut f = Function::new([(2, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_64_TAPE,
        global: GLOBAL_64_TAPE,
        local: i,
    }
    .grow(&mut f, n, 16);
    f.local_get(i);
    f.local_get(x);
    f.f64_store(wasm_encoder::MemArg {
        offset: 0,
        align: 3,
        memory_index: MEM_64_TAPE,
    });
    f.local_get(i);
    f.local_get(y);
    f.f64_store(wasm_encoder::MemArg {
        offset: 8,
        align: 3,
        memory_index: MEM_64_TAPE,
    });
    f.local_get(x);
    f.local_get(y);
    f.f64_mul();
    f.end();
    f
}

fn func_f64_div_fwd() -> Fun {
    let (x, y, z, i, n) = (0, 1, 2, 3, 4);
    let mut f = Function::new([
        (1, wasm_encoder::ValType::F64),
        (2, wasm_encoder::ValType::I32),
    ])
    .into();
    Tape {
        memory: MEM_64_TAPE,
        global: GLOBAL_64_TAPE,
        local: i,
    }
    .grow(&mut f, n, 16);
    f.local_get(i);
    f.local_get(y);
    f.f64_store(wasm_encoder::MemArg {
        offset: 0,
        align: 3,
        memory_index: MEM_64_TAPE,
    });
    f.local_get(i);
    f.local_get(x);
    f.local_get(y);
    f.f64_div();
    f.local_tee(z);
    f.f64_store(wasm_encoder::MemArg {
        offset: 8,
        align: 3,
        memory_index: MEM_64_TAPE,
    });
    f.local_get(z);
    f.end();
    f
}

fn func_f32_mul_bwd() -> Fun {
    let (dz, i) = (0, 1);
    let mut f = Function::new([(1, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .shrink(&mut f, 8);
    f.local_get(dz);
    f.local_get(i);
    f.f32_load(wasm_encoder::MemArg {
        offset: 4,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.f32_mul();
    f.local_get(dz);
    f.local_get(i);
    f.f32_load(wasm_encoder::MemArg {
        offset: 0,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.f32_mul();
    f.end();
    f
}

fn func_f32_div_bwd() -> Fun {
    let (dz, dx, i) = (0, 1, 2);
    let mut f = Function::new([
        (1, wasm_encoder::ValType::F32),
        (1, wasm_encoder::ValType::I32),
    ])
    .into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .shrink(&mut f, 8);
    f.local_get(dz);
    f.local_get(i);
    f.f32_load(wasm_encoder::MemArg {
        offset: 0,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.f32_div();
    f.local_tee(dx);
    f.local_get(dx);
    f.local_get(i);
    f.f32_load(wasm_encoder::MemArg {
        offset: 4,
        align: 2,
        memory_index: MEM_32_TAPE,
    });
    f.f32_neg();
    f.f32_mul();
    f.end();
    f
}

fn func_f64_mul_bwd() -> Fun {
    let (dz, i) = (0, 1);
    let mut f = Function::new([(1, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_64_TAPE,
        global: GLOBAL_64_TAPE,
        local: i,
    }
    .shrink(&mut f, 16);
    f.local_get(dz);
    f.local_get(i);
    f.f64_load(wasm_encoder::MemArg {
        offset: 8,
        align: 3,
        memory_index: MEM_64_TAPE,
    });
    f.f64_mul();
    f.local_get(dz);
    f.local_get(i);
    f.f64_load(wasm_encoder::MemArg {
        offset: 0,
        align: 3,
        memory_index: MEM_64_TAPE,
    });
    f.f64_mul();
    f.end();
    f
}

fn func_f64_div_bwd() -> Fun {
    let (dz, dx, i) = (0, 1, 2);
    let mut f = Function::new([
        (1, wasm_encoder::ValType::F64),
        (1, wasm_encoder::ValType::I32),
    ])
    .into();
    Tape {
        memory: MEM_64_TAPE,
        global: GLOBAL_64_TAPE,
        local: i,
    }
    .shrink(&mut f, 16);
    f.local_get(dz);
    f.local_get(i);
    f.f64_load(wasm_encoder::MemArg {
        offset: 0,
        align: 3,
        memory_index: MEM_64_TAPE,
    });
    f.f64_div();
    f.local_tee(dx);
    f.local_get(dx);
    f.local_get(i);
    f.f64_load(wasm_encoder::MemArg {
        offset: 8,
        align: 3,
        memory_index: MEM_64_TAPE,
    });
    f.f64_neg();
    f.f64_mul();
    f.end();
    f
}
