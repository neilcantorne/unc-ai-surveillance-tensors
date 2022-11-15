// Modules
mod device;
mod factory;
mod backend;
pub(crate) mod reflection;
mod array_buffer;

// Exported structs
pub use device::Device;
pub use factory::Factory;

pub(crate) use backend::BackendError;
pub(crate) use backend::OpenClErrorCode;
pub(self) use array_buffer::ArrayBuffer;