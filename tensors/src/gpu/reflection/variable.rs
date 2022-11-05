
pub struct Variable {
    pub identifier: String,
    pub value: VariableValue,
}

pub enum VariableValue {
    Uninitialized,

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    F32(f32),
    F64(f64),

    Array(Box<[VariableValue]>),
}