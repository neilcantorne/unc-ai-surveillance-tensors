use super::backend::OpenCl;

pub struct Device {
    pub(crate) inner: Box<dyn DeviceInner>
}

pub(crate) trait DeviceInner {

}

impl Device {
    
}

pub(crate) struct OpenClDevice {
    pub open_cl: OpenCl,
    pub id: usize
}

impl DeviceInner for OpenClDevice {
    
}