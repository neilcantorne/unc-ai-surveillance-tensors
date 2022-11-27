#include "tensors/src/kernels/rgb.cl"

__kernel void convolve_rgb(
    const global Rgb* input,
    global Rgb* filter,
    global Rgb* output) {
    const int idx = get_global_id(0);
    output[idx] = rgb_mul(input[idx], filter[idx]);
}