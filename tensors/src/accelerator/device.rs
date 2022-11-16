use super::backend::{OpenCl, ParamName};

pub struct Device {
    pub(in crate::accelerator) inner: Box<dyn DeviceInner>
}

pub(crate) trait DeviceInner {
    fn name(&self) -> crate::Result<String>;
    fn device_type(&self) -> crate::Result<DeviceType>;
    fn vendor(&self) -> crate::Result<String>;
}

impl Device {
    #[inline(always)]
    pub fn name(&self) -> crate::Result<String> {
        self.inner.name()
    }

    #[inline(always)]
    pub fn device_type(&self) -> crate::Result<DeviceType> {
        self.inner.device_type()
    }

    #[inline(always)]
    pub fn vendor(&self) -> crate::Result<String> {
        self.inner.vendor()
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

    #[allow(invalid_value)]
    fn device_type(&self) -> crate::Result<DeviceType> {
        unsafe {
            let mut buffer = std::mem::MaybeUninit::uninit().assume_init();

            self.open_cl.get_device_info(
                self.id,
                ParamName::DeviceType,
                std::mem::size_of::<super::backend::DeviceType>(),
                &mut buffer as *mut super::backend::DeviceType as *mut (),
                std::ptr::null_mut())
            .to_result()?;

            match buffer {
                super::backend::DeviceType::Cpu => Ok(DeviceType::Cpu),
                super::backend::DeviceType::Gpu => Ok(DeviceType::Gpu),
                super::backend::DeviceType::Accelerator => Ok(DeviceType::Accelerator),
                _ => Err(crate::Error::from("Invalid Device Type")),
            }
        }
    }

    #[allow(invalid_value)]
    fn vendor(&self) -> crate::Result<String> {
        unsafe {
            let mut buffer = super::ArrayBuffer::<i8>::new({
                let mut size = 0usize;
            
                self.open_cl.get_device_info(
                    self.id,
                    ParamName::DeviceVendor,
                    0,
                    std::ptr::null_mut(),
                    &mut size)
                    .to_result()?;
                
                size
            });

            self.open_cl.get_device_info(
                self.id,
                ParamName::DeviceVendor,
                buffer.size(),
                buffer.as_mut_ptr() as *mut (),
                std::ptr::null_mut())
            .to_result()?;
            
            buffer.rust_string()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DeviceType {
    Cpu,
    Gpu,
    Accelerator,
}

impl std::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(&self, f)
    }
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Cpu => f.write_str("CPU"),
            DeviceType::Gpu => f.write_str("GPU"),
            DeviceType::Accelerator => f.write_str("Accelerator"),
        }
    }
}