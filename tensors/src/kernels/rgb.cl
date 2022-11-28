typedef struct {
    float r;
    float g;
    float b;
} Rgb;

inline Rgb rgb_mul(Rgb a, Rgb b) {
    return (Rgb) {
        .r = a.r * b.r,
        .g = a.g * b.g,
        .b = a.b * b.b,
    };
}

inline void rgb_add_assign(Rgb* a, Rgb b) {
    *a = (Rgb) {
        .r = a->r * b.r,
        .g = a->g * b.g,
        .b = a->b * b.b,
    };
}

inline Rgb rgb_zero() {
    return (Rgb) {
        .r = 0,
        .g = 0,
        .b = 0,
    };
}