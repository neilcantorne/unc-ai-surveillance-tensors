use super::backend::OpenClFactory;

pub struct Factory {
    open_cl: Option<OpenClFactory> 
}

impl Factory {
    pub fn new() -> Self {
        Self { open_cl: None }
    }
}