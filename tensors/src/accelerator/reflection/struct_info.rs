
#[derive(Debug, Clone)]
pub struct StructInfo {
    pub identifier: super::Identifier,
    pub fields: Vec<super::FieldInfo>
}

impl StructInfo {
    pub fn memory_size(&self) -> usize {
        self.fields.iter().fold(0usize, |total, field| {
            total + field.date_type.memory_size()
        })
    }
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