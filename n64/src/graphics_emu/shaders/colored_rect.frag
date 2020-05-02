#version 450

layout(location = 0) out vec4 o_color;

layout(set = 0, binding = 0) uniform Locals {
    vec4 u_color;
    vec4 u_scale;
};

void main() {
    o_color = u_color;
}
