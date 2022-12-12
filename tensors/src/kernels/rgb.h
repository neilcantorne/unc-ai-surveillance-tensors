typedef struct {
    float r;
    float g;
    float b;
} Rgb;


Rgb rgb_mul(Rgb a, Rgb b);

Rgb rgb_add(Rgb a, Rgb b);

Rgb rgb_zero();
