pub struct Kernel<P: KernelParameter> {
    inner: Box<dyn KernelInner<P>>
}

pub(crate) trait KernelInner<P: KernelParameter> {
    
}

pub trait KernelParameter {

}