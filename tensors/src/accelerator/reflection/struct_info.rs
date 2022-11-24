
#[derive(Debug, Clone)]
pub struct StructInfo {
    pub identifier: super::Identifier,
    pub fields: Vec<super::FieldInfo>
}

pub trait AsKernelStruct : super::AsKernelType {
    fn struct_info() -> StructInfo;
}

impl<T: AsKernelStruct> super::AsKernelType for T {
    #[inline]
    fn type_info() -> super::TypeInfo {
        super::TypeInfo::Struct(Self::struct_info())
    }
}