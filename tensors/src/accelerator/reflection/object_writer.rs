use std::ptr::NonNull;
use std::alloc::Layout;

pub struct ObjectWriter {
    ptr: NonNull<u8>,
    layout: Layout,
}