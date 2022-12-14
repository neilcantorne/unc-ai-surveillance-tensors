use crate::accelerator::PushAsKernelArg;

pub trait LayerBuilder where Self::Input: PushAsKernelArg, Self::Output: PushAsKernelArg {
    type Input;
    type Output;

    fn build<'a>(&self,
        builder: &'a mut super::ModelBuilder,
        input: super::LayerBuffer<'a, Self::Input>)
        -> crate::Result<super::LayerBuffer<'a, Self::Output>>;
}
