use std::ffi::CString;

pub struct Code {
    pub(in crate::accelerator) inner: Box<dyn CodeInner>
}

impl Code {
    #[inline]
    pub fn get_kernel<P: super::KernelParameter>(&self, kernel_name: impl Into<Vec<u8>>) -> crate::Result<super::Kernel<P>> {
        unsafe {
            std::mem::transmute(self.inner.get_kernel(
                CString::new(kernel_name)
                    .ok()
                    .ok_or(crate::Error::from("Failed conversion to C string"))?))
        }
    }
}

pub(in crate::accelerator) trait CodeInner: Drop {
    fn get_kernel(&self, kernel_name: CString) -> crate::Result<super::Kernel<()>>;
}

pub(in crate::accelerator) struct OpenClCode {
    pub(crate) open_cl: super::backend::OpenCl,
    pub(crate) program: usize,
}

impl CodeInner for OpenClCode {
    #[allow(invalid_value)]
    fn get_kernel(&self, kernel_name: CString) -> crate::Result<super::Kernel<()>> {
        Ok(super::Kernel {
                inner: Box::new(super::kernel::OpenClKernel {
                    open_cl: self.open_cl.clone(),
                    kernel: unsafe {
                        let mut error = std::mem::MaybeUninit::uninit().assume_init();
                        let kernel = self.open_cl.create_kernel(self.program, kernel_name.as_ptr(), &mut error);

                        error.to_result()?;
                        kernel
                    }
                })
            }
        )
    }
}

impl Drop for OpenClCode {
    fn drop(&mut self) {
        unsafe { self.open_cl.release_program(self.program).to_result().unwrap() }
    }
}