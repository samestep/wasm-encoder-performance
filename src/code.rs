// Control instructions.
Instruction::Unreachable => sink.push(0x00),
Instruction::Nop => sink.push(0x01),
Instruction::Block(bt) => {
    sink.push(0x02);
    bt.encode(sink);
}
Instruction::Loop(bt) => {
    sink.push(0x03);
    bt.encode(sink);
}
Instruction::If(bt) => {
    sink.push(0x04);
    bt.encode(sink);
}
Instruction::Else => sink.push(0x05),
Instruction::Try(bt) => {
    sink.push(0x06);
    bt.encode(sink);
}
Instruction::Catch(t) => {
    sink.push(0x07);
    t.encode(sink);
}
Instruction::Throw(t) => {
    sink.push(0x08);
    t.encode(sink);
}
Instruction::Rethrow(l) => {
    sink.push(0x09);
    l.encode(sink);
}
Instruction::ThrowRef => {
    sink.push(0x0A);
}
Instruction::End => sink.push(0x0B),
Instruction::Br(l) => {
    sink.push(0x0C);
    l.encode(sink);
}
Instruction::BrIf(l) => {
    sink.push(0x0D);
    l.encode(sink);
}
Instruction::BrTable(ref ls, l) => {
    sink.push(0x0E);
    ls.encode(sink);
    l.encode(sink);
}
Instruction::BrOnNull(l) => {
    sink.push(0xD5);
    l.encode(sink);
}
Instruction::BrOnNonNull(l) => {
    sink.push(0xD6);
    l.encode(sink);
}
Instruction::Return => sink.push(0x0F),
Instruction::Call(f) => {
    sink.push(0x10);
    f.encode(sink);
}
Instruction::CallRef(ty) => {
    sink.push(0x14);
    ty.encode(sink);
}
Instruction::CallIndirect {
    type_index,
    table_index,
} => {
    sink.push(0x11);
    type_index.encode(sink);
    table_index.encode(sink);
}
Instruction::ReturnCallRef(ty) => {
    sink.push(0x15);
    ty.encode(sink);
}

Instruction::ReturnCall(f) => {
    sink.push(0x12);
    f.encode(sink);
}
Instruction::ReturnCallIndirect {
    type_index,
    table_index,
} => {
    sink.push(0x13);
    type_index.encode(sink);
    table_index.encode(sink);
}
Instruction::Delegate(l) => {
    sink.push(0x18);
    l.encode(sink);
}
Instruction::CatchAll => {
    sink.push(0x19);
}

// Parametric instructions.
Instruction::Drop => sink.push(0x1A),
Instruction::Select => sink.push(0x1B),
Instruction::TypedSelect(ty) => {
    sink.push(0x1c);
    [ty].encode(sink);
}

Instruction::TryTable(ty, ref catches) => {
    sink.push(0x1f);
    ty.encode(sink);
    catches.encode(sink);
}

// Variable instructions.
Instruction::LocalGet(l) => {
    sink.push(0x20);
    l.encode(sink);
}
Instruction::LocalSet(l) => {
    sink.push(0x21);
    l.encode(sink);
}
Instruction::LocalTee(l) => {
    sink.push(0x22);
    l.encode(sink);
}
Instruction::GlobalGet(g) => {
    sink.push(0x23);
    g.encode(sink);
}
Instruction::GlobalSet(g) => {
    sink.push(0x24);
    g.encode(sink);
}
Instruction::TableGet(table) => {
    sink.push(0x25);
    table.encode(sink);
}
Instruction::TableSet(table) => {
    sink.push(0x26);
    table.encode(sink);
}

// Memory instructions.
Instruction::I32Load(m) => {
    sink.push(0x28);
    m.encode(sink);
}
Instruction::I64Load(m) => {
    sink.push(0x29);
    m.encode(sink);
}
Instruction::F32Load(m) => {
    sink.push(0x2A);
    m.encode(sink);
}
Instruction::F64Load(m) => {
    sink.push(0x2B);
    m.encode(sink);
}
Instruction::I32Load8S(m) => {
    sink.push(0x2C);
    m.encode(sink);
}
Instruction::I32Load8U(m) => {
    sink.push(0x2D);
    m.encode(sink);
}
Instruction::I32Load16S(m) => {
    sink.push(0x2E);
    m.encode(sink);
}
Instruction::I32Load16U(m) => {
    sink.push(0x2F);
    m.encode(sink);
}
Instruction::I64Load8S(m) => {
    sink.push(0x30);
    m.encode(sink);
}
Instruction::I64Load8U(m) => {
    sink.push(0x31);
    m.encode(sink);
}
Instruction::I64Load16S(m) => {
    sink.push(0x32);
    m.encode(sink);
}
Instruction::I64Load16U(m) => {
    sink.push(0x33);
    m.encode(sink);
}
Instruction::I64Load32S(m) => {
    sink.push(0x34);
    m.encode(sink);
}
Instruction::I64Load32U(m) => {
    sink.push(0x35);
    m.encode(sink);
}
Instruction::I32Store(m) => {
    sink.push(0x36);
    m.encode(sink);
}
Instruction::I64Store(m) => {
    sink.push(0x37);
    m.encode(sink);
}
Instruction::F32Store(m) => {
    sink.push(0x38);
    m.encode(sink);
}
Instruction::F64Store(m) => {
    sink.push(0x39);
    m.encode(sink);
}
Instruction::I32Store8(m) => {
    sink.push(0x3A);
    m.encode(sink);
}
Instruction::I32Store16(m) => {
    sink.push(0x3B);
    m.encode(sink);
}
Instruction::I64Store8(m) => {
    sink.push(0x3C);
    m.encode(sink);
}
Instruction::I64Store16(m) => {
    sink.push(0x3D);
    m.encode(sink);
}
Instruction::I64Store32(m) => {
    sink.push(0x3E);
    m.encode(sink);
}
Instruction::MemorySize(i) => {
    sink.push(0x3F);
    i.encode(sink);
}
Instruction::MemoryGrow(i) => {
    sink.push(0x40);
    i.encode(sink);
}
Instruction::MemoryInit { mem, data_index } => {
    sink.push(0xfc);
    sink.push(0x08);
    data_index.encode(sink);
    mem.encode(sink);
}
Instruction::DataDrop(data) => {
    sink.push(0xfc);
    sink.push(0x09);
    data.encode(sink);
}
Instruction::MemoryCopy { src_mem, dst_mem } => {
    sink.push(0xfc);
    sink.push(0x0a);
    dst_mem.encode(sink);
    src_mem.encode(sink);
}
Instruction::MemoryFill(mem) => {
    sink.push(0xfc);
    sink.push(0x0b);
    mem.encode(sink);
}
Instruction::MemoryDiscard(mem) => {
    sink.push(0xfc);
    sink.push(0x12);
    mem.encode(sink);
}

// Numeric instructions.
Instruction::I32Const(x) => {
    sink.push(0x41);
    x.encode(sink);
}
Instruction::I64Const(x) => {
    sink.push(0x42);
    x.encode(sink);
}
Instruction::F32Const(x) => {
    sink.push(0x43);
    let x = x.to_bits();
    sink.extend(x.to_le_bytes().iter().copied());
}
Instruction::F64Const(x) => {
    sink.push(0x44);
    let x = x.to_bits();
    sink.extend(x.to_le_bytes().iter().copied());
}
Instruction::I32Eqz => sink.push(0x45),
Instruction::I32Eq => sink.push(0x46),
Instruction::I32Ne => sink.push(0x47),
Instruction::I32LtS => sink.push(0x48),
Instruction::I32LtU => sink.push(0x49),
Instruction::I32GtS => sink.push(0x4A),
Instruction::I32GtU => sink.push(0x4B),
Instruction::I32LeS => sink.push(0x4C),
Instruction::I32LeU => sink.push(0x4D),
Instruction::I32GeS => sink.push(0x4E),
Instruction::I32GeU => sink.push(0x4F),
Instruction::I64Eqz => sink.push(0x50),
Instruction::I64Eq => sink.push(0x51),
Instruction::I64Ne => sink.push(0x52),
Instruction::I64LtS => sink.push(0x53),
Instruction::I64LtU => sink.push(0x54),
Instruction::I64GtS => sink.push(0x55),
Instruction::I64GtU => sink.push(0x56),
Instruction::I64LeS => sink.push(0x57),
Instruction::I64LeU => sink.push(0x58),
Instruction::I64GeS => sink.push(0x59),
Instruction::I64GeU => sink.push(0x5A),
Instruction::F32Eq => sink.push(0x5B),
Instruction::F32Ne => sink.push(0x5C),
Instruction::F32Lt => sink.push(0x5D),
Instruction::F32Gt => sink.push(0x5E),
Instruction::F32Le => sink.push(0x5F),
Instruction::F32Ge => sink.push(0x60),
Instruction::F64Eq => sink.push(0x61),
Instruction::F64Ne => sink.push(0x62),
Instruction::F64Lt => sink.push(0x63),
Instruction::F64Gt => sink.push(0x64),
Instruction::F64Le => sink.push(0x65),
Instruction::F64Ge => sink.push(0x66),
Instruction::I32Clz => sink.push(0x67),
Instruction::I32Ctz => sink.push(0x68),
Instruction::I32Popcnt => sink.push(0x69),
Instruction::I32Add => sink.push(0x6A),
Instruction::I32Sub => sink.push(0x6B),
Instruction::I32Mul => sink.push(0x6C),
Instruction::I32DivS => sink.push(0x6D),
Instruction::I32DivU => sink.push(0x6E),
Instruction::I32RemS => sink.push(0x6F),
Instruction::I32RemU => sink.push(0x70),
Instruction::I32And => sink.push(0x71),
Instruction::I32Or => sink.push(0x72),
Instruction::I32Xor => sink.push(0x73),
Instruction::I32Shl => sink.push(0x74),
Instruction::I32ShrS => sink.push(0x75),
Instruction::I32ShrU => sink.push(0x76),
Instruction::I32Rotl => sink.push(0x77),
Instruction::I32Rotr => sink.push(0x78),
Instruction::I64Clz => sink.push(0x79),
Instruction::I64Ctz => sink.push(0x7A),
Instruction::I64Popcnt => sink.push(0x7B),
Instruction::I64Add => sink.push(0x7C),
Instruction::I64Sub => sink.push(0x7D),
Instruction::I64Mul => sink.push(0x7E),
Instruction::I64DivS => sink.push(0x7F),
Instruction::I64DivU => sink.push(0x80),
Instruction::I64RemS => sink.push(0x81),
Instruction::I64RemU => sink.push(0x82),
Instruction::I64And => sink.push(0x83),
Instruction::I64Or => sink.push(0x84),
Instruction::I64Xor => sink.push(0x85),
Instruction::I64Shl => sink.push(0x86),
Instruction::I64ShrS => sink.push(0x87),
Instruction::I64ShrU => sink.push(0x88),
Instruction::I64Rotl => sink.push(0x89),
Instruction::I64Rotr => sink.push(0x8A),
Instruction::F32Abs => sink.push(0x8B),
Instruction::F32Neg => sink.push(0x8C),
Instruction::F32Ceil => sink.push(0x8D),
Instruction::F32Floor => sink.push(0x8E),
Instruction::F32Trunc => sink.push(0x8F),
Instruction::F32Nearest => sink.push(0x90),
Instruction::F32Sqrt => sink.push(0x91),
Instruction::F32Add => sink.push(0x92),
Instruction::F32Sub => sink.push(0x93),
Instruction::F32Mul => sink.push(0x94),
Instruction::F32Div => sink.push(0x95),
Instruction::F32Min => sink.push(0x96),
Instruction::F32Max => sink.push(0x97),
Instruction::F32Copysign => sink.push(0x98),
Instruction::F64Abs => sink.push(0x99),
Instruction::F64Neg => sink.push(0x9A),
Instruction::F64Ceil => sink.push(0x9B),
Instruction::F64Floor => sink.push(0x9C),
Instruction::F64Trunc => sink.push(0x9D),
Instruction::F64Nearest => sink.push(0x9E),
Instruction::F64Sqrt => sink.push(0x9F),
Instruction::F64Add => sink.push(0xA0),
Instruction::F64Sub => sink.push(0xA1),
Instruction::F64Mul => sink.push(0xA2),
Instruction::F64Div => sink.push(0xA3),
Instruction::F64Min => sink.push(0xA4),
Instruction::F64Max => sink.push(0xA5),
Instruction::F64Copysign => sink.push(0xA6),
Instruction::I32WrapI64 => sink.push(0xA7),
Instruction::I32TruncF32S => sink.push(0xA8),
Instruction::I32TruncF32U => sink.push(0xA9),
Instruction::I32TruncF64S => sink.push(0xAA),
Instruction::I32TruncF64U => sink.push(0xAB),
Instruction::I64ExtendI32S => sink.push(0xAC),
Instruction::I64ExtendI32U => sink.push(0xAD),
Instruction::I64TruncF32S => sink.push(0xAE),
Instruction::I64TruncF32U => sink.push(0xAF),
Instruction::I64TruncF64S => sink.push(0xB0),
Instruction::I64TruncF64U => sink.push(0xB1),
Instruction::F32ConvertI32S => sink.push(0xB2),
Instruction::F32ConvertI32U => sink.push(0xB3),
Instruction::F32ConvertI64S => sink.push(0xB4),
Instruction::F32ConvertI64U => sink.push(0xB5),
Instruction::F32DemoteF64 => sink.push(0xB6),
Instruction::F64ConvertI32S => sink.push(0xB7),
Instruction::F64ConvertI32U => sink.push(0xB8),
Instruction::F64ConvertI64S => sink.push(0xB9),
Instruction::F64ConvertI64U => sink.push(0xBA),
Instruction::F64PromoteF32 => sink.push(0xBB),
Instruction::I32ReinterpretF32 => sink.push(0xBC),
Instruction::I64ReinterpretF64 => sink.push(0xBD),
Instruction::F32ReinterpretI32 => sink.push(0xBE),
Instruction::F64ReinterpretI64 => sink.push(0xBF),
Instruction::I32Extend8S => sink.push(0xC0),
Instruction::I32Extend16S => sink.push(0xC1),
Instruction::I64Extend8S => sink.push(0xC2),
Instruction::I64Extend16S => sink.push(0xC3),
Instruction::I64Extend32S => sink.push(0xC4),

Instruction::I32TruncSatF32S => {
    sink.push(0xFC);
    sink.push(0x00);
}
Instruction::I32TruncSatF32U => {
    sink.push(0xFC);
    sink.push(0x01);
}
Instruction::I32TruncSatF64S => {
    sink.push(0xFC);
    sink.push(0x02);
}
Instruction::I32TruncSatF64U => {
    sink.push(0xFC);
    sink.push(0x03);
}
Instruction::I64TruncSatF32S => {
    sink.push(0xFC);
    sink.push(0x04);
}
Instruction::I64TruncSatF32U => {
    sink.push(0xFC);
    sink.push(0x05);
}
Instruction::I64TruncSatF64S => {
    sink.push(0xFC);
    sink.push(0x06);
}
Instruction::I64TruncSatF64U => {
    sink.push(0xFC);
    sink.push(0x07);
}

// Reference types instructions.
Instruction::RefNull(ty) => {
    sink.push(0xd0);
    ty.encode(sink);
}
Instruction::RefIsNull => sink.push(0xd1),
Instruction::RefFunc(f) => {
    sink.push(0xd2);
    f.encode(sink);
}
Instruction::RefEq => sink.push(0xd3),
Instruction::RefAsNonNull => sink.push(0xd4),

// GC instructions.
Instruction::StructNew(type_index) => {
    sink.push(0xfb);
    sink.push(0x00);
    type_index.encode(sink);
}
Instruction::StructNewDefault(type_index) => {
    sink.push(0xfb);
    sink.push(0x01);
    type_index.encode(sink);
}
Instruction::StructGet {
    struct_type_index,
    field_index,
} => {
    sink.push(0xfb);
    sink.push(0x02);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructGetS {
    struct_type_index,
    field_index,
} => {
    sink.push(0xfb);
    sink.push(0x03);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructGetU {
    struct_type_index,
    field_index,
} => {
    sink.push(0xfb);
    sink.push(0x04);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructSet {
    struct_type_index,
    field_index,
} => {
    sink.push(0xfb);
    sink.push(0x05);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::ArrayNew(type_index) => {
    sink.push(0xfb);
    sink.push(0x06);
    type_index.encode(sink);
}
Instruction::ArrayNewDefault(type_index) => {
    sink.push(0xfb);
    sink.push(0x07);
    type_index.encode(sink);
}
Instruction::ArrayNewFixed {
    array_type_index,
    array_size,
} => {
    sink.push(0xfb);
    sink.push(0x08);
    array_type_index.encode(sink);
    array_size.encode(sink);
}
Instruction::ArrayNewData {
    array_type_index,
    array_data_index,
} => {
    sink.push(0xfb);
    sink.push(0x09);
    array_type_index.encode(sink);
    array_data_index.encode(sink);
}
Instruction::ArrayNewElem {
    array_type_index,
    array_elem_index,
} => {
    sink.push(0xfb);
    sink.push(0x0a);
    array_type_index.encode(sink);
    array_elem_index.encode(sink);
}
Instruction::ArrayGet(type_index) => {
    sink.push(0xfb);
    sink.push(0x0b);
    type_index.encode(sink);
}
Instruction::ArrayGetS(type_index) => {
    sink.push(0xfb);
    sink.push(0x0c);
    type_index.encode(sink);
}
Instruction::ArrayGetU(type_index) => {
    sink.push(0xfb);
    sink.push(0x0d);
    type_index.encode(sink);
}
Instruction::ArraySet(type_index) => {
    sink.push(0xfb);
    sink.push(0x0e);
    type_index.encode(sink);
}
Instruction::ArrayLen => {
    sink.push(0xfb);
    sink.push(0x0f);
}
Instruction::ArrayFill(type_index) => {
    sink.push(0xfb);
    sink.push(0x10);
    type_index.encode(sink);
}
Instruction::ArrayCopy {
    array_type_index_dst,
    array_type_index_src,
} => {
    sink.push(0xfb);
    sink.push(0x11);
    array_type_index_dst.encode(sink);
    array_type_index_src.encode(sink);
}
Instruction::ArrayInitData {
    array_type_index,
    array_data_index,
} => {
    sink.push(0xfb);
    sink.push(0x12);
    array_type_index.encode(sink);
    array_data_index.encode(sink);
}
Instruction::ArrayInitElem {
    array_type_index,
    array_elem_index,
} => {
    sink.push(0xfb);
    sink.push(0x13);
    array_type_index.encode(sink);
    array_elem_index.encode(sink);
}
Instruction::RefTestNonNull(heap_type) => {
    sink.push(0xfb);
    sink.push(0x14);
    heap_type.encode(sink);
}
Instruction::RefTestNullable(heap_type) => {
    sink.push(0xfb);
    sink.push(0x15);
    heap_type.encode(sink);
}
Instruction::RefCastNonNull(heap_type) => {
    sink.push(0xfb);
    sink.push(0x16);
    heap_type.encode(sink);
}
Instruction::RefCastNullable(heap_type) => {
    sink.push(0xfb);
    sink.push(0x17);
    heap_type.encode(sink);
}
Instruction::BrOnCast {
    relative_depth,
    from_ref_type,
    to_ref_type,
} => {
    sink.push(0xfb);
    sink.push(0x18);
    let cast_flags =
        (from_ref_type.nullable as u8) | ((to_ref_type.nullable as u8) << 1);
    sink.push(cast_flags);
    relative_depth.encode(sink);
    from_ref_type.heap_type.encode(sink);
    to_ref_type.heap_type.encode(sink);
}
Instruction::BrOnCastFail {
    relative_depth,
    from_ref_type,
    to_ref_type,
} => {
    sink.push(0xfb);
    sink.push(0x19);
    let cast_flags =
        (from_ref_type.nullable as u8) | ((to_ref_type.nullable as u8) << 1);
    sink.push(cast_flags);
    relative_depth.encode(sink);
    from_ref_type.heap_type.encode(sink);
    to_ref_type.heap_type.encode(sink);
}
Instruction::AnyConvertExtern => {
    sink.push(0xfb);
    sink.push(0x1a);
}
Instruction::ExternConvertAny => {
    sink.push(0xfb);
    sink.push(0x1b);
}
Instruction::RefI31 => {
    sink.push(0xfb);
    sink.push(0x1c);
}
Instruction::I31GetS => {
    sink.push(0xfb);
    sink.push(0x1d);
}
Instruction::I31GetU => {
    sink.push(0xfb);
    sink.push(0x1e);
}

// Bulk memory instructions.
Instruction::TableInit { elem_index, table } => {
    sink.push(0xfc);
    sink.push(0x0c);
    elem_index.encode(sink);
    table.encode(sink);
}
Instruction::ElemDrop(segment) => {
    sink.push(0xfc);
    sink.push(0x0d);
    segment.encode(sink);
}
Instruction::TableCopy {
    src_table,
    dst_table,
} => {
    sink.push(0xfc);
    sink.push(0x0e);
    dst_table.encode(sink);
    src_table.encode(sink);
}
Instruction::TableGrow(table) => {
    sink.push(0xfc);
    sink.push(0x0f);
    table.encode(sink);
}
Instruction::TableSize(table) => {
    sink.push(0xfc);
    sink.push(0x10);
    table.encode(sink);
}
Instruction::TableFill(table) => {
    sink.push(0xfc);
    sink.push(0x11);
    table.encode(sink);
}

// SIMD instructions.
Instruction::V128Load(memarg) => {
    sink.push(0xFD);
    0x00u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load8x8S(memarg) => {
    sink.push(0xFD);
    0x01u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load8x8U(memarg) => {
    sink.push(0xFD);
    0x02u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load16x4S(memarg) => {
    sink.push(0xFD);
    0x03u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load16x4U(memarg) => {
    sink.push(0xFD);
    0x04u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load32x2S(memarg) => {
    sink.push(0xFD);
    0x05u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load32x2U(memarg) => {
    sink.push(0xFD);
    0x06u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load8Splat(memarg) => {
    sink.push(0xFD);
    0x07u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load16Splat(memarg) => {
    sink.push(0xFD);
    0x08u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load32Splat(memarg) => {
    sink.push(0xFD);
    0x09u32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load64Splat(memarg) => {
    sink.push(0xFD);
    0x0Au32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Store(memarg) => {
    sink.push(0xFD);
    0x0Bu32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Const(x) => {
    sink.push(0xFD);
    0x0Cu32.encode(sink);
    sink.extend(x.to_le_bytes().iter().copied());
}
Instruction::I8x16Shuffle(lanes) => {
    sink.push(0xFD);
    0x0Du32.encode(sink);
    assert!(lanes.iter().all(|l: &u8| *l < 32));
    sink.extend(lanes.iter().copied());
}
Instruction::I8x16Swizzle => {
    sink.push(0xFD);
    0x0Eu32.encode(sink);
}
Instruction::I8x16Splat => {
    sink.push(0xFD);
    0x0Fu32.encode(sink);
}
Instruction::I16x8Splat => {
    sink.push(0xFD);
    0x10u32.encode(sink);
}
Instruction::I32x4Splat => {
    sink.push(0xFD);
    0x11u32.encode(sink);
}
Instruction::I64x2Splat => {
    sink.push(0xFD);
    0x12u32.encode(sink);
}
Instruction::F32x4Splat => {
    sink.push(0xFD);
    0x13u32.encode(sink);
}
Instruction::F64x2Splat => {
    sink.push(0xFD);
    0x14u32.encode(sink);
}
Instruction::I8x16ExtractLaneS(lane) => {
    sink.push(0xFD);
    0x15u32.encode(sink);
    assert!(lane < 16);
    sink.push(lane);
}
Instruction::I8x16ExtractLaneU(lane) => {
    sink.push(0xFD);
    0x16u32.encode(sink);
    assert!(lane < 16);
    sink.push(lane);
}
Instruction::I8x16ReplaceLane(lane) => {
    sink.push(0xFD);
    0x17u32.encode(sink);
    assert!(lane < 16);
    sink.push(lane);
}
Instruction::I16x8ExtractLaneS(lane) => {
    sink.push(0xFD);
    0x18u32.encode(sink);
    assert!(lane < 8);
    sink.push(lane);
}
Instruction::I16x8ExtractLaneU(lane) => {
    sink.push(0xFD);
    0x19u32.encode(sink);
    assert!(lane < 8);
    sink.push(lane);
}
Instruction::I16x8ReplaceLane(lane) => {
    sink.push(0xFD);
    0x1Au32.encode(sink);
    assert!(lane < 8);
    sink.push(lane);
}
Instruction::I32x4ExtractLane(lane) => {
    sink.push(0xFD);
    0x1Bu32.encode(sink);
    assert!(lane < 4);
    sink.push(lane);
}
Instruction::I32x4ReplaceLane(lane) => {
    sink.push(0xFD);
    0x1Cu32.encode(sink);
    assert!(lane < 4);
    sink.push(lane);
}
Instruction::I64x2ExtractLane(lane) => {
    sink.push(0xFD);
    0x1Du32.encode(sink);
    assert!(lane < 2);
    sink.push(lane);
}
Instruction::I64x2ReplaceLane(lane) => {
    sink.push(0xFD);
    0x1Eu32.encode(sink);
    assert!(lane < 2);
    sink.push(lane);
}
Instruction::F32x4ExtractLane(lane) => {
    sink.push(0xFD);
    0x1Fu32.encode(sink);
    assert!(lane < 4);
    sink.push(lane);
}
Instruction::F32x4ReplaceLane(lane) => {
    sink.push(0xFD);
    0x20u32.encode(sink);
    assert!(lane < 4);
    sink.push(lane);
}
Instruction::F64x2ExtractLane(lane) => {
    sink.push(0xFD);
    0x21u32.encode(sink);
    assert!(lane < 2);
    sink.push(lane);
}
Instruction::F64x2ReplaceLane(lane) => {
    sink.push(0xFD);
    0x22u32.encode(sink);
    assert!(lane < 2);
    sink.push(lane);
}

Instruction::I8x16Eq => {
    sink.push(0xFD);
    0x23u32.encode(sink);
}
Instruction::I8x16Ne => {
    sink.push(0xFD);
    0x24u32.encode(sink);
}
Instruction::I8x16LtS => {
    sink.push(0xFD);
    0x25u32.encode(sink);
}
Instruction::I8x16LtU => {
    sink.push(0xFD);
    0x26u32.encode(sink);
}
Instruction::I8x16GtS => {
    sink.push(0xFD);
    0x27u32.encode(sink);
}
Instruction::I8x16GtU => {
    sink.push(0xFD);
    0x28u32.encode(sink);
}
Instruction::I8x16LeS => {
    sink.push(0xFD);
    0x29u32.encode(sink);
}
Instruction::I8x16LeU => {
    sink.push(0xFD);
    0x2Au32.encode(sink);
}
Instruction::I8x16GeS => {
    sink.push(0xFD);
    0x2Bu32.encode(sink);
}
Instruction::I8x16GeU => {
    sink.push(0xFD);
    0x2Cu32.encode(sink);
}
Instruction::I16x8Eq => {
    sink.push(0xFD);
    0x2Du32.encode(sink);
}
Instruction::I16x8Ne => {
    sink.push(0xFD);
    0x2Eu32.encode(sink);
}
Instruction::I16x8LtS => {
    sink.push(0xFD);
    0x2Fu32.encode(sink);
}
Instruction::I16x8LtU => {
    sink.push(0xFD);
    0x30u32.encode(sink);
}
Instruction::I16x8GtS => {
    sink.push(0xFD);
    0x31u32.encode(sink);
}
Instruction::I16x8GtU => {
    sink.push(0xFD);
    0x32u32.encode(sink);
}
Instruction::I16x8LeS => {
    sink.push(0xFD);
    0x33u32.encode(sink);
}
Instruction::I16x8LeU => {
    sink.push(0xFD);
    0x34u32.encode(sink);
}
Instruction::I16x8GeS => {
    sink.push(0xFD);
    0x35u32.encode(sink);
}
Instruction::I16x8GeU => {
    sink.push(0xFD);
    0x36u32.encode(sink);
}
Instruction::I32x4Eq => {
    sink.push(0xFD);
    0x37u32.encode(sink);
}
Instruction::I32x4Ne => {
    sink.push(0xFD);
    0x38u32.encode(sink);
}
Instruction::I32x4LtS => {
    sink.push(0xFD);
    0x39u32.encode(sink);
}
Instruction::I32x4LtU => {
    sink.push(0xFD);
    0x3Au32.encode(sink);
}
Instruction::I32x4GtS => {
    sink.push(0xFD);
    0x3Bu32.encode(sink);
}
Instruction::I32x4GtU => {
    sink.push(0xFD);
    0x3Cu32.encode(sink);
}
Instruction::I32x4LeS => {
    sink.push(0xFD);
    0x3Du32.encode(sink);
}
Instruction::I32x4LeU => {
    sink.push(0xFD);
    0x3Eu32.encode(sink);
}
Instruction::I32x4GeS => {
    sink.push(0xFD);
    0x3Fu32.encode(sink);
}
Instruction::I32x4GeU => {
    sink.push(0xFD);
    0x40u32.encode(sink);
}
Instruction::F32x4Eq => {
    sink.push(0xFD);
    0x41u32.encode(sink);
}
Instruction::F32x4Ne => {
    sink.push(0xFD);
    0x42u32.encode(sink);
}
Instruction::F32x4Lt => {
    sink.push(0xFD);
    0x43u32.encode(sink);
}
Instruction::F32x4Gt => {
    sink.push(0xFD);
    0x44u32.encode(sink);
}
Instruction::F32x4Le => {
    sink.push(0xFD);
    0x45u32.encode(sink);
}
Instruction::F32x4Ge => {
    sink.push(0xFD);
    0x46u32.encode(sink);
}
Instruction::F64x2Eq => {
    sink.push(0xFD);
    0x47u32.encode(sink);
}
Instruction::F64x2Ne => {
    sink.push(0xFD);
    0x48u32.encode(sink);
}
Instruction::F64x2Lt => {
    sink.push(0xFD);
    0x49u32.encode(sink);
}
Instruction::F64x2Gt => {
    sink.push(0xFD);
    0x4Au32.encode(sink);
}
Instruction::F64x2Le => {
    sink.push(0xFD);
    0x4Bu32.encode(sink);
}
Instruction::F64x2Ge => {
    sink.push(0xFD);
    0x4Cu32.encode(sink);
}
Instruction::V128Not => {
    sink.push(0xFD);
    0x4Du32.encode(sink);
}
Instruction::V128And => {
    sink.push(0xFD);
    0x4Eu32.encode(sink);
}
Instruction::V128AndNot => {
    sink.push(0xFD);
    0x4Fu32.encode(sink);
}
Instruction::V128Or => {
    sink.push(0xFD);
    0x50u32.encode(sink);
}
Instruction::V128Xor => {
    sink.push(0xFD);
    0x51u32.encode(sink);
}
Instruction::V128Bitselect => {
    sink.push(0xFD);
    0x52u32.encode(sink);
}
Instruction::V128AnyTrue => {
    sink.push(0xFD);
    0x53u32.encode(sink);
}
Instruction::I8x16Abs => {
    sink.push(0xFD);
    0x60u32.encode(sink);
}
Instruction::I8x16Neg => {
    sink.push(0xFD);
    0x61u32.encode(sink);
}
Instruction::I8x16Popcnt => {
    sink.push(0xFD);
    0x62u32.encode(sink);
}
Instruction::I8x16AllTrue => {
    sink.push(0xFD);
    0x63u32.encode(sink);
}
Instruction::I8x16Bitmask => {
    sink.push(0xFD);
    0x64u32.encode(sink);
}
Instruction::I8x16NarrowI16x8S => {
    sink.push(0xFD);
    0x65u32.encode(sink);
}
Instruction::I8x16NarrowI16x8U => {
    sink.push(0xFD);
    0x66u32.encode(sink);
}
Instruction::I8x16Shl => {
    sink.push(0xFD);
    0x6bu32.encode(sink);
}
Instruction::I8x16ShrS => {
    sink.push(0xFD);
    0x6cu32.encode(sink);
}
Instruction::I8x16ShrU => {
    sink.push(0xFD);
    0x6du32.encode(sink);
}
Instruction::I8x16Add => {
    sink.push(0xFD);
    0x6eu32.encode(sink);
}
Instruction::I8x16AddSatS => {
    sink.push(0xFD);
    0x6fu32.encode(sink);
}
Instruction::I8x16AddSatU => {
    sink.push(0xFD);
    0x70u32.encode(sink);
}
Instruction::I8x16Sub => {
    sink.push(0xFD);
    0x71u32.encode(sink);
}
Instruction::I8x16SubSatS => {
    sink.push(0xFD);
    0x72u32.encode(sink);
}
Instruction::I8x16SubSatU => {
    sink.push(0xFD);
    0x73u32.encode(sink);
}
Instruction::I8x16MinS => {
    sink.push(0xFD);
    0x76u32.encode(sink);
}
Instruction::I8x16MinU => {
    sink.push(0xFD);
    0x77u32.encode(sink);
}
Instruction::I8x16MaxS => {
    sink.push(0xFD);
    0x78u32.encode(sink);
}
Instruction::I8x16MaxU => {
    sink.push(0xFD);
    0x79u32.encode(sink);
}
Instruction::I8x16AvgrU => {
    sink.push(0xFD);
    0x7Bu32.encode(sink);
}
Instruction::I16x8ExtAddPairwiseI8x16S => {
    sink.push(0xFD);
    0x7Cu32.encode(sink);
}
Instruction::I16x8ExtAddPairwiseI8x16U => {
    sink.push(0xFD);
    0x7Du32.encode(sink);
}
Instruction::I32x4ExtAddPairwiseI16x8S => {
    sink.push(0xFD);
    0x7Eu32.encode(sink);
}
Instruction::I32x4ExtAddPairwiseI16x8U => {
    sink.push(0xFD);
    0x7Fu32.encode(sink);
}
Instruction::I16x8Abs => {
    sink.push(0xFD);
    0x80u32.encode(sink);
}
Instruction::I16x8Neg => {
    sink.push(0xFD);
    0x81u32.encode(sink);
}
Instruction::I16x8Q15MulrSatS => {
    sink.push(0xFD);
    0x82u32.encode(sink);
}
Instruction::I16x8AllTrue => {
    sink.push(0xFD);
    0x83u32.encode(sink);
}
Instruction::I16x8Bitmask => {
    sink.push(0xFD);
    0x84u32.encode(sink);
}
Instruction::I16x8NarrowI32x4S => {
    sink.push(0xFD);
    0x85u32.encode(sink);
}
Instruction::I16x8NarrowI32x4U => {
    sink.push(0xFD);
    0x86u32.encode(sink);
}
Instruction::I16x8ExtendLowI8x16S => {
    sink.push(0xFD);
    0x87u32.encode(sink);
}
Instruction::I16x8ExtendHighI8x16S => {
    sink.push(0xFD);
    0x88u32.encode(sink);
}
Instruction::I16x8ExtendLowI8x16U => {
    sink.push(0xFD);
    0x89u32.encode(sink);
}
Instruction::I16x8ExtendHighI8x16U => {
    sink.push(0xFD);
    0x8Au32.encode(sink);
}
Instruction::I16x8Shl => {
    sink.push(0xFD);
    0x8Bu32.encode(sink);
}
Instruction::I16x8ShrS => {
    sink.push(0xFD);
    0x8Cu32.encode(sink);
}
Instruction::I16x8ShrU => {
    sink.push(0xFD);
    0x8Du32.encode(sink);
}
Instruction::I16x8Add => {
    sink.push(0xFD);
    0x8Eu32.encode(sink);
}
Instruction::I16x8AddSatS => {
    sink.push(0xFD);
    0x8Fu32.encode(sink);
}
Instruction::I16x8AddSatU => {
    sink.push(0xFD);
    0x90u32.encode(sink);
}
Instruction::I16x8Sub => {
    sink.push(0xFD);
    0x91u32.encode(sink);
}
Instruction::I16x8SubSatS => {
    sink.push(0xFD);
    0x92u32.encode(sink);
}
Instruction::I16x8SubSatU => {
    sink.push(0xFD);
    0x93u32.encode(sink);
}
Instruction::I16x8Mul => {
    sink.push(0xFD);
    0x95u32.encode(sink);
}
Instruction::I16x8MinS => {
    sink.push(0xFD);
    0x96u32.encode(sink);
}
Instruction::I16x8MinU => {
    sink.push(0xFD);
    0x97u32.encode(sink);
}
Instruction::I16x8MaxS => {
    sink.push(0xFD);
    0x98u32.encode(sink);
}
Instruction::I16x8MaxU => {
    sink.push(0xFD);
    0x99u32.encode(sink);
}
Instruction::I16x8AvgrU => {
    sink.push(0xFD);
    0x9Bu32.encode(sink);
}
Instruction::I16x8ExtMulLowI8x16S => {
    sink.push(0xFD);
    0x9Cu32.encode(sink);
}
Instruction::I16x8ExtMulHighI8x16S => {
    sink.push(0xFD);
    0x9Du32.encode(sink);
}
Instruction::I16x8ExtMulLowI8x16U => {
    sink.push(0xFD);
    0x9Eu32.encode(sink);
}
Instruction::I16x8ExtMulHighI8x16U => {
    sink.push(0xFD);
    0x9Fu32.encode(sink);
}
Instruction::I32x4Abs => {
    sink.push(0xFD);
    0xA0u32.encode(sink);
}
Instruction::I32x4Neg => {
    sink.push(0xFD);
    0xA1u32.encode(sink);
}
Instruction::I32x4AllTrue => {
    sink.push(0xFD);
    0xA3u32.encode(sink);
}
Instruction::I32x4Bitmask => {
    sink.push(0xFD);
    0xA4u32.encode(sink);
}
Instruction::I32x4ExtendLowI16x8S => {
    sink.push(0xFD);
    0xA7u32.encode(sink);
}
Instruction::I32x4ExtendHighI16x8S => {
    sink.push(0xFD);
    0xA8u32.encode(sink);
}
Instruction::I32x4ExtendLowI16x8U => {
    sink.push(0xFD);
    0xA9u32.encode(sink);
}
Instruction::I32x4ExtendHighI16x8U => {
    sink.push(0xFD);
    0xAAu32.encode(sink);
}
Instruction::I32x4Shl => {
    sink.push(0xFD);
    0xABu32.encode(sink);
}
Instruction::I32x4ShrS => {
    sink.push(0xFD);
    0xACu32.encode(sink);
}
Instruction::I32x4ShrU => {
    sink.push(0xFD);
    0xADu32.encode(sink);
}
Instruction::I32x4Add => {
    sink.push(0xFD);
    0xAEu32.encode(sink);
}
Instruction::I32x4Sub => {
    sink.push(0xFD);
    0xB1u32.encode(sink);
}
Instruction::I32x4Mul => {
    sink.push(0xFD);
    0xB5u32.encode(sink);
}
Instruction::I32x4MinS => {
    sink.push(0xFD);
    0xB6u32.encode(sink);
}
Instruction::I32x4MinU => {
    sink.push(0xFD);
    0xB7u32.encode(sink);
}
Instruction::I32x4MaxS => {
    sink.push(0xFD);
    0xB8u32.encode(sink);
}
Instruction::I32x4MaxU => {
    sink.push(0xFD);
    0xB9u32.encode(sink);
}
Instruction::I32x4DotI16x8S => {
    sink.push(0xFD);
    0xBAu32.encode(sink);
}
Instruction::I32x4ExtMulLowI16x8S => {
    sink.push(0xFD);
    0xBCu32.encode(sink);
}
Instruction::I32x4ExtMulHighI16x8S => {
    sink.push(0xFD);
    0xBDu32.encode(sink);
}
Instruction::I32x4ExtMulLowI16x8U => {
    sink.push(0xFD);
    0xBEu32.encode(sink);
}
Instruction::I32x4ExtMulHighI16x8U => {
    sink.push(0xFD);
    0xBFu32.encode(sink);
}
Instruction::I64x2Abs => {
    sink.push(0xFD);
    0xC0u32.encode(sink);
}
Instruction::I64x2Neg => {
    sink.push(0xFD);
    0xC1u32.encode(sink);
}
Instruction::I64x2AllTrue => {
    sink.push(0xFD);
    0xC3u32.encode(sink);
}
Instruction::I64x2Bitmask => {
    sink.push(0xFD);
    0xC4u32.encode(sink);
}
Instruction::I64x2ExtendLowI32x4S => {
    sink.push(0xFD);
    0xC7u32.encode(sink);
}
Instruction::I64x2ExtendHighI32x4S => {
    sink.push(0xFD);
    0xC8u32.encode(sink);
}
Instruction::I64x2ExtendLowI32x4U => {
    sink.push(0xFD);
    0xC9u32.encode(sink);
}
Instruction::I64x2ExtendHighI32x4U => {
    sink.push(0xFD);
    0xCAu32.encode(sink);
}
Instruction::I64x2Shl => {
    sink.push(0xFD);
    0xCBu32.encode(sink);
}
Instruction::I64x2ShrS => {
    sink.push(0xFD);
    0xCCu32.encode(sink);
}
Instruction::I64x2ShrU => {
    sink.push(0xFD);
    0xCDu32.encode(sink);
}
Instruction::I64x2Add => {
    sink.push(0xFD);
    0xCEu32.encode(sink);
}
Instruction::I64x2Sub => {
    sink.push(0xFD);
    0xD1u32.encode(sink);
}
Instruction::I64x2Mul => {
    sink.push(0xFD);
    0xD5u32.encode(sink);
}
Instruction::I64x2ExtMulLowI32x4S => {
    sink.push(0xFD);
    0xDCu32.encode(sink);
}
Instruction::I64x2ExtMulHighI32x4S => {
    sink.push(0xFD);
    0xDDu32.encode(sink);
}
Instruction::I64x2ExtMulLowI32x4U => {
    sink.push(0xFD);
    0xDEu32.encode(sink);
}
Instruction::I64x2ExtMulHighI32x4U => {
    sink.push(0xFD);
    0xDFu32.encode(sink);
}
Instruction::F32x4Ceil => {
    sink.push(0xFD);
    0x67u32.encode(sink);
}
Instruction::F32x4Floor => {
    sink.push(0xFD);
    0x68u32.encode(sink);
}
Instruction::F32x4Trunc => {
    sink.push(0xFD);
    0x69u32.encode(sink);
}
Instruction::F32x4Nearest => {
    sink.push(0xFD);
    0x6Au32.encode(sink);
}
Instruction::F32x4Abs => {
    sink.push(0xFD);
    0xE0u32.encode(sink);
}
Instruction::F32x4Neg => {
    sink.push(0xFD);
    0xE1u32.encode(sink);
}
Instruction::F32x4Sqrt => {
    sink.push(0xFD);
    0xE3u32.encode(sink);
}
Instruction::F32x4Add => {
    sink.push(0xFD);
    0xE4u32.encode(sink);
}
Instruction::F32x4Sub => {
    sink.push(0xFD);
    0xE5u32.encode(sink);
}
Instruction::F32x4Mul => {
    sink.push(0xFD);
    0xE6u32.encode(sink);
}
Instruction::F32x4Div => {
    sink.push(0xFD);
    0xE7u32.encode(sink);
}
Instruction::F32x4Min => {
    sink.push(0xFD);
    0xE8u32.encode(sink);
}
Instruction::F32x4Max => {
    sink.push(0xFD);
    0xE9u32.encode(sink);
}
Instruction::F32x4PMin => {
    sink.push(0xFD);
    0xEAu32.encode(sink);
}
Instruction::F32x4PMax => {
    sink.push(0xFD);
    0xEBu32.encode(sink);
}
Instruction::F64x2Ceil => {
    sink.push(0xFD);
    0x74u32.encode(sink);
}
Instruction::F64x2Floor => {
    sink.push(0xFD);
    0x75u32.encode(sink);
}
Instruction::F64x2Trunc => {
    sink.push(0xFD);
    0x7Au32.encode(sink);
}
Instruction::F64x2Nearest => {
    sink.push(0xFD);
    0x94u32.encode(sink);
}
Instruction::F64x2Abs => {
    sink.push(0xFD);
    0xECu32.encode(sink);
}
Instruction::F64x2Neg => {
    sink.push(0xFD);
    0xEDu32.encode(sink);
}
Instruction::F64x2Sqrt => {
    sink.push(0xFD);
    0xEFu32.encode(sink);
}
Instruction::F64x2Add => {
    sink.push(0xFD);
    0xF0u32.encode(sink);
}
Instruction::F64x2Sub => {
    sink.push(0xFD);
    0xF1u32.encode(sink);
}
Instruction::F64x2Mul => {
    sink.push(0xFD);
    0xF2u32.encode(sink);
}
Instruction::F64x2Div => {
    sink.push(0xFD);
    0xF3u32.encode(sink);
}
Instruction::F64x2Min => {
    sink.push(0xFD);
    0xF4u32.encode(sink);
}
Instruction::F64x2Max => {
    sink.push(0xFD);
    0xF5u32.encode(sink);
}
Instruction::F64x2PMin => {
    sink.push(0xFD);
    0xF6u32.encode(sink);
}
Instruction::F64x2PMax => {
    sink.push(0xFD);
    0xF7u32.encode(sink);
}
Instruction::I32x4TruncSatF32x4S => {
    sink.push(0xFD);
    0xF8u32.encode(sink);
}
Instruction::I32x4TruncSatF32x4U => {
    sink.push(0xFD);
    0xF9u32.encode(sink);
}
Instruction::F32x4ConvertI32x4S => {
    sink.push(0xFD);
    0xFAu32.encode(sink);
}
Instruction::F32x4ConvertI32x4U => {
    sink.push(0xFD);
    0xFBu32.encode(sink);
}
Instruction::I32x4TruncSatF64x2SZero => {
    sink.push(0xFD);
    0xFCu32.encode(sink);
}
Instruction::I32x4TruncSatF64x2UZero => {
    sink.push(0xFD);
    0xFDu32.encode(sink);
}
Instruction::F64x2ConvertLowI32x4S => {
    sink.push(0xFD);
    0xFEu32.encode(sink);
}
Instruction::F64x2ConvertLowI32x4U => {
    sink.push(0xFD);
    0xFFu32.encode(sink);
}
Instruction::F32x4DemoteF64x2Zero => {
    sink.push(0xFD);
    0x5Eu32.encode(sink);
}
Instruction::F64x2PromoteLowF32x4 => {
    sink.push(0xFD);
    0x5Fu32.encode(sink);
}
Instruction::V128Load32Zero(memarg) => {
    sink.push(0xFD);
    0x5Cu32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load64Zero(memarg) => {
    sink.push(0xFD);
    0x5Du32.encode(sink);
    memarg.encode(sink);
}
Instruction::V128Load8Lane { memarg, lane } => {
    sink.push(0xFD);
    0x54u32.encode(sink);
    memarg.encode(sink);
    assert!(lane < 16);
    sink.push(lane);
}
Instruction::V128Load16Lane { memarg, lane } => {
    sink.push(0xFD);
    0x55u32.encode(sink);
    memarg.encode(sink);
    assert!(lane < 8);
    sink.push(lane);
}
Instruction::V128Load32Lane { memarg, lane } => {
    sink.push(0xFD);
    0x56u32.encode(sink);
    memarg.encode(sink);
    assert!(lane < 4);
    sink.push(lane);
}
Instruction::V128Load64Lane { memarg, lane } => {
    sink.push(0xFD);
    0x57u32.encode(sink);
    memarg.encode(sink);
    assert!(lane < 2);
    sink.push(lane);
}
Instruction::V128Store8Lane { memarg, lane } => {
    sink.push(0xFD);
    0x58u32.encode(sink);
    memarg.encode(sink);
    assert!(lane < 16);
    sink.push(lane);
}
Instruction::V128Store16Lane { memarg, lane } => {
    sink.push(0xFD);
    0x59u32.encode(sink);
    memarg.encode(sink);
    assert!(lane < 8);
    sink.push(lane);
}
Instruction::V128Store32Lane { memarg, lane } => {
    sink.push(0xFD);
    0x5Au32.encode(sink);
    memarg.encode(sink);
    assert!(lane < 4);
    sink.push(lane);
}
Instruction::V128Store64Lane { memarg, lane } => {
    sink.push(0xFD);
    0x5Bu32.encode(sink);
    memarg.encode(sink);
    assert!(lane < 2);
    sink.push(lane);
}
Instruction::I64x2Eq => {
    sink.push(0xFD);
    0xD6u32.encode(sink);
}
Instruction::I64x2Ne => {
    sink.push(0xFD);
    0xD7u32.encode(sink);
}
Instruction::I64x2LtS => {
    sink.push(0xFD);
    0xD8u32.encode(sink);
}
Instruction::I64x2GtS => {
    sink.push(0xFD);
    0xD9u32.encode(sink);
}
Instruction::I64x2LeS => {
    sink.push(0xFD);
    0xDAu32.encode(sink);
}
Instruction::I64x2GeS => {
    sink.push(0xFD);
    0xDBu32.encode(sink);
}
Instruction::I8x16RelaxedSwizzle => {
    sink.push(0xFD);
    0x100u32.encode(sink);
}
Instruction::I32x4RelaxedTruncF32x4S => {
    sink.push(0xFD);
    0x101u32.encode(sink);
}
Instruction::I32x4RelaxedTruncF32x4U => {
    sink.push(0xFD);
    0x102u32.encode(sink);
}
Instruction::I32x4RelaxedTruncF64x2SZero => {
    sink.push(0xFD);
    0x103u32.encode(sink);
}
Instruction::I32x4RelaxedTruncF64x2UZero => {
    sink.push(0xFD);
    0x104u32.encode(sink);
}
Instruction::F32x4RelaxedMadd => {
    sink.push(0xFD);
    0x105u32.encode(sink);
}
Instruction::F32x4RelaxedNmadd => {
    sink.push(0xFD);
    0x106u32.encode(sink);
}
Instruction::F64x2RelaxedMadd => {
    sink.push(0xFD);
    0x107u32.encode(sink);
}
Instruction::F64x2RelaxedNmadd => {
    sink.push(0xFD);
    0x108u32.encode(sink);
}
Instruction::I8x16RelaxedLaneselect => {
    sink.push(0xFD);
    0x109u32.encode(sink);
}
Instruction::I16x8RelaxedLaneselect => {
    sink.push(0xFD);
    0x10Au32.encode(sink);
}
Instruction::I32x4RelaxedLaneselect => {
    sink.push(0xFD);
    0x10Bu32.encode(sink);
}
Instruction::I64x2RelaxedLaneselect => {
    sink.push(0xFD);
    0x10Cu32.encode(sink);
}
Instruction::F32x4RelaxedMin => {
    sink.push(0xFD);
    0x10Du32.encode(sink);
}
Instruction::F32x4RelaxedMax => {
    sink.push(0xFD);
    0x10Eu32.encode(sink);
}
Instruction::F64x2RelaxedMin => {
    sink.push(0xFD);
    0x10Fu32.encode(sink);
}
Instruction::F64x2RelaxedMax => {
    sink.push(0xFD);
    0x110u32.encode(sink);
}
Instruction::I16x8RelaxedQ15mulrS => {
    sink.push(0xFD);
    0x111u32.encode(sink);
}
Instruction::I16x8RelaxedDotI8x16I7x16S => {
    sink.push(0xFD);
    0x112u32.encode(sink);
}
Instruction::I32x4RelaxedDotI8x16I7x16AddS => {
    sink.push(0xFD);
    0x113u32.encode(sink);
}

// Atomic instructions from the thread proposal
Instruction::MemoryAtomicNotify(memarg) => {
    sink.push(0xFE);
    sink.push(0x00);
    memarg.encode(sink);
}
Instruction::MemoryAtomicWait32(memarg) => {
    sink.push(0xFE);
    sink.push(0x01);
    memarg.encode(sink);
}
Instruction::MemoryAtomicWait64(memarg) => {
    sink.push(0xFE);
    sink.push(0x02);
    memarg.encode(sink);
}
Instruction::AtomicFence => {
    sink.push(0xFE);
    sink.push(0x03);
    sink.push(0x00);
}
Instruction::I32AtomicLoad(memarg) => {
    sink.push(0xFE);
    sink.push(0x10);
    memarg.encode(sink);
}
Instruction::I64AtomicLoad(memarg) => {
    sink.push(0xFE);
    sink.push(0x11);
    memarg.encode(sink);
}
Instruction::I32AtomicLoad8U(memarg) => {
    sink.push(0xFE);
    sink.push(0x12);
    memarg.encode(sink);
}
Instruction::I32AtomicLoad16U(memarg) => {
    sink.push(0xFE);
    sink.push(0x13);
    memarg.encode(sink);
}
Instruction::I64AtomicLoad8U(memarg) => {
    sink.push(0xFE);
    sink.push(0x14);
    memarg.encode(sink);
}
Instruction::I64AtomicLoad16U(memarg) => {
    sink.push(0xFE);
    sink.push(0x15);
    memarg.encode(sink);
}
Instruction::I64AtomicLoad32U(memarg) => {
    sink.push(0xFE);
    sink.push(0x16);
    memarg.encode(sink);
}
Instruction::I32AtomicStore(memarg) => {
    sink.push(0xFE);
    sink.push(0x17);
    memarg.encode(sink);
}
Instruction::I64AtomicStore(memarg) => {
    sink.push(0xFE);
    sink.push(0x18);
    memarg.encode(sink);
}
Instruction::I32AtomicStore8(memarg) => {
    sink.push(0xFE);
    sink.push(0x19);
    memarg.encode(sink);
}
Instruction::I32AtomicStore16(memarg) => {
    sink.push(0xFE);
    sink.push(0x1A);
    memarg.encode(sink);
}
Instruction::I64AtomicStore8(memarg) => {
    sink.push(0xFE);
    sink.push(0x1B);
    memarg.encode(sink);
}
Instruction::I64AtomicStore16(memarg) => {
    sink.push(0xFE);
    sink.push(0x1C);
    memarg.encode(sink);
}
Instruction::I64AtomicStore32(memarg) => {
    sink.push(0xFE);
    sink.push(0x1D);
    memarg.encode(sink);
}
Instruction::I32AtomicRmwAdd(memarg) => {
    sink.push(0xFE);
    sink.push(0x1E);
    memarg.encode(sink);
}
Instruction::I64AtomicRmwAdd(memarg) => {
    sink.push(0xFE);
    sink.push(0x1F);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw8AddU(memarg) => {
    sink.push(0xFE);
    sink.push(0x20);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw16AddU(memarg) => {
    sink.push(0xFE);
    sink.push(0x21);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw8AddU(memarg) => {
    sink.push(0xFE);
    sink.push(0x22);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw16AddU(memarg) => {
    sink.push(0xFE);
    sink.push(0x23);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw32AddU(memarg) => {
    sink.push(0xFE);
    sink.push(0x24);
    memarg.encode(sink);
}
Instruction::I32AtomicRmwSub(memarg) => {
    sink.push(0xFE);
    sink.push(0x25);
    memarg.encode(sink);
}
Instruction::I64AtomicRmwSub(memarg) => {
    sink.push(0xFE);
    sink.push(0x26);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw8SubU(memarg) => {
    sink.push(0xFE);
    sink.push(0x27);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw16SubU(memarg) => {
    sink.push(0xFE);
    sink.push(0x28);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw8SubU(memarg) => {
    sink.push(0xFE);
    sink.push(0x29);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw16SubU(memarg) => {
    sink.push(0xFE);
    sink.push(0x2A);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw32SubU(memarg) => {
    sink.push(0xFE);
    sink.push(0x2B);
    memarg.encode(sink);
}
Instruction::I32AtomicRmwAnd(memarg) => {
    sink.push(0xFE);
    sink.push(0x2C);
    memarg.encode(sink);
}
Instruction::I64AtomicRmwAnd(memarg) => {
    sink.push(0xFE);
    sink.push(0x2D);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw8AndU(memarg) => {
    sink.push(0xFE);
    sink.push(0x2E);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw16AndU(memarg) => {
    sink.push(0xFE);
    sink.push(0x2F);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw8AndU(memarg) => {
    sink.push(0xFE);
    sink.push(0x30);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw16AndU(memarg) => {
    sink.push(0xFE);
    sink.push(0x31);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw32AndU(memarg) => {
    sink.push(0xFE);
    sink.push(0x32);
    memarg.encode(sink);
}
Instruction::I32AtomicRmwOr(memarg) => {
    sink.push(0xFE);
    sink.push(0x33);
    memarg.encode(sink);
}
Instruction::I64AtomicRmwOr(memarg) => {
    sink.push(0xFE);
    sink.push(0x34);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw8OrU(memarg) => {
    sink.push(0xFE);
    sink.push(0x35);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw16OrU(memarg) => {
    sink.push(0xFE);
    sink.push(0x36);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw8OrU(memarg) => {
    sink.push(0xFE);
    sink.push(0x37);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw16OrU(memarg) => {
    sink.push(0xFE);
    sink.push(0x38);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw32OrU(memarg) => {
    sink.push(0xFE);
    sink.push(0x39);
    memarg.encode(sink);
}
Instruction::I32AtomicRmwXor(memarg) => {
    sink.push(0xFE);
    sink.push(0x3A);
    memarg.encode(sink);
}
Instruction::I64AtomicRmwXor(memarg) => {
    sink.push(0xFE);
    sink.push(0x3B);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw8XorU(memarg) => {
    sink.push(0xFE);
    sink.push(0x3C);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw16XorU(memarg) => {
    sink.push(0xFE);
    sink.push(0x3D);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw8XorU(memarg) => {
    sink.push(0xFE);
    sink.push(0x3E);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw16XorU(memarg) => {
    sink.push(0xFE);
    sink.push(0x3F);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw32XorU(memarg) => {
    sink.push(0xFE);
    sink.push(0x40);
    memarg.encode(sink);
}
Instruction::I32AtomicRmwXchg(memarg) => {
    sink.push(0xFE);
    sink.push(0x41);
    memarg.encode(sink);
}
Instruction::I64AtomicRmwXchg(memarg) => {
    sink.push(0xFE);
    sink.push(0x42);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw8XchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x43);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw16XchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x44);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw8XchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x45);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw16XchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x46);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw32XchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x47);
    memarg.encode(sink);
}
Instruction::I32AtomicRmwCmpxchg(memarg) => {
    sink.push(0xFE);
    sink.push(0x48);
    memarg.encode(sink);
}
Instruction::I64AtomicRmwCmpxchg(memarg) => {
    sink.push(0xFE);
    sink.push(0x49);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw8CmpxchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x4A);
    memarg.encode(sink);
}
Instruction::I32AtomicRmw16CmpxchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x4B);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw8CmpxchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x4C);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw16CmpxchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x4D);
    memarg.encode(sink);
}
Instruction::I64AtomicRmw32CmpxchgU(memarg) => {
    sink.push(0xFE);
    sink.push(0x4E);
    memarg.encode(sink);
}

// Atomic instructions from the shared-everything-threads proposal
Instruction::GlobalAtomicGet {
    ordering,
    global_index,
} => {
    sink.push(0xFE);
    sink.push(0x4F);
    ordering.encode(sink);
    global_index.encode(sink);
}
Instruction::GlobalAtomicSet {
    ordering,
    global_index,
} => {
    sink.push(0xFE);
    sink.push(0x50);
    ordering.encode(sink);
    global_index.encode(sink);
}
Instruction::GlobalAtomicRmwAdd {
    ordering,
    global_index,
} => {
    sink.push(0xFE);
    sink.push(0x51);
    ordering.encode(sink);
    global_index.encode(sink);
}
Instruction::GlobalAtomicRmwSub {
    ordering,
    global_index,
} => {
    sink.push(0xFE);
    sink.push(0x52);
    ordering.encode(sink);
    global_index.encode(sink);
}
Instruction::GlobalAtomicRmwAnd {
    ordering,
    global_index,
} => {
    sink.push(0xFE);
    sink.push(0x53);
    ordering.encode(sink);
    global_index.encode(sink);
}
Instruction::GlobalAtomicRmwOr {
    ordering,
    global_index,
} => {
    sink.push(0xFE);
    sink.push(0x54);
    ordering.encode(sink);
    global_index.encode(sink);
}
Instruction::GlobalAtomicRmwXor {
    ordering,
    global_index,
} => {
    sink.push(0xFE);
    sink.push(0x55);
    ordering.encode(sink);
    global_index.encode(sink);
}
Instruction::GlobalAtomicRmwXchg {
    ordering,
    global_index,
} => {
    sink.push(0xFE);
    sink.push(0x56);
    ordering.encode(sink);
    global_index.encode(sink);
}
Instruction::GlobalAtomicRmwCmpxchg {
    ordering,
    global_index,
} => {
    sink.push(0xFE);
    sink.push(0x57);
    ordering.encode(sink);
    global_index.encode(sink);
}
Instruction::TableAtomicGet {
    ordering,
    table_index,
} => {
    sink.push(0xFE);
    sink.push(0x58);
    ordering.encode(sink);
    table_index.encode(sink);
}
Instruction::TableAtomicSet {
    ordering,
    table_index,
} => {
    sink.push(0xFE);
    sink.push(0x59);
    ordering.encode(sink);
    table_index.encode(sink);
}
Instruction::TableAtomicRmwXchg {
    ordering,
    table_index,
} => {
    sink.push(0xFE);
    sink.push(0x5A);
    ordering.encode(sink);
    table_index.encode(sink);
}
Instruction::TableAtomicRmwCmpxchg {
    ordering,
    table_index,
} => {
    sink.push(0xFE);
    sink.push(0x5B);
    ordering.encode(sink);
    table_index.encode(sink);
}
Instruction::StructAtomicGet {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x5C);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicGetS {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x5D);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicGetU {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x5E);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicSet {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x5F);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicRmwAdd {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x60);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicRmwSub {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x61);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicRmwAnd {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x62);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicRmwOr {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x63);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicRmwXor {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x64);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicRmwXchg {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x65);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::StructAtomicRmwCmpxchg {
    ordering,
    struct_type_index,
    field_index,
} => {
    sink.push(0xFE);
    sink.push(0x66);
    ordering.encode(sink);
    struct_type_index.encode(sink);
    field_index.encode(sink);
}
Instruction::ArrayAtomicGet {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x67);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicGetS {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x68);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicGetU {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x69);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicSet {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x6A);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicRmwAdd {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x6B);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicRmwSub {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x6C);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicRmwAnd {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x6D);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicRmwOr {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x6E);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicRmwXor {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x6F);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicRmwXchg {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x70);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::ArrayAtomicRmwCmpxchg {
    ordering,
    array_type_index,
} => {
    sink.push(0xFE);
    sink.push(0x71);
    ordering.encode(sink);
    array_type_index.encode(sink);
}
Instruction::RefI31Shared => {
    sink.push(0xFE);
    sink.push(0x72);
}
Instruction::ContNew(type_index) => {
    sink.push(0xE0);
    type_index.encode(sink);
}
Instruction::ContBind {
    argument_index,
    result_index,
} => {
    sink.push(0xE1);
    argument_index.encode(sink);
    result_index.encode(sink);
}
Instruction::Suspend(tag_index) => {
    sink.push(0xE2);
    tag_index.encode(sink);
}
Instruction::Resume {
    cont_type_index,
    ref resume_table,
} => {
    sink.push(0xE3);
    cont_type_index.encode(sink);
    resume_table.encode(sink);
}
Instruction::ResumeThrow {
    cont_type_index,
    tag_index,
    ref resume_table,
} => {
    sink.push(0xE4);
    cont_type_index.encode(sink);
    tag_index.encode(sink);
    resume_table.encode(sink);
}
Instruction::Switch {
    cont_type_index,
    tag_index,
} => {
    sink.push(0xE5);
    cont_type_index.encode(sink);
    tag_index.encode(sink);
}
Instruction::I64Add128 => {
    sink.push(0xFC);
    19u32.encode(sink);
}
Instruction::I64Sub128 => {
    sink.push(0xFC);
    20u32.encode(sink);
}
Instruction::I64MulWideS => {
    sink.push(0xFC);
    21u32.encode(sink);
}
Instruction::I64MulWideU => {
    sink.push(0xFC);
    22u32.encode(sink);
}
