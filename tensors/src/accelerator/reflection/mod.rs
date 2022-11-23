// Modules
mod variable;
mod struct_info;
mod identifier;

// Exports
pub use variable::{
    VariableValue,
    Variable
};

pub use struct_info::StructInfo;
pub use identifier::Identifier;