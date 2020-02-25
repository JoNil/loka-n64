#version 450

layout(location = 0) in vec3 a_pos;

layout(std430, set = 0, binding = 0) buffer Locals {
    vec4 u_color;
    vec3 u_offset_and_scale;
    float u_Scale;
};

void main() {
    gl_Position = vec4(a_pos, 1.0);
}
