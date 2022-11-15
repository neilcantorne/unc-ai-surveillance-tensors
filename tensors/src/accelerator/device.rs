use super::backend::{OpenCl, ParamName};

pub struct Device {
    pub(in crate::accelerator) inner: Box<dyn DeviceInner>
}

pub(crate) trait DeviceInner {
    fn name(&self) -> crate::Result<String>;
}

impl Device {
    #[inline(always)]
    pub fn name(&self) -> crate::Result<String> {
        self.inner.name()
    }
}

pub(crate) struct OpenClDevice {
    pub open_cl: OpenCl,
    pub id: usize
}

impl DeviceInner for OpenClDevice {
    fn name(&self) -> crate::Result<String> {
        unsafe {
            let mut buffer = super::ArrayBuffer::<i8>::new({
                let mut size = 0usize;
            
                self.open_cl.get_device_info(
                    self.id,
                    ParamName::DeviceName,
                    0,
                    std::ptr::null_mut(),
                    &mut size)
                    .to_result()?;
                
                size
            });
            
            self.open_cl.get_device_info(
                self.id,
                ParamName::DeviceName,
                buffer.size(),
                buffer.as_mut_ptr() as *mut (),
                std::ptr::null_mut())
                .to_result()?;

            buffer.rust_string()
        }
    }
}