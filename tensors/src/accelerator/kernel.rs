use impl_trait_for_tuples::impl_for_tuples;

pub struct Kernel<P: KernelParameter> {
    pub(in crate::accelerator) inner: Box<dyn KernelInner<P>>
}

impl<P: KernelParameter> Kernel<P> {
    pub fn invoke(&self, args: P) -> crate::Result<()> {
        self.inner.invoke(args)
    }
}

pub(in crate::accelerator) trait KernelInner<P: KernelParameter> {
    fn invoke(&self, args: P) -> crate::Result<()>;
}

pub(in crate::accelerator) struct OpenClKernel {
    pub(super) open_cl: super::backend::OpenCl,
    pub(super) kernel: usize,
}

impl<P: KernelParameter> KernelInner<P> for OpenClKernel {
    fn invoke(&self, args: P) -> crate::Result<()> {
        let mut stack_inner = super::kernel_args_stack::OpenClKernelArgsStack {
            kernel: self,
            index_counter: 0,
        };

        args.push_args(&mut super::KernelArgsStack {
            inner: &mut stack_inner,
        });

        Ok(())
    }
}

pub trait PushAsKernelArg {
    fn push(&self, stack: &mut super::KernelArgsStack<'_>);
}

pub trait KernelParameter {
    fn push_args<'a>(&self, stack: &'a mut super::KernelArgsStack<'a>);
}

#[impl_for_tuples(0, 12)]
#[tuple_types_custom_trait_bound(PushAsKernelArg)]
impl KernelParameter for Tuple {
    #[inline]
    fn push_args<'a>(&self, stack: &'a mut super::KernelArgsStack<'a>) {
        for_tuples!( #( PushAsKernelArg::push(&self.Tuple, stack); )* );
    }
}
