mod tensor;

// Exported Structures
pub use tensor::Tensor;
pub use tensor::HorizontalIter as TensorHorizontalIter;
pub use tensor::VerticalIter as TensorVerticalIter;

// Exported Literals
pub use macros::tensor;

#[cfg(test)]
mod tests;