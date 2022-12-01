use super::backend::OpenCl;

pub struct Context {
    pub(in crate::accelerator) inner: Box<dyn ContextInner>
}

impl Context {
    #[inline(always)]
    pub fn load_code(&self, code_data: &[u8]) -> crate::Result<super::Code> {
        self.inner.load_code(code_data)
    }
}

pub(in crate::accelerator) trait ContextInner: Drop {
    fn load_code(&self, code_data: &[u8]) -> crate::Result<super::Code>;
}

pub(in crate::accelerator) struct OpenContext {
    pub open_cl: OpenCl,
    pub context: usize,
    pub device: usize,
}

impl ContextInner for OpenContext {
    #[allow(invalid_value)]
    fn load_code(&self, code_data: &[u8]) -> crate::Result<super::Code> {
        Ok(super::Code {
            inner: Box::new(super::code::OpenClCode {
                open_cl: self.open_cl.clone(),
                program: unsafe {
                    let mut error = std::mem::MaybeUninit::uninit().assume_init();

                    let program = self.open_cl.create_program_with_il(
                        self.context,
                        code_data.as_ptr(),
                        code_data.len(),
                        &mut error);
                    
                    error.to_result()?;
                    program
                }
            })
        })
    }
}

impl Drop for OpenContext {
    fn drop(&mut self) {
        unsafe { self.open_cl.release_context(self.context); }
    }
}