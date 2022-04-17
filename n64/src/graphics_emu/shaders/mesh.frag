#version 460

layout(location = 0) in vec2 v_tex_coord;
layout(location = 1) in vec4 v_color;
layout(location = 0) out vec4 o_color;

struct Uniforms {
    mat4 u_transform;
    vec2 u_screen_size;
    ivec2 u_combine_mode,
    vec4 u_prim_color;
    vec4 u_env_color;
};

layout(std430, set = 0, binding = 0) readonly buffer Locals {
    Uniforms uniforms[];
};

layout(set = 0, binding = 1) uniform texture2D t_tex;
layout(set = 0, binding = 2) uniform sampler s_tex;

void main() {

    vec4 shade_color = v_color;
    vec4 texel_color = texture(sampler2D(t_tex, s_tex), v_tex_coord);
    vec4 prim_color = uniforms[gl_InstanceIndex].u_prim_color;
    vec4 env_color = uniforms[gl_InstanceIndex].u_env_color;

    vec4 a = vec4(0.0);
    vec4 b = vec4(0.0);
    vec4 c = vec4(0.0);
    vec4 d = vec4(0.0);

    o_color.rgb = (a.rgb - b.rgb) * c.rgb + d.rgb;
    o_color.a = (a.a - b.a) * c.a + d.a
}
