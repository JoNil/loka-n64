#version 460

layout(location = 0) in flat uint v_instance_id;
layout(location = 0) out vec4 o_color;

struct Uniforms {
    vec4 u_color;
    vec4 u_offset_and_scale;
};

layout(std430, set = 0, binding = 0) buffer Locals {
    Uniforms uniforms[];
};

void main() {
    o_color = uniforms[v_instance_id].u_color;
}
