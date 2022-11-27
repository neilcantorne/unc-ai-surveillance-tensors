// Modules
mod variable;
mod identifier;
mod type_info;
mod struct_info;
mod field_info;
mod function_info;
mod implementation_info;
mod object_writer;

// Exports
pub use variable::{
    VariableValue,
    Variable
};

pub use identifier::Identifier;
pub use object_writer::ObjectWriter;
pub use type_info::{
    TypeInfo,
    AsKernelType
};
pub use struct_info::StructInfo;
pub use field_info::FieldInfo;
pub use function_info::{
    FunctionInfo,
    ParameterInfo,
    ParameterVariants
};
pub use implementation_info::ImplementationInfo;