#[derive(Clone, Copy)]
pub struct Tensor3d<T, const L: usize, const R: usize, const C: usize>([[[T; C]; R]; L]);

impl<T, const L: usize, const R: usize, const C: usize>
    crate::accelerator::PushAsKernelArg for Tensor3d<T, L, R, C> {
    fn push(&self, stack: &mut crate::accelerator::KernelArgsStack<'_>) {
        let address = self as *const Self as *const ();
        stack.push_c_buffer(address, std::mem::size_of::<Self>())
    }
}

impl<T: Default + Copy, const L: usize, const R: usize, const C: usize> Default for Tensor3d<T, L, R, C> {
    fn default() -> Self {
        Self([[[T::default(); C]; R]; L])
    }
}
