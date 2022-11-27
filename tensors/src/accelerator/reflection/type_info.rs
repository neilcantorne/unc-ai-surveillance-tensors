
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

    Ref(Box<super::TypeInfo>),
    MutRef(Box<super::TypeInfo>),

    Struct(super::StructInfo),

    Array { element_type: Box<super::TypeInfo>, size: usize },
}

impl TypeInfo {
    #[inline]
    pub fn memory_size(&self, device: &crate::accelerator::Device) -> crate::Result<usize> {
        match self {
            TypeInfo::I8 | TypeInfo::U8 => Ok(1),
            TypeInfo::I16 | TypeInfo::U16 => Ok(2),
            TypeInfo::I32 | TypeInfo::U32 | TypeInfo::F32  => Ok(4),
            TypeInfo::I64 | TypeInfo::U64 | TypeInfo::F64 => Ok(8),
            TypeInfo::Struct(info) => info.memory_size(device),
            TypeInfo::Array {
                element_type,
                size
            } => Ok(element_type.memory_size(device)? * size),
            TypeInfo::Ref(_) | TypeInfo::MutRef(_) => device.pointer_size(),
        }
    }
}

pub trait AsKernelType {
    fn type_info() -> TypeInfo;
    fn write_to_memory(self, writer: &mut super::ObjectWriter);
}

impl AsKernelType for i8 {
    fn type_info() -> TypeInfo { TypeInfo::I8 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl AsKernelType for i16 {
    fn type_info() -> TypeInfo { TypeInfo::I16 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl AsKernelType for i32 {
    fn type_info() -> TypeInfo { TypeInfo::I32 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl AsKernelType for i64 {
    fn type_info() -> TypeInfo { TypeInfo::I64 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl AsKernelType for u8 {
    fn type_info() -> TypeInfo { TypeInfo::U8 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl AsKernelType for u16 {
    fn type_info() -> TypeInfo { TypeInfo::U16 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl AsKernelType for u32 {
    fn type_info() -> TypeInfo { TypeInfo::U32 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl AsKernelType for u64 {
    fn type_info() -> TypeInfo { TypeInfo::U64 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl AsKernelType for f32 {
    fn type_info() -> TypeInfo { TypeInfo::F32 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl AsKernelType for f64 {
    fn type_info() -> TypeInfo { TypeInfo::F64 }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl<T: AsKernelType, const SIZE: usize> AsKernelType for [T; SIZE] {
    fn type_info() -> TypeInfo {
        TypeInfo::Array {
            element_type: Box::new(T::type_info()),
            size: SIZE
        }
    }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl<'a, T> AsKernelType for &'a T
    where T: Sized + AsKernelType {
    fn type_info() -> TypeInfo { TypeInfo::Ref(Box::new(T::type_info())) }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}

impl<'a, T> AsKernelType for &'a mut T
    where T: Sized + AsKernelType {
    fn type_info() -> TypeInfo { TypeInfo::Ref(Box::new(T::type_info())) }

    fn write_to_memory(self, writer: &mut super::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}