#version 450

layout(location = 0) out vec4 o_Color;

layout(set = 0, binding = 0) uniform Locals {
    vec4 u_Color;
};

void main() {
    o_Color = u_Color;
}
