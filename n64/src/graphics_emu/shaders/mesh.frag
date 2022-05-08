#version 460

layout(location = 0) in vec2 v_tex_coord;
layout(location = 1) in vec4 v_color;

layout(location = 2) in flat uvec2 v_color_combiner_mode;
layout(location = 3) in flat uvec2 v_blend_mode;
layout(location = 4) in flat vec4 v_prim_color;
layout(location = 5) in flat vec4 v_env_color;
layout(location = 6) in flat vec4 v_blend_color;
layout(location = 7) in flat vec4 v_fog_color;

layout(location = 0) out vec4 o_color;

layout(set = 0, binding = 1) uniform texture2D t_tex;
layout(set = 0, binding = 2) uniform sampler s_tex;

void main() {

    vec4 shade_color = v_color;
    vec4 texel_color = texture(sampler2D(t_tex, s_tex), v_tex_coord);
    vec4 prim_color = v_prim_color;
    vec4 env_color = v_env_color;
    vec4 blend_color = v_blend_color;
    vec4 fog_color = v_fog_color;

    uvec2 color_combiner_mode = v_color_combiner_mode;

    vec4 a = vec4(0.0);
    vec4 b = vec4(0.0);
    vec4 c = vec4(0.0);
    vec4 d = vec4(0.0);

    switch ((color_combiner_mode.x >> (52 - 32)) & 0xf) {
        case 1:
            a.rgb = texel_color.rgb;
            break;
        case 3:
            a.rgb = prim_color.rgb;
            break;
        case 4:
            a.rgb = shade_color.rgb;
            break;
        case 5:
            a.rgb = env_color.rgb;
            break;
        case 7:
            // Implement noise
            a.rgb = vec3(1.0);
            break;
        case 6:
            a.rgb = vec3(1.0);
            break;
        case 8:
            a.rgb = vec3(0.0);
            break;
        default:
            break;
    }

    switch ((color_combiner_mode.y >> 28) & 0xf) {
        case 1:
            b.rgb = texel_color.rgb;
            break;
        case 3:
            b.rgb = prim_color.rgb;
            break;
        case 4:
            b.rgb = shade_color.rgb;
            break;
        case 5:
            b.rgb = env_color.rgb;
            break;
        case 8:
            b.rgb = vec3(0.0);
            break;
        default:
            break;
    }

    switch ((color_combiner_mode.x >> (47 - 32)) & 0x1f) {
        case 1:
            c.rgb = texel_color.rgb;
            break;
        case 3:
            c.rgb = prim_color.rgb;
            break;
        case 4:
            c.rgb = shade_color.rgb;
            break;
        case 5:
            c.rgb = env_color.rgb;
            break;
        case 8:
            c.rgb = vec3(texel_color.a);
            break;
        case 10:
            c.rgb = vec3(prim_color.a);
            break;
        case 11:
            c.rgb = vec3(shade_color.a);
            break;
        case 12:
            c.rgb = vec3(env_color.a);
            break;
        case 16:
            c.rgb = vec3(0.0);
            break;
        default:
            break;
    }

    switch ((color_combiner_mode.y >> 15) & 0x7) {
        case 1:
            d.rgb = texel_color.rgb;
            break;
        case 3:
            d.rgb = prim_color.rgb;
            break;
        case 4:
            d.rgb = shade_color.rgb;
            break;
        case 5:
            d.rgb = env_color.rgb;
            break;
        case 6:
            d.rgb = vec3(1.0);
            break;
        case 7:
            d.rgb = vec3(0.0);
            break;
        default:
            break;
    }

    switch ((color_combiner_mode.x >> (44 - 32)) & 0x7) {
        case 1:
            a.a = texel_color.a;
            break;
        case 3:
            a.a = prim_color.a;
            break;
        case 4:
            a.a = shade_color.a;
            break;
        case 5:
            a.a = env_color.a;
            break;
        case 6:
            a.a = 1.0;
            break;
        case 7:
            a.a = 0.0;
            break;
        default:
            break;
    }

    switch ((color_combiner_mode.y >> 12) & 0x7) {
        case 1:
            b.a = texel_color.a;
            break;
        case 3:
            b.a = prim_color.a;
            break;
        case 4:
            b.a = shade_color.a;
            break;
        case 5:
            b.a = env_color.a;
            break;
        case 6:
            b.a = 1.0;
            break;
        case 7:
            b.a = 0.0;
            break;
        default:
            break;
    }

    switch ((color_combiner_mode.x >> (41 - 32)) & 0x7) {
        case 1:
            c.a = texel_color.a;
            break;
        case 3:
            c.a = prim_color.a;
            break;
        case 4:
            c.a = shade_color.a;
            break;
        case 5:
            c.a = env_color.a;
            break;
        case 7:
            c.a = 0.0;
            break;
        default:
            break;
    }

    switch ((color_combiner_mode.y >> 9) & 0x7) {
        case 1:
            d.a = texel_color.a;
            break;
        case 3:
            d.a = prim_color.a;
            break;
        case 4:
            d.a = shade_color.a;
            break;
        case 5:
            d.a = env_color.a;
            break;
        case 6:
            d.a = 1.0;
            break;
        case 7:
            d.a = 0.0;
            break;
        default:
            break;
    }

    o_color.rgb = (a.rgb - b.rgb) * c.rgb + d.rgb;
    o_color.a = (a.a - b.a) * c.a + d.a;
}
