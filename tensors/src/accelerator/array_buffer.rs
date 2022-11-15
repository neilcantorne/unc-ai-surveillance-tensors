use std::alloc::Layout;

pub struct ArrayBuffer<T: Sized> {
    ptr: *mut T,
    layout: Layout
}