// Modules
mod variable;
mod identifier;
mod struct_info;
mod field_info;

// Exports
pub use variable::{
    VariableValue,
    Variable
};

pub use identifier::Identifier;
pub use struct_info::StructInfo;
pub use field_info::FieldInfo;