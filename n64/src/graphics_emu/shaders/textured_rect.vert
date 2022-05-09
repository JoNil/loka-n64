#version 460

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_tex_coord;

layout(location = 0) out vec2 v_tex_coord;
layout(location = 1) out vec4 v_color;
layout(location = 2) out flat uvec2 v_color_combiner_mode;
layout(location = 3) out flat uvec2 v_blend_mode;
layout(location = 4) out flat vec4 v_prim_color;
layout(location = 5) out flat vec4 v_env_color;
layout(location = 6) out flat vec4 v_blend_color;
layout(location = 7) out flat vec4 v_fog_color;

struct Uniforms {
    vec4 u_offset_and_scale;
    uvec2 u_color_combiner_mode;
    uvec2 u_blend_mode;
    vec4 u_prim_color;
    vec4 u_env_color;
    vec4 u_blend_color;
    vec4 u_fog_color;
};

layout(std430, set = 0, binding = 0) readonly buffer Locals {
    Uniforms uniforms[];
};

void main() {
    v_tex_coord = a_tex_coord;
    v_color = vec4(0.0);
    v_color_combiner_mode = uniforms[gl_InstanceIndex].u_color_combiner_mode;
    v_blend_mode = uniforms[gl_InstanceIndex].u_blend_mode;
    v_prim_color = uniforms[gl_InstanceIndex].u_prim_color;
    v_env_color = uniforms[gl_InstanceIndex].u_env_color;
    v_blend_color = uniforms[gl_InstanceIndex].u_blend_color;
    v_fog_color = uniforms[gl_InstanceIndex].u_fog_color;

    vec2 offset = uniforms[gl_InstanceIndex].u_offset_and_scale.xy;
    vec2 scale = uniforms[gl_InstanceIndex].u_offset_and_scale.zw;

    gl_Position = vec4(vec3(scale*a_pos.xy + offset, a_pos.z), 1.0);
}
