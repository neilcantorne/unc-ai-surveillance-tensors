use crate::accelerator::{ PushAsKernelArg, KernelArgsStack };

#[derive(Clone, Copy)]
pub struct Rgb<T>
    where T: Sized + Default {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Default for Rgb<T>
    where T: Sized + Default {
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default() }
    }
}

impl<T: Sized + Default> PushAsKernelArg for Rgb<T> {
    fn push(&self, stack: &mut KernelArgsStack<'_>) {
        stack.push_c_buffer(
            self as *const Self as *const (),
            std::mem::size_of::<Self>())
    }
}
