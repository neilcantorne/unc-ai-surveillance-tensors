mod tensor;

// Exported Structures
pub use tensor::Tensor;

// Exported Literals
pub use macros::tensor;

#[cfg(test)]
mod tests;