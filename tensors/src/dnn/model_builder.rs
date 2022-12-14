use std::marker::PhantomData;

pub struct ModelBuilder {
    context: crate::accelerator::Context,
}

impl ModelBuilder {
    #[inline]
    pub fn load_binary_code(&self, binary: &[u8]) -> crate::Result<crate::accelerator::Code> {
        self.context.load_code(binary)
    }

    #[inline]
    pub fn new_buffer<'a, T: crate::accelerator::PushAsKernelArg + Default>(&'a mut self, value: T) -> LayerBuffer<'a, T> {
        todo!()
    }
}

pub struct LayerBuffer<'a, T: crate::accelerator::PushAsKernelArg> {
    reference: &'a T
}
