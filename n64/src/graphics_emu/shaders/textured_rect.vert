#version 460

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 0) out vec2 v_tex_coord;
layout(location = 1) out vec4 v_blend_color;

struct Uniforms {
    vec4 u_offset_and_scale;
    vec4 u_blend_color;
};

layout(std430, set = 0, binding = 0) buffer Locals {
    Uniforms uniforms[];
};

void main() {

    v_tex_coord = a_tex_coord;
    v_blend_color = uniforms[gl_InstanceIndex].u_blend_color;

    vec2 offset = uniforms[gl_InstanceIndex].u_offset_and_scale.xy;
    vec2 scale = uniforms[gl_InstanceIndex].u_offset_and_scale.zw;

    gl_Position = vec4(vec3(scale*a_pos.xy + offset, a_pos.z), 1.0);
}
