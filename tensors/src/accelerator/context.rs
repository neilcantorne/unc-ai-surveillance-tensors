use super::backend::OpenCl;

pub struct Context {
    pub(crate) inner: Box<dyn ContextInner>
}

pub(crate) trait ContextInner: Drop {
    
}

pub(crate) struct OpenContext {
    pub open_cl: OpenCl,
    pub context: usize,
    pub device: usize,
}

impl ContextInner for OpenContext {
    
}

impl Drop for OpenContext {
    fn drop(&mut self) {
        unsafe { self.open_cl.release_context(self.context); }
    }
}