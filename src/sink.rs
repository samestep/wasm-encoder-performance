use wasm_encoder::{
    BlockType, Catch, Encode, Handle, HeapType, Instruction, Lane, MemArg, Ordering, RefType,
    ValType,
};

pub fn encode_instruction(instruction: &Instruction, bytes: &mut Vec<u8>) {
    let mut sink = InstructionSink::new(bytes);
    match *instruction {
        // Control instructions.
        Instruction::Unreachable => sink.unreachable(),
        Instruction::Nop => sink.nop(),
        Instruction::Block(bt) => sink.block(bt),
        Instruction::Loop(bt) => sink.loop_(bt),
        Instruction::If(bt) => sink.if_(bt),
        Instruction::Else => sink.else_(),
        Instruction::Try(bt) => sink.try_(bt),
        Instruction::Catch(t) => sink.catch(TagIdx(t)),
        Instruction::Throw(t) => sink.throw(TagIdx(t)),
        Instruction::Rethrow(l) => sink.rethrow(LabelIdx(l)),
        Instruction::ThrowRef => sink.throw_ref(),
        Instruction::End => sink.end(),
        Instruction::Br(l) => sink.br(LabelIdx(l)),
        Instruction::BrIf(l) => sink.br_if(LabelIdx(l)),
        Instruction::BrTable(ref ls, l) => {
            sink.br_table(ls.iter().copied().map(LabelIdx), LabelIdx(l))
        }
        Instruction::BrOnNull(l) => sink.br_on_null(LabelIdx(l)),
        Instruction::BrOnNonNull(l) => sink.br_on_non_null(LabelIdx(l)),
        Instruction::Return => sink.return_(),
        Instruction::Call(f) => sink.call(FuncIdx(f)),
        Instruction::CallRef(ty) => sink.call_ref(TypeIdx(ty)),
        Instruction::CallIndirect {
            type_index,
            table_index,
        } => sink.call_indirect(TableIdx(table_index), TypeIdx(type_index)),
        Instruction::ReturnCallRef(ty) => sink.return_call_ref(TypeIdx(ty)),

        Instruction::ReturnCall(f) => sink.return_call(FuncIdx(f)),
        Instruction::ReturnCallIndirect {
            type_index,
            table_index,
        } => sink.return_call_indirect(TableIdx(table_index), TypeIdx(type_index)),
        Instruction::Delegate(l) => sink.delegate(LabelIdx(l)),
        Instruction::CatchAll => sink.catch_all(),

        // Parametric instructions.
        Instruction::Drop => sink.drop(),
        Instruction::Select => sink.select(),
        Instruction::TypedSelect(ty) => sink.typed_select(ty),

        Instruction::TryTable(ty, ref catches) => sink.try_table(ty, catches.iter().cloned()),

        // Variable instructions.
        Instruction::LocalGet(l) => sink.local_get(LocalIdx(l)),
        Instruction::LocalSet(l) => sink.local_set(LocalIdx(l)),
        Instruction::LocalTee(l) => sink.local_tee(LocalIdx(l)),
        Instruction::GlobalGet(g) => sink.global_get(GlobalIdx(g)),
        Instruction::GlobalSet(g) => sink.global_set(GlobalIdx(g)),
        Instruction::TableGet(table) => sink.table_get(TableIdx(table)),
        Instruction::TableSet(table) => sink.table_set(TableIdx(table)),

        // Memory instructions.
        Instruction::I32Load(m) => sink.i32_load(m),
        Instruction::I64Load(m) => sink.i64_load(m),
        Instruction::F32Load(m) => sink.f32_load(m),
        Instruction::F64Load(m) => sink.f64_load(m),
        Instruction::I32Load8S(m) => sink.i32_load8_s(m),
        Instruction::I32Load8U(m) => sink.i32_load8_u(m),
        Instruction::I32Load16S(m) => sink.i32_load16_s(m),
        Instruction::I32Load16U(m) => sink.i32_load16_u(m),
        Instruction::I64Load8S(m) => sink.i64_load8_s(m),
        Instruction::I64Load8U(m) => sink.i64_load8_u(m),
        Instruction::I64Load16S(m) => sink.i64_load16_s(m),
        Instruction::I64Load16U(m) => sink.i64_load16_u(m),
        Instruction::I64Load32S(m) => sink.i64_load32_s(m),
        Instruction::I64Load32U(m) => sink.i64_load32_u(m),
        Instruction::I32Store(m) => sink.i32_store(m),
        Instruction::I64Store(m) => sink.i64_store(m),
        Instruction::F32Store(m) => sink.f32_store(m),
        Instruction::F64Store(m) => sink.f64_store(m),
        Instruction::I32Store8(m) => sink.i32_store8(m),
        Instruction::I32Store16(m) => sink.i32_store16(m),
        Instruction::I64Store8(m) => sink.i64_store8(m),
        Instruction::I64Store16(m) => sink.i64_store16(m),
        Instruction::I64Store32(m) => sink.i64_store32(m),
        Instruction::MemorySize(i) => sink.memory_size(MemIdx(i)),
        Instruction::MemoryGrow(i) => sink.memory_grow(MemIdx(i)),
        Instruction::MemoryInit { mem, data_index } => {
            sink.memory_init(MemIdx(mem), DataIdx(data_index))
        }
        Instruction::DataDrop(data) => sink.data_drop(DataIdx(data)),
        Instruction::MemoryCopy { src_mem, dst_mem } => {
            sink.memory_copy(MemIdx(dst_mem), MemIdx(src_mem))
        }
        Instruction::MemoryFill(mem) => sink.memory_fill(MemIdx(mem)),
        Instruction::MemoryDiscard(mem) => sink.memory_discard(MemIdx(mem)),

        // Numeric instructions.
        Instruction::I32Const(x) => sink.i32_const(x),
        Instruction::I64Const(x) => sink.i64_const(x),
        Instruction::F32Const(x) => sink.f32_const(x),
        Instruction::F64Const(x) => sink.f64_const(x),
        Instruction::I32Eqz => sink.i32_eqz(),
        Instruction::I32Eq => sink.i32_eq(),
        Instruction::I32Ne => sink.i32_ne(),
        Instruction::I32LtS => sink.i32_lt_s(),
        Instruction::I32LtU => sink.i32_lt_u(),
        Instruction::I32GtS => sink.i32_gt_s(),
        Instruction::I32GtU => sink.i32_gt_u(),
        Instruction::I32LeS => sink.i32_le_s(),
        Instruction::I32LeU => sink.i32_le_u(),
        Instruction::I32GeS => sink.i32_ge_s(),
        Instruction::I32GeU => sink.i32_ge_u(),
        Instruction::I64Eqz => sink.i64_eqz(),
        Instruction::I64Eq => sink.i64_eq(),
        Instruction::I64Ne => sink.i64_ne(),
        Instruction::I64LtS => sink.i64_lt_s(),
        Instruction::I64LtU => sink.i64_lt_u(),
        Instruction::I64GtS => sink.i64_gt_s(),
        Instruction::I64GtU => sink.i64_gt_u(),
        Instruction::I64LeS => sink.i64_le_s(),
        Instruction::I64LeU => sink.i64_le_u(),
        Instruction::I64GeS => sink.i64_ge_s(),
        Instruction::I64GeU => sink.i64_ge_u(),
        Instruction::F32Eq => sink.f32_eq(),
        Instruction::F32Ne => sink.f32_ne(),
        Instruction::F32Lt => sink.f32_lt(),
        Instruction::F32Gt => sink.f32_gt(),
        Instruction::F32Le => sink.f32_le(),
        Instruction::F32Ge => sink.f32_ge(),
        Instruction::F64Eq => sink.f64_eq(),
        Instruction::F64Ne => sink.f64_ne(),
        Instruction::F64Lt => sink.f64_lt(),
        Instruction::F64Gt => sink.f64_gt(),
        Instruction::F64Le => sink.f64_le(),
        Instruction::F64Ge => sink.f64_ge(),
        Instruction::I32Clz => sink.i32_clz(),
        Instruction::I32Ctz => sink.i32_ctz(),
        Instruction::I32Popcnt => sink.i32_popcnt(),
        Instruction::I32Add => sink.i32_add(),
        Instruction::I32Sub => sink.i32_sub(),
        Instruction::I32Mul => sink.i32_mul(),
        Instruction::I32DivS => sink.i32_div_s(),
        Instruction::I32DivU => sink.i32_div_u(),
        Instruction::I32RemS => sink.i32_rem_s(),
        Instruction::I32RemU => sink.i32_rem_u(),
        Instruction::I32And => sink.i32_and(),
        Instruction::I32Or => sink.i32_or(),
        Instruction::I32Xor => sink.i32_xor(),
        Instruction::I32Shl => sink.i32_shl(),
        Instruction::I32ShrS => sink.i32_shr_s(),
        Instruction::I32ShrU => sink.i32_shr_u(),
        Instruction::I32Rotl => sink.i32_rotl(),
        Instruction::I32Rotr => sink.i32_rotr(),
        Instruction::I64Clz => sink.i64_clz(),
        Instruction::I64Ctz => sink.i64_ctz(),
        Instruction::I64Popcnt => sink.i64_popcnt(),
        Instruction::I64Add => sink.i64_add(),
        Instruction::I64Sub => sink.i64_sub(),
        Instruction::I64Mul => sink.i64_mul(),
        Instruction::I64DivS => sink.i64_div_s(),
        Instruction::I64DivU => sink.i64_div_u(),
        Instruction::I64RemS => sink.i64_rem_s(),
        Instruction::I64RemU => sink.i64_rem_u(),
        Instruction::I64And => sink.i64_and(),
        Instruction::I64Or => sink.i64_or(),
        Instruction::I64Xor => sink.i64_xor(),
        Instruction::I64Shl => sink.i64_shl(),
        Instruction::I64ShrS => sink.i64_shr_s(),
        Instruction::I64ShrU => sink.i64_shr_u(),
        Instruction::I64Rotl => sink.i64_rotl(),
        Instruction::I64Rotr => sink.i64_rotr(),
        Instruction::F32Abs => sink.f32_abs(),
        Instruction::F32Neg => sink.f32_neg(),
        Instruction::F32Ceil => sink.f32_ceil(),
        Instruction::F32Floor => sink.f32_floor(),
        Instruction::F32Trunc => sink.f32_trunc(),
        Instruction::F32Nearest => sink.f32_nearest(),
        Instruction::F32Sqrt => sink.f32_sqrt(),
        Instruction::F32Add => sink.f32_add(),
        Instruction::F32Sub => sink.f32_sub(),
        Instruction::F32Mul => sink.f32_mul(),
        Instruction::F32Div => sink.f32_div(),
        Instruction::F32Min => sink.f32_min(),
        Instruction::F32Max => sink.f32_max(),
        Instruction::F32Copysign => sink.f32_copysign(),
        Instruction::F64Abs => sink.f64_abs(),
        Instruction::F64Neg => sink.f64_neg(),
        Instruction::F64Ceil => sink.f64_ceil(),
        Instruction::F64Floor => sink.f64_floor(),
        Instruction::F64Trunc => sink.f64_trunc(),
        Instruction::F64Nearest => sink.f64_nearest(),
        Instruction::F64Sqrt => sink.f64_sqrt(),
        Instruction::F64Add => sink.f64_add(),
        Instruction::F64Sub => sink.f64_sub(),
        Instruction::F64Mul => sink.f64_mul(),
        Instruction::F64Div => sink.f64_div(),
        Instruction::F64Min => sink.f64_min(),
        Instruction::F64Max => sink.f64_max(),
        Instruction::F64Copysign => sink.f64_copysign(),
        Instruction::I32WrapI64 => sink.i32_wrap_i64(),
        Instruction::I32TruncF32S => sink.i32_trunc_f32_s(),
        Instruction::I32TruncF32U => sink.i32_trunc_f32_u(),
        Instruction::I32TruncF64S => sink.i32_trunc_f64_s(),
        Instruction::I32TruncF64U => sink.i32_trunc_f64_u(),
        Instruction::I64ExtendI32S => sink.i64_extend_i32_s(),
        Instruction::I64ExtendI32U => sink.i64_extend_i32_u(),
        Instruction::I64TruncF32S => sink.i64_trunc_f32_s(),
        Instruction::I64TruncF32U => sink.i64_trunc_f32_u(),
        Instruction::I64TruncF64S => sink.i64_trunc_f64_s(),
        Instruction::I64TruncF64U => sink.i64_trunc_f64_u(),
        Instruction::F32ConvertI32S => sink.f32_convert_i32_s(),
        Instruction::F32ConvertI32U => sink.f32_convert_i32_u(),
        Instruction::F32ConvertI64S => sink.f32_convert_i64_s(),
        Instruction::F32ConvertI64U => sink.f32_convert_i64_u(),
        Instruction::F32DemoteF64 => sink.f32_demote_f64(),
        Instruction::F64ConvertI32S => sink.f64_convert_i32_s(),
        Instruction::F64ConvertI32U => sink.f64_convert_i32_u(),
        Instruction::F64ConvertI64S => sink.f64_convert_i64_s(),
        Instruction::F64ConvertI64U => sink.f64_convert_i64_u(),
        Instruction::F64PromoteF32 => sink.f64_promote_f32(),
        Instruction::I32ReinterpretF32 => sink.i32_reinterpret_f32(),
        Instruction::I64ReinterpretF64 => sink.i64_reinterpret_f64(),
        Instruction::F32ReinterpretI32 => sink.f32_reinterpret_i32(),
        Instruction::F64ReinterpretI64 => sink.f64_reinterpret_i64(),
        Instruction::I32Extend8S => sink.i32_extend8_s(),
        Instruction::I32Extend16S => sink.i32_extend16_s(),
        Instruction::I64Extend8S => sink.i64_extend8_s(),
        Instruction::I64Extend16S => sink.i64_extend16_s(),
        Instruction::I64Extend32S => sink.i64_extend32_s(),

        Instruction::I32TruncSatF32S => sink.i32_trunc_sat_f32_s(),
        Instruction::I32TruncSatF32U => sink.i32_trunc_sat_f32_u(),
        Instruction::I32TruncSatF64S => sink.i32_trunc_sat_f64_s(),
        Instruction::I32TruncSatF64U => sink.i32_trunc_sat_f64_u(),
        Instruction::I64TruncSatF32S => sink.i64_trunc_sat_f32_s(),
        Instruction::I64TruncSatF32U => sink.i64_trunc_sat_f32_u(),
        Instruction::I64TruncSatF64S => sink.i64_trunc_sat_f64_s(),
        Instruction::I64TruncSatF64U => sink.i64_trunc_sat_f64_u(),

        // Reference types instructions.
        Instruction::RefNull(ty) => sink.ref_null(ty),
        Instruction::RefIsNull => sink.ref_is_null(),
        Instruction::RefFunc(f) => sink.ref_func(FuncIdx(f)),
        Instruction::RefEq => sink.ref_eq(),
        Instruction::RefAsNonNull => sink.ref_as_non_null(),

        // GC instructions.
        Instruction::StructNew(type_index) => sink.struct_new(TypeIdx(type_index)),
        Instruction::StructNewDefault(type_index) => sink.struct_new_default(TypeIdx(type_index)),
        Instruction::StructGet {
            struct_type_index,
            field_index,
        } => sink.struct_get(TypeIdx(struct_type_index), FieldIdx(field_index)),
        Instruction::StructGetS {
            struct_type_index,
            field_index,
        } => sink.struct_get_s(TypeIdx(struct_type_index), FieldIdx(field_index)),
        Instruction::StructGetU {
            struct_type_index,
            field_index,
        } => sink.struct_get_u(TypeIdx(struct_type_index), FieldIdx(field_index)),
        Instruction::StructSet {
            struct_type_index,
            field_index,
        } => sink.struct_set(TypeIdx(struct_type_index), FieldIdx(field_index)),
        Instruction::ArrayNew(type_index) => sink.array_new(TypeIdx(type_index)),
        Instruction::ArrayNewDefault(type_index) => sink.array_new_default(TypeIdx(type_index)),
        Instruction::ArrayNewFixed {
            array_type_index,
            array_size,
        } => sink.array_new_fixed(TypeIdx(array_type_index), array_size),
        Instruction::ArrayNewData {
            array_type_index,
            array_data_index,
        } => sink.array_new_data(TypeIdx(array_type_index), DataIdx(array_data_index)),
        Instruction::ArrayNewElem {
            array_type_index,
            array_elem_index,
        } => sink.array_new_elem(TypeIdx(array_type_index), ElemIdx(array_elem_index)),
        Instruction::ArrayGet(type_index) => sink.array_get(TypeIdx(type_index)),
        Instruction::ArrayGetS(type_index) => sink.array_get_s(TypeIdx(type_index)),
        Instruction::ArrayGetU(type_index) => sink.array_get_u(TypeIdx(type_index)),
        Instruction::ArraySet(type_index) => sink.array_set(TypeIdx(type_index)),
        Instruction::ArrayLen => sink.array_len(),
        Instruction::ArrayFill(type_index) => sink.array_fill(TypeIdx(type_index)),
        Instruction::ArrayCopy {
            array_type_index_dst,
            array_type_index_src,
        } => sink.array_copy(TypeIdx(array_type_index_dst), TypeIdx(array_type_index_src)),
        Instruction::ArrayInitData {
            array_type_index,
            array_data_index,
        } => sink.array_init_data(TypeIdx(array_type_index), DataIdx(array_data_index)),
        Instruction::ArrayInitElem {
            array_type_index,
            array_elem_index,
        } => sink.array_init_elem(TypeIdx(array_type_index), ElemIdx(array_elem_index)),
        Instruction::RefTestNonNull(heap_type) => sink.ref_test_non_null(heap_type),
        Instruction::RefTestNullable(heap_type) => sink.ref_test_nullable(heap_type),
        Instruction::RefCastNonNull(heap_type) => sink.ref_cast_non_null(heap_type),
        Instruction::RefCastNullable(heap_type) => sink.ref_cast_nullable(heap_type),
        Instruction::BrOnCast {
            relative_depth,
            from_ref_type,
            to_ref_type,
        } => sink.br_on_cast(LabelIdx(relative_depth), from_ref_type, to_ref_type),
        Instruction::BrOnCastFail {
            relative_depth,
            from_ref_type,
            to_ref_type,
        } => sink.br_on_cast_fail(LabelIdx(relative_depth), from_ref_type, to_ref_type),
        Instruction::AnyConvertExtern => sink.any_convert_extern(),
        Instruction::ExternConvertAny => sink.extern_convert_any(),
        Instruction::RefI31 => sink.ref_i31(),
        Instruction::I31GetS => sink.i31_get_s(),
        Instruction::I31GetU => sink.i31_get_u(),

        // Bulk memory instructions.
        Instruction::TableInit { elem_index, table } => {
            sink.table_init(TableIdx(table), ElemIdx(elem_index))
        }
        Instruction::ElemDrop(segment) => sink.elem_drop(ElemIdx(segment)),
        Instruction::TableCopy {
            src_table,
            dst_table,
        } => sink.table_copy(TableIdx(dst_table), TableIdx(src_table)),
        Instruction::TableGrow(table) => sink.table_grow(TableIdx(table)),
        Instruction::TableSize(table) => sink.table_size(TableIdx(table)),
        Instruction::TableFill(table) => sink.table_fill(TableIdx(table)),

        // SIMD instructions.
        Instruction::V128Load(memarg) => sink.v128_load(memarg),
        Instruction::V128Load8x8S(memarg) => sink.v128_load8x8_s(memarg),
        Instruction::V128Load8x8U(memarg) => sink.v128_load8x8_u(memarg),
        Instruction::V128Load16x4S(memarg) => sink.v128_load16x4_s(memarg),
        Instruction::V128Load16x4U(memarg) => sink.v128_load16x4_u(memarg),
        Instruction::V128Load32x2S(memarg) => sink.v128_load32x2_s(memarg),
        Instruction::V128Load32x2U(memarg) => sink.v128_load32x2_u(memarg),
        Instruction::V128Load8Splat(memarg) => sink.v128_load8_splat(memarg),
        Instruction::V128Load16Splat(memarg) => sink.v128_load16_splat(memarg),
        Instruction::V128Load32Splat(memarg) => sink.v128_load32_splat(memarg),
        Instruction::V128Load64Splat(memarg) => sink.v128_load64_splat(memarg),
        Instruction::V128Store(memarg) => sink.v128_store(memarg),
        Instruction::V128Const(x) => sink.v128_const(x),
        Instruction::I8x16Shuffle(lanes) => sink.i8x16_shuffle(lanes),
        Instruction::I8x16Swizzle => sink.i8x16_swizzle(),
        Instruction::I8x16Splat => sink.i8x16_splat(),
        Instruction::I16x8Splat => sink.i16x8_splat(),
        Instruction::I32x4Splat => sink.i32x4_splat(),
        Instruction::I64x2Splat => sink.i64x2_splat(),
        Instruction::F32x4Splat => sink.f32x4_splat(),
        Instruction::F64x2Splat => sink.f64x2_splat(),
        Instruction::I8x16ExtractLaneS(lane) => sink.i8x16_extract_lane_s(lane),
        Instruction::I8x16ExtractLaneU(lane) => sink.i8x16_extract_lane_u(lane),
        Instruction::I8x16ReplaceLane(lane) => sink.i8x16_replace_lane(lane),
        Instruction::I16x8ExtractLaneS(lane) => sink.i16x8_extract_lane_s(lane),
        Instruction::I16x8ExtractLaneU(lane) => sink.i16x8_extract_lane_u(lane),
        Instruction::I16x8ReplaceLane(lane) => sink.i16x8_replace_lane(lane),
        Instruction::I32x4ExtractLane(lane) => sink.i32x4_extract_lane(lane),
        Instruction::I32x4ReplaceLane(lane) => sink.i32x4_replace_lane(lane),
        Instruction::I64x2ExtractLane(lane) => sink.i64x2_extract_lane(lane),
        Instruction::I64x2ReplaceLane(lane) => sink.i64x2_replace_lane(lane),
        Instruction::F32x4ExtractLane(lane) => sink.f32x4_extract_lane(lane),
        Instruction::F32x4ReplaceLane(lane) => sink.f32x4_replace_lane(lane),
        Instruction::F64x2ExtractLane(lane) => sink.f64x2_extract_lane(lane),
        Instruction::F64x2ReplaceLane(lane) => sink.f64x2_replace_lane(lane),

        Instruction::I8x16Eq => sink.i8x16_eq(),
        Instruction::I8x16Ne => sink.i8x16_ne(),
        Instruction::I8x16LtS => sink.i8x16_lt_s(),
        Instruction::I8x16LtU => sink.i8x16_lt_u(),
        Instruction::I8x16GtS => sink.i8x16_gt_s(),
        Instruction::I8x16GtU => sink.i8x16_gt_u(),
        Instruction::I8x16LeS => sink.i8x16_le_s(),
        Instruction::I8x16LeU => sink.i8x16_le_u(),
        Instruction::I8x16GeS => sink.i8x16_ge_s(),
        Instruction::I8x16GeU => sink.i8x16_ge_u(),
        Instruction::I16x8Eq => sink.i16x8_eq(),
        Instruction::I16x8Ne => sink.i16x8_ne(),
        Instruction::I16x8LtS => sink.i16x8_lt_s(),
        Instruction::I16x8LtU => sink.i16x8_lt_u(),
        Instruction::I16x8GtS => sink.i16x8_gt_s(),
        Instruction::I16x8GtU => sink.i16x8_gt_u(),
        Instruction::I16x8LeS => sink.i16x8_le_s(),
        Instruction::I16x8LeU => sink.i16x8_le_u(),
        Instruction::I16x8GeS => sink.i16x8_ge_s(),
        Instruction::I16x8GeU => sink.i16x8_ge_u(),
        Instruction::I32x4Eq => sink.i32x4_eq(),
        Instruction::I32x4Ne => sink.i32x4_ne(),
        Instruction::I32x4LtS => sink.i32x4_lt_s(),
        Instruction::I32x4LtU => sink.i32x4_lt_u(),
        Instruction::I32x4GtS => sink.i32x4_gt_s(),
        Instruction::I32x4GtU => sink.i32x4_gt_u(),
        Instruction::I32x4LeS => sink.i32x4_le_s(),
        Instruction::I32x4LeU => sink.i32x4_le_u(),
        Instruction::I32x4GeS => sink.i32x4_ge_s(),
        Instruction::I32x4GeU => sink.i32x4_ge_u(),
        Instruction::F32x4Eq => sink.f32x4_eq(),
        Instruction::F32x4Ne => sink.f32x4_ne(),
        Instruction::F32x4Lt => sink.f32x4_lt(),
        Instruction::F32x4Gt => sink.f32x4_gt(),
        Instruction::F32x4Le => sink.f32x4_le(),
        Instruction::F32x4Ge => sink.f32x4_ge(),
        Instruction::F64x2Eq => sink.f64x2_eq(),
        Instruction::F64x2Ne => sink.f64x2_ne(),
        Instruction::F64x2Lt => sink.f64x2_lt(),
        Instruction::F64x2Gt => sink.f64x2_gt(),
        Instruction::F64x2Le => sink.f64x2_le(),
        Instruction::F64x2Ge => sink.f64x2_ge(),
        Instruction::V128Not => sink.v128_not(),
        Instruction::V128And => sink.v128_and(),
        Instruction::V128AndNot => sink.v128_andnot(),
        Instruction::V128Or => sink.v128_or(),
        Instruction::V128Xor => sink.v128_xor(),
        Instruction::V128Bitselect => sink.v128_bitselect(),
        Instruction::V128AnyTrue => sink.v128_any_true(),
        Instruction::I8x16Abs => sink.i8x16_abs(),
        Instruction::I8x16Neg => sink.i8x16_neg(),
        Instruction::I8x16Popcnt => sink.i8x16_popcnt(),
        Instruction::I8x16AllTrue => sink.i8x16_all_true(),
        Instruction::I8x16Bitmask => sink.i8x16_bitmask(),
        Instruction::I8x16NarrowI16x8S => sink.i8x16_narrow_i16x8_s(),
        Instruction::I8x16NarrowI16x8U => sink.i8x16_narrow_i16x8_u(),
        Instruction::I8x16Shl => sink.i8x16_shl(),
        Instruction::I8x16ShrS => sink.i8x16_shr_s(),
        Instruction::I8x16ShrU => sink.i8x16_shr_u(),
        Instruction::I8x16Add => sink.i8x16_add(),
        Instruction::I8x16AddSatS => sink.i8x16_add_sat_s(),
        Instruction::I8x16AddSatU => sink.i8x16_add_sat_u(),
        Instruction::I8x16Sub => sink.i8x16_sub(),
        Instruction::I8x16SubSatS => sink.i8x16_sub_sat_s(),
        Instruction::I8x16SubSatU => sink.i8x16_sub_sat_u(),
        Instruction::I8x16MinS => sink.i8x16_min_s(),
        Instruction::I8x16MinU => sink.i8x16_min_u(),
        Instruction::I8x16MaxS => sink.i8x16_max_s(),
        Instruction::I8x16MaxU => sink.i8x16_max_u(),
        Instruction::I8x16AvgrU => sink.i8x16_avgr_u(),
        Instruction::I16x8ExtAddPairwiseI8x16S => sink.i16x8_extadd_pairwise_i8x16_s(),
        Instruction::I16x8ExtAddPairwiseI8x16U => sink.i16x8_extadd_pairwise_i8x16_u(),
        Instruction::I32x4ExtAddPairwiseI16x8S => sink.i32x4_extadd_pairwise_i16x8_s(),
        Instruction::I32x4ExtAddPairwiseI16x8U => sink.i32x4_extadd_pairwise_i16x8_u(),
        Instruction::I16x8Abs => sink.i16x8_abs(),
        Instruction::I16x8Neg => sink.i16x8_neg(),
        Instruction::I16x8Q15MulrSatS => sink.i16x8_q15mulr_sat_s(),
        Instruction::I16x8AllTrue => sink.i16x8_all_true(),
        Instruction::I16x8Bitmask => sink.i16x8_bitmask(),
        Instruction::I16x8NarrowI32x4S => sink.i16x8_narrow_i32x4_s(),
        Instruction::I16x8NarrowI32x4U => sink.i16x8_narrow_i32x4_u(),
        Instruction::I16x8ExtendLowI8x16S => sink.i16x8_extend_low_i8x16_s(),
        Instruction::I16x8ExtendHighI8x16S => sink.i16x8_extend_high_i8x16_s(),
        Instruction::I16x8ExtendLowI8x16U => sink.i16x8_extend_low_i8x16_u(),
        Instruction::I16x8ExtendHighI8x16U => sink.i16x8_extend_high_i8x16_u(),
        Instruction::I16x8Shl => sink.i16x8_shl(),
        Instruction::I16x8ShrS => sink.i16x8_shr_s(),
        Instruction::I16x8ShrU => sink.i16x8_shr_u(),
        Instruction::I16x8Add => sink.i16x8_add(),
        Instruction::I16x8AddSatS => sink.i16x8_add_sat_s(),
        Instruction::I16x8AddSatU => sink.i16x8_add_sat_u(),
        Instruction::I16x8Sub => sink.i16x8_sub(),
        Instruction::I16x8SubSatS => sink.i16x8_sub_sat_s(),
        Instruction::I16x8SubSatU => sink.i16x8_sub_sat_u(),
        Instruction::I16x8Mul => sink.i16x8_mul(),
        Instruction::I16x8MinS => sink.i16x8_min_s(),
        Instruction::I16x8MinU => sink.i16x8_min_u(),
        Instruction::I16x8MaxS => sink.i16x8_max_s(),
        Instruction::I16x8MaxU => sink.i16x8_max_u(),
        Instruction::I16x8AvgrU => sink.i16x8_avgr_u(),
        Instruction::I16x8ExtMulLowI8x16S => sink.i16x8_extmul_low_i8x16_s(),
        Instruction::I16x8ExtMulHighI8x16S => sink.i16x8_extmul_high_i8x16_s(),
        Instruction::I16x8ExtMulLowI8x16U => sink.i16x8_extmul_low_i8x16_u(),
        Instruction::I16x8ExtMulHighI8x16U => sink.i16x8_extmul_high_i8x16_u(),
        Instruction::I32x4Abs => sink.i32x4_abs(),
        Instruction::I32x4Neg => sink.i32x4_neg(),
        Instruction::I32x4AllTrue => sink.i32x4_all_true(),
        Instruction::I32x4Bitmask => sink.i32x4_bitmask(),
        Instruction::I32x4ExtendLowI16x8S => sink.i32x4_extend_low_i16x8_s(),
        Instruction::I32x4ExtendHighI16x8S => sink.i32x4_extend_high_i16x8_s(),
        Instruction::I32x4ExtendLowI16x8U => sink.i32x4_extend_low_i16x8_u(),
        Instruction::I32x4ExtendHighI16x8U => sink.i32x4_extend_high_i16x8_u(),
        Instruction::I32x4Shl => sink.i32x4_shl(),
        Instruction::I32x4ShrS => sink.i32x4_shr_s(),
        Instruction::I32x4ShrU => sink.i32x4_shr_u(),
        Instruction::I32x4Add => sink.i32x4_add(),
        Instruction::I32x4Sub => sink.i32x4_sub(),
        Instruction::I32x4Mul => sink.i32x4_mul(),
        Instruction::I32x4MinS => sink.i32x4_min_s(),
        Instruction::I32x4MinU => sink.i32x4_min_u(),
        Instruction::I32x4MaxS => sink.i32x4_max_s(),
        Instruction::I32x4MaxU => sink.i32x4_max_u(),
        Instruction::I32x4DotI16x8S => sink.i32x4_dot_i16x8_s(),
        Instruction::I32x4ExtMulLowI16x8S => sink.i32x4_extmul_low_i16x8_s(),
        Instruction::I32x4ExtMulHighI16x8S => sink.i32x4_extmul_high_i16x8_s(),
        Instruction::I32x4ExtMulLowI16x8U => sink.i32x4_extmul_low_i16x8_u(),
        Instruction::I32x4ExtMulHighI16x8U => sink.i32x4_extmul_high_i16x8_u(),
        Instruction::I64x2Abs => sink.i64x2_abs(),
        Instruction::I64x2Neg => sink.i64x2_neg(),
        Instruction::I64x2AllTrue => sink.i64x2_all_true(),
        Instruction::I64x2Bitmask => sink.i64x2_bitmask(),
        Instruction::I64x2ExtendLowI32x4S => sink.i64x2_extend_low_i32x4_s(),
        Instruction::I64x2ExtendHighI32x4S => sink.i64x2_extend_high_i32x4_s(),
        Instruction::I64x2ExtendLowI32x4U => sink.i64x2_extend_low_i32x4_u(),
        Instruction::I64x2ExtendHighI32x4U => sink.i64x2_extend_high_i32x4_u(),
        Instruction::I64x2Shl => sink.i64x2_shl(),
        Instruction::I64x2ShrS => sink.i64x2_shr_s(),
        Instruction::I64x2ShrU => sink.i64x2_shr_u(),
        Instruction::I64x2Add => sink.i64x2_add(),
        Instruction::I64x2Sub => sink.i64x2_sub(),
        Instruction::I64x2Mul => sink.i64x2_mul(),
        Instruction::I64x2ExtMulLowI32x4S => sink.i64x2_extmul_low_i32x4_s(),
        Instruction::I64x2ExtMulHighI32x4S => sink.i64x2_extmul_high_i32x4_s(),
        Instruction::I64x2ExtMulLowI32x4U => sink.i64x2_extmul_low_i32x4_u(),
        Instruction::I64x2ExtMulHighI32x4U => sink.i64x2_extmul_high_i32x4_u(),
        Instruction::F32x4Ceil => sink.f32x4_ceil(),
        Instruction::F32x4Floor => sink.f32x4_floor(),
        Instruction::F32x4Trunc => sink.f32x4_trunc(),
        Instruction::F32x4Nearest => sink.f32x4_nearest(),
        Instruction::F32x4Abs => sink.f32x4_abs(),
        Instruction::F32x4Neg => sink.f32x4_neg(),
        Instruction::F32x4Sqrt => sink.f32x4_sqrt(),
        Instruction::F32x4Add => sink.f32x4_add(),
        Instruction::F32x4Sub => sink.f32x4_sub(),
        Instruction::F32x4Mul => sink.f32x4_mul(),
        Instruction::F32x4Div => sink.f32x4_div(),
        Instruction::F32x4Min => sink.f32x4_min(),
        Instruction::F32x4Max => sink.f32x4_max(),
        Instruction::F32x4PMin => sink.f32x4_pmin(),
        Instruction::F32x4PMax => sink.f32x4_pmax(),
        Instruction::F64x2Ceil => sink.f64x2_ceil(),
        Instruction::F64x2Floor => sink.f64x2_floor(),
        Instruction::F64x2Trunc => sink.f64x2_trunc(),
        Instruction::F64x2Nearest => sink.f64x2_nearest(),
        Instruction::F64x2Abs => sink.f64x2_abs(),
        Instruction::F64x2Neg => sink.f64x2_neg(),
        Instruction::F64x2Sqrt => sink.f64x2_sqrt(),
        Instruction::F64x2Add => sink.f64x2_add(),
        Instruction::F64x2Sub => sink.f64x2_sub(),
        Instruction::F64x2Mul => sink.f64x2_mul(),
        Instruction::F64x2Div => sink.f64x2_div(),
        Instruction::F64x2Min => sink.f64x2_min(),
        Instruction::F64x2Max => sink.f64x2_max(),
        Instruction::F64x2PMin => sink.f64x2_pmin(),
        Instruction::F64x2PMax => sink.f64x2_pmax(),
        Instruction::I32x4TruncSatF32x4S => sink.i32x4_trunc_sat_f32x4_s(),
        Instruction::I32x4TruncSatF32x4U => sink.i32x4_trunc_sat_f32x4_u(),
        Instruction::F32x4ConvertI32x4S => sink.f32x4_convert_i32x4_s(),
        Instruction::F32x4ConvertI32x4U => sink.f32x4_convert_i32x4_u(),
        Instruction::I32x4TruncSatF64x2SZero => sink.i32x4_trunc_sat_f64x2_s_zero(),
        Instruction::I32x4TruncSatF64x2UZero => sink.i32x4_trunc_sat_f64x2_u_zero(),
        Instruction::F64x2ConvertLowI32x4S => sink.f64x2_convert_low_i32x4_s(),
        Instruction::F64x2ConvertLowI32x4U => sink.f64x2_convert_low_i32x4_u(),
        Instruction::F32x4DemoteF64x2Zero => sink.f32x4_demote_f64x2_zero(),
        Instruction::F64x2PromoteLowF32x4 => sink.f64x2_promote_low_f32x4(),
        Instruction::V128Load32Zero(memarg) => sink.v128_load32_zero(memarg),
        Instruction::V128Load64Zero(memarg) => sink.v128_load64_zero(memarg),
        Instruction::V128Load8Lane { memarg, lane } => sink.v128_load8_lane(memarg, lane),
        Instruction::V128Load16Lane { memarg, lane } => sink.v128_load16_lane(memarg, lane),
        Instruction::V128Load32Lane { memarg, lane } => sink.v128_load32_lane(memarg, lane),
        Instruction::V128Load64Lane { memarg, lane } => sink.v128_load64_lane(memarg, lane),
        Instruction::V128Store8Lane { memarg, lane } => sink.v128_store8_lane(memarg, lane),
        Instruction::V128Store16Lane { memarg, lane } => sink.v128_store16_lane(memarg, lane),
        Instruction::V128Store32Lane { memarg, lane } => sink.v128_store32_lane(memarg, lane),
        Instruction::V128Store64Lane { memarg, lane } => sink.v128_store64_lane(memarg, lane),
        Instruction::I64x2Eq => sink.i64x2_eq(),
        Instruction::I64x2Ne => sink.i64x2_ne(),
        Instruction::I64x2LtS => sink.i64x2_lt_s(),
        Instruction::I64x2GtS => sink.i64x2_gt_s(),
        Instruction::I64x2LeS => sink.i64x2_le_s(),
        Instruction::I64x2GeS => sink.i64x2_ge_s(),
        Instruction::I8x16RelaxedSwizzle => sink.i8x16_relaxed_swizzle(),
        Instruction::I32x4RelaxedTruncF32x4S => sink.i32x4_relaxed_trunc_f32x4_s(),
        Instruction::I32x4RelaxedTruncF32x4U => sink.i32x4_relaxed_trunc_f32x4_u(),
        Instruction::I32x4RelaxedTruncF64x2SZero => sink.i32x4_relaxed_trunc_f64x2_s_zero(),
        Instruction::I32x4RelaxedTruncF64x2UZero => sink.i32x4_relaxed_trunc_f64x2_u_zero(),
        Instruction::F32x4RelaxedMadd => sink.f32x4_relaxed_madd(),
        Instruction::F32x4RelaxedNmadd => sink.f32x4_relaxed_nmadd(),
        Instruction::F64x2RelaxedMadd => sink.f64x2_relaxed_madd(),
        Instruction::F64x2RelaxedNmadd => sink.f64x2_relaxed_nmadd(),
        Instruction::I8x16RelaxedLaneselect => sink.i8x16_relaxed_laneselect(),
        Instruction::I16x8RelaxedLaneselect => sink.i16x8_relaxed_laneselect(),
        Instruction::I32x4RelaxedLaneselect => sink.i32x4_relaxed_laneselect(),
        Instruction::I64x2RelaxedLaneselect => sink.i64x2_relaxed_laneselect(),
        Instruction::F32x4RelaxedMin => sink.f32x4_relaxed_min(),
        Instruction::F32x4RelaxedMax => sink.f32x4_relaxed_max(),
        Instruction::F64x2RelaxedMin => sink.f64x2_relaxed_min(),
        Instruction::F64x2RelaxedMax => sink.f64x2_relaxed_max(),
        Instruction::I16x8RelaxedQ15mulrS => sink.i16x8_relaxed_q15mulr_s(),
        Instruction::I16x8RelaxedDotI8x16I7x16S => sink.i16x8_relaxed_dot_i8x16_i7x16_s(),
        Instruction::I32x4RelaxedDotI8x16I7x16AddS => sink.i32x4_relaxed_dot_i8x16_i7x16_add_s(),

        // Atomic instructions from the thread proposal
        Instruction::MemoryAtomicNotify(memarg) => sink.memory_atomic_notify(memarg),
        Instruction::MemoryAtomicWait32(memarg) => sink.memory_atomic_wait32(memarg),
        Instruction::MemoryAtomicWait64(memarg) => sink.memory_atomic_wait64(memarg),
        Instruction::AtomicFence => sink.atomic_fence(),
        Instruction::I32AtomicLoad(memarg) => sink.i32_atomic_load(memarg),
        Instruction::I64AtomicLoad(memarg) => sink.i64_atomic_load(memarg),
        Instruction::I32AtomicLoad8U(memarg) => sink.i32_atomic_load8_u(memarg),
        Instruction::I32AtomicLoad16U(memarg) => sink.i32_atomic_load16_u(memarg),
        Instruction::I64AtomicLoad8U(memarg) => sink.i64_atomic_load8_u(memarg),
        Instruction::I64AtomicLoad16U(memarg) => sink.i64_atomic_load16_u(memarg),
        Instruction::I64AtomicLoad32U(memarg) => sink.i64_atomic_load32_u(memarg),
        Instruction::I32AtomicStore(memarg) => sink.i32_atomic_store(memarg),
        Instruction::I64AtomicStore(memarg) => sink.i64_atomic_store(memarg),
        Instruction::I32AtomicStore8(memarg) => sink.i32_atomic_store8(memarg),
        Instruction::I32AtomicStore16(memarg) => sink.i32_atomic_store16(memarg),
        Instruction::I64AtomicStore8(memarg) => sink.i64_atomic_store8(memarg),
        Instruction::I64AtomicStore16(memarg) => sink.i64_atomic_store16(memarg),
        Instruction::I64AtomicStore32(memarg) => sink.i64_atomic_store32(memarg),
        Instruction::I32AtomicRmwAdd(memarg) => sink.i32_atomic_rmw_add(memarg),
        Instruction::I64AtomicRmwAdd(memarg) => sink.i64_atomic_rmw_add(memarg),
        Instruction::I32AtomicRmw8AddU(memarg) => sink.i32_atomic_rmw8_add_u(memarg),
        Instruction::I32AtomicRmw16AddU(memarg) => sink.i32_atomic_rmw16_add_u(memarg),
        Instruction::I64AtomicRmw8AddU(memarg) => sink.i64_atomic_rmw8_add_u(memarg),
        Instruction::I64AtomicRmw16AddU(memarg) => sink.i64_atomic_rmw16_add_u(memarg),
        Instruction::I64AtomicRmw32AddU(memarg) => sink.i64_atomic_rmw32_add_u(memarg),
        Instruction::I32AtomicRmwSub(memarg) => sink.i32_atomic_rmw_sub(memarg),
        Instruction::I64AtomicRmwSub(memarg) => sink.i64_atomic_rmw_sub(memarg),
        Instruction::I32AtomicRmw8SubU(memarg) => sink.i32_atomic_rmw8_sub_u(memarg),
        Instruction::I32AtomicRmw16SubU(memarg) => sink.i32_atomic_rmw16_sub_u(memarg),
        Instruction::I64AtomicRmw8SubU(memarg) => sink.i64_atomic_rmw8_sub_u(memarg),
        Instruction::I64AtomicRmw16SubU(memarg) => sink.i64_atomic_rmw16_sub_u(memarg),
        Instruction::I64AtomicRmw32SubU(memarg) => sink.i64_atomic_rmw32_sub_u(memarg),
        Instruction::I32AtomicRmwAnd(memarg) => sink.i32_atomic_rmw_and(memarg),
        Instruction::I64AtomicRmwAnd(memarg) => sink.i64_atomic_rmw_and(memarg),
        Instruction::I32AtomicRmw8AndU(memarg) => sink.i32_atomic_rmw8_and_u(memarg),
        Instruction::I32AtomicRmw16AndU(memarg) => sink.i32_atomic_rmw16_and_u(memarg),
        Instruction::I64AtomicRmw8AndU(memarg) => sink.i64_atomic_rmw8_and_u(memarg),
        Instruction::I64AtomicRmw16AndU(memarg) => sink.i64_atomic_rmw16_and_u(memarg),
        Instruction::I64AtomicRmw32AndU(memarg) => sink.i64_atomic_rmw32_and_u(memarg),
        Instruction::I32AtomicRmwOr(memarg) => sink.i32_atomic_rmw_or(memarg),
        Instruction::I64AtomicRmwOr(memarg) => sink.i64_atomic_rmw_or(memarg),
        Instruction::I32AtomicRmw8OrU(memarg) => sink.i32_atomic_rmw8_or_u(memarg),
        Instruction::I32AtomicRmw16OrU(memarg) => sink.i32_atomic_rmw16_or_u(memarg),
        Instruction::I64AtomicRmw8OrU(memarg) => sink.i64_atomic_rmw8_or_u(memarg),
        Instruction::I64AtomicRmw16OrU(memarg) => sink.i64_atomic_rmw16_or_u(memarg),
        Instruction::I64AtomicRmw32OrU(memarg) => sink.i64_atomic_rmw32_or_u(memarg),
        Instruction::I32AtomicRmwXor(memarg) => sink.i32_atomic_rmw_xor(memarg),
        Instruction::I64AtomicRmwXor(memarg) => sink.i64_atomic_rmw_xor(memarg),
        Instruction::I32AtomicRmw8XorU(memarg) => sink.i32_atomic_rmw8_xor_u(memarg),
        Instruction::I32AtomicRmw16XorU(memarg) => sink.i32_atomic_rmw16_xor_u(memarg),
        Instruction::I64AtomicRmw8XorU(memarg) => sink.i64_atomic_rmw8_xor_u(memarg),
        Instruction::I64AtomicRmw16XorU(memarg) => sink.i64_atomic_rmw16_xor_u(memarg),
        Instruction::I64AtomicRmw32XorU(memarg) => sink.i64_atomic_rmw32_xor_u(memarg),
        Instruction::I32AtomicRmwXchg(memarg) => sink.i32_atomic_rmw_xchg(memarg),
        Instruction::I64AtomicRmwXchg(memarg) => sink.i64_atomic_rmw_xchg(memarg),
        Instruction::I32AtomicRmw8XchgU(memarg) => sink.i32_atomic_rmw8_xchg_u(memarg),
        Instruction::I32AtomicRmw16XchgU(memarg) => sink.i32_atomic_rmw16_xchg_u(memarg),
        Instruction::I64AtomicRmw8XchgU(memarg) => sink.i64_atomic_rmw8_xchg_u(memarg),
        Instruction::I64AtomicRmw16XchgU(memarg) => sink.i64_atomic_rmw16_xchg_u(memarg),
        Instruction::I64AtomicRmw32XchgU(memarg) => sink.i64_atomic_rmw32_xchg_u(memarg),
        Instruction::I32AtomicRmwCmpxchg(memarg) => sink.i32_atomic_rmw_cmpxchg(memarg),
        Instruction::I64AtomicRmwCmpxchg(memarg) => sink.i64_atomic_rmw_cmpxchg(memarg),
        Instruction::I32AtomicRmw8CmpxchgU(memarg) => sink.i32_atomic_rmw8_cmpxchg_u(memarg),
        Instruction::I32AtomicRmw16CmpxchgU(memarg) => sink.i32_atomic_rmw16_cmpxchg_u(memarg),
        Instruction::I64AtomicRmw8CmpxchgU(memarg) => sink.i64_atomic_rmw8_cmpxchg_u(memarg),
        Instruction::I64AtomicRmw16CmpxchgU(memarg) => sink.i64_atomic_rmw16_cmpxchg_u(memarg),
        Instruction::I64AtomicRmw32CmpxchgU(memarg) => sink.i64_atomic_rmw32_cmpxchg_u(memarg),

        // Atomic instructions from the shared-everything-threads proposal
        Instruction::GlobalAtomicGet {
            ordering,
            global_index,
        } => sink.global_atomic_get(ordering, GlobalIdx(global_index)),
        Instruction::GlobalAtomicSet {
            ordering,
            global_index,
        } => sink.global_atomic_set(ordering, GlobalIdx(global_index)),
        Instruction::GlobalAtomicRmwAdd {
            ordering,
            global_index,
        } => sink.global_atomic_rmw_add(ordering, GlobalIdx(global_index)),
        Instruction::GlobalAtomicRmwSub {
            ordering,
            global_index,
        } => sink.global_atomic_rmw_sub(ordering, GlobalIdx(global_index)),
        Instruction::GlobalAtomicRmwAnd {
            ordering,
            global_index,
        } => sink.global_atomic_rmw_and(ordering, GlobalIdx(global_index)),
        Instruction::GlobalAtomicRmwOr {
            ordering,
            global_index,
        } => sink.global_atomic_rmw_or(ordering, GlobalIdx(global_index)),
        Instruction::GlobalAtomicRmwXor {
            ordering,
            global_index,
        } => sink.global_atomic_rmw_xor(ordering, GlobalIdx(global_index)),
        Instruction::GlobalAtomicRmwXchg {
            ordering,
            global_index,
        } => sink.global_atomic_rmw_xchg(ordering, GlobalIdx(global_index)),
        Instruction::GlobalAtomicRmwCmpxchg {
            ordering,
            global_index,
        } => sink.global_atomic_rmw_cmpxchg(ordering, GlobalIdx(global_index)),
        Instruction::TableAtomicGet {
            ordering,
            table_index,
        } => sink.table_atomic_get(ordering, TableIdx(table_index)),
        Instruction::TableAtomicSet {
            ordering,
            table_index,
        } => sink.table_atomic_set(ordering, TableIdx(table_index)),
        Instruction::TableAtomicRmwXchg {
            ordering,
            table_index,
        } => sink.table_atomic_rmw_xchg(ordering, TableIdx(table_index)),
        Instruction::TableAtomicRmwCmpxchg {
            ordering,
            table_index,
        } => sink.table_atomic_rmw_cmpxchg(ordering, TableIdx(table_index)),
        Instruction::StructAtomicGet {
            ordering,
            struct_type_index,
            field_index,
        } => sink.struct_atomic_get(ordering, TypeIdx(struct_type_index), FieldIdx(field_index)),
        Instruction::StructAtomicGetS {
            ordering,
            struct_type_index,
            field_index,
        } => sink.struct_atomic_get_s(ordering, TypeIdx(struct_type_index), FieldIdx(field_index)),
        Instruction::StructAtomicGetU {
            ordering,
            struct_type_index,
            field_index,
        } => sink.struct_atomic_get_u(ordering, TypeIdx(struct_type_index), FieldIdx(field_index)),
        Instruction::StructAtomicSet {
            ordering,
            struct_type_index,
            field_index,
        } => sink.struct_atomic_set(ordering, TypeIdx(struct_type_index), FieldIdx(field_index)),
        Instruction::StructAtomicRmwAdd {
            ordering,
            struct_type_index,
            field_index,
        } => {
            sink.struct_atomic_rmw_add(ordering, TypeIdx(struct_type_index), FieldIdx(field_index))
        }
        Instruction::StructAtomicRmwSub {
            ordering,
            struct_type_index,
            field_index,
        } => {
            sink.struct_atomic_rmw_sub(ordering, TypeIdx(struct_type_index), FieldIdx(field_index))
        }
        Instruction::StructAtomicRmwAnd {
            ordering,
            struct_type_index,
            field_index,
        } => {
            sink.struct_atomic_rmw_and(ordering, TypeIdx(struct_type_index), FieldIdx(field_index))
        }
        Instruction::StructAtomicRmwOr {
            ordering,
            struct_type_index,
            field_index,
        } => sink.struct_atomic_rmw_or(ordering, TypeIdx(struct_type_index), FieldIdx(field_index)),
        Instruction::StructAtomicRmwXor {
            ordering,
            struct_type_index,
            field_index,
        } => {
            sink.struct_atomic_rmw_xor(ordering, TypeIdx(struct_type_index), FieldIdx(field_index))
        }
        Instruction::StructAtomicRmwXchg {
            ordering,
            struct_type_index,
            field_index,
        } => {
            sink.struct_atomic_rmw_xchg(ordering, TypeIdx(struct_type_index), FieldIdx(field_index))
        }
        Instruction::StructAtomicRmwCmpxchg {
            ordering,
            struct_type_index,
            field_index,
        } => sink.struct_atomic_rmw_cmpxchg(
            ordering,
            TypeIdx(struct_type_index),
            FieldIdx(field_index),
        ),
        Instruction::ArrayAtomicGet {
            ordering,
            array_type_index,
        } => sink.array_atomic_get(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicGetS {
            ordering,
            array_type_index,
        } => sink.array_atomic_get_s(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicGetU {
            ordering,
            array_type_index,
        } => sink.array_atomic_get_u(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicSet {
            ordering,
            array_type_index,
        } => sink.array_atomic_set(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicRmwAdd {
            ordering,
            array_type_index,
        } => sink.array_atomic_rmw_add(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicRmwSub {
            ordering,
            array_type_index,
        } => sink.array_atomic_rmw_sub(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicRmwAnd {
            ordering,
            array_type_index,
        } => sink.array_atomic_rmw_and(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicRmwOr {
            ordering,
            array_type_index,
        } => sink.array_atomic_rmw_or(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicRmwXor {
            ordering,
            array_type_index,
        } => sink.array_atomic_rmw_xor(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicRmwXchg {
            ordering,
            array_type_index,
        } => sink.array_atomic_rmw_xchg(ordering, TypeIdx(array_type_index)),
        Instruction::ArrayAtomicRmwCmpxchg {
            ordering,
            array_type_index,
        } => sink.array_atomic_rmw_cmpxchg(ordering, TypeIdx(array_type_index)),
        Instruction::RefI31Shared => sink.ref_i31_shared(),
        Instruction::ContNew(type_index) => sink.cont_new(TypeIdx(type_index)),
        Instruction::ContBind {
            argument_index,
            result_index,
        } => sink.cont_bind(TypeIdx(argument_index), TypeIdx(result_index)),
        Instruction::Suspend(tag_index) => sink.suspend(TagIdx(tag_index)),
        Instruction::Resume {
            cont_type_index,
            ref resume_table,
        } => sink.resume(TypeIdx(cont_type_index), resume_table.iter().cloned()),
        Instruction::ResumeThrow {
            cont_type_index,
            tag_index,
            ref resume_table,
        } => sink.resume_throw(
            TypeIdx(cont_type_index),
            TagIdx(tag_index),
            resume_table.iter().cloned(),
        ),
        Instruction::Switch {
            cont_type_index,
            tag_index,
        } => sink.switch(TypeIdx(cont_type_index), TagIdx(tag_index)),
        Instruction::I64Add128 => sink.i64_add128(),
        Instruction::I64Sub128 => sink.i64_sub128(),
        Instruction::I64MulWideS => sink.i64_mul_wide_s(),
        Instruction::I64MulWideU => sink.i64_mul_wide_u(),
        _ => unimplemented!(),
    };
}

fn encode_vec<T, V>(elements: V, sink: &mut Vec<u8>)
where
    T: Encode,
    V: IntoIterator<Item = T>,
    V::IntoIter: ExactSizeIterator,
{
    let elements = elements.into_iter();
    u32::try_from(elements.len()).unwrap().encode(sink);
    for x in elements {
        x.encode(sink);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TypeIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct FuncIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct TableIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct MemIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct TagIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct GlobalIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct ElemIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct DataIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct LocalIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct LabelIdx(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct FieldIdx(pub u32);

impl Encode for TypeIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for FuncIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for TableIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for MemIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for TagIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for GlobalIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for ElemIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for DataIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for LocalIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for LabelIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl Encode for FieldIdx {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

impl From<MemIdx> for u32 {
    fn from(value: MemIdx) -> Self {
        value.0
    }
}

pub struct InstructionSink<'a> {
    sink: &'a mut Vec<u8>,
}

impl<'a> InstructionSink<'a> {
    /// Create an instruction encoder pointing to the given byte sink.
    pub fn new(sink: &'a mut Vec<u8>) -> Self {
        Self { sink }
    }

    // Control instructions.

    /// Encode [`Instruction::Unreachable`].
    pub fn unreachable(&mut self) -> &mut Self {
        self.sink.push(0x00);
        self
    }

    /// Encode [`Instruction::Nop`].
    pub fn nop(&mut self) -> &mut Self {
        self.sink.push(0x01);
        self
    }

    /// Encode [`Instruction::Block`].
    pub fn block(&mut self, bt: BlockType) -> &mut Self {
        self.sink.push(0x02);
        bt.encode(self.sink);
        self
    }

    /// Encode [`Instruction::Loop`].
    pub fn loop_(&mut self, bt: BlockType) -> &mut Self {
        self.sink.push(0x03);
        bt.encode(self.sink);
        self
    }

    /// Encode [`Instruction::If`].
    pub fn if_(&mut self, bt: BlockType) -> &mut Self {
        self.sink.push(0x04);
        bt.encode(self.sink);
        self
    }

    /// Encode [`Instruction::Else`].
    pub fn else_(&mut self) -> &mut Self {
        self.sink.push(0x05);
        self
    }

    /// Encode [`Instruction::End`].
    pub fn end(&mut self) -> &mut Self {
        self.sink.push(0x0B);
        self
    }

    /// Encode [`Instruction::Br`].
    pub fn br(&mut self, l: LabelIdx) -> &mut Self {
        self.sink.push(0x0C);
        l.encode(self.sink);
        self
    }

    /// Encode [`Instruction::BrIf`].
    pub fn br_if(&mut self, l: LabelIdx) -> &mut Self {
        self.sink.push(0x0D);
        l.encode(self.sink);
        self
    }

    /// Encode [`Instruction::BrTable`].
    pub fn br_table(
        &mut self,
        ls: impl IntoIterator<Item = LabelIdx, IntoIter: ExactSizeIterator>,
        l: LabelIdx,
    ) -> &mut Self {
        self.sink.push(0x0E);
        encode_vec(ls, self.sink);
        l.encode(self.sink);
        self
    }

    /// Encode [`Instruction::BrOnNull`].
    pub fn br_on_null(&mut self, l: LabelIdx) -> &mut Self {
        self.sink.push(0xD5);
        l.encode(self.sink);
        self
    }

    /// Encode [`Instruction::BrOnNonNull`].
    pub fn br_on_non_null(&mut self, l: LabelIdx) -> &mut Self {
        self.sink.push(0xD6);
        l.encode(self.sink);
        self
    }

    /// Encode [`Instruction::Return`].
    pub fn return_(&mut self) -> &mut Self {
        self.sink.push(0x0F);
        self
    }

    /// Encode [`Instruction::Call`].
    pub fn call(&mut self, f: FuncIdx) -> &mut Self {
        self.sink.push(0x10);
        f.encode(self.sink);
        self
    }

    /// Encode [`Instruction::CallRef`].
    pub fn call_ref(&mut self, ty: TypeIdx) -> &mut Self {
        self.sink.push(0x14);
        ty.encode(self.sink);
        self
    }

    /// Encode [`Instruction::CallIndirect`].
    pub fn call_indirect(&mut self, table_index: TableIdx, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0x11);
        type_index.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ReturnCallRef`].
    pub fn return_call_ref(&mut self, ty: TypeIdx) -> &mut Self {
        self.sink.push(0x15);
        ty.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ReturnCall`].
    pub fn return_call(&mut self, f: FuncIdx) -> &mut Self {
        self.sink.push(0x12);
        f.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ReturnCallIndirect`].
    pub fn return_call_indirect(
        &mut self,
        table_index: TableIdx,
        type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0x13);
        type_index.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TryTable`].
    pub fn try_table(
        &mut self,
        ty: BlockType,
        catches: impl IntoIterator<Item = Catch, IntoIter: ExactSizeIterator>,
    ) -> &mut Self {
        self.sink.push(0x1f);
        ty.encode(self.sink);
        encode_vec(catches, self.sink);
        self
    }

    /// Encode [`Instruction::Throw`].
    pub fn throw(&mut self, t: TagIdx) -> &mut Self {
        self.sink.push(0x08);
        t.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ThrowRef`].
    pub fn throw_ref(&mut self) -> &mut Self {
        self.sink.push(0x0A);
        self
    }

    // Deprecated exception-handling instructions

    /// Encode [`Instruction::Try`].
    pub fn try_(&mut self, bt: BlockType) -> &mut Self {
        self.sink.push(0x06);
        bt.encode(self.sink);
        self
    }

    /// Encode [`Instruction::Delegate`].
    pub fn delegate(&mut self, l: LabelIdx) -> &mut Self {
        self.sink.push(0x18);
        l.encode(self.sink);
        self
    }

    /// Encode [`Instruction::Catch`].
    pub fn catch(&mut self, t: TagIdx) -> &mut Self {
        self.sink.push(0x07);
        t.encode(self.sink);
        self
    }

    /// Encode [`Instruction::CatchAll`].
    pub fn catch_all(&mut self) -> &mut Self {
        self.sink.push(0x19);
        self
    }

    /// Encode [`Instruction::Rethrow`].
    pub fn rethrow(&mut self, l: LabelIdx) -> &mut Self {
        self.sink.push(0x09);
        l.encode(self.sink);
        self
    }

    // Parametric instructions.

    /// Encode [`Instruction::Drop`].
    pub fn drop(&mut self) -> &mut Self {
        self.sink.push(0x1A);
        self
    }

    /// Encode [`Instruction::Select`].
    pub fn select(&mut self) -> &mut Self {
        self.sink.push(0x1B);
        self
    }

    // Variable instructions.

    /// Encode [`Instruction::LocalGet`].
    pub fn local_get(&mut self, l: LocalIdx) -> &mut Self {
        self.sink.push(0x20);
        l.encode(self.sink);
        self
    }

    /// Encode [`Instruction::LocalSet`].
    pub fn local_set(&mut self, l: LocalIdx) -> &mut Self {
        self.sink.push(0x21);
        l.encode(self.sink);
        self
    }

    /// Encode [`Instruction::LocalTee`].
    pub fn local_tee(&mut self, l: LocalIdx) -> &mut Self {
        self.sink.push(0x22);
        l.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalGet`].
    pub fn global_get(&mut self, g: GlobalIdx) -> &mut Self {
        self.sink.push(0x23);
        g.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalSet`].
    pub fn global_set(&mut self, g: GlobalIdx) -> &mut Self {
        self.sink.push(0x24);
        g.encode(self.sink);
        self
    }

    // Memory instructions.

    /// Encode [`Instruction::I32Load`].
    pub fn i32_load(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x28);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Load`].
    pub fn i64_load(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x29);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32Load`].
    pub fn f32_load(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2A);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64Load`].
    pub fn f64_load(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2B);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32Load8S`].
    pub fn i32_load8_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2C);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32Load8U`].
    pub fn i32_load8_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2D);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32Load16S`].
    pub fn i32_load16_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2E);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32Load16U`].
    pub fn i32_load16_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x2F);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Load8S`].
    pub fn i64_load8_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x30);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Load8U`].
    pub fn i64_load8_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x31);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Load16S`].
    pub fn i64_load16_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x32);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Load16U`].
    pub fn i64_load16_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x33);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Load32S`].
    pub fn i64_load32_s(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x34);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Load32U`].
    pub fn i64_load32_u(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x35);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32Store`].
    pub fn i32_store(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x36);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Store`].
    pub fn i64_store(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x37);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32Store`].
    pub fn f32_store(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x38);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64Store`].
    pub fn f64_store(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x39);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32Store8`].
    pub fn i32_store8(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3A);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32Store16`].
    pub fn i32_store16(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3B);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Store8`].
    pub fn i64_store8(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3C);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Store16`].
    pub fn i64_store16(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3D);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Store32`].
    pub fn i64_store32(&mut self, m: MemArg) -> &mut Self {
        self.sink.push(0x3E);
        m.encode(self.sink);
        self
    }

    /// Encode [`Instruction::MemorySize`].
    pub fn memory_size(&mut self, i: MemIdx) -> &mut Self {
        self.sink.push(0x3F);
        i.encode(self.sink);
        self
    }

    /// Encode [`Instruction::MemoryGrow`].
    pub fn memory_grow(&mut self, i: MemIdx) -> &mut Self {
        self.sink.push(0x40);
        i.encode(self.sink);
        self
    }

    /// Encode [`Instruction::MemoryInit`].
    pub fn memory_init(&mut self, mem: MemIdx, data_index: DataIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x08);
        data_index.encode(self.sink);
        mem.encode(self.sink);
        self
    }

    /// Encode [`Instruction::DataDrop`].
    pub fn data_drop(&mut self, data: DataIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x09);
        data.encode(self.sink);
        self
    }

    /// Encode [`Instruction::MemoryCopy`].
    pub fn memory_copy(&mut self, dst_mem: MemIdx, src_mem: MemIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0a);
        dst_mem.encode(self.sink);
        src_mem.encode(self.sink);
        self
    }

    /// Encode [`Instruction::MemoryFill`].
    pub fn memory_fill(&mut self, mem: MemIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0b);
        mem.encode(self.sink);
        self
    }

    /// Encode [`Instruction::MemoryDiscard`].
    pub fn memory_discard(&mut self, mem: MemIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x12);
        mem.encode(self.sink);
        self
    }

    // Numeric instructions.

    /// Encode [`Instruction::I32Const`].
    pub fn i32_const(&mut self, x: i32) -> &mut Self {
        self.sink.push(0x41);
        x.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Const`].
    pub fn i64_const(&mut self, x: i64) -> &mut Self {
        self.sink.push(0x42);
        x.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32Const`].
    pub fn f32_const(&mut self, x: f32) -> &mut Self {
        self.sink.push(0x43);
        let x = x.to_bits();
        self.sink.extend(x.to_le_bytes().iter().copied());
        self
    }

    /// Encode [`Instruction::F64Const`].
    pub fn f64_const(&mut self, x: f64) -> &mut Self {
        self.sink.push(0x44);
        let x = x.to_bits();
        self.sink.extend(x.to_le_bytes().iter().copied());
        self
    }

    /// Encode [`Instruction::I32Eqz`].
    pub fn i32_eqz(&mut self) -> &mut Self {
        self.sink.push(0x45);
        self
    }

    /// Encode [`Instruction::I32Eq`].
    pub fn i32_eq(&mut self) -> &mut Self {
        self.sink.push(0x46);
        self
    }

    /// Encode [`Instruction::I32Ne`].
    pub fn i32_ne(&mut self) -> &mut Self {
        self.sink.push(0x47);
        self
    }

    /// Encode [`Instruction::I32LtS`].
    pub fn i32_lt_s(&mut self) -> &mut Self {
        self.sink.push(0x48);
        self
    }

    /// Encode [`Instruction::I32LtU`].
    pub fn i32_lt_u(&mut self) -> &mut Self {
        self.sink.push(0x49);
        self
    }

    /// Encode [`Instruction::I32GtS`].
    pub fn i32_gt_s(&mut self) -> &mut Self {
        self.sink.push(0x4A);
        self
    }

    /// Encode [`Instruction::I32GtU`].
    pub fn i32_gt_u(&mut self) -> &mut Self {
        self.sink.push(0x4B);
        self
    }

    /// Encode [`Instruction::I32LeS`].
    pub fn i32_le_s(&mut self) -> &mut Self {
        self.sink.push(0x4C);
        self
    }

    /// Encode [`Instruction::I32LeU`].
    pub fn i32_le_u(&mut self) -> &mut Self {
        self.sink.push(0x4D);
        self
    }

    /// Encode [`Instruction::I32GeS`].
    pub fn i32_ge_s(&mut self) -> &mut Self {
        self.sink.push(0x4E);
        self
    }

    /// Encode [`Instruction::I32GeU`].
    pub fn i32_ge_u(&mut self) -> &mut Self {
        self.sink.push(0x4F);
        self
    }

    /// Encode [`Instruction::I64Eqz`].
    pub fn i64_eqz(&mut self) -> &mut Self {
        self.sink.push(0x50);
        self
    }

    /// Encode [`Instruction::I64Eq`].
    pub fn i64_eq(&mut self) -> &mut Self {
        self.sink.push(0x51);
        self
    }

    /// Encode [`Instruction::I64Ne`].
    pub fn i64_ne(&mut self) -> &mut Self {
        self.sink.push(0x52);
        self
    }

    /// Encode [`Instruction::I64LtS`].
    pub fn i64_lt_s(&mut self) -> &mut Self {
        self.sink.push(0x53);
        self
    }

    /// Encode [`Instruction::I64LtU`].
    pub fn i64_lt_u(&mut self) -> &mut Self {
        self.sink.push(0x54);
        self
    }

    /// Encode [`Instruction::I64GtS`].
    pub fn i64_gt_s(&mut self) -> &mut Self {
        self.sink.push(0x55);
        self
    }

    /// Encode [`Instruction::I64GtU`].
    pub fn i64_gt_u(&mut self) -> &mut Self {
        self.sink.push(0x56);
        self
    }

    /// Encode [`Instruction::I64LeS`].
    pub fn i64_le_s(&mut self) -> &mut Self {
        self.sink.push(0x57);
        self
    }

    /// Encode [`Instruction::I64LeU`].
    pub fn i64_le_u(&mut self) -> &mut Self {
        self.sink.push(0x58);
        self
    }

    /// Encode [`Instruction::I64GeS`].
    pub fn i64_ge_s(&mut self) -> &mut Self {
        self.sink.push(0x59);
        self
    }

    /// Encode [`Instruction::I64GeU`].
    pub fn i64_ge_u(&mut self) -> &mut Self {
        self.sink.push(0x5A);
        self
    }

    /// Encode [`Instruction::F32Eq`].
    pub fn f32_eq(&mut self) -> &mut Self {
        self.sink.push(0x5B);
        self
    }

    /// Encode [`Instruction::F32Ne`].
    pub fn f32_ne(&mut self) -> &mut Self {
        self.sink.push(0x5C);
        self
    }

    /// Encode [`Instruction::F32Lt`].
    pub fn f32_lt(&mut self) -> &mut Self {
        self.sink.push(0x5D);
        self
    }

    /// Encode [`Instruction::F32Gt`].
    pub fn f32_gt(&mut self) -> &mut Self {
        self.sink.push(0x5E);
        self
    }

    /// Encode [`Instruction::F32Le`].
    pub fn f32_le(&mut self) -> &mut Self {
        self.sink.push(0x5F);
        self
    }

    /// Encode [`Instruction::F32Ge`].
    pub fn f32_ge(&mut self) -> &mut Self {
        self.sink.push(0x60);
        self
    }

    /// Encode [`Instruction::F64Eq`].
    pub fn f64_eq(&mut self) -> &mut Self {
        self.sink.push(0x61);
        self
    }

    /// Encode [`Instruction::F64Ne`].
    pub fn f64_ne(&mut self) -> &mut Self {
        self.sink.push(0x62);
        self
    }

    /// Encode [`Instruction::F64Lt`].
    pub fn f64_lt(&mut self) -> &mut Self {
        self.sink.push(0x63);
        self
    }

    /// Encode [`Instruction::F64Gt`].
    pub fn f64_gt(&mut self) -> &mut Self {
        self.sink.push(0x64);
        self
    }

    /// Encode [`Instruction::F64Le`].
    pub fn f64_le(&mut self) -> &mut Self {
        self.sink.push(0x65);
        self
    }

    /// Encode [`Instruction::F64Ge`].
    pub fn f64_ge(&mut self) -> &mut Self {
        self.sink.push(0x66);
        self
    }

    /// Encode [`Instruction::I32Clz`].
    pub fn i32_clz(&mut self) -> &mut Self {
        self.sink.push(0x67);
        self
    }

    /// Encode [`Instruction::I32Ctz`].
    pub fn i32_ctz(&mut self) -> &mut Self {
        self.sink.push(0x68);
        self
    }

    /// Encode [`Instruction::I32Popcnt`].
    pub fn i32_popcnt(&mut self) -> &mut Self {
        self.sink.push(0x69);
        self
    }

    /// Encode [`Instruction::I32Add`].
    pub fn i32_add(&mut self) -> &mut Self {
        self.sink.push(0x6A);
        self
    }

    /// Encode [`Instruction::I32Sub`].
    pub fn i32_sub(&mut self) -> &mut Self {
        self.sink.push(0x6B);
        self
    }

    /// Encode [`Instruction::I32Mul`].
    pub fn i32_mul(&mut self) -> &mut Self {
        self.sink.push(0x6C);
        self
    }

    /// Encode [`Instruction::I32DivS`].
    pub fn i32_div_s(&mut self) -> &mut Self {
        self.sink.push(0x6D);
        self
    }

    /// Encode [`Instruction::I32DivU`].
    pub fn i32_div_u(&mut self) -> &mut Self {
        self.sink.push(0x6E);
        self
    }

    /// Encode [`Instruction::I32RemS`].
    pub fn i32_rem_s(&mut self) -> &mut Self {
        self.sink.push(0x6F);
        self
    }

    /// Encode [`Instruction::I32RemU`].
    pub fn i32_rem_u(&mut self) -> &mut Self {
        self.sink.push(0x70);
        self
    }

    /// Encode [`Instruction::I32And`].
    pub fn i32_and(&mut self) -> &mut Self {
        self.sink.push(0x71);
        self
    }

    /// Encode [`Instruction::I32Or`].
    pub fn i32_or(&mut self) -> &mut Self {
        self.sink.push(0x72);
        self
    }

    /// Encode [`Instruction::I32Xor`].
    pub fn i32_xor(&mut self) -> &mut Self {
        self.sink.push(0x73);
        self
    }

    /// Encode [`Instruction::I32Shl`].
    pub fn i32_shl(&mut self) -> &mut Self {
        self.sink.push(0x74);
        self
    }

    /// Encode [`Instruction::I32ShrS`].
    pub fn i32_shr_s(&mut self) -> &mut Self {
        self.sink.push(0x75);
        self
    }

    /// Encode [`Instruction::I32ShrU`].
    pub fn i32_shr_u(&mut self) -> &mut Self {
        self.sink.push(0x76);
        self
    }

    /// Encode [`Instruction::I32Rotl`].
    pub fn i32_rotl(&mut self) -> &mut Self {
        self.sink.push(0x77);
        self
    }

    /// Encode [`Instruction::I32Rotr`].
    pub fn i32_rotr(&mut self) -> &mut Self {
        self.sink.push(0x78);
        self
    }

    /// Encode [`Instruction::I64Clz`].
    pub fn i64_clz(&mut self) -> &mut Self {
        self.sink.push(0x79);
        self
    }

    /// Encode [`Instruction::I64Ctz`].
    pub fn i64_ctz(&mut self) -> &mut Self {
        self.sink.push(0x7A);
        self
    }

    /// Encode [`Instruction::I64Popcnt`].
    pub fn i64_popcnt(&mut self) -> &mut Self {
        self.sink.push(0x7B);
        self
    }

    /// Encode [`Instruction::I64Add`].
    pub fn i64_add(&mut self) -> &mut Self {
        self.sink.push(0x7C);
        self
    }

    /// Encode [`Instruction::I64Sub`].
    pub fn i64_sub(&mut self) -> &mut Self {
        self.sink.push(0x7D);
        self
    }

    /// Encode [`Instruction::I64Mul`].
    pub fn i64_mul(&mut self) -> &mut Self {
        self.sink.push(0x7E);
        self
    }

    /// Encode [`Instruction::I64DivS`].
    pub fn i64_div_s(&mut self) -> &mut Self {
        self.sink.push(0x7F);
        self
    }

    /// Encode [`Instruction::I64DivU`].
    pub fn i64_div_u(&mut self) -> &mut Self {
        self.sink.push(0x80);
        self
    }

    /// Encode [`Instruction::I64RemS`].
    pub fn i64_rem_s(&mut self) -> &mut Self {
        self.sink.push(0x81);
        self
    }

    /// Encode [`Instruction::I64RemU`].
    pub fn i64_rem_u(&mut self) -> &mut Self {
        self.sink.push(0x82);
        self
    }

    /// Encode [`Instruction::I64And`].
    pub fn i64_and(&mut self) -> &mut Self {
        self.sink.push(0x83);
        self
    }

    /// Encode [`Instruction::I64Or`].
    pub fn i64_or(&mut self) -> &mut Self {
        self.sink.push(0x84);
        self
    }

    /// Encode [`Instruction::I64Xor`].
    pub fn i64_xor(&mut self) -> &mut Self {
        self.sink.push(0x85);
        self
    }

    /// Encode [`Instruction::I64Shl`].
    pub fn i64_shl(&mut self) -> &mut Self {
        self.sink.push(0x86);
        self
    }

    /// Encode [`Instruction::I64ShrS`].
    pub fn i64_shr_s(&mut self) -> &mut Self {
        self.sink.push(0x87);
        self
    }

    /// Encode [`Instruction::I64ShrU`].
    pub fn i64_shr_u(&mut self) -> &mut Self {
        self.sink.push(0x88);
        self
    }

    /// Encode [`Instruction::I64Rotl`].
    pub fn i64_rotl(&mut self) -> &mut Self {
        self.sink.push(0x89);
        self
    }

    /// Encode [`Instruction::I64Rotr`].
    pub fn i64_rotr(&mut self) -> &mut Self {
        self.sink.push(0x8A);
        self
    }

    /// Encode [`Instruction::F32Abs`].
    pub fn f32_abs(&mut self) -> &mut Self {
        self.sink.push(0x8B);
        self
    }

    /// Encode [`Instruction::F32Neg`].
    pub fn f32_neg(&mut self) -> &mut Self {
        self.sink.push(0x8C);
        self
    }

    /// Encode [`Instruction::F32Ceil`].
    pub fn f32_ceil(&mut self) -> &mut Self {
        self.sink.push(0x8D);
        self
    }

    /// Encode [`Instruction::F32Floor`].
    pub fn f32_floor(&mut self) -> &mut Self {
        self.sink.push(0x8E);
        self
    }

    /// Encode [`Instruction::F32Trunc`].
    pub fn f32_trunc(&mut self) -> &mut Self {
        self.sink.push(0x8F);
        self
    }

    /// Encode [`Instruction::F32Nearest`].
    pub fn f32_nearest(&mut self) -> &mut Self {
        self.sink.push(0x90);
        self
    }

    /// Encode [`Instruction::F32Sqrt`].
    pub fn f32_sqrt(&mut self) -> &mut Self {
        self.sink.push(0x91);
        self
    }

    /// Encode [`Instruction::F32Add`].
    pub fn f32_add(&mut self) -> &mut Self {
        self.sink.push(0x92);
        self
    }

    /// Encode [`Instruction::F32Sub`].
    pub fn f32_sub(&mut self) -> &mut Self {
        self.sink.push(0x93);
        self
    }

    /// Encode [`Instruction::F32Mul`].
    pub fn f32_mul(&mut self) -> &mut Self {
        self.sink.push(0x94);
        self
    }

    /// Encode [`Instruction::F32Div`].
    pub fn f32_div(&mut self) -> &mut Self {
        self.sink.push(0x95);
        self
    }

    /// Encode [`Instruction::F32Min`].
    pub fn f32_min(&mut self) -> &mut Self {
        self.sink.push(0x96);
        self
    }

    /// Encode [`Instruction::F32Max`].
    pub fn f32_max(&mut self) -> &mut Self {
        self.sink.push(0x97);
        self
    }

    /// Encode [`Instruction::F32Copysign`].
    pub fn f32_copysign(&mut self) -> &mut Self {
        self.sink.push(0x98);
        self
    }

    /// Encode [`Instruction::F64Abs`].
    pub fn f64_abs(&mut self) -> &mut Self {
        self.sink.push(0x99);
        self
    }

    /// Encode [`Instruction::F64Neg`].
    pub fn f64_neg(&mut self) -> &mut Self {
        self.sink.push(0x9A);
        self
    }

    /// Encode [`Instruction::F64Ceil`].
    pub fn f64_ceil(&mut self) -> &mut Self {
        self.sink.push(0x9B);
        self
    }

    /// Encode [`Instruction::F64Floor`].
    pub fn f64_floor(&mut self) -> &mut Self {
        self.sink.push(0x9C);
        self
    }

    /// Encode [`Instruction::F64Trunc`].
    pub fn f64_trunc(&mut self) -> &mut Self {
        self.sink.push(0x9D);
        self
    }

    /// Encode [`Instruction::F64Nearest`].
    pub fn f64_nearest(&mut self) -> &mut Self {
        self.sink.push(0x9E);
        self
    }

    /// Encode [`Instruction::F64Sqrt`].
    pub fn f64_sqrt(&mut self) -> &mut Self {
        self.sink.push(0x9F);
        self
    }

    /// Encode [`Instruction::F64Add`].
    pub fn f64_add(&mut self) -> &mut Self {
        self.sink.push(0xA0);
        self
    }

    /// Encode [`Instruction::F64Sub`].
    pub fn f64_sub(&mut self) -> &mut Self {
        self.sink.push(0xA1);
        self
    }

    /// Encode [`Instruction::F64Mul`].
    pub fn f64_mul(&mut self) -> &mut Self {
        self.sink.push(0xA2);
        self
    }

    /// Encode [`Instruction::F64Div`].
    pub fn f64_div(&mut self) -> &mut Self {
        self.sink.push(0xA3);
        self
    }

    /// Encode [`Instruction::F64Min`].
    pub fn f64_min(&mut self) -> &mut Self {
        self.sink.push(0xA4);
        self
    }

    /// Encode [`Instruction::F64Max`].
    pub fn f64_max(&mut self) -> &mut Self {
        self.sink.push(0xA5);
        self
    }

    /// Encode [`Instruction::F64Copysign`].
    pub fn f64_copysign(&mut self) -> &mut Self {
        self.sink.push(0xA6);
        self
    }

    /// Encode [`Instruction::I32WrapI64`].
    pub fn i32_wrap_i64(&mut self) -> &mut Self {
        self.sink.push(0xA7);
        self
    }

    /// Encode [`Instruction::I32TruncF32S`].
    pub fn i32_trunc_f32_s(&mut self) -> &mut Self {
        self.sink.push(0xA8);
        self
    }

    /// Encode [`Instruction::I32TruncF32U`].
    pub fn i32_trunc_f32_u(&mut self) -> &mut Self {
        self.sink.push(0xA9);
        self
    }

    /// Encode [`Instruction::I32TruncF64S`].
    pub fn i32_trunc_f64_s(&mut self) -> &mut Self {
        self.sink.push(0xAA);
        self
    }

    /// Encode [`Instruction::I32TruncF64U`].
    pub fn i32_trunc_f64_u(&mut self) -> &mut Self {
        self.sink.push(0xAB);
        self
    }

    /// Encode [`Instruction::I64ExtendI32S`].
    pub fn i64_extend_i32_s(&mut self) -> &mut Self {
        self.sink.push(0xAC);
        self
    }

    /// Encode [`Instruction::I64ExtendI32U`].
    pub fn i64_extend_i32_u(&mut self) -> &mut Self {
        self.sink.push(0xAD);
        self
    }

    /// Encode [`Instruction::I64TruncF32S`].
    pub fn i64_trunc_f32_s(&mut self) -> &mut Self {
        self.sink.push(0xAE);
        self
    }

    /// Encode [`Instruction::I64TruncF32U`].
    pub fn i64_trunc_f32_u(&mut self) -> &mut Self {
        self.sink.push(0xAF);
        self
    }

    /// Encode [`Instruction::I64TruncF64S`].
    pub fn i64_trunc_f64_s(&mut self) -> &mut Self {
        self.sink.push(0xB0);
        self
    }

    /// Encode [`Instruction::I64TruncF64U`].
    pub fn i64_trunc_f64_u(&mut self) -> &mut Self {
        self.sink.push(0xB1);
        self
    }

    /// Encode [`Instruction::F32ConvertI32S`].
    pub fn f32_convert_i32_s(&mut self) -> &mut Self {
        self.sink.push(0xB2);
        self
    }

    /// Encode [`Instruction::F32ConvertI32U`].
    pub fn f32_convert_i32_u(&mut self) -> &mut Self {
        self.sink.push(0xB3);
        self
    }

    /// Encode [`Instruction::F32ConvertI64S`].
    pub fn f32_convert_i64_s(&mut self) -> &mut Self {
        self.sink.push(0xB4);
        self
    }

    /// Encode [`Instruction::F32ConvertI64U`].
    pub fn f32_convert_i64_u(&mut self) -> &mut Self {
        self.sink.push(0xB5);
        self
    }

    /// Encode [`Instruction::F32DemoteF64`].
    pub fn f32_demote_f64(&mut self) -> &mut Self {
        self.sink.push(0xB6);
        self
    }

    /// Encode [`Instruction::F64ConvertI32S`].
    pub fn f64_convert_i32_s(&mut self) -> &mut Self {
        self.sink.push(0xB7);
        self
    }

    /// Encode [`Instruction::F64ConvertI32U`].
    pub fn f64_convert_i32_u(&mut self) -> &mut Self {
        self.sink.push(0xB8);
        self
    }

    /// Encode [`Instruction::F64ConvertI64S`].
    pub fn f64_convert_i64_s(&mut self) -> &mut Self {
        self.sink.push(0xB9);
        self
    }

    /// Encode [`Instruction::F64ConvertI64U`].
    pub fn f64_convert_i64_u(&mut self) -> &mut Self {
        self.sink.push(0xBA);
        self
    }

    /// Encode [`Instruction::F64PromoteF32`].
    pub fn f64_promote_f32(&mut self) -> &mut Self {
        self.sink.push(0xBB);
        self
    }

    /// Encode [`Instruction::I32ReinterpretF32`].
    pub fn i32_reinterpret_f32(&mut self) -> &mut Self {
        self.sink.push(0xBC);
        self
    }

    /// Encode [`Instruction::I64ReinterpretF64`].
    pub fn i64_reinterpret_f64(&mut self) -> &mut Self {
        self.sink.push(0xBD);
        self
    }

    /// Encode [`Instruction::F32ReinterpretI32`].
    pub fn f32_reinterpret_i32(&mut self) -> &mut Self {
        self.sink.push(0xBE);
        self
    }

    /// Encode [`Instruction::F64ReinterpretI64`].
    pub fn f64_reinterpret_i64(&mut self) -> &mut Self {
        self.sink.push(0xBF);
        self
    }

    /// Encode [`Instruction::I32Extend8S`].
    pub fn i32_extend8_s(&mut self) -> &mut Self {
        self.sink.push(0xC0);
        self
    }

    /// Encode [`Instruction::I32Extend16S`].
    pub fn i32_extend16_s(&mut self) -> &mut Self {
        self.sink.push(0xC1);
        self
    }

    /// Encode [`Instruction::I64Extend8S`].
    pub fn i64_extend8_s(&mut self) -> &mut Self {
        self.sink.push(0xC2);
        self
    }

    /// Encode [`Instruction::I64Extend16S`].
    pub fn i64_extend16_s(&mut self) -> &mut Self {
        self.sink.push(0xC3);
        self
    }

    /// Encode [`Instruction::I64Extend32S`].
    pub fn i64_extend32_s(&mut self) -> &mut Self {
        self.sink.push(0xC4);
        self
    }

    /// Encode [`Instruction::I32TruncSatF32S`].
    pub fn i32_trunc_sat_f32_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x00);
        self
    }

    /// Encode [`Instruction::I32TruncSatF32U`].
    pub fn i32_trunc_sat_f32_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x01);
        self
    }

    /// Encode [`Instruction::I32TruncSatF64S`].
    pub fn i32_trunc_sat_f64_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x02);
        self
    }

    /// Encode [`Instruction::I32TruncSatF64U`].
    pub fn i32_trunc_sat_f64_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x03);
        self
    }

    /// Encode [`Instruction::I64TruncSatF32S`].
    pub fn i64_trunc_sat_f32_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x04);
        self
    }

    /// Encode [`Instruction::I64TruncSatF32U`].
    pub fn i64_trunc_sat_f32_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x05);
        self
    }

    /// Encode [`Instruction::I64TruncSatF64S`].
    pub fn i64_trunc_sat_f64_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x06);
        self
    }

    /// Encode [`Instruction::I64TruncSatF64U`].
    pub fn i64_trunc_sat_f64_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        self.sink.push(0x07);
        self
    }

    // Reference types instructions.

    /// Encode [`Instruction::TypedSelect`].
    pub fn typed_select(&mut self, ty: ValType) -> &mut Self {
        self.sink.push(0x1c);
        [ty].encode(self.sink);
        self
    }

    /// Encode [`Instruction::RefNull`].
    pub fn ref_null(&mut self, ty: HeapType) -> &mut Self {
        self.sink.push(0xd0);
        ty.encode(self.sink);
        self
    }

    /// Encode [`Instruction::RefIsNull`].
    pub fn ref_is_null(&mut self) -> &mut Self {
        self.sink.push(0xd1);
        self
    }

    /// Encode [`Instruction::RefFunc`].
    pub fn ref_func(&mut self, f: FuncIdx) -> &mut Self {
        self.sink.push(0xd2);
        f.encode(self.sink);
        self
    }

    /// Encode [`Instruction::RefEq`].
    pub fn ref_eq(&mut self) -> &mut Self {
        self.sink.push(0xd3);
        self
    }

    /// Encode [`Instruction::RefAsNonNull`].
    pub fn ref_as_non_null(&mut self) -> &mut Self {
        self.sink.push(0xd4);
        self
    }

    // GC types instructions.

    /// Encode [`Instruction::StructNew`].
    pub fn struct_new(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x00);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructNewDefault`].
    pub fn struct_new_default(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x01);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructGet`].
    pub fn struct_get(&mut self, struct_type_index: TypeIdx, field_index: FieldIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x02);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructGetS`].
    pub fn struct_get_s(&mut self, struct_type_index: TypeIdx, field_index: FieldIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x03);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructGetU`].
    pub fn struct_get_u(&mut self, struct_type_index: TypeIdx, field_index: FieldIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x04);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructSet`].
    pub fn struct_set(&mut self, struct_type_index: TypeIdx, field_index: FieldIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x05);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayNew`].
    pub fn array_new(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x06);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayNewDefault`].
    pub fn array_new_default(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x07);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayNewFixed`].
    pub fn array_new_fixed(&mut self, array_type_index: TypeIdx, array_size: u32) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x08);
        array_type_index.encode(self.sink);
        array_size.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayNewData`].
    pub fn array_new_data(
        &mut self,
        array_type_index: TypeIdx,
        array_data_index: DataIdx,
    ) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x09);
        array_type_index.encode(self.sink);
        array_data_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayNewElem`].
    pub fn array_new_elem(
        &mut self,
        array_type_index: TypeIdx,
        array_elem_index: ElemIdx,
    ) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0a);
        array_type_index.encode(self.sink);
        array_elem_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayGet`].
    pub fn array_get(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0b);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayGetS`].
    pub fn array_get_s(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0c);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayGetU`].
    pub fn array_get_u(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0d);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArraySet`].
    pub fn array_set(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0e);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayLen`].
    pub fn array_len(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x0f);
        self
    }

    /// Encode [`Instruction::ArrayFill`].
    pub fn array_fill(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x10);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayCopy`].
    pub fn array_copy(
        &mut self,
        array_type_index_dst: TypeIdx,
        array_type_index_src: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x11);
        array_type_index_dst.encode(self.sink);
        array_type_index_src.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayInitData`].
    pub fn array_init_data(
        &mut self,
        array_type_index: TypeIdx,
        array_data_index: DataIdx,
    ) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x12);
        array_type_index.encode(self.sink);
        array_data_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayInitElem`].
    pub fn array_init_elem(
        &mut self,
        array_type_index: TypeIdx,
        array_elem_index: ElemIdx,
    ) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x13);
        array_type_index.encode(self.sink);
        array_elem_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::RefTestNonNull`].
    pub fn ref_test_non_null(&mut self, heap_type: HeapType) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x14);
        heap_type.encode(self.sink);
        self
    }

    /// Encode [`Instruction::RefTestNullable`].
    pub fn ref_test_nullable(&mut self, heap_type: HeapType) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x15);
        heap_type.encode(self.sink);
        self
    }

    /// Encode [`Instruction::RefCastNonNull`].
    pub fn ref_cast_non_null(&mut self, heap_type: HeapType) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x16);
        heap_type.encode(self.sink);
        self
    }

    /// Encode [`Instruction::RefCastNullable`].
    pub fn ref_cast_nullable(&mut self, heap_type: HeapType) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x17);
        heap_type.encode(self.sink);
        self
    }

    /// Encode [`Instruction::BrOnCast`].
    pub fn br_on_cast(
        &mut self,
        relative_depth: LabelIdx,
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

    /// Encode [`Instruction::BrOnCastFail`].
    pub fn br_on_cast_fail(
        &mut self,
        relative_depth: LabelIdx,
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

    /// Encode [`Instruction::AnyConvertExtern`].
    pub fn any_convert_extern(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1a);
        self
    }

    /// Encode [`Instruction::ExternConvertAny`].
    pub fn extern_convert_any(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1b);
        self
    }

    /// Encode [`Instruction::RefI31`].
    pub fn ref_i31(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1c);
        self
    }

    /// Encode [`Instruction::I31GetS`].
    pub fn i31_get_s(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1d);
        self
    }

    /// Encode [`Instruction::I31GetU`].
    pub fn i31_get_u(&mut self) -> &mut Self {
        self.sink.push(0xfb);
        self.sink.push(0x1e);
        self
    }

    // Bulk memory instructions.

    /// Encode [`Instruction::TableInit`].
    pub fn table_init(&mut self, table: TableIdx, elem_index: ElemIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0c);
        elem_index.encode(self.sink);
        table.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ElemDrop`].
    pub fn elem_drop(&mut self, segment: ElemIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0d);
        segment.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableFill`].
    pub fn table_fill(&mut self, table: TableIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x11);
        table.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableSet`].
    pub fn table_set(&mut self, table: TableIdx) -> &mut Self {
        self.sink.push(0x26);
        table.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableGet`].
    pub fn table_get(&mut self, table: TableIdx) -> &mut Self {
        self.sink.push(0x25);
        table.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableGrow`].
    pub fn table_grow(&mut self, table: TableIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0f);
        table.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableSize`].
    pub fn table_size(&mut self, table: TableIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x10);
        table.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableCopy`].
    pub fn table_copy(&mut self, dst_table: TableIdx, src_table: TableIdx) -> &mut Self {
        self.sink.push(0xfc);
        self.sink.push(0x0e);
        dst_table.encode(self.sink);
        src_table.encode(self.sink);
        self
    }

    // SIMD instructions.

    /// Encode [`Instruction::V128Load`].
    pub fn v128_load(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x00u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load8x8S`].
    pub fn v128_load8x8_s(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x01u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load8x8U`].
    pub fn v128_load8x8_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x02u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load16x4S`].
    pub fn v128_load16x4_s(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x03u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load16x4U`].
    pub fn v128_load16x4_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x04u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load32x2S`].
    pub fn v128_load32x2_s(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x05u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load32x2U`].
    pub fn v128_load32x2_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x06u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load8Splat`].
    pub fn v128_load8_splat(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x07u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load16Splat`].
    pub fn v128_load16_splat(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x08u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load32Splat`].
    pub fn v128_load32_splat(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x09u32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load64Splat`].
    pub fn v128_load64_splat(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x0Au32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load32Zero`].
    pub fn v128_load32_zero(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x5Cu32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load64Zero`].
    pub fn v128_load64_zero(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x5Du32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Store`].
    pub fn v128_store(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFD);
        0x0Bu32.encode(self.sink);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Load8Lane`].
    pub fn v128_load8_lane(&mut self, memarg: MemArg, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x54u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::V128Load16Lane`].
    pub fn v128_load16_lane(&mut self, memarg: MemArg, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x55u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::V128Load32Lane`].
    pub fn v128_load32_lane(&mut self, memarg: MemArg, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x56u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::V128Load64Lane`].
    pub fn v128_load64_lane(&mut self, memarg: MemArg, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x57u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::V128Store8Lane`].
    pub fn v128_store8_lane(&mut self, memarg: MemArg, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x58u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::V128Store16Lane`].
    pub fn v128_store16_lane(&mut self, memarg: MemArg, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x59u32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::V128Store32Lane`].
    pub fn v128_store32_lane(&mut self, memarg: MemArg, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x5Au32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::V128Store64Lane`].
    pub fn v128_store64_lane(&mut self, memarg: MemArg, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x5Bu32.encode(self.sink);
        memarg.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::V128Const`].
    pub fn v128_const(&mut self, x: i128) -> &mut Self {
        self.sink.push(0xFD);
        0x0Cu32.encode(self.sink);
        self.sink.extend(x.to_le_bytes().iter().copied());
        self
    }

    /// Encode [`Instruction::I8x16Shuffle`].
    pub fn i8x16_shuffle(&mut self, lanes: [Lane; 16]) -> &mut Self {
        self.sink.push(0xFD);
        0x0Du32.encode(self.sink);
        assert!(lanes.iter().all(|l: &u8| *l < 32));
        self.sink.extend(lanes.iter().copied());
        self
    }

    /// Encode [`Instruction::I8x16ExtractLaneS`].
    pub fn i8x16_extract_lane_s(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x15u32.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I8x16ExtractLaneU`].
    pub fn i8x16_extract_lane_u(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x16u32.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I8x16ReplaceLane`].
    pub fn i8x16_replace_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x17u32.encode(self.sink);
        assert!(lane < 16);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I16x8ExtractLaneS`].
    pub fn i16x8_extract_lane_s(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x18u32.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I16x8ExtractLaneU`].
    pub fn i16x8_extract_lane_u(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x19u32.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I16x8ReplaceLane`].
    pub fn i16x8_replace_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x1Au32.encode(self.sink);
        assert!(lane < 8);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I32x4ExtractLane`].
    pub fn i32x4_extract_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x1Bu32.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I32x4ReplaceLane`].
    pub fn i32x4_replace_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x1Cu32.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I64x2ExtractLane`].
    pub fn i64x2_extract_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x1Du32.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I64x2ReplaceLane`].
    pub fn i64x2_replace_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x1Eu32.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::F32x4ExtractLane`].
    pub fn f32x4_extract_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x1Fu32.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::F32x4ReplaceLane`].
    pub fn f32x4_replace_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x20u32.encode(self.sink);
        assert!(lane < 4);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::F64x2ExtractLane`].
    pub fn f64x2_extract_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x21u32.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::F64x2ReplaceLane`].
    pub fn f64x2_replace_lane(&mut self, lane: Lane) -> &mut Self {
        self.sink.push(0xFD);
        0x22u32.encode(self.sink);
        assert!(lane < 2);
        self.sink.push(lane);
        self
    }

    /// Encode [`Instruction::I8x16Swizzle`].
    pub fn i8x16_swizzle(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x0Eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Splat`].
    pub fn i8x16_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x0Fu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Splat`].
    pub fn i16x8_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Splat`].
    pub fn i32x4_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x11u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Splat`].
    pub fn i64x2_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x12u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Splat`].
    pub fn f32x4_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x13u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Splat`].
    pub fn f64x2_splat(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x14u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Eq`].
    pub fn i8x16_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x23u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Ne`].
    pub fn i8x16_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x24u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16LtS`].
    pub fn i8x16_lt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x25u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16LtU`].
    pub fn i8x16_lt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x26u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16GtS`].
    pub fn i8x16_gt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x27u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16GtU`].
    pub fn i8x16_gt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x28u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16LeS`].
    pub fn i8x16_le_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x29u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16LeU`].
    pub fn i8x16_le_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Au32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16GeS`].
    pub fn i8x16_ge_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Bu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16GeU`].
    pub fn i8x16_ge_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Cu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Eq`].
    pub fn i16x8_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Du32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Ne`].
    pub fn i16x8_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8LtS`].
    pub fn i16x8_lt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x2Fu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8LtU`].
    pub fn i16x8_lt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x30u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8GtS`].
    pub fn i16x8_gt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x31u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8GtU`].
    pub fn i16x8_gt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x32u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8LeS`].
    pub fn i16x8_le_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x33u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8LeU`].
    pub fn i16x8_le_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x34u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8GeS`].
    pub fn i16x8_ge_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x35u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8GeU`].
    pub fn i16x8_ge_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x36u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Eq`].
    pub fn i32x4_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x37u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Ne`].
    pub fn i32x4_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x38u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4LtS`].
    pub fn i32x4_lt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x39u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4LtU`].
    pub fn i32x4_lt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Au32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4GtS`].
    pub fn i32x4_gt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Bu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4GtU`].
    pub fn i32x4_gt_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Cu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4LeS`].
    pub fn i32x4_le_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Du32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4LeU`].
    pub fn i32x4_le_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4GeS`].
    pub fn i32x4_ge_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x3Fu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4GeU`].
    pub fn i32x4_ge_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x40u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Eq`].
    pub fn i64x2_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD6u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Ne`].
    pub fn i64x2_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD7u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2LtS`].
    pub fn i64x2_lt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD8u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2GtS`].
    pub fn i64x2_gt_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD9u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2LeS`].
    pub fn i64x2_le_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDAu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2GeS`].
    pub fn i64x2_ge_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDBu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Eq`].
    pub fn f32x4_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x41u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Ne`].
    pub fn f32x4_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x42u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Lt`].
    pub fn f32x4_lt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x43u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Gt`].
    pub fn f32x4_gt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x44u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Le`].
    pub fn f32x4_le(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x45u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Ge`].
    pub fn f32x4_ge(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x46u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Eq`].
    pub fn f64x2_eq(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x47u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Ne`].
    pub fn f64x2_ne(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x48u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Lt`].
    pub fn f64x2_lt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x49u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Gt`].
    pub fn f64x2_gt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Au32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Le`].
    pub fn f64x2_le(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Bu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Ge`].
    pub fn f64x2_ge(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Cu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Not`].
    pub fn v128_not(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Du32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128And`].
    pub fn v128_and(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128AndNot`].
    pub fn v128_andnot(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x4Fu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Or`].
    pub fn v128_or(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x50u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Xor`].
    pub fn v128_xor(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x51u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128Bitselect`].
    pub fn v128_bitselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x52u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::V128AnyTrue`].
    pub fn v128_any_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x53u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Abs`].
    pub fn i8x16_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x60u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Neg`].
    pub fn i8x16_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x61u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Popcnt`].
    pub fn i8x16_popcnt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x62u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16AllTrue`].
    pub fn i8x16_all_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x63u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Bitmask`].
    pub fn i8x16_bitmask(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x64u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16NarrowI16x8S`].
    pub fn i8x16_narrow_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x65u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16NarrowI16x8U`].
    pub fn i8x16_narrow_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x66u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Shl`].
    pub fn i8x16_shl(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6bu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16ShrS`].
    pub fn i8x16_shr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6cu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16ShrU`].
    pub fn i8x16_shr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6du32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Add`].
    pub fn i8x16_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16AddSatS`].
    pub fn i8x16_add_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6fu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16AddSatU`].
    pub fn i8x16_add_sat_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x70u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16Sub`].
    pub fn i8x16_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x71u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16SubSatS`].
    pub fn i8x16_sub_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x72u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16SubSatU`].
    pub fn i8x16_sub_sat_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x73u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16MinS`].
    pub fn i8x16_min_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x76u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16MinU`].
    pub fn i8x16_min_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x77u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16MaxS`].
    pub fn i8x16_max_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x78u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16MaxU`].
    pub fn i8x16_max_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x79u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16AvgrU`].
    pub fn i8x16_avgr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Bu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtAddPairwiseI8x16S`].
    pub fn i16x8_extadd_pairwise_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Cu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtAddPairwiseI8x16U`].
    pub fn i16x8_extadd_pairwise_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Du32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Abs`].
    pub fn i16x8_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x80u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Neg`].
    pub fn i16x8_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x81u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Q15MulrSatS`].
    pub fn i16x8_q15mulr_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x82u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8AllTrue`].
    pub fn i16x8_all_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x83u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Bitmask`].
    pub fn i16x8_bitmask(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x84u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8NarrowI32x4S`].
    pub fn i16x8_narrow_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x85u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8NarrowI32x4U`].
    pub fn i16x8_narrow_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x86u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtendLowI8x16S`].
    pub fn i16x8_extend_low_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x87u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtendHighI8x16S`].
    pub fn i16x8_extend_high_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x88u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtendLowI8x16U`].
    pub fn i16x8_extend_low_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x89u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtendHighI8x16U`].
    pub fn i16x8_extend_high_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Au32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Shl`].
    pub fn i16x8_shl(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Bu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ShrS`].
    pub fn i16x8_shr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Cu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ShrU`].
    pub fn i16x8_shr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Du32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Add`].
    pub fn i16x8_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8AddSatS`].
    pub fn i16x8_add_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x8Fu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8AddSatU`].
    pub fn i16x8_add_sat_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x90u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Sub`].
    pub fn i16x8_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x91u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8SubSatS`].
    pub fn i16x8_sub_sat_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x92u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8SubSatU`].
    pub fn i16x8_sub_sat_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x93u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8Mul`].
    pub fn i16x8_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x95u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8MinS`].
    pub fn i16x8_min_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x96u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8MinU`].
    pub fn i16x8_min_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x97u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8MaxS`].
    pub fn i16x8_max_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x98u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8MaxU`].
    pub fn i16x8_max_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x99u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8AvgrU`].
    pub fn i16x8_avgr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Bu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtMulLowI8x16S`].
    pub fn i16x8_extmul_low_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Cu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtMulHighI8x16S`].
    pub fn i16x8_extmul_high_i8x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Du32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtMulLowI8x16U`].
    pub fn i16x8_extmul_low_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8ExtMulHighI8x16U`].
    pub fn i16x8_extmul_high_i8x16_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x9Fu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtAddPairwiseI16x8S`].
    pub fn i32x4_extadd_pairwise_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtAddPairwiseI16x8U`].
    pub fn i32x4_extadd_pairwise_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Fu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Abs`].
    pub fn i32x4_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA0u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Neg`].
    pub fn i32x4_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA1u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4AllTrue`].
    pub fn i32x4_all_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA3u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Bitmask`].
    pub fn i32x4_bitmask(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA4u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtendLowI16x8S`].
    pub fn i32x4_extend_low_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA7u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtendHighI16x8S`].
    pub fn i32x4_extend_high_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA8u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtendLowI16x8U`].
    pub fn i32x4_extend_low_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xA9u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtendHighI16x8U`].
    pub fn i32x4_extend_high_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xAAu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Shl`].
    pub fn i32x4_shl(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xABu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ShrS`].
    pub fn i32x4_shr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xACu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ShrU`].
    pub fn i32x4_shr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xADu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Add`].
    pub fn i32x4_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xAEu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Sub`].
    pub fn i32x4_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB1u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4Mul`].
    pub fn i32x4_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB5u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4MinS`].
    pub fn i32x4_min_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB6u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4MinU`].
    pub fn i32x4_min_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB7u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4MaxS`].
    pub fn i32x4_max_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB8u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4MaxU`].
    pub fn i32x4_max_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xB9u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4DotI16x8S`].
    pub fn i32x4_dot_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBAu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtMulLowI16x8S`].
    pub fn i32x4_extmul_low_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBCu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtMulHighI16x8S`].
    pub fn i32x4_extmul_high_i16x8_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBDu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtMulLowI16x8U`].
    pub fn i32x4_extmul_low_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBEu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4ExtMulHighI16x8U`].
    pub fn i32x4_extmul_high_i16x8_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xBFu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Abs`].
    pub fn i64x2_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC0u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Neg`].
    pub fn i64x2_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC1u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2AllTrue`].
    pub fn i64x2_all_true(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC3u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Bitmask`].
    pub fn i64x2_bitmask(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC4u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ExtendLowI32x4S`].
    pub fn i64x2_extend_low_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC7u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ExtendHighI32x4S`].
    pub fn i64x2_extend_high_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC8u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ExtendLowI32x4U`].
    pub fn i64x2_extend_low_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xC9u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ExtendHighI32x4U`].
    pub fn i64x2_extend_high_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCAu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Shl`].
    pub fn i64x2_shl(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCBu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ShrS`].
    pub fn i64x2_shr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCCu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ShrU`].
    pub fn i64x2_shr_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCDu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Add`].
    pub fn i64x2_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xCEu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Sub`].
    pub fn i64x2_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD1u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2Mul`].
    pub fn i64x2_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xD5u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ExtMulLowI32x4S`].
    pub fn i64x2_extmul_low_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDCu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ExtMulHighI32x4S`].
    pub fn i64x2_extmul_high_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDDu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ExtMulLowI32x4U`].
    pub fn i64x2_extmul_low_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDEu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2ExtMulHighI32x4U`].
    pub fn i64x2_extmul_high_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xDFu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Ceil`].
    pub fn f32x4_ceil(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x67u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Floor`].
    pub fn f32x4_floor(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x68u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Trunc`].
    pub fn f32x4_trunc(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x69u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Nearest`].
    pub fn f32x4_nearest(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x6Au32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Abs`].
    pub fn f32x4_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE0u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Neg`].
    pub fn f32x4_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE1u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Sqrt`].
    pub fn f32x4_sqrt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE3u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Add`].
    pub fn f32x4_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE4u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Sub`].
    pub fn f32x4_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE5u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Mul`].
    pub fn f32x4_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE6u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Div`].
    pub fn f32x4_div(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE7u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Min`].
    pub fn f32x4_min(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE8u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4Max`].
    pub fn f32x4_max(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xE9u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4PMin`].
    pub fn f32x4_pmin(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xEAu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4PMax`].
    pub fn f32x4_pmax(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xEBu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Ceil`].
    pub fn f64x2_ceil(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x74u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Floor`].
    pub fn f64x2_floor(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x75u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Trunc`].
    pub fn f64x2_trunc(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x7Au32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Nearest`].
    pub fn f64x2_nearest(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x94u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Abs`].
    pub fn f64x2_abs(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xECu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Neg`].
    pub fn f64x2_neg(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xEDu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Sqrt`].
    pub fn f64x2_sqrt(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xEFu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Add`].
    pub fn f64x2_add(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF0u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Sub`].
    pub fn f64x2_sub(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF1u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Mul`].
    pub fn f64x2_mul(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF2u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Div`].
    pub fn f64x2_div(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF3u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Min`].
    pub fn f64x2_min(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF4u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2Max`].
    pub fn f64x2_max(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF5u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2PMin`].
    pub fn f64x2_pmin(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF6u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2PMax`].
    pub fn f64x2_pmax(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF7u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4TruncSatF32x4S`].
    pub fn i32x4_trunc_sat_f32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF8u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4TruncSatF32x4U`].
    pub fn i32x4_trunc_sat_f32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xF9u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4ConvertI32x4S`].
    pub fn f32x4_convert_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFAu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4ConvertI32x4U`].
    pub fn f32x4_convert_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFBu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4TruncSatF64x2SZero`].
    pub fn i32x4_trunc_sat_f64x2_s_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFCu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4TruncSatF64x2UZero`].
    pub fn i32x4_trunc_sat_f64x2_u_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFDu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2ConvertLowI32x4S`].
    pub fn f64x2_convert_low_i32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFEu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2ConvertLowI32x4U`].
    pub fn f64x2_convert_low_i32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0xFFu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4DemoteF64x2Zero`].
    pub fn f32x4_demote_f64x2_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x5Eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2PromoteLowF32x4`].
    pub fn f64x2_promote_low_f32x4(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x5Fu32.encode(self.sink);
        self
    }

    // Relaxed simd proposal

    /// Encode [`Instruction::I8x16RelaxedSwizzle`].
    pub fn i8x16_relaxed_swizzle(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x100u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4RelaxedTruncF32x4S`].
    pub fn i32x4_relaxed_trunc_f32x4_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x101u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4RelaxedTruncF32x4U`].
    pub fn i32x4_relaxed_trunc_f32x4_u(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x102u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4RelaxedTruncF64x2SZero`].
    pub fn i32x4_relaxed_trunc_f64x2_s_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x103u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4RelaxedTruncF64x2UZero`].
    pub fn i32x4_relaxed_trunc_f64x2_u_zero(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x104u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4RelaxedMadd`].
    pub fn f32x4_relaxed_madd(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x105u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4RelaxedNmadd`].
    pub fn f32x4_relaxed_nmadd(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x106u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2RelaxedMadd`].
    pub fn f64x2_relaxed_madd(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x107u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2RelaxedNmadd`].
    pub fn f64x2_relaxed_nmadd(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x108u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I8x16RelaxedLaneselect`].
    pub fn i8x16_relaxed_laneselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x109u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8RelaxedLaneselect`].
    pub fn i16x8_relaxed_laneselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Au32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4RelaxedLaneselect`].
    pub fn i32x4_relaxed_laneselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Bu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64x2RelaxedLaneselect`].
    pub fn i64x2_relaxed_laneselect(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Cu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4RelaxedMin`].
    pub fn f32x4_relaxed_min(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Du32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F32x4RelaxedMax`].
    pub fn f32x4_relaxed_max(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Eu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2RelaxedMin`].
    pub fn f64x2_relaxed_min(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x10Fu32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::F64x2RelaxedMax`].
    pub fn f64x2_relaxed_max(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x110u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8RelaxedQ15mulrS`].
    pub fn i16x8_relaxed_q15mulr_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x111u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I16x8RelaxedDotI8x16I7x16S`].
    pub fn i16x8_relaxed_dot_i8x16_i7x16_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x112u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32x4RelaxedDotI8x16I7x16AddS`].
    pub fn i32x4_relaxed_dot_i8x16_i7x16_add_s(&mut self) -> &mut Self {
        self.sink.push(0xFD);
        0x113u32.encode(self.sink);
        self
    }

    // Atomic instructions (the threads proposal)

    /// Encode [`Instruction::MemoryAtomicNotify`].
    pub fn memory_atomic_notify(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x00);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::MemoryAtomicWait32`].
    pub fn memory_atomic_wait32(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x01);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::MemoryAtomicWait64`].
    pub fn memory_atomic_wait64(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x02);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::AtomicFence`].
    pub fn atomic_fence(&mut self) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x03);
        self.sink.push(0x00);
        self
    }

    /// Encode [`Instruction::I32AtomicLoad`].
    pub fn i32_atomic_load(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x10);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicLoad`].
    pub fn i64_atomic_load(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x11);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicLoad8U`].
    pub fn i32_atomic_load8_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x12);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicLoad16U`].
    pub fn i32_atomic_load16_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x13);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicLoad8U`].
    pub fn i64_atomic_load8_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x14);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicLoad16U`].
    pub fn i64_atomic_load16_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x15);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicLoad32U`].
    pub fn i64_atomic_load32_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x16);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicStore`].
    pub fn i32_atomic_store(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x17);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicStore`].
    pub fn i64_atomic_store(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x18);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicStore8`].
    pub fn i32_atomic_store8(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x19);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicStore16`].
    pub fn i32_atomic_store16(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1A);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicStore8`].
    pub fn i64_atomic_store8(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1B);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicStore16`].
    pub fn i64_atomic_store16(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1C);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicStore32`].
    pub fn i64_atomic_store32(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1D);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmwAdd`].
    pub fn i32_atomic_rmw_add(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1E);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmwAdd`].
    pub fn i64_atomic_rmw_add(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x1F);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw8AddU`].
    pub fn i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x20);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw16AddU`].
    pub fn i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x21);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw8AddU`].
    pub fn i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x22);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw16AddU`].
    pub fn i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x23);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw32AddU`].
    pub fn i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x24);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmwSub`].
    pub fn i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x25);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmwSub`].
    pub fn i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x26);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw8SubU`].
    pub fn i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x27);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw16SubU`].
    pub fn i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x28);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw8SubU`].
    pub fn i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x29);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw16SubU`].
    pub fn i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2A);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw32SubU`].
    pub fn i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2B);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmwAnd`].
    pub fn i32_atomic_rmw_and(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2C);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmwAnd`].
    pub fn i64_atomic_rmw_and(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2D);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw8AndU`].
    pub fn i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2E);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw16AndU`].
    pub fn i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x2F);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw8AndU`].
    pub fn i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x30);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw16AndU`].
    pub fn i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x31);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw32AndU`].
    pub fn i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x32);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmwOr`].
    pub fn i32_atomic_rmw_or(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x33);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmwOr`].
    pub fn i64_atomic_rmw_or(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x34);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw8OrU`].
    pub fn i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x35);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw16OrU`].
    pub fn i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x36);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw8OrU`].
    pub fn i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x37);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw16OrU`].
    pub fn i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x38);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw32OrU`].
    pub fn i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x39);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmwXor`].
    pub fn i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3A);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmwXor`].
    pub fn i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3B);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw8XorU`].
    pub fn i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3C);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw16XorU`].
    pub fn i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3D);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw8XorU`].
    pub fn i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3E);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw16XorU`].
    pub fn i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x3F);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw32XorU`].
    pub fn i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x40);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmwXchg`].
    pub fn i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x41);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmwXchg`].
    pub fn i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x42);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw8XchgU`].
    pub fn i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x43);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw16XchgU`].
    pub fn i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x44);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw8XchgU`].
    pub fn i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x45);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw16XchgU`].
    pub fn i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x46);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw32XchgU`].
    pub fn i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x47);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmwCmpxchg`].
    pub fn i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x48);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmwCmpxchg`].
    pub fn i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x49);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw8CmpxchgU`].
    pub fn i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4A);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I32AtomicRmw16CmpxchgU`].
    pub fn i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4B);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw8CmpxchgU`].
    pub fn i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4C);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw16CmpxchgU`].
    pub fn i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4D);
        memarg.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64AtomicRmw32CmpxchgU`].
    pub fn i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4E);
        memarg.encode(self.sink);
        self
    }

    // More atomic instructions (the shared-everything-threads proposal)

    /// Encode [`Instruction::GlobalAtomicGet`].
    pub fn global_atomic_get(&mut self, ordering: Ordering, global_index: GlobalIdx) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x4F);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalAtomicSet`].
    pub fn global_atomic_set(&mut self, ordering: Ordering, global_index: GlobalIdx) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x50);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalAtomicRmwAdd`].
    pub fn global_atomic_rmw_add(
        &mut self,
        ordering: Ordering,
        global_index: GlobalIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x51);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalAtomicRmwSub`].
    pub fn global_atomic_rmw_sub(
        &mut self,
        ordering: Ordering,
        global_index: GlobalIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x52);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalAtomicRmwAnd`].
    pub fn global_atomic_rmw_and(
        &mut self,
        ordering: Ordering,
        global_index: GlobalIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x53);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalAtomicRmwOr`].
    pub fn global_atomic_rmw_or(
        &mut self,
        ordering: Ordering,
        global_index: GlobalIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x54);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalAtomicRmwXor`].
    pub fn global_atomic_rmw_xor(
        &mut self,
        ordering: Ordering,
        global_index: GlobalIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x55);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalAtomicRmwXchg`].
    pub fn global_atomic_rmw_xchg(
        &mut self,
        ordering: Ordering,
        global_index: GlobalIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x56);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::GlobalAtomicRmwCmpxchg`].
    pub fn global_atomic_rmw_cmpxchg(
        &mut self,
        ordering: Ordering,
        global_index: GlobalIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x57);
        ordering.encode(self.sink);
        global_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableAtomicGet`].
    pub fn table_atomic_get(&mut self, ordering: Ordering, table_index: TableIdx) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x58);
        ordering.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableAtomicSet`].
    pub fn table_atomic_set(&mut self, ordering: Ordering, table_index: TableIdx) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x59);
        ordering.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableAtomicRmwXchg`].
    pub fn table_atomic_rmw_xchg(
        &mut self,
        ordering: Ordering,
        table_index: TableIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5A);
        ordering.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::TableAtomicRmwCmpxchg`].
    pub fn table_atomic_rmw_cmpxchg(
        &mut self,
        ordering: Ordering,
        table_index: TableIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5B);
        ordering.encode(self.sink);
        table_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicGet`].
    pub fn struct_atomic_get(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5C);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicGetS`].
    pub fn struct_atomic_get_s(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5D);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicGetU`].
    pub fn struct_atomic_get_u(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5E);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicSet`].
    pub fn struct_atomic_set(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x5F);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicRmwAdd`].
    pub fn struct_atomic_rmw_add(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x60);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicRmwSub`].
    pub fn struct_atomic_rmw_sub(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x61);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicRmwAnd`].
    pub fn struct_atomic_rmw_and(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x62);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicRmwOr`].
    pub fn struct_atomic_rmw_or(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x63);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicRmwXor`].
    pub fn struct_atomic_rmw_xor(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x64);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicRmwXchg`].
    pub fn struct_atomic_rmw_xchg(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x65);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::StructAtomicRmwCmpxchg`].
    pub fn struct_atomic_rmw_cmpxchg(
        &mut self,
        ordering: Ordering,
        struct_type_index: TypeIdx,
        field_index: FieldIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x66);
        ordering.encode(self.sink);
        struct_type_index.encode(self.sink);
        field_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicGet`].
    pub fn array_atomic_get(&mut self, ordering: Ordering, array_type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x67);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicGetS`].
    pub fn array_atomic_get_s(
        &mut self,
        ordering: Ordering,
        array_type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x68);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicGetU`].
    pub fn array_atomic_get_u(
        &mut self,
        ordering: Ordering,
        array_type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x69);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicSet`].
    pub fn array_atomic_set(&mut self, ordering: Ordering, array_type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6A);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicRmwAdd`].
    pub fn array_atomic_rmw_add(
        &mut self,
        ordering: Ordering,
        array_type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6B);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicRmwSub`].
    pub fn array_atomic_rmw_sub(
        &mut self,
        ordering: Ordering,
        array_type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6C);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicRmwAnd`].
    pub fn array_atomic_rmw_and(
        &mut self,
        ordering: Ordering,
        array_type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6D);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicRmwOr`].
    pub fn array_atomic_rmw_or(
        &mut self,
        ordering: Ordering,
        array_type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6E);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicRmwXor`].
    pub fn array_atomic_rmw_xor(
        &mut self,
        ordering: Ordering,
        array_type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x6F);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicRmwXchg`].
    pub fn array_atomic_rmw_xchg(
        &mut self,
        ordering: Ordering,
        array_type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x70);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ArrayAtomicRmwCmpxchg`].
    pub fn array_atomic_rmw_cmpxchg(
        &mut self,
        ordering: Ordering,
        array_type_index: TypeIdx,
    ) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x71);
        ordering.encode(self.sink);
        array_type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::RefI31Shared`].
    pub fn ref_i31_shared(&mut self) -> &mut Self {
        self.sink.push(0xFE);
        self.sink.push(0x72);
        self
    }

    // Stack switching

    /// Encode [`Instruction::ContNew`].
    pub fn cont_new(&mut self, type_index: TypeIdx) -> &mut Self {
        self.sink.push(0xE0);
        type_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::ContBind`].
    pub fn cont_bind(&mut self, argument_index: TypeIdx, result_index: TypeIdx) -> &mut Self {
        self.sink.push(0xE1);
        argument_index.encode(self.sink);
        result_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::Suspend`].
    pub fn suspend(&mut self, tag_index: TagIdx) -> &mut Self {
        self.sink.push(0xE2);
        tag_index.encode(self.sink);
        self
    }

    /// Encode [`Instruction::Resume`].
    pub fn resume(
        &mut self,
        cont_type_index: TypeIdx,
        resume_table: impl IntoIterator<Item = Handle, IntoIter: ExactSizeIterator>,
    ) -> &mut Self {
        self.sink.push(0xE3);
        cont_type_index.encode(self.sink);
        encode_vec(resume_table, self.sink);
        self
    }

    /// Encode [`Instruction::ResumeThrow`].
    pub fn resume_throw(
        &mut self,
        cont_type_index: TypeIdx,
        tag_index: TagIdx,
        resume_table: impl IntoIterator<Item = Handle, IntoIter: ExactSizeIterator>,
    ) -> &mut Self {
        self.sink.push(0xE4);
        cont_type_index.encode(self.sink);
        tag_index.encode(self.sink);
        encode_vec(resume_table, self.sink);
        self
    }

    /// Encode [`Instruction::Switch`].
    pub fn switch(&mut self, cont_type_index: TypeIdx, tag_index: TagIdx) -> &mut Self {
        self.sink.push(0xE5);
        cont_type_index.encode(self.sink);
        tag_index.encode(self.sink);
        self
    }

    // Wide Arithmetic

    /// Encode [`Instruction::I64Add128`].
    pub fn i64_add128(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        19u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64Sub128`].
    pub fn i64_sub128(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        20u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64MulWideS`].
    pub fn i64_mul_wide_s(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        21u32.encode(self.sink);
        self
    }

    /// Encode [`Instruction::I64MulWideU`].
    pub fn i64_mul_wide_u(&mut self) -> &mut Self {
        self.sink.push(0xFC);
        22u32.encode(self.sink);
        self
    }
}
