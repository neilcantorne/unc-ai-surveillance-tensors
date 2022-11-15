use std::alloc::Layout;

pub struct ArrayBuffer<T: Sized> {
    ptr: *mut T,
    layout: Layout
}

impl<T: Sized> ArrayBuffer<T> {
    pub fn new(size: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, std::mem::size_of::<T>());
            let ptr = std::alloc::alloc(layout) as *mut T;

            Self { layout, ptr }
        }
    }
}

impl<T: Sized> Drop for ArrayBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, self.layout)
        }
    }
}