use std::fmt::{ Debug, Formatter };

use crate::accelerator::BackendError;

#[derive(Clone)]
pub struct Error(pub(crate) ErrorVariants);
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub(crate) enum ErrorVariants {
    Message(String),
    BackendError(crate::accelerator::BackendError),
    OpenClError(crate::accelerator::OpenClErrorCode)
}

impl From<BackendError> for Error {
    fn from(error: BackendError) -> Self {
        Self(ErrorVariants::BackendError(error))
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self(ErrorVariants::Message(message))
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self(ErrorVariants::Message(message.to_owned()))
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            ErrorVariants::BackendError(backend_error)
                => <BackendError as Debug>::fmt(backend_error, f),
            ErrorVariants::OpenClError(open_cl_error_code)
                => <crate::accelerator::OpenClErrorCode as Debug>::fmt(open_cl_error_code, f),
            ErrorVariants::Message(message) => f.write_str(message),
        }
    }
}