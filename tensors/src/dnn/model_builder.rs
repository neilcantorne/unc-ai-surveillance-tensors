pub struct ModelBuilder {
    context: crate::accelerator::Context,
}

impl ModelBuilder {
    #[inline]
    pub fn load_binary_code(&self, binary: &[u8]) -> crate::Result<crate::accelerator::Code> {
        self.context.load_code(binary)
    }
}
