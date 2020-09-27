#version 460

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 0) out vec2 v_tex_coord;
layout(location = 1) out vec4 v_blend_color;

struct Uniforms {
    mat4 u_transform;
    vec4 u_blend_color;
};

layout(std430, set = 0, binding = 0) buffer Locals {
    Uniforms uniforms[];
};

void main() {

    v_blend_color = uniforms[gl_InstanceIndex].u_blend_color;

    gl_Position = uniforms[gl_InstanceIndex].u_transform * vec4(a_pos, 1.0);
}
