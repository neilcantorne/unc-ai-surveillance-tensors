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