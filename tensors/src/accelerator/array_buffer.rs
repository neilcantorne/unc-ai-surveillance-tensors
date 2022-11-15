use std::alloc::Layout;

pub struct ArrayBuffer<T: Sized> {
    ptr: *mut T,
    layout: Layout
}

impl<T: Sized> ArrayBuffer<T> {
    pub fn new(size: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align_unchecked(
                size * std::mem::size_of::<T>(),
                std::mem::size_of::<T>());
            let ptr = std::alloc::alloc(layout) as *mut T;

            Self { layout, ptr }
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.layout.size() / std::mem::size_of::<T>()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }
}

impl<T: Sized> Drop for ArrayBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, self.layout)
        }
    }
}

impl ArrayBuffer<i8> {
    pub fn rust_string(&self) -> crate::Result<String> {
        Ok(unsafe {
            std::ffi::CStr::from_ptr(self.ptr)
            .to_str()
            .ok()
            .ok_or(crate::Error::from("Failed to convert from C-String".to_owned()))?
            .to_owned()
        })
    }
}