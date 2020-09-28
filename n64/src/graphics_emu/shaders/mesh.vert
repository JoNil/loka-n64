#version 460

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 2) in vec4 a_color;
layout(location = 0) out vec2 v_tex_coord;
layout(location = 1) out vec4 v_color;

struct Uniforms {
    mat4 u_transform;
};

layout(std430, set = 0, binding = 0) buffer Locals {
    Uniforms uniforms[];
};

void main() {
    v_tex_coord = a_tex_coord;
    v_color = a_color;
    gl_Position = uniforms[gl_InstanceIndex].u_transform * vec4(a_pos, 1.0);
}
