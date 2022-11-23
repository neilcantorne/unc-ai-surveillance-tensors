// Modules
mod variable;
mod identifier;
mod data_type;
mod struct_info;
mod field_info;

// Exports
pub use variable::{
    VariableValue,
    Variable
};

pub use identifier::Identifier;
pub use data_type::DataType;
pub use struct_info::StructInfo;
pub use field_info::FieldInfo;