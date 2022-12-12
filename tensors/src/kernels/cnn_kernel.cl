#include "rgb.h"

typedef struct {
    uint output_feature_stride;
    uint filter_feature_stride;
    uint stride_x;
    uint stride_y;
    uint stride_row;
    uint filter_width;
    uint filter_height;
    uint input_width;
    uint output_width;
} CnnParam;

kernel void convolve_rgb(
    const global Rgb* input,
    global Rgb* filter,
    global Rgb* output,
    CnnParam params) {
    const uint feature_index = get_global_id(0);
    const uint x = get_global_id(1);
    const uint y = get_global_id(2);

    const uint input_index = y * params.stride_x * params.input_width + x * params.stride_x;
    const uint output_index =
        feature_index * params.output_feature_stride + x + y * params.output_width;

    output[output_index] = rgb_zero();

    for (int iy = 0; iy < params.filter_height; iy++) {
        int filter_roffset = params.filter_width * iy;

        for (int ix = 0; ix < params.stride_x; ix++) {
            uint filter_index = ix + filter_roffset;
            output[output_index] = rgb_add(output[output_index],
                rgb_mul(filter[filter_index + params.filter_feature_stride],
                    input[filter_index + filter_roffset]));
        }
    }
}
