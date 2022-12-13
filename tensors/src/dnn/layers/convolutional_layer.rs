use crate::Rgb;

pub struct ConvolutionalLayer<const WIDTH: u32, const HEIGHT: u32> {

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

impl crate::accelerator::AsKernelArg for CnnParam {

}

impl<const WIDTH: u32, const HEIGHT: u32> crate::dnn::LayerBuilder for ConvolutionalLayer<WIDTH,  HEIGHT> {
    fn build(&self, builder: &mut crate::dnn::ModelBuilder) -> crate::Result<()> {
        let code = builder.load_binary_code(include_bytes!("../../kernels/spirv/cnn_kernel.spv"))?;
        let kernel = code.get_kernel::<(CnnParam, Rgb<f32>)>("")?;
        Ok(())
    }
}
