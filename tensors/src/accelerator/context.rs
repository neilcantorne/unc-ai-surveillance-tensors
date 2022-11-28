pub struct Context {
    pub(crate) inner: Box<dyn ContextInner>
}

pub(crate) trait ContextInner {
    
}