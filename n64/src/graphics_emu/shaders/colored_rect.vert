#version 450

layout(location = 0) in vec3 a_pos;

layout(set = 0, binding = 0) uniform Locals {
    vec4 u_color;
    vec4 u_offset_and_scale;
};

void main() {
    vec2 offset = u_offset_and_scale.xy;
    vec2 scale = u_offset_and_scale.zw;

    gl_Position = vec4(vec3(scale*a_pos.xy + offset, a_pos.z), 1.0);
}
