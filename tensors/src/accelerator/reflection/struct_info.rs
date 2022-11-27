
#[derive(Debug, Clone)]
pub struct StructInfo {
    pub identifier: super::Identifier,
    pub fields: Vec<super::FieldInfo>
}

impl StructInfo {
    pub fn memory_size(&self, device: &crate::accelerator::Device) -> crate::Result<usize> {
        let mut accumulator = 0usize;

        for field in &self.fields {
            accumulator += field.date_type.memory_size(device)?
        }

        Ok(accumulator)
    }
}

pub trait AsKernelStruct : super::AsKernelType {
    fn struct_info() -> StructInfo;
    fn write_to_memory(self, writer: &mut super::ObjectWriter);
}

impl<T: AsKernelStruct> super::AsKernelType for T {
    #[inline]
    fn type_info() -> super::TypeInfo {
        super::TypeInfo::Struct(Self::struct_info())
    }

    #[inline]
    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        <T as AsKernelStruct>::write_to_memory(self, writer)
    }
}