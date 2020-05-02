#version 450

layout(location = 0) in vec2 v_tex_coord;
layout(location = 0) out vec4 o_color;

layout(set = 0, binding = 0) uniform texture2D t_color;
layout(set = 0, binding = 1) uniform sampler s_color;

void main() {
    o_color = texture(sampler2D(t_color, s_color), v_tex_coord);
}
