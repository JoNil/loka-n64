#version 460

layout(location = 0) in flat uint v_instance_id;
layout(location = 1) in vec2 v_tex_coord;
layout(location = 0) out vec4 o_color;

layout(set = 0, binding = 1) uniform texture2DArray t_tex;
layout(set = 0, binding = 2) uniform sampler s_tex;

struct Uniforms {
    uvec4 u_texture;
    vec4 u_offset_and_scale;
};

layout(std430, set = 0, binding = 0) buffer Locals {
    Uniforms uniforms[];
};

void main() {
    o_color = texture(sampler2DArray(t_tex, s_tex), vec3(v_tex_coord, uniforms[v_instance_id].u_texture.x));
}
