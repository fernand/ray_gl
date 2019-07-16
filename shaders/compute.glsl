#version 430
layout(local_size_x = 1, local_size_y = 1) in;
layout(rgba32f, binding = 0) uniform image2D pixels;

float InterleavedGradientNoise(ivec2 xy) {
  return fract(52.9829189f
              * fract(xy.x * 0.06711056f
                   + xy.y * 0.00583715f));
}

void main() {
    vec4 pixel = vec4(1.0, 0.0, 0.0, 1.0);
    ivec2 pixel_coords = ivec2(gl_GlobalInvocationID.xy);
    imageStore(pixels, pixel_coords, pixel);
}
