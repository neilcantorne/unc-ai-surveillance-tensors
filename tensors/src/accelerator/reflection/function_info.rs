pub struct FunctionInfo {
    pub name: super::Identifier,
    pub return_type: Option<super::TypeInfo>,
    pub parameters: super::ParameterInfo,
}

pub struct ParameterInfo {
    pub variant: super::ParameterVariants,
    pub is_mut: bool,
}

pub enum ParameterVariants {
    SelfParam { is_ref: bool },
    Typed { name: super::Identifier, param_type: super::TypeInfo}
}