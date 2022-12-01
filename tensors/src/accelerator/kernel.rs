pub struct Kernel<P: KernelParameter> {
    pub(in crate::accelerator) inner: Box<dyn KernelInner<P>>
}

pub(in crate::accelerator) trait KernelInner<P: KernelParameter> {
    
}

pub(in crate::accelerator) struct OpenClKernel {
    pub(super) open_cl: super::backend::OpenCl,
    pub(super) kernel: usize,
}

impl<P: KernelParameter> KernelInner<P> for OpenClKernel {

}

pub trait KernelParameter {

}

impl KernelParameter for () {

}