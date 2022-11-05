mod tensor;
mod tensor_ops;

// Exported Structures
pub use tensor::Tensor;
pub use tensor::HorizontalIter as TensorHorizontalIter;
pub use tensor::VerticalIter as TensorVerticalIter;

// Exported modules
pub mod ops { pub use super::tensor_ops::*; }
pub mod gpu;

// Exported Literals
pub use macros::tensor;

#[cfg(test)]
mod tests;