use std::mem::size_of;
use std::ptr::NonNull;
use std::alloc::{
    alloc,
    dealloc,
    Layout
};

pub struct ObjectWriter {
    ptr: NonNull<()>,
    last: *mut u8,
    current: NonNull<()>,
    layout: Layout,
}

impl ObjectWriter {
    pub(in crate::accelerator) fn new(size: usize, align: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, align);
            let ptr = NonNull::new(alloc(layout).cast())
                .expect("ObjectBuffer memory allocation failed");
            
            Self {
                ptr,
                layout,
                current: ptr,
                last: ptr
                    .as_ptr()
                    .cast::<u8>()
                    .add(size),
            }
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.layout.size()
    }

    #[inline]
    pub fn write<T: Sized + crate::reflection::AsKernelType>(&mut self, value: T) -> crate::Result<()> {
        unsafe {
            let next = self.current
            .as_ptr()
            .cast::<u8>()
            .add(size_of::<T>());

            if next > self.last {
                return Err(crate::Error::from("Memory overflow"));
            }

            Ok({
                *(self.current.cast::<T>().as_mut()) = value.into();
                self.current = NonNull::new_unchecked(next.cast())
            })
        }
    }

    #[inline]
    pub unsafe fn write_unchecked<T: Sized + crate::reflection::AsKernelType>(&mut self, value: T) {
        *(self.current.cast::<T>().as_mut()) = value.into();
        self.current = NonNull::new_unchecked(
            self.current
            .as_ptr()
            .cast::<T>()
            .add(1)
            .cast()
        );
    }
}

impl Drop for ObjectWriter {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr.as_ptr().cast(), self.layout)
        }
    }
}