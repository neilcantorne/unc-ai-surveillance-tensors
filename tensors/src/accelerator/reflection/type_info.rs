
#[derive(Debug, Clone)]
pub enum TypeInfo {
    I8,
    I16,
    I32,
    I64,

    U8,
    U16,
    U32,
    U64,

    F32,
    F64,

    Struct(super::StructInfo),

    Array { element_type: Box<super::TypeInfo>, size: usize },
}