#version 460

layout(location = 0) in vec3 a_pos;
layout(location = 0) out flat uint v_instance_id;

struct Uniforms {
    vec4 u_color;
    vec4 u_offset_and_scale;
};

layout(std430, set = 0, binding = 0) readonly buffer Locals {
    Uniforms uniforms[];
};

void main() {

    v_instance_id = gl_InstanceIndex;

    vec2 offset = uniforms[gl_InstanceIndex].u_offset_and_scale.xy;
    vec2 scale = uniforms[gl_InstanceIndex].u_offset_and_scale.zw;

    gl_Position = vec4(vec3(scale*a_pos.xy + offset, a_pos.z), 1.0);
}
