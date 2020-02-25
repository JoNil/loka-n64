#version 450

layout(location = 0) out vec4 o_color;

layout(std430, set = 0, binding = 0) buffer Locals {
    vec4 u_color;
    vec3 u_offset_and_scale;
    float u_Scale;
};

void main() {
    o_color = u_color;
}
