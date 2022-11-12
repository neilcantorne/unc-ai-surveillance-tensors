use std::fmt::{ Debug, Formatter };

use crate::accelerator::BackendError;

#[derive(Clone)]
pub struct Error(ErrorVariants);
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub(crate) enum ErrorVariants {
    BackendError(crate::accelerator::BackendError),
}

impl From<BackendError> for Error {
    fn from(error: BackendError) -> Self {
        Self(ErrorVariants::BackendError(error))
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            ErrorVariants::BackendError(backend_error)
                => <BackendError as Debug>::fmt(backend_error, f),
        }
    }
}