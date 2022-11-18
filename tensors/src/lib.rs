mod tensor;
mod rgb;
mod tensor_ops;
mod error;

// Exported Structures
pub use tensor::Tensor;
pub use rgb::Rgb;
pub use tensor::HorizontalIter as TensorHorizontalIter;
pub use tensor::VerticalIter as TensorVerticalIter;
pub use error::{ Error, Result };

// Exported modules
pub mod ops { pub use super::tensor_ops::*; }
pub mod accelerator;

// Exported Literals
pub use macros::tensor;

#[cfg(test)]
mod tests;