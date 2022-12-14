use crate::{
    Rgb,
    accelerator::{ PushAsKernelArg, KernelArgsStack }, Tensor, Tensor3d
};

pub struct ConvolutionalLayer<const FILTER_COUNT: usize,
    const FILER_WIDTH: usize, const FILTER_HEIGHT: usize,
    const INPUT_WIDTH: usize, const INPUT_HEIGHT: usize,
    const OUTPUT_WIDTH: usize, const OUTPUT_HEIGHT: usize> {

}

struct CnnParam {
    output_feature_stride: u32,
    filter_feature_stride: u32,
    stride_x: u32,
    stride_y: u32,
    stride_row: u32,
    filter_width: u32,
    filter_height: u32,
    input_width: u32,
    output_width: u32,
}

impl PushAsKernelArg for CnnParam {
    fn push(&self, stack: &mut KernelArgsStack<'_>) {
        let address = self as *const Self as *const ();
        stack.push_c_buffer(address, std::mem::size_of::<Self>())
    }
}

impl<const FC: usize,
    const FW: usize, const FH: usize,
    const IW: usize, const IH: usize,
    const OW: usize, const OH: usize>
    crate::dnn::LayerBuilder for ConvolutionalLayer<FC, FW, FH, IW, IH, OH, OW> {
    type Input = Tensor<Rgb<f32>, IH, IW>;
    type Output = Tensor3d<Rgb<f32>, FC, OH, IW>;

    fn build<'a>(&self,
        builder: &'a mut crate::dnn::ModelBuilder,
        input: crate::dnn::LayerBuffer<'a, Self::Input>)
        -> crate::Result<crate::dnn::LayerBuffer<'a, Self::Output>> {
        let code = builder.load_binary_code(include_bytes!("../../kernels/spirv/cnn_kernel.spv"))?;
        let output = builder.new_buffer(Tensor3d::<Rgb<f32>, FC, OH, IW>::default());
        let kernel = code.get_kernel::<(CnnParam, Rgb<f32>)>("")?;

        Ok(output)
    }

}
