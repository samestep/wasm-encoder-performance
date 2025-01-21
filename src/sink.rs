use wasm_encoder::{BlockType, Catch, Encode, Handle, HeapType, MemArg, Ordering, RefType};

pub struct InstructionSink<'a> {
    sink: &'a mut Vec<u8>,
}

impl<'a> InstructionSink<'a> {
    pub fn new(sink: &'a mut Vec<u8>) -> Self {
        Self { sink }
    }

    // Control instructions.

    pub fn unreachable(&mut self) -> &mut Self {
        self.sink.push(0x00);
        self
    }

    pub fn nop(&mut self) -> &mut Self {
        self.sink.push(0x01);
        self
    }

    pub fn block(&mut self, bt: BlockType) -> &mut Self {
        self.sink.push(0x02);
        bt.encode(self.sink);
        self
    }

    pub fn loop_(&mut self, bt: BlockType) -> &mut Self {
        self.sink.push(0x03);
        bt.encode(self.sink);
        self
    }

    pub fn if_(&mut self, bt: BlockType) -> &mut Self {
        self.sink.push(0x04);
        bt.encode(self.sink);
        self
    }

    pub fn else_(&mut self) -> &mut Self {
        self.sink.push(0x05);
        self
    }

    pub fn try_(&mut self, bt: BlockType) -> &mut Self {
        self.sink.push(0x06);
        bt.encode(self.sink);
        self
    }

    pub fn catch(&mut self, t: u32) -> &mut Self {
        self.sink.push(0x07);
        t.encode(self.sink);
        self
    }

    pub fn throw(&mut self, t: u32) -> &mut Self {
        self.sink.push(0x08);
        t.encode(self.sink);
        self
    }

    pub fn rethrow(&mut self, l: u32) -> &mut Self {
        self.sink.push(0x09);
        l.encode(self.sink);
        self
    }

    pub fn throw_ref(&mut self) -> &mut Self {
        self.sink.push(0x0A);
        self
    }

    pub fn end(&mut self) -> &mut Self {
        self.sink.push(0x0B);
        self
    }

    pub fn br(&mut self, l: u32) -> &mut Self {
        self.sink.push(0x0C);
        l.encode(self.sink);
        self
    }

    pub fn br_if(&mut self, l: u32) -> &mut Self {
        self.sink.push(0x0D);
        l.encode(self.sink);
        self
    }

    pub fn br_table(&mut self, ls: &[u32], l: u32) -> &mut Self {
        self.sink.push(0x0E);
        ls.encode(self.sink);
        l.encode(self.sink);
        self
    }

    pub fn br_on_null(&mut self, l: u32) -> &mut Self {
        self.sink.push(0xD5);
        l.encode(self.sink);
        self
    }

    pub fn br_on_non_null(&mut self, l: u32) -> &mut Self {
        self.sink.push(0xD6);
        l.encode(self.sink);
        self
    }

    pub fn return_(&mut self) -> &mut Self {
        self.sink.push(0x0F);
        self
    }

    pub fn call(&mut self, f: u32) -> &mut Self {
        self.sink.push(0x10);
        f.encode(self.sink);
        self
    }

    pub fn call_ref(&mut self, ty: u32) -> &mut Self {
        self.sink.push(0x14);
        ty.encode(self.sink);
        self
    }

    pub fn call_indirect(&mut self, type_index: u32, table_index: u32) -> &mut Self {
        self.sink.push(0x11);
        type_index.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    pub fn return_call_ref(&mut self, ty: u32) -> &mut Self {
        self.sink.push(0x15);
        ty.encode(self.sink);
        self
    }

    pub fn return_call(&mut self, f: u32) -> &mut Self {
        self.sink.push(0x12);
        f.encode(self.sink);
        self
    }

    pub fn return_call_indirect(&mut self, type_index: u32, table_index: u32) -> &mut Self {
        self.sink.push(0x13);
        type_index.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    pub fn delegate(&mut self, l: u32) -> &mut Self {
        self.sink.push(0x18);
        l.encode(self.sink);
        self
    }

    pub fn catch_all(&mut self) -> &mut Self {
        self.sink.push(0x19);
        self
    }

    // Parametric instructions.

    pub fn drop(&mut self) -> &mut Self {
        self.sink.push(0x1A);
        self
    }

    pub fn select(&mut self) -> &mut Self {
        self.sink.push(0x1B);
        self
    }

    pub fn typed_select(&mut self, ty: u32) -> &mut Self {
        self.sink.push(0x1c);
        [ty].encode(self.sink);
        self
    }

    pub fn try_table(&mut self, ty: u32, catches: &[Catch]) -> &mut Self {
        self.sink.push(0x1f);
        ty.encode(self.sink);
        catches.encode(self.sink);
        self
    }

    // Variable instructions.

    pub fn local_get(&mut self, l: u32) -> &mut Self {
        self.sink.push(0x20);
        l.encode(self.sink);
        self
    }

    pub fn local_set(&mut self, l: u32) -> &mut Self {
        self.sink.push(0x21);
        l.encode(self.sink);
        self
    }

    pub fn local_tee(&mut self, l: u32) -> &mut Self {
        self.sink.push(0x22);
        l.encode(self.sink);
        self
    }

    pub fn global_get(&mut self, g: u32) -> &mut Self {
        self.sink.push(0x23);
        g.encode(self.sink);
        self
    }

    pub fn global_set(&mut self, g: u32) -> &mut Self {
        self.sink.push(0x24);
        g.encode(self.sink);
        self
    }

    pub fn table_get(&mut self, table: u32) -> &mut Self {
        self.sink.push(0x25);
        table.encode(self.sink);
        self
    }

    pub fn table_set(&mut self, table: u32) -> &mut Self {
        self.sink.push(0x26);
        table.encode(self.sink);
        self
    }

    // Memory instructions.

    pub fn i32_load(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x28);
        m.encode(self.sink);
        self
    }

    pub fn i64_load(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x29);
        m.encode(self.sink);
        self
    }

    pub fn f32_load(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2A);
        m.encode(self.sink);
        self
    }

    pub fn f64_load(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2B);
        m.encode(self.sink);
        self
    }

    pub fn i32_load8_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2C);
        m.encode(self.sink);
        self
    }

    pub fn i32_load8_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2D);
        m.encode(self.sink);
        self
    }

    pub fn i32_load16_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2E);
        m.encode(self.sink);
        self
    }

    pub fn i32_load16_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2F);
        m.encode(self.sink);
        self
    }

    pub fn i64_load8_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x30);
        m.encode(self.sink);
        self
    }

    pub fn i64_load8_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x31);
        m.encode(self.sink);
        self
    }

    pub fn i64_load16_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x32);
        m.encode(self.sink);
        self
    }

    pub fn i64_load16_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x33);
        m.encode(self.sink);
        self
    }

    pub fn i64_load32_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x34);
        m.encode(self.sink);
        self
    }

    pub fn i64_load32_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x35);
        m.encode(self.sink);
        self
    }

    pub fn i32_store(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x36);
        m.encode(self.sink);
        self
    }

    pub fn i64_store(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x37);
        m.encode(self.sink);
        self
    }

    pub fn f32_store(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x38);
        m.encode(self.sink);
        self
    }

    pub fn f64_store(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x39);
        m.encode(self.sink);
        self
    }

    pub fn i32_store8(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3A);
        m.encode(self.sink);
        self
    }

    pub fn i32_store16(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3B);
        m.encode(self.sink);
        self
    }

    pub fn i64_store8(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3C);
        m.encode(self.sink);
        self
    }

    pub fn i64_store16(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3D);
        m.encode(self.sink);
        self
    }

    pub fn i64_store32(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3E);
        m.encode(self.sink);
        self
    }

    pub fn memory_size(&mut self, i: u32) -> &mut Self {
        self.sink.push(0x3F);
        i.encode(self.sink);
        self
    }

    pub fn memory_grow(&mut self, i: u32) -> &mut Self {
        self.sink.push(0x40);
        i.encode(self.sink);
        self
    }

    pub fn memory_init(&mut self, mem: MemArg, data_index: u32) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x08);
        data_index.encode(self.sink);
        mem.encode(self.sink);
        self
    }

    pub fn data_drop(&mut self, data: u32) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x09);
        data.encode(self.sink);
        self
    }

    pub fn memory_copy(&mut self, src_mem: u32, dst_mem: u32) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0a);
        dst_mem.encode(self.sink);
        src_mem.encode(self.sink);
        self
    }

    pub fn memory_fill(&mut self, mem: MemArg) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0b);
        mem.encode(self.sink);
        self
    }

    pub fn memory_discard(&mut self, mem: MemArg) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x12);
        mem.encode(self.sink);
        self
    }

    // Numeric instructions.

    pub fn i32_const(&mut self, x: i32) -> &mut Self {
        self.sink.push(0x41);
        x.encode(self.sink);
        self
    }

    pub fn i64_const(&mut self, x: i64) -> &mut Self {
        self.sink.push(0x42);
        x.encode(self.sink);
        self
    }

    pub fn f32_const(&mut self, x: f32) -> &mut Self {
        self.sink.push(0x43);
        let x = x.to_bits();
        self.sink.extend(x.to_le_bytes().iter().copied());
        self
    }

    pub fn f64_const(&mut self, x: f64) -> &mut Self {
        self.sink.push(0x44);
        let x = x.to_bits();
        self.sink.extend(x.to_le_bytes().iter().copied());
        self
    }

    pub fn i32_eqz(&mut self) -> &mut Self {
        self.sink.push(0x45);
        self
    }

    pub fn i32_eq(&mut self) -> &mut Self {
        self.sink.push(0x46);
        self
    }

    pub fn i32_ne(&mut self) -> &mut Self {
        self.sink.push(0x47);
        self
    }

    pub fn i32_lt_s(&mut self) -> &mut Self {
        self.sink.push(0x48);
        self
    }

    pub fn i32_lt_u(&mut self) -> &mut Self {
        self.sink.push(0x49);
        self
    }

    pub fn i32_gt_s(&mut self) -> &mut Self {
        self.sink.push(0x4A);
        self
    }

    pub fn i32_gt_u(&mut self) -> &mut Self {
        self.sink.push(0x4B);
        self
    }

    pub fn i32_le_s(&mut self) -> &mut Self {
        self.sink.push(0x4C);
        self
    }

    pub fn i32_le_u(&mut self) -> &mut Self {
        self.sink.push(0x4D);
        self
    }

    pub fn i32_ge_s(&mut self) -> &mut Self {
        self.sink.push(0x4E);
        self
    }

    pub fn i32_ge_u(&mut self) -> &mut Self {
        self.sink.push(0x4F);
        self
    }

    pub fn i64_eqz(&mut self) -> &mut Self {
        self.sink.push(0x50);
        self
    }

    pub fn i64_eq(&mut self) -> &mut Self {
        self.sink.push(0x51);
        self
    }

    pub fn i64_ne(&mut self) -> &mut Self {
        self.sink.push(0x52);
        self
    }

    pub fn i64_lt_s(&mut self) -> &mut Self {
        self.sink.push(0x53);
        self
    }

    pub fn i64_lt_u(&mut self) -> &mut Self {
        self.sink.push(0x54);
        self
    }

    pub fn i64_gt_s(&mut self) -> &mut Self {
        self.sink.push(0x55);
        self
    }

    pub fn i64_gt_u(&mut self) -> &mut Self {
        self.sink.push(0x56);
        self
    }

    pub fn i64_le_s(&mut self) -> &mut Self {
        self.sink.push(0x57);
        self
    }

    pub fn i64_le_u(&mut self) -> &mut Self {
        self.sink.push(0x58);
        self
    }

    pub fn i64_ge_s(&mut self) -> &mut Self {
        self.sink.push(0x59);
        self
    }

    pub fn i64_ge_u(&mut self) -> &mut Self {
        self.sink.push(0x5A);
        self
    }

    pub fn f32_eq(&mut self) -> &mut Self {
        self.sink.push(0x5B);
        self
    }

    pub fn f32_ne(&mut self) -> &mut Self {
        self.sink.push(0x5C);
        self
    }

    pub fn f32_lt(&mut self) -> &mut Self {
        self.sink.push(0x5D);
        self
    }

    pub fn f32_gt(&mut self) -> &mut Self {
        self.sink.push(0x5E);
        self
    }

    pub fn f32_le(&mut self) -> &mut Self {
        self.sink.push(0x5F);
        self
    }

    pub fn f32_ge(&mut self) -> &mut Self {
        self.sink.push(0x60);
        self
    }

    pub fn f64_eq(&mut self) -> &mut Self {
        self.sink.push(0x61);
        self
    }

    pub fn f64_ne(&mut self) -> &mut Self {
        self.sink.push(0x62);
        self
    }

    pub fn f64_lt(&mut self) -> &mut Self {
        self.sink.push(0x63);
        self
    }

    pub fn f64_gt(&mut self) -> &mut Self {
        self.sink.push(0x64);
        self
    }

    pub fn f64_le(&mut self) -> &mut Self {
        self.sink.push(0x65);
        self
    }

    pub fn f64_ge(&mut self) -> &mut Self {
        self.sink.push(0x66);
        self
    }

    pub fn i32_clz(&mut self) -> &mut Self {
        self.sink.push(0x67);
        self
    }

    pub fn i32_ctz(&mut self) -> &mut Self {
        self.sink.push(0x68);
        self
    }

    pub fn i32_popcnt(&mut self) -> &mut Self {
        self.sink.push(0x69);
        self
    }

    pub fn i32_add(&mut self) -> &mut Self {
        self.sink.push(0x6A);
        self
    }

    pub fn i32_sub(&mut self) -> &mut Self {
        self.sink.push(0x6B);
        self
    }

    pub fn i32_mul(&mut self) -> &mut Self {
        self.sink.push(0x6C);
        self
    }

    pub fn i32_div_s(&mut self) -> &mut Self {
        self.sink.push(0x6D);
        self
    }

    pub fn i32_div_u(&mut self) -> &mut Self {
        self.sink.push(0x6E);
        self
    }

    pub fn i32_rem_s(&mut self) -> &mut Self {
        self.sink.push(0x6F);
        self
    }

    pub fn i32_rem_u(&mut self) -> &mut Self {
        self.sink.push(0x70);
        self
    }

    pub fn i32_and(&mut self) -> &mut Self {
        self.sink.push(0x71);
        self
    }

    pub fn i32_or(&mut self) -> &mut Self {
        self.sink.push(0x72);
        self
    }

    pub fn i32_xor(&mut self) -> &mut Self {
        self.sink.push(0x73);
        self
    }

    pub fn i32_shl(&mut self) -> &mut Self {
        self.sink.push(0x74);
        self
    }

    pub fn i32_shr_s(&mut self) -> &mut Self {
        self.sink.push(0x75);
        self
    }

    pub fn i32_shr_u(&mut self) -> &mut Self {
        self.sink.push(0x76);
        self
    }

    pub fn i32_rotl(&mut self) -> &mut Self {
        self.sink.push(0x77);
        self
    }

    pub fn i32_rotr(&mut self) -> &mut Self {
        self.sink.push(0x78);
        self
    }

    pub fn i64_clz(&mut self) -> &mut Self {
        self.sink.push(0x79);
        self
    }

    pub fn i64_ctz(&mut self) -> &mut Self {
        self.sink.push(0x7A);
        self
    }

    pub fn i64_popcnt(&mut self) -> &mut Self {
        self.sink.push(0x7B);
        self
    }

    pub fn i64_add(&mut self) -> &mut Self {
        self.sink.push(0x7C);
        self
    }

    pub fn i64_sub(&mut self) -> &mut Self {
        self.sink.push(0x7D);
        self
    }

    pub fn i64_mul(&mut self) -> &mut Self {
        self.sink.push(0x7E);
        self
    }

    pub fn i64_div_s(&mut self) -> &mut Self {
        self.sink.push(0x7F);
        self
    }

    pub fn i64_div_u(&mut self) -> &mut Self {
        self.sink.push(0x80);
        self
    }

    pub fn i64_rem_s(&mut self) -> &mut Self {
        self.sink.push(0x81);
        self
    }

    pub fn i64_rem_u(&mut self) -> &mut Self {
        self.sink.push(0x82);
        self
    }

    pub fn i64_and(&mut self) -> &mut Self {
        self.sink.push(0x83);
        self
    }

    pub fn i64_or(&mut self) -> &mut Self {
        self.sink.push(0x84);
        self
    }

    pub fn i64_xor(&mut self) -> &mut Self {
        self.sink.push(0x85);
        self
    }

    pub fn i64_shl(&mut self) -> &mut Self {
        self.sink.push(0x86);
        self
    }

    pub fn i64_shr_s(&mut self) -> &mut Self {
        self.sink.push(0x87);
        self
    }

    pub fn i64_shr_u(&mut self) -> &mut Self {
        self.sink.push(0x88);
        self
    }

    pub fn i64_rotl(&mut self) -> &mut Self {
        self.sink.push(0x89);
        self
    }

    pub fn i64_rotr(&mut self) -> &mut Self {
        self.sink.push(0x8A);
        self
    }

    pub fn f32_abs(&mut self) -> &mut Self {
        self.sink.push(0x8B);
        self
    }

    pub fn f32_neg(&mut self) -> &mut Self {
        self.sink.push(0x8C);
        self
    }

    pub fn f32_ceil(&mut self) -> &mut Self {
        self.sink.push(0x8D);
        self
    }

    pub fn f32_floor(&mut self) -> &mut Self {
        self.sink.push(0x8E);
        self
    }

    pub fn f32_trunc(&mut self) -> &mut Self {
        self.sink.push(0x8F);
        self
    }

    pub fn f32_nearest(&mut self) -> &mut Self {
        self.sink.push(0x90);
        self
    }

    pub fn f32_sqrt(&mut self) -> &mut Self {
        self.sink.push(0x91);
        self
    }

    pub fn f32_add(&mut self) -> &mut Self {
        self.sink.push(0x92);
        self
    }

    pub fn f32_sub(&mut self) -> &mut Self {
        self.sink.push(0x93);
        self
    }

    pub fn f32_mul(&mut self) -> &mut Self {
        self.sink.push(0x94);
        self
    }

    pub fn f32_div(&mut self) -> &mut Self {
        self.sink.push(0x95);
        self
    }

    pub fn f32_min(&mut self) -> &mut Self {
        self.sink.push(0x96);
        self
    }

    pub fn f32_max(&mut self) -> &mut Self {
        self.sink.push(0x97);
        self
    }

    pub fn f32_copysign(&mut self) -> &mut Self {
        self.sink.push(0x98);
        self
    }

    pub fn f64_abs(&mut self) -> &mut Self {
        self.sink.push(0x99);
        self
    }

    pub fn f64_neg(&mut self) -> &mut Self {
        self.sink.push(0x9A);
        self
    }

    pub fn f64_ceil(&mut self) -> &mut Self {
        self.sink.push(0x9B);
        self
    }

    pub fn f64_floor(&mut self) -> &mut Self {
        self.sink.push(0x9C);
        self
    }

    pub fn f64_trunc(&mut self) -> &mut Self {
        self.sink.push(0x9D);
        self
    }

    pub fn f64_nearest(&mut self) -> &mut Self {
        self.sink.push(0x9E);
        self
    }

    pub fn f64_sqrt(&mut self) -> &mut Self {
        self.sink.push(0x9F);
        self
    }

    pub fn f64_add(&mut self) -> &mut Self {
        self.sink.push(0xA0);
        self
    }

    pub fn f64_sub(&mut self) -> &mut Self {
        self.sink.push(0xA1);
        self
    }

    pub fn f64_mul(&mut self) -> &mut Self {
        self.sink.push(0xA2);
        self
    }

    pub fn f64_div(&mut self) -> &mut Self {
        self.sink.push(0xA3);
        self
    }

    pub fn f64_min(&mut self) -> &mut Self {
        self.sink.push(0xA4);
        self
    }

    pub fn f64_max(&mut self) -> &mut Self {
        self.sink.push(0xA5);
        self
    }

    pub fn f64_copysign(&mut self) -> &mut Self {
        self.sink.push(0xA6);
        self
    }

    pub fn i32_wrap_i64(&mut self) -> &mut Self {
        self.sink.push(0xA7);
        self
    }

    pub fn i32_trunc_f32_s(&mut self) -> &mut Self {
        self.sink.push(0xA8);
        self
    }

    pub fn i32_trunc_f32_u(&mut self) -> &mut Self {
        self.sink.push(0xA9);
        self
    }

    pub fn i32_trunc_f64_s(&mut self) -> &mut Self {
        self.sink.push(0xAA);
        self
    }

    pub fn i32_trunc_f64_u(&mut self) -> &mut Self {
        self.sink.push(0xAB);
        self
    }

    pub fn i64_extend_i32_s(&mut self) -> &mut Self {
        self.sink.push(0xAC);
        self
    }

    pub fn i64_extend_i32_u(&mut self) -> &mut Self {
        self.sink.push(0xAD);
        self
    }

    pub fn i64_trunc_f32_s(&mut self) -> &mut Self {
        self.sink.push(0xAE);
        self
    }

    pub fn i64_trunc_f32_u(&mut self) -> &mut Self {
        self.sink.push(0xAF);
        self
    }

    pub fn i64_trunc_f64_s(&mut self) -> &mut Self {
        self.sink.push(0xB0);
        self
    }

    pub fn i64_trunc_f64_u(&mut self) -> &mut Self {
        self.sink.push(0xB1);
        self
    }

    pub fn f32_convert_i32_s(&mut self) -> &mut Self {
        self.sink.push(0xB2);
        self
    }

    pub fn f32_convert_i32_u(&mut self) -> &mut Self {
        self.sink.push(0xB3);
        self
    }

    pub fn f32_convert_i64_s(&mut self) -> &mut Self {
        self.sink.push(0xB4);
        self
    }

    pub fn f32_convert_i64_u(&mut self) -> &mut Self {
        self.sink.push(0xB5);
        self
    }

    pub fn f32_demote_f64(&mut self) -> &mut Self {
        self.sink.push(0xB6);
        self
    }

    pub fn f64_convert_i32_s(&mut self) -> &mut Self {
        self.sink.push(0xB7);
        self
    }

    pub fn f64_convert_i32_u(&mut self) -> &mut Self {
        self.sink.push(0xB8);
        self
    }

    pub fn f64_convert_i64_s(&mut self) -> &mut Self {
        self.sink.push(0xB9);
        self
    }

    pub fn f64_convert_i64_u(&mut self) -> &mut Self {
        self.sink.push(0xBA);
        self
    }

    pub fn f64_promote_f32(&mut self) -> &mut Self {
        self.sink.push(0xBB);
        self
    }

    pub fn i32_reinterpret_f32(&mut self) -> &mut Self {
        self.sink.push(0xBC);
        self
    }

    pub fn i64_reinterpret_f64(&mut self) -> &mut Self {
        self.sink.push(0xBD);
        self
    }

    pub fn f32_reinterpret_i32(&mut self) -> &mut Self {
        self.sink.push(0xBE);
        self
    }

    pub fn f64_reinterpret_i64(&mut self) -> &mut Self {
        self.sink.push(0xBF);
        self
    }

    pub fn i32_extend8_s(&mut self) -> &mut Self {
        self.sink.push(0xC0);
        self
    }

    pub fn i32_extend16_s(&mut self) -> &mut Self {
        self.sink.push(0xC1);
        self
    }

    pub fn i64_extend8_s(&mut self) -> &mut Self {
        self.sink.push(0xC2);
        self
    }

    pub fn i64_extend16_s(&mut self) -> &mut Self {
        self.sink.push(0xC3);
        self
    }

    pub fn i64_extend32_s(&mut self) -> &mut Self {
        self.sink.push(0xC4);
        self
    }

    pub fn i32_trunc_sat_f32_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x00);
        self
    }

    pub fn i32_trunc_sat_f32_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x01);
        self
    }

    pub fn i32_trunc_sat_f64_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x02);
        self
    }

    pub fn i32_trunc_sat_f64_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x03);
        self
    }

    pub fn i64_trunc_sat_f32_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x04);
        self
    }

    pub fn i64_trunc_sat_f32_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x05);
        self
    }

    pub fn i64_trunc_sat_f64_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x06);
        self
    }

    pub fn i64_trunc_sat_f64_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x07);
        self
    }

    // Reference types instructions.

    pub fn ref_null(&mut self, ty: u32) -> &mut Self {
        self.sink.push(0xd0);
        ty.encode(self.sink);
        self
    }

    pub fn ref_is_null(&mut self) -> &mut Self {
        self.sink.push(0xd1);
        self
    }

    pub fn ref_func(&mut self, f: u32) -> &mut Self {
        self.sink.push(0xd2);
        f.encode(self.sink);
        self
    }

    pub fn ref_eq(&mut self) -> &mut Self {
        self.sink.push(0xd3);
        self
    }

    pub fn ref_as_non_null(&mut self) -> &mut Self {
        self.sink.push(0xd4);
        self
    }

    // GC instructions.

    pub fn struct_new(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x00);
        type_index.encode(self.sink);
        self
    }

    pub fn struct_new_default(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x01);
        type_index.encode(self.sink);
        self
    }

    pub fn struct_get(&mut self, struct_type_index: u32, field_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x02);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_get_s(&mut self, struct_type_index: u32, field_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x03);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_get_u(&mut self, struct_type_index: u32, field_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x04);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_set(&mut self, struct_type_index: u32, field_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x05);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn array_new(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x06);
        type_index.encode(self.sink);
        self
    }

    pub fn array_new_default(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x07);
        type_index.encode(self.sink);
        self
    }

    pub fn array_new_fixed(&mut self, array_type_index: u32, array_size: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x08);
        array_type_index.encode(self.sink);
        array_size.encode(self.sink);
        self
    }

    pub fn array_new_data(&mut self, array_type_index: u32, array_data_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x09);
        array_type_index.encode(self.sink);
        array_data_index.encode(self.sink);
        self
    }

    pub fn array_new_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0a);
        array_type_index.encode(self.sink);
        array_elem_index.encode(self.sink);
        self
    }

    pub fn array_get(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0b);
        type_index.encode(self.sink);
        self
    }

    pub fn array_get_s(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0c);
        type_index.encode(self.sink);
        self
    }

    pub fn array_get_u(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0d);
        type_index.encode(self.sink);
        self
    }

    pub fn array_set(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0e);
        type_index.encode(self.sink);
        self
    }

    pub fn array_len(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0f);
        self
    }

    pub fn array_fill(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x10);
        type_index.encode(self.sink);
        self
    }

    pub fn array_copy(
        &mut self,
        array_type_index_dst: u32,
        array_type_index_src: u32,
    ) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x11);
        array_type_index_dst.encode(self.sink);
        array_type_index_src.encode(self.sink);
        self
    }

    pub fn array_init_data(&mut self, array_type_index: u32, array_data_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x12);
        array_type_index.encode(self.sink);
        array_data_index.encode(self.sink);
        self
    }

    pub fn array_init_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x13);
        array_type_index.encode(self.sink);
        array_elem_index.encode(self.sink);
        self
    }

    pub fn ref_test_non_null(&mut self, heap_type: HeapType) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x14);
        heap_type.encode(self.sink);
        self
    }

    pub fn ref_test_nullable(&mut self, heap_type: HeapType) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x15);
        heap_type.encode(self.sink);
        self
    }

    pub fn ref_cast_non_null(&mut self, heap_type: HeapType) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x16);
        heap_type.encode(self.sink);
        self
    }

    pub fn ref_cast_nullable(&mut self, heap_type: HeapType) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x17);
        heap_type.encode(self.sink);
        self
    }

    pub fn br_on_cast(
        &mut self,
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    ) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x18);
        let cast_flags = (from_ref_type.nullable as u8) | ((to_ref_type.nullable as u8) << 1);
        self.sink.push(cast_flags);
        relative_depth.encode(self.sink);
        from_ref_type.heap_type.encode(self.sink);
        to_ref_type.heap_type.encode(self.sink);
        self
    }

    pub fn br_on_cast_fail(
        &mut self,
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    ) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x19);
        let cast_flags = (from_ref_type.nullable as u8) | ((to_ref_type.nullable as u8) << 1);
        self.sink.push(cast_flags);
        relative_depth.encode(self.sink);
        from_ref_type.heap_type.encode(self.sink);
        to_ref_type.heap_type.encode(self.sink);
        self
    }

    pub fn any_convert_extern(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1a);
        self
    }

    pub fn extern_convert_any(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1b);
        self
    }

    pub fn ref_i31(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1c);
        self
    }

    pub fn i31_get_s(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1d);
        self
    }

    pub fn i31_get_u(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1e);
        self
    }

    // Bulk memory instructions.

    pub fn table_init(&mut self, elem_index: u32, table: u32) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0c);
        elem_index.encode(self.sink);
        table.encode(self.sink);
        self
    }

    pub fn elem_drop(&mut self, segment: u32) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0d);
        segment.encode(self.sink);
        self
    }

    pub fn table_copy(&mut self, src_table: u32, dst_table: u32) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0e);
        dst_table.encode(self.sink);
        src_table.encode(self.sink);
        self
    }

    pub fn table_grow(&mut self, table: u32) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0f);
        table.encode(self.sink);
        self
    }

    pub fn table_size(&mut self, table: u32) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x10);
        table.encode(self.sink);
        self
    }

    pub fn table_fill(&mut self, table: u32) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x11);
        table.encode(self.sink);
        self
    }

    // SIMD instructions.

    pub fn v128_load(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x00u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load8x8_s(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x01u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load8x8_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x02u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load16x4_s(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x03u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load16x4_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x04u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load32x2_s(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x05u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load32x2_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x06u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load8_splat(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x07u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load16_splat(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x08u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load32_splat(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x09u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load64_splat(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x0Au32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_store(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x0Bu32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_const(&mut self, x: i128) -> &mut Self {
        self.sink.push(0xFD);
        0x0Cu32.encode(self.sink);
        self.sink.extend(x.to_le_bytes().iter().copied());
        self
    }

    pub fn i8x16_shuffle(&mut self, lanes: [u8; 16]) -> &mut Self {
        self.sink.push(0xFD);
        0x0Du32.encode(self.sink);
        assert!(lanes.iter().all(|l: &u8| *l < 32));
        self.sink.extend(lanes.iter().copied());
        self
    }

    pub fn i8x16_swizzle(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x0Eu32.encode(self.sink);
        self
    }

    pub fn i8x16_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x0Fu32.encode(self.sink);
        self
    }

    pub fn i16x8_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10u32.encode(self.sink);
        self
    }

    pub fn i32x4_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x11u32.encode(self.sink);
        self
    }

    pub fn i64x2_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x12u32.encode(self.sink);
        self
    }

    pub fn f32x4_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x13u32.encode(self.sink);
        self
    }

    pub fn f64x2_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x14u32.encode(self.sink);
        self
    }

    pub fn i8x16_extract_lane_s(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x15u32.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    pub fn i8x16_extract_lane_u(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x16u32.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    pub fn i8x16_replace_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x17u32.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    pub fn i16x8_extract_lane_s(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x18u32.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    pub fn i16x8_extract_lane_u(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x19u32.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    pub fn i16x8_replace_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x1Au32.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    pub fn i32x4_extract_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x1Bu32.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    pub fn i32x4_replace_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x1Cu32.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    pub fn i64x2_extract_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x1Du32.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    pub fn i64x2_replace_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x1Eu32.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    pub fn f32x4_extract_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x1Fu32.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    pub fn f32x4_replace_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x20u32.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    pub fn f64x2_extract_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x21u32.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    pub fn f64x2_replace_lane(&mut self, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x22u32.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    pub fn i8x16_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x23u32.encode(self.sink);
        self
    }

    pub fn i8x16_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x24u32.encode(self.sink);
        self
    }

    pub fn i8x16_lt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x25u32.encode(self.sink);
        self
    }

    pub fn i8x16_lt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x26u32.encode(self.sink);
        self
    }

    pub fn i8x16_gt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x27u32.encode(self.sink);
        self
    }

    pub fn i8x16_gt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x28u32.encode(self.sink);
        self
    }

    pub fn i8x16_le_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x29u32.encode(self.sink);
        self
    }

    pub fn i8x16_le_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Au32.encode(self.sink);
        self
    }

    pub fn i8x16_ge_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Bu32.encode(self.sink);
        self
    }

    pub fn i8x16_ge_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Cu32.encode(self.sink);
        self
    }

    pub fn i16x8_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Du32.encode(self.sink);
        self
    }

    pub fn i16x8_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Eu32.encode(self.sink);
        self
    }

    pub fn i16x8_lt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Fu32.encode(self.sink);
        self
    }

    pub fn i16x8_lt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x30u32.encode(self.sink);
        self
    }

    pub fn i16x8_gt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x31u32.encode(self.sink);
        self
    }

    pub fn i16x8_gt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x32u32.encode(self.sink);
        self
    }

    pub fn i16x8_le_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x33u32.encode(self.sink);
        self
    }

    pub fn i16x8_le_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x34u32.encode(self.sink);
        self
    }

    pub fn i16x8_ge_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x35u32.encode(self.sink);
        self
    }

    pub fn i16x8_ge_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x36u32.encode(self.sink);
        self
    }

    pub fn i32x4_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x37u32.encode(self.sink);
        self
    }

    pub fn i32x4_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x38u32.encode(self.sink);
        self
    }

    pub fn i32x4_lt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x39u32.encode(self.sink);
        self
    }

    pub fn i32x4_lt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Au32.encode(self.sink);
        self
    }

    pub fn i32x4_gt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Bu32.encode(self.sink);
        self
    }

    pub fn i32x4_gt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Cu32.encode(self.sink);
        self
    }

    pub fn i32x4_le_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Du32.encode(self.sink);
        self
    }

    pub fn i32x4_le_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Eu32.encode(self.sink);
        self
    }

    pub fn i32x4_ge_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Fu32.encode(self.sink);
        self
    }

    pub fn i32x4_ge_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x40u32.encode(self.sink);
        self
    }

    pub fn f32x4_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x41u32.encode(self.sink);
        self
    }

    pub fn f32x4_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x42u32.encode(self.sink);
        self
    }

    pub fn f32x4_lt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x43u32.encode(self.sink);
        self
    }

    pub fn f32x4_gt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x44u32.encode(self.sink);
        self
    }

    pub fn f32x4_le(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x45u32.encode(self.sink);
        self
    }

    pub fn f32x4_ge(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x46u32.encode(self.sink);
        self
    }

    pub fn f64x2_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x47u32.encode(self.sink);
        self
    }

    pub fn f64x2_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x48u32.encode(self.sink);
        self
    }

    pub fn f64x2_lt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x49u32.encode(self.sink);
        self
    }

    pub fn f64x2_gt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Au32.encode(self.sink);
        self
    }

    pub fn f64x2_le(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Bu32.encode(self.sink);
        self
    }

    pub fn f64x2_ge(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Cu32.encode(self.sink);
        self
    }

    pub fn v128_not(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Du32.encode(self.sink);
        self
    }

    pub fn v128_and(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Eu32.encode(self.sink);
        self
    }

    pub fn v128_andnot(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Fu32.encode(self.sink);
        self
    }

    pub fn v128_or(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x50u32.encode(self.sink);
        self
    }

    pub fn v128_xor(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x51u32.encode(self.sink);
        self
    }

    pub fn v128_bitselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x52u32.encode(self.sink);
        self
    }

    pub fn v128_any_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x53u32.encode(self.sink);
        self
    }

    pub fn i8x16_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x60u32.encode(self.sink);
        self
    }

    pub fn i8x16_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x61u32.encode(self.sink);
        self
    }

    pub fn i8x16_popcnt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x62u32.encode(self.sink);
        self
    }

    pub fn i8x16_all_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x63u32.encode(self.sink);
        self
    }

    pub fn i8x16_bitmask(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x64u32.encode(self.sink);
        self
    }

    pub fn i8x16_narrow_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x65u32.encode(self.sink);
        self
    }

    pub fn i8x16_narrow_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x66u32.encode(self.sink);
        self
    }

    pub fn i8x16_shl(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6bu32.encode(self.sink);
        self
    }

    pub fn i8x16_shr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6cu32.encode(self.sink);
        self
    }

    pub fn i8x16_shr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6du32.encode(self.sink);
        self
    }

    pub fn i8x16_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6eu32.encode(self.sink);
        self
    }

    pub fn i8x16_add_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6fu32.encode(self.sink);
        self
    }

    pub fn i8x16_add_sat_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x70u32.encode(self.sink);
        self
    }

    pub fn i8x16_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x71u32.encode(self.sink);
        self
    }

    pub fn i8x16_sub_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x72u32.encode(self.sink);
        self
    }

    pub fn i8x16_sub_sat_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x73u32.encode(self.sink);
        self
    }

    pub fn i8x16_min_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x76u32.encode(self.sink);
        self
    }

    pub fn i8x16_min_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x77u32.encode(self.sink);
        self
    }

    pub fn i8x16_max_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x78u32.encode(self.sink);
        self
    }

    pub fn i8x16_max_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x79u32.encode(self.sink);
        self
    }

    pub fn i8x16_avgr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Bu32.encode(self.sink);
        self
    }

    pub fn i16x8_extadd_pairwise_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Cu32.encode(self.sink);
        self
    }

    pub fn i16x8_extadd_pairwise_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Du32.encode(self.sink);
        self
    }

    pub fn i32x4_extadd_pairwise_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Eu32.encode(self.sink);
        self
    }

    pub fn i32x4_extadd_pairwise_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Fu32.encode(self.sink);
        self
    }

    pub fn i16x8_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x80u32.encode(self.sink);
        self
    }

    pub fn i16x8_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x81u32.encode(self.sink);
        self
    }

    pub fn i16x8_q15mulr_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x82u32.encode(self.sink);
        self
    }

    pub fn i16x8_all_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x83u32.encode(self.sink);
        self
    }

    pub fn i16x8_bitmask(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x84u32.encode(self.sink);
        self
    }

    pub fn i16x8_narrow_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x85u32.encode(self.sink);
        self
    }

    pub fn i16x8_narrow_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x86u32.encode(self.sink);
        self
    }

    pub fn i16x8_extend_low_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x87u32.encode(self.sink);
        self
    }

    pub fn i16x8_extend_high_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x88u32.encode(self.sink);
        self
    }

    pub fn i16x8_extend_low_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x89u32.encode(self.sink);
        self
    }

    pub fn i16x8_extend_high_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Au32.encode(self.sink);
        self
    }

    pub fn i16x8_shl(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Bu32.encode(self.sink);
        self
    }

    pub fn i16x8_shr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Cu32.encode(self.sink);
        self
    }

    pub fn i16x8_shr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Du32.encode(self.sink);
        self
    }

    pub fn i16x8_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Eu32.encode(self.sink);
        self
    }

    pub fn i16x8_add_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Fu32.encode(self.sink);
        self
    }

    pub fn i16x8_add_sat_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x90u32.encode(self.sink);
        self
    }

    pub fn i16x8_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x91u32.encode(self.sink);
        self
    }

    pub fn i16x8_sub_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x92u32.encode(self.sink);
        self
    }

    pub fn i16x8_sub_sat_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x93u32.encode(self.sink);
        self
    }

    pub fn i16x8_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x95u32.encode(self.sink);
        self
    }

    pub fn i16x8_min_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x96u32.encode(self.sink);
        self
    }

    pub fn i16x8_min_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x97u32.encode(self.sink);
        self
    }

    pub fn i16x8_max_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x98u32.encode(self.sink);
        self
    }

    pub fn i16x8_max_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x99u32.encode(self.sink);
        self
    }

    pub fn i16x8_avgr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Bu32.encode(self.sink);
        self
    }

    pub fn i16x8_extmul_low_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Cu32.encode(self.sink);
        self
    }

    pub fn i16x8_extmul_high_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Du32.encode(self.sink);
        self
    }

    pub fn i16x8_extmul_low_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Eu32.encode(self.sink);
        self
    }

    pub fn i16x8_extmul_high_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Fu32.encode(self.sink);
        self
    }

    pub fn i32x4_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA0u32.encode(self.sink);
        self
    }

    pub fn i32x4_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA1u32.encode(self.sink);
        self
    }

    pub fn i32x4_all_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA3u32.encode(self.sink);
        self
    }

    pub fn i32x4_bitmask(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA4u32.encode(self.sink);
        self
    }

    pub fn i32x4_extend_low_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA7u32.encode(self.sink);
        self
    }

    pub fn i32x4_extend_high_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA8u32.encode(self.sink);
        self
    }

    pub fn i32x4_extend_low_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA9u32.encode(self.sink);
        self
    }

    pub fn i32x4_extend_high_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xAAu32.encode(self.sink);
        self
    }

    pub fn i32x4_shl(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xABu32.encode(self.sink);
        self
    }

    pub fn i32x4_shr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xACu32.encode(self.sink);
        self
    }

    pub fn i32x4_shr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xADu32.encode(self.sink);
        self
    }

    pub fn i32x4_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xAEu32.encode(self.sink);
        self
    }

    pub fn i32x4_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB1u32.encode(self.sink);
        self
    }

    pub fn i32x4_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB5u32.encode(self.sink);
        self
    }

    pub fn i32x4_min_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB6u32.encode(self.sink);
        self
    }

    pub fn i32x4_min_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB7u32.encode(self.sink);
        self
    }

    pub fn i32x4_max_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB8u32.encode(self.sink);
        self
    }

    pub fn i32x4_max_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB9u32.encode(self.sink);
        self
    }

    pub fn i32x4_dot_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBAu32.encode(self.sink);
        self
    }

    pub fn i32x4_extmul_low_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBCu32.encode(self.sink);
        self
    }

    pub fn i32x4_extmul_high_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBDu32.encode(self.sink);
        self
    }

    pub fn i32x4_extmul_low_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBEu32.encode(self.sink);
        self
    }

    pub fn i32x4_extmul_high_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBFu32.encode(self.sink);
        self
    }

    pub fn i64x2_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC0u32.encode(self.sink);
        self
    }

    pub fn i64x2_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC1u32.encode(self.sink);
        self
    }

    pub fn i64x2_all_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC3u32.encode(self.sink);
        self
    }

    pub fn i64x2_bitmask(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC4u32.encode(self.sink);
        self
    }

    pub fn i64x2_extend_low_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC7u32.encode(self.sink);
        self
    }

    pub fn i64x2_extend_high_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC8u32.encode(self.sink);
        self
    }

    pub fn i64x2_extend_low_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC9u32.encode(self.sink);
        self
    }

    pub fn i64x2_extend_high_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCAu32.encode(self.sink);
        self
    }

    pub fn i64x2_shl(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCBu32.encode(self.sink);
        self
    }

    pub fn i64x2_shr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCCu32.encode(self.sink);
        self
    }

    pub fn i64x2_shr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCDu32.encode(self.sink);
        self
    }

    pub fn i64x2_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCEu32.encode(self.sink);
        self
    }

    pub fn i64x2_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD1u32.encode(self.sink);
        self
    }

    pub fn i64x2_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD5u32.encode(self.sink);
        self
    }

    pub fn i64x2_extmul_low_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDCu32.encode(self.sink);
        self
    }

    pub fn i64x2_extmul_high_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDDu32.encode(self.sink);
        self
    }

    pub fn i64x2_extmul_low_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDEu32.encode(self.sink);
        self
    }

    pub fn i64x2_extmul_high_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDFu32.encode(self.sink);
        self
    }

    pub fn f32x4_ceil(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x67u32.encode(self.sink);
        self
    }

    pub fn f32x4_floor(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x68u32.encode(self.sink);
        self
    }

    pub fn f32x4_trunc(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x69u32.encode(self.sink);
        self
    }

    pub fn f32x4_nearest(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6Au32.encode(self.sink);
        self
    }

    pub fn f32x4_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE0u32.encode(self.sink);
        self
    }

    pub fn f32x4_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE1u32.encode(self.sink);
        self
    }

    pub fn f32x4_sqrt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE3u32.encode(self.sink);
        self
    }

    pub fn f32x4_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE4u32.encode(self.sink);
        self
    }

    pub fn f32x4_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE5u32.encode(self.sink);
        self
    }

    pub fn f32x4_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE6u32.encode(self.sink);
        self
    }

    pub fn f32x4_div(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE7u32.encode(self.sink);
        self
    }

    pub fn f32x4_min(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE8u32.encode(self.sink);
        self
    }

    pub fn f32x4_max(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE9u32.encode(self.sink);
        self
    }

    pub fn f32x4_pmin(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xEAu32.encode(self.sink);
        self
    }

    pub fn f32x4_pmax(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xEBu32.encode(self.sink);
        self
    }

    pub fn f64x2_ceil(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x74u32.encode(self.sink);
        self
    }

    pub fn f64x2_floor(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x75u32.encode(self.sink);
        self
    }

    pub fn f64x2_trunc(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Au32.encode(self.sink);
        self
    }

    pub fn f64x2_nearest(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x94u32.encode(self.sink);
        self
    }

    pub fn f64x2_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xECu32.encode(self.sink);
        self
    }

    pub fn f64x2_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xEDu32.encode(self.sink);
        self
    }

    pub fn f64x2_sqrt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xEFu32.encode(self.sink);
        self
    }

    pub fn f64x2_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF0u32.encode(self.sink);
        self
    }

    pub fn f64x2_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF1u32.encode(self.sink);
        self
    }

    pub fn f64x2_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF2u32.encode(self.sink);
        self
    }

    pub fn f64x2_div(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF3u32.encode(self.sink);
        self
    }

    pub fn f64x2_min(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF4u32.encode(self.sink);
        self
    }

    pub fn f64x2_max(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF5u32.encode(self.sink);
        self
    }

    pub fn f64x2_pmin(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF6u32.encode(self.sink);
        self
    }

    pub fn f64x2_pmax(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF7u32.encode(self.sink);
        self
    }

    pub fn i32x4_trunc_sat_f32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF8u32.encode(self.sink);
        self
    }

    pub fn i32x4_trunc_sat_f32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF9u32.encode(self.sink);
        self
    }

    pub fn f32x4_convert_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFAu32.encode(self.sink);
        self
    }

    pub fn f32x4_convert_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFBu32.encode(self.sink);
        self
    }

    pub fn i32x4_trunc_sat_f64x2_s_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFCu32.encode(self.sink);
        self
    }

    pub fn i32x4_trunc_sat_f64x2_u_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFDu32.encode(self.sink);
        self
    }

    pub fn f64x2_convert_low_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFEu32.encode(self.sink);
        self
    }

    pub fn f64x2_convert_low_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFFu32.encode(self.sink);
        self
    }

    pub fn f32x4_demote_f64x2_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x5Eu32.encode(self.sink);
        self
    }

    pub fn f64x2_promote_low_f32x4(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x5Fu32.encode(self.sink);
        self
    }

    pub fn v128_load32_zero(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x5Cu32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load64_zero(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x5Du32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    pub fn v128_load8_lane(&mut self, memarg: MemArg, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x54u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    pub fn v128_load16_lane(&mut self, memarg: MemArg, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x55u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    pub fn v128_load32_lane(&mut self, memarg: MemArg, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x56u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    pub fn v128_load64_lane(&mut self, memarg: MemArg, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x57u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    pub fn v128_store8_lane(&mut self, memarg: MemArg, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x58u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    pub fn v128_store16_lane(&mut self, memarg: MemArg, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x59u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    pub fn v128_store32_lane(&mut self, memarg: MemArg, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x5Au32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    pub fn v128_store64_lane(&mut self, memarg: MemArg, lane: u8) -> &mut Self {
        self.sink.push(0xFD);
        0x5Bu32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    pub fn i64x2_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD6u32.encode(self.sink);
        self
    }

    pub fn i64x2_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD7u32.encode(self.sink);
        self
    }

    pub fn i64x2_lt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD8u32.encode(self.sink);
        self
    }

    pub fn i64x2_gt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD9u32.encode(self.sink);
        self
    }

    pub fn i64x2_le_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDAu32.encode(self.sink);
        self
    }

    pub fn i64x2_ge_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDBu32.encode(self.sink);
        self
    }

    pub fn i8x16_relaxed_swizzle(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x100u32.encode(self.sink);
        self
    }

    pub fn i32x4_relaxed_trunc_f32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x101u32.encode(self.sink);
        self
    }

    pub fn i32x4_relaxed_trunc_f32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x102u32.encode(self.sink);
        self
    }

    pub fn i32x4_relaxed_trunc_f64x2_s_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x103u32.encode(self.sink);
        self
    }

    pub fn i32x4_relaxed_trunc_f64x2_u_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x104u32.encode(self.sink);
        self
    }

    pub fn f32x4_relaxed_madd(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x105u32.encode(self.sink);
        self
    }

    pub fn f32x4_relaxed_nmadd(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x106u32.encode(self.sink);
        self
    }

    pub fn f64x2_relaxed_madd(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x107u32.encode(self.sink);
        self
    }

    pub fn f64x2_relaxed_nmadd(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x108u32.encode(self.sink);
        self
    }

    pub fn i8x16_relaxed_laneselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x109u32.encode(self.sink);
        self
    }

    pub fn i16x8_relaxed_laneselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Au32.encode(self.sink);
        self
    }

    pub fn i32x4_relaxed_laneselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Bu32.encode(self.sink);
        self
    }

    pub fn i64x2_relaxed_laneselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Cu32.encode(self.sink);
        self
    }

    pub fn f32x4_relaxed_min(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Du32.encode(self.sink);
        self
    }

    pub fn f32x4_relaxed_max(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Eu32.encode(self.sink);
        self
    }

    pub fn f64x2_relaxed_min(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Fu32.encode(self.sink);
        self
    }

    pub fn f64x2_relaxed_max(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x110u32.encode(self.sink);
        self
    }

    pub fn i16x8_relaxed_q15mulr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x111u32.encode(self.sink);
        self
    }

    pub fn i16x8_relaxed_dot_i8x16_i7x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x112u32.encode(self.sink);
        self
    }

    pub fn i32x4_relaxed_dot_i8x16_i7x16_add_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x113u32.encode(self.sink);
        self
    }

    // Atomic instructions from the thread proposal

    pub fn memory_atomic_notify(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x00);
        memarg.encode(self.sink);
        self
    }

    pub fn memory_atomic_wait32(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x01);
        memarg.encode(self.sink);
        self
    }

    pub fn memory_atomic_wait64(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x02);
        memarg.encode(self.sink);
        self
    }

    pub fn atomic_fence(&mut self) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x03);
        self.sink.push(0x00);
        self
    }

    pub fn i32_atomic_load(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x10);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_load(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x11);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_load8_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x12);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_load16_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x13);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_load8_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x14);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_load16_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x15);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_load32_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x16);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_store(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x17);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_store(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x18);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_store8(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x19);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_store16(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1A);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_store8(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1B);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_store16(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1C);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_store32(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1D);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw_add(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1E);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw_add(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1F);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x20);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x21);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x22);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x23);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x24);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x25);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x26);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x27);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x28);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x29);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2A);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2B);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw_and(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2C);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw_and(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2D);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2E);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2F);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x30);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x31);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x32);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw_or(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x33);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw_or(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x34);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x35);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x36);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x37);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x38);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x39);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3A);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3B);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3C);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3D);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3E);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3F);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x40);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x41);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x42);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x43);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x44);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x45);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x46);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x47);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x48);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x49);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4A);
        memarg.encode(self.sink);
        self
    }

    pub fn i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4B);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4C);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4D);
        memarg.encode(self.sink);
        self
    }

    pub fn i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4E);
        memarg.encode(self.sink);
        self
    }

    // Atomic instructions from the shared-everything-threads proposal

    pub fn global_atomic_get(&mut self, ordering: Ordering, global_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4F);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    pub fn global_atomic_set(&mut self, ordering: Ordering, global_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x50);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    pub fn global_atomic_rmw_add(&mut self, ordering: Ordering, global_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x51);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    pub fn global_atomic_rmw_sub(&mut self, ordering: Ordering, global_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x52);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    pub fn global_atomic_rmw_and(&mut self, ordering: Ordering, global_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x53);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    pub fn global_atomic_rmw_or(&mut self, ordering: Ordering, global_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x54);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    pub fn global_atomic_rmw_xor(&mut self, ordering: Ordering, global_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x55);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    pub fn global_atomic_rmw_xchg(&mut self, ordering: Ordering, global_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x56);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    pub fn global_atomic_rmw_cmpxchg(
        &mut self,
        ordering: Ordering,
        global_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x57);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    pub fn table_atomic_get(&mut self, ordering: Ordering, table_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x58);
        ordering.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    pub fn table_atomic_set(&mut self, ordering: Ordering, table_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x59);
        ordering.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    pub fn table_atomic_rmw_xchg(&mut self, ordering: Ordering, table_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5A);
        ordering.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    pub fn table_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, table_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5B);
        ordering.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_get(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5C);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_get_s(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5D);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_get_u(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5E);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_set(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5F);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_rmw_add(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x60);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_rmw_sub(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x61);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_rmw_and(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x62);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_rmw_or(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x63);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_rmw_xor(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x64);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_rmw_xchg(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x65);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn struct_atomic_rmw_cmpxchg(
        &mut self,
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x66);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    pub fn array_atomic_get(&mut self, ordering: Ordering, array_type_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x67);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_get_s(&mut self, ordering: Ordering, array_type_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x68);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_get_u(&mut self, ordering: Ordering, array_type_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x69);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_set(&mut self, ordering: Ordering, array_type_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6A);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_rmw_add(&mut self, ordering: Ordering, array_type_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6B);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_rmw_sub(&mut self, ordering: Ordering, array_type_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6C);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_rmw_and(&mut self, ordering: Ordering, array_type_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6D);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_rmw_or(&mut self, ordering: Ordering, array_type_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6E);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_rmw_xor(&mut self, ordering: Ordering, array_type_index: u32) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6F);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_rmw_xchg(
        &mut self,
        ordering: Ordering,
        array_type_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x70);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn array_atomic_rmw_cmpxchg(
        &mut self,
        ordering: Ordering,
        array_type_index: u32,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x71);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    pub fn ref_i31_shared(&mut self) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x72);
        self
    }

    pub fn cont_new(&mut self, type_index: u32) -> &mut Self {
        self.sink.push(0xE0);
        type_index.encode(self.sink);
        self
    }

    pub fn cont_bind(&mut self, argument_index: u32, result_index: u32) -> &mut Self {
        self.sink.push(0xE1);
        argument_index.encode(self.sink);
        result_index.encode(self.sink);
        self
    }

    pub fn suspend(&mut self, tag_index: u32) -> &mut Self {
        self.sink.push(0xE2);
        tag_index.encode(self.sink);
        self
    }

    pub fn resume(&mut self, cont_type_index: u32, resume_table: &[Handle]) -> &mut Self {
        self.sink.push(0xE3);
        cont_type_index.encode(self.sink);
        resume_table.encode(self.sink);
        self
    }

    pub fn resume_throw(
        &mut self,
        cont_type_index: u32,
        tag_index: u32,
        resume_table: &[Handle],
    ) -> &mut Self {
        self.sink.push(0xE4);
        cont_type_index.encode(self.sink);
        tag_index.encode(self.sink);
        resume_table.encode(self.sink);
        self
    }

    pub fn switch(&mut self, cont_type_index: u32, tag_index: u32) -> &mut Self {
        self.sink.push(0xE5);
        cont_type_index.encode(self.sink);
        tag_index.encode(self.sink);
        self
    }

    pub fn i64_add128(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        19u32.encode(self.sink);
        self
    }

    pub fn i64_sub128(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        20u32.encode(self.sink);
        self
    }

    pub fn i64_mul_wide_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        21u32.encode(self.sink);
        self
    }

    pub fn i64_mul_wide_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        22u32.encode(self.sink);
        self
    }
}
