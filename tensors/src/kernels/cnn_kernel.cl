#include "tensors/src/kernels/rgb.cl"

typedef struct {
    uint features_count;
    uint stride_x;
    uint stride_row;
    uint filter_width;
    uint filter_height;
    uint input_width;
    uint output_width;
} CnnParam;

__kernel void convolve_rgb(
    const global Rgb* input,
    global Rgb** filter,
    global Rgb** output,
    CnnParam params) {
    const uint feature_index = get_global_id(0);
    const uint x = get_global_id(1);
    const uint y = get_global_id(2);

    const uint input_index = y * params.input_width + x * params.stride_x;
    const uint output_index = x + y * params.output_width;
    
    output[feature_index][output_index] = rgb_zero();

    for (int iy = 0; iy < params.filter_height; iy++) {
        int filter_roffset = params.filter_width * iy;

        for (int ix = 0; ix < params.stride_x; ix++) {
            uint filter_index = ix + filter_roffset;
            rgb_add_assign(&output[feature_index][output_index],
                rgb_mul(filter[feature_index][filter_index],
                    input[filter_index + filter_roffset]));
        }
    }
}