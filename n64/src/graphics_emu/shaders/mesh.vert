#version 460

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 2) in vec4 a_color;
layout(location = 0) out vec2 v_tex_coord;
layout(location = 1) out vec4 v_color;

struct Uniforms {
    mat4 u_transform;
    vec4 u_screen_size;
};

layout(std430, set = 0, binding = 0) readonly buffer Locals {
    Uniforms uniforms[];
};

void main() {
    v_tex_coord = a_tex_coord;
    v_color = a_color;
    vec4 position = uniforms[gl_InstanceIndex].u_transform * vec4(a_pos, 1.0);
    gl_Position =
        vec4(
            -1.0 + 2.0 * position.x / uniforms[gl_InstanceIndex].u_screen_size.x,
            1.0 - 2.0 * position.y / uniforms[gl_InstanceIndex].u_screen_size.y,
            position.zw);
}
