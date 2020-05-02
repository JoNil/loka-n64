#version 450

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 0) out vec2 v_tex_coord;

layout(set = 0, binding = 0) uniform Locals {
    vec4 u_offset_and_scale;
};

void main() {

    v_tex_coord = a_tex_coord;

    vec2 offset = u_offset_and_scale.xy;
    vec2 scale = u_offset_and_scale.zw;

    gl_Position = vec4(vec3(scale*a_pos.xy + offset, a_pos.z), 1.0);
}
