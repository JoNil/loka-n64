#version 450

layout(location = 0) in vec2 v_tex_coord;
layout(location = 0) out vec4 o_color;

layout(set = 0, binding = 1) uniform texture2D t_tex;
layout(set = 0, binding = 2) uniform sampler s_tex;

void main() {
    o_color = texture(sampler2D(t_tex, s_tex), v_tex_coord);
}
