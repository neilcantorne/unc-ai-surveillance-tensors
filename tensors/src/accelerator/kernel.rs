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

pub trait AsKernelArg {

}

pub trait KernelParameter { }

impl KernelParameter for () { }

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: AsKernelArg),+> KernelParameter for ($($name,)+) { }
    }
}

tuple_impls! { T1 }
tuple_impls! { T1 T2  }
tuple_impls! { T1 T2  T3 }
tuple_impls! { T1 T2  T3 T4 }
tuple_impls! { T1 T2  T3 T4 T5 }
tuple_impls! { T1 T2  T3 T4 T5 T6 }
tuple_impls! { T1 T2  T3 T4 T5 T6 T7 }
tuple_impls! { T1 T2  T3 T4 T5 T6 T7 T8 }
tuple_impls! { T1 T2  T3 T4 T5 T6 T7 T8 T9 }
tuple_impls! { T1 T2  T3 T4 T5 T6 T7 T8 T9 T10}
tuple_impls! { T1 T2  T3 T4 T5 T6 T7 T8 T9 T10 T11 }
tuple_impls! { T1 T2  T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 }
