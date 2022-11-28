use super::backend::OpenCl;

pub struct Context {
    pub(crate) inner: Box<dyn ContextInner>
}

pub(crate) trait ContextInner {
    
}

pub(crate) struct OpenContext {
    pub open_cl: OpenCl,
    pub context: usize,
    pub device: usize,
}