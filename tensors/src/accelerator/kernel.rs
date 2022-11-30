pub struct Kernel<P: KernelParameter> {
    inner: Box<dyn KernelInner<P>>
}

pub(crate) trait KernelInner<P: KernelParameter> {
    
}

pub(crate) struct OpenClKernel {
    pub(super) open_cl: super::backend::OpenCl,
    pub(super) kernel: usize,
}

impl<P: KernelParameter> KernelInner<P> for OpenClKernel {

}

pub trait KernelParameter {

}