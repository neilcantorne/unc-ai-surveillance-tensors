use crate::accelerator::reflection::{
    AsKernelStruct,
    AsKernelType,
    StructInfo,
    Identifier,
    FieldInfo
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

impl<T> AsKernelStruct for Rgb<T>
    where T: Sized + Default + AsKernelType {
    fn struct_info() -> StructInfo {
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
    }
}