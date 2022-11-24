use std::ptr::NonNull;
use std::alloc::{
    alloc,
    dealloc,
    Layout
};

pub struct ObjectWriter {
    ptr: NonNull<u8>,
    layout: Layout,
}

impl ObjectWriter {
    pub(in crate::accelerator) fn new(size: usize, align: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, align);
            Self {
                layout,
                ptr: NonNull::new(alloc(layout))
                    .expect("ObjectBuffer memory allocation failed")
            }
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.layout.size()
    }
}

impl Drop for ObjectWriter {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr.as_mut(), self.layout)
        }
    }
}