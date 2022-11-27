use crate::reflection::{
    AsKernelType,
    StructInfo,
    Identifier,
    FieldInfo,
    TypeInfo
};

pub struct Rgb<T>
    where T: Sized + Default {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Default for Rgb<T>
    where T: Sized + Default {
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default() }
    }
}

impl<T> AsKernelType for Rgb<T>
    where T: Sized + Default + AsKernelType {
    fn type_info() -> TypeInfo {
        TypeInfo::Struct(
            StructInfo {
                identifier: Identifier::from("Rgb"),
                fields: {
                    let mut fields = Vec::with_capacity(3);
                    
                    fields.push(FieldInfo {
                        identifier: Identifier::from("r"),
                        date_type: <T as AsKernelType>::type_info()
                    });
    
                    fields.push(FieldInfo {
                        identifier: Identifier::from("g"),
                        date_type: <T as AsKernelType>::type_info()
                    });
    
                    fields.push(FieldInfo {
                        identifier: Identifier::from("b"),
                        date_type: <T as AsKernelType>::type_info()
                    });
    
                    fields
                },
            }
        )
    }

    fn write_to_memory(self, writer: &mut crate::accelerator::reflection::ObjectWriter) {
        unsafe { writer.write_unchecked(self); }
    }
}