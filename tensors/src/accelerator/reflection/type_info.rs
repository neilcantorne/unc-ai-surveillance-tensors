
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

pub trait AsKernelType {
    fn type_info() -> TypeInfo;
}

impl AsKernelType for i8 {
    fn type_info() -> TypeInfo { TypeInfo::I8 }
}

impl AsKernelType for i16 {
    fn type_info() -> TypeInfo { TypeInfo::I16 }
}

impl AsKernelType for i32 {
    fn type_info() -> TypeInfo { TypeInfo::I32 }
}

impl AsKernelType for i64 {
    fn type_info() -> TypeInfo { TypeInfo::I64 }
}

impl AsKernelType for u8 {
    fn type_info() -> TypeInfo { TypeInfo::U8 }
}

impl AsKernelType for u16 {
    fn type_info() -> TypeInfo { TypeInfo::U16 }
}

impl AsKernelType for u32 {
    fn type_info() -> TypeInfo { TypeInfo::U32 }
}

impl AsKernelType for u64 {
    fn type_info() -> TypeInfo { TypeInfo::U64 }
}

impl AsKernelType for f32 {
    fn type_info() -> TypeInfo { TypeInfo::F32 }
}

impl AsKernelType for f64 {
    fn type_info() -> TypeInfo { TypeInfo::F64 }
}

impl<T: AsKernelType, const SIZE: usize> AsKernelType for [T; SIZE] {
    fn type_info() -> TypeInfo {
        TypeInfo::Array {
            element_type: Box::new(T::type_info()),
            size: SIZE
        }
    }
}