#version 430
layout(local_size_x = 1, local_size_y = 1) in;
layout(rgba32f, binding = 0) uniform image2D pixels;

struct Material {
    vec3 albedo;
    float fuzz;
};

struct Sphere {
    vec3 center;
    float radius;
    uint mat_i;
};

struct Camera {
    vec3 origin;
    vec3 lower_left_corner;
    vec3 horizontal;
    vec3 vertical;
};

uniform Material materials[4] = Material[4](
    Material(vec3(0.8, 0.3, 0.3), 0.0),
    Material(vec3(0.8, 0.8, 0.0), 0.0),
    Material(vec3(0.8, 0.6, 0.2), 1.0),
    Material(vec3(0.8, 0.8, 0.8), 0.3)
);

uniform Sphere objects[4] = Sphere[4](
    Sphere(vec3(0.0, 0.0, -1.0), 0.5, 0),
    Sphere(vec3(0.0, -100.5, -1.0), 100.0, 1),
    Sphere(vec3(1.0, 0.0, -1.0), 0.5, 2),
    Sphere(vec3(-1.0, 0.0, -1.0), 0.5, 3)
);

uniform Camera cam = Camera(
    vec3(-2.0, 2.0, 1.0),
    vec3(-2.0859715983286593, 1.1254692789725564, 0.05650831512990695),
    vec3(1.029463283198752, -0.0, 1.029463283198752),
    vec3(0.2971804518378177, 0.5943609036756354, -0.2971804518378177)
);

float InterleavedGradientNoise(ivec2 xy) {
  return fract(52.9829189f
              * fract(xy.x * 0.06711056f
                   + xy.y * 0.00583715f));
}

void main() {
    vec4 color = vec4(1.0, 0.0, 0.0, 1.0);
    imageStore(pixels, ivec2(gl_GlobalInvocationID.xy), color);
}
