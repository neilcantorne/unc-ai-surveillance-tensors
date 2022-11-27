
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