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
        f.sink()
            .global_get(self.global)
            .local_tee(self.local)
            .i32_const(bytes + 65535)
            .i32_add()
            .i32_const(16)
            .i32_shr_u()
            .memory_size(self.memory)
            .i32_sub()
            .local_tee(local)
            .if_(wasm_encoder::BlockType::Empty)
            .local_get(local)
            .memory_grow(self.memory)
            .drop()
            .end()
            .local_get(self.local)
            .i32_const(bytes)
            .i32_add()
            .global_set(self.global);
    }

    fn shrink(self, f: &mut Fun, bytes: i32) {
        f.sink()
            .global_get(self.global)
            .i32_const(bytes)
            .i32_sub()
            .local_tee(self.local)
            .global_set(self.global);
    }
}

fn func_control_store() -> Fun {
    let [k, i, n] = [0, 1, 2];
    let mut f = Function::new([(2, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .grow(&mut f, n, 4);
    f.sink()
        .local_get(i)
        .local_get(k)
        .i32_store(wasm_encoder::MemArg {
            offset: 0,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .end();
    f
}

fn func_control_load() -> Fun {
    let [i] = [0];
    let mut f = Function::new([(1, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .shrink(&mut f, 4);
    f.sink()
        .local_get(i)
        .i32_load(wasm_encoder::MemArg {
            offset: 0,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .end();
    f
}

fn func_f32_mul_fwd() -> Fun {
    let [x, y, i, n] = [0, 1, 2, 3];
    let mut f = Function::new([(2, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .grow(&mut f, n, 8);
    f.sink()
        .local_get(i)
        .local_get(x)
        .f32_store(wasm_encoder::MemArg {
            offset: 0,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .local_get(i)
        .local_get(y)
        .f32_store(wasm_encoder::MemArg {
            offset: 4,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .local_get(x)
        .local_get(y)
        .f32_mul()
        .end();
    f
}

fn func_f32_div_fwd() -> Fun {
    let [x, y, z, i, n] = [0, 1, 2, 3, 4];
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
    f.sink()
        .local_get(i)
        .local_get(y)
        .f32_store(wasm_encoder::MemArg {
            offset: 0,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .local_get(i)
        .local_get(x)
        .local_get(y)
        .f32_div()
        .local_tee(z)
        .f32_store(wasm_encoder::MemArg {
            offset: 4,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .local_get(z)
        .end();
    f
}

fn func_f64_mul_fwd() -> Fun {
    let [x, y, i, n] = [0, 1, 2, 3];
    let mut f = Function::new([(2, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_64_TAPE,
        global: GLOBAL_64_TAPE,
        local: i,
    }
    .grow(&mut f, n, 16);
    f.sink()
        .local_get(i)
        .local_get(x)
        .f64_store(wasm_encoder::MemArg {
            offset: 0,
            align: 3,
            memory_index: MEM_64_TAPE,
        })
        .local_get(i)
        .local_get(y)
        .f64_store(wasm_encoder::MemArg {
            offset: 8,
            align: 3,
            memory_index: MEM_64_TAPE,
        })
        .local_get(x)
        .local_get(y)
        .f64_mul()
        .end();
    f
}

fn func_f64_div_fwd() -> Fun {
    let [x, y, z, i, n] = [0, 1, 2, 3, 4];
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
    f.sink()
        .local_get(i)
        .local_get(y)
        .f64_store(wasm_encoder::MemArg {
            offset: 0,
            align: 3,
            memory_index: MEM_64_TAPE,
        })
        .local_get(i)
        .local_get(x)
        .local_get(y)
        .f64_div()
        .local_tee(z)
        .f64_store(wasm_encoder::MemArg {
            offset: 8,
            align: 3,
            memory_index: MEM_64_TAPE,
        })
        .local_get(z)
        .end();
    f
}

fn func_f32_mul_bwd() -> Fun {
    let [dz, i] = [0, 1];
    let mut f = Function::new([(1, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_32_TAPE,
        global: GLOBAL_32_TAPE,
        local: i,
    }
    .shrink(&mut f, 8);
    f.sink()
        .local_get(dz)
        .local_get(i)
        .f32_load(wasm_encoder::MemArg {
            offset: 4,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .f32_mul()
        .local_get(dz)
        .local_get(i)
        .f32_load(wasm_encoder::MemArg {
            offset: 0,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .f32_mul()
        .end();
    f
}

fn func_f32_div_bwd() -> Fun {
    let [dz, dx, i] = [0, 1, 2];
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
    f.sink()
        .local_get(dz)
        .local_get(i)
        .f32_load(wasm_encoder::MemArg {
            offset: 0,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .f32_div()
        .local_tee(dx)
        .local_get(dx)
        .local_get(i)
        .f32_load(wasm_encoder::MemArg {
            offset: 4,
            align: 2,
            memory_index: MEM_32_TAPE,
        })
        .f32_neg()
        .f32_mul()
        .end();
    f
}

fn func_f64_mul_bwd() -> Fun {
    let [dz, i] = [0, 1];
    let mut f = Function::new([(1, wasm_encoder::ValType::I32)]).into();
    Tape {
        memory: MEM_64_TAPE,
        global: GLOBAL_64_TAPE,
        local: i,
    }
    .shrink(&mut f, 16);
    f.sink()
        .local_get(dz)
        .local_get(i)
        .f64_load(wasm_encoder::MemArg {
            offset: 8,
            align: 3,
            memory_index: MEM_64_TAPE,
        })
        .f64_mul()
        .local_get(dz)
        .local_get(i)
        .f64_load(wasm_encoder::MemArg {
            offset: 0,
            align: 3,
            memory_index: MEM_64_TAPE,
        })
        .f64_mul()
        .end();
    f
}

fn func_f64_div_bwd() -> Fun {
    let [dz, dx, i] = [0, 1, 2];
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
    f.sink()
        .local_get(dz)
        .local_get(i)
        .f64_load(wasm_encoder::MemArg {
            offset: 0,
            align: 3,
            memory_index: MEM_64_TAPE,
        })
        .f64_div()
        .local_tee(dx)
        .local_get(dx)
        .local_get(i)
        .f64_load(wasm_encoder::MemArg {
            offset: 8,
            align: 3,
            memory_index: MEM_64_TAPE,
        })
        .f64_neg()
        .f64_mul()
        .end();
    f
}
