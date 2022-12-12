#include "rgb.h"

Rgb rgb_mul(Rgb a, Rgb b) {
    return (Rgb) {
        .r = a.r * b.r,
        .g = a.g * b.g,
        .b = a.b * b.b,
    };
}

Rgb rgb_add(Rgb a, Rgb b) {
    return (Rgb) {
        .r = a.r * b.r,
        .g = a.g * b.g,
        .b = a.b * b.b,
    };
}

Rgb rgb_zero() {
    return (Rgb) {
        .r = 0,
        .g = 0,
        .b = 0,
    };
}
