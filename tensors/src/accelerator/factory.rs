use super::{
    Device,
    backend::{ OpenClFactory, DeviceType }, 
    device::OpenClDevice 
};

pub struct Factory {
    open_cl: Option<OpenClFactory> 
}

impl Factory {
    pub fn new() -> Self {
        Self { open_cl: None }
    }

    pub fn get_devices(&mut self) -> crate::Result<Vec<Device>> {
        if self.open_cl.is_none() {
            self.open_cl = Some(OpenClFactory::new()?)
        }

        let open_cl = self.open_cl
            .as_mut()
            .unwrap()
            .create();
        
        let num_devices = unsafe {
            let mut buffer = 0;
            
            open_cl.get_device_id(0usize,
                DeviceType::All,
                0,
                std::ptr::null_mut(),
                &mut buffer)
                .to_result()?;
            
            buffer as usize
        };

        let devices = unsafe {
            let mut ids = Vec::with_capacity(num_devices);
            ids.set_len(num_devices);

            open_cl.get_device_id(0,
                DeviceType::All,
                num_devices as u32,
                ids.as_mut_ptr(),
                std::ptr::null_mut())
                .to_result()?;

            ids.iter().map(|id| {
                Device {
                    inner: Box::new(OpenClDevice {
                        open_cl: open_cl.clone(),
                        id: *id,
                    })
                }
            }).collect()
        };

        Ok(devices)
    }
}