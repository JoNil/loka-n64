use super::{Texture, TextureMut};
use crate::graphics::Graphics;
use n64_macros::debugln;
use n64_math::{Color, Vec2, Vec3};
use n64_sys::rdp;
use rdp_command_builder::*;

mod rdp_command_builder;

fn float_to_unsigned_int_frac(val: f32) -> (u16, u16) {
    if 0.0 >= val {
        return (u16::MAX, u16::MAX);
    }

    let integer_part = libm::floorf(val);

    if (u16::MAX as f32) < integer_part {
        return (u16::MAX, u16::MAX);
    }

    let fractal_part = val - integer_part;

    (
        integer_part as u16,
        libm::floorf(fractal_part * ((1 << 16) as f32)) as u16,
    )
}

fn f32_to_fixed_16_16(val: f32) -> i32 {
    if (i16::MAX as f32) < val {
        return i32::MAX;
    } else if (i16::MIN as f32) > val {
        return i32::MIN;
    }

    (val * (1 << 16) as f32) as i32
}

// Dx/Dy of edge from p0 to p1.
// Dx/Dy (kx + m = y)
// x = (y-m)/k
// dx : 1/k
fn edge_slope(p0: Vec3, p1: Vec3) -> i32 {
    // TODO: ZERO DIVISION  (old epsilon 0.01)
    if 1.0 > libm::fabsf(p1.1 - p0.1) {
        return f32_to_fixed_16_16(p1.0 - p0.0);
    }
    f32_to_fixed_16_16((p1.0 - p0.0) / (p1.1 - p0.1))
}
fn edge_slope_OLD(p0: Vec3, p1: Vec3) -> i32 {
    // TODO: ZERO DIVISION  (old epsilon 0.01)
    let a = libm::floorf((p1.0 - p0.0) * 4.0) / 4.0;
    let b = (p1.1 - p0.1); //libm::floorf((p1.1 - p0.1) * 4.0) / 4.0;
    n64_macros::debugln!("a, b {} {}", a, b);
    if b == 0.0 {
        return f32_to_fixed_16_16(a);
    }

    f32_to_fixed_16_16(a / b)
}

// kx + m = y
// kx0 + m = y0
// kx1 + m = y1
// k(x1 - x0) = y1 - y0
// k = (y1 - y0)/(x1-x0)
// x0 * (y1 - y0)/(x1-x0) + m = y0
// m = y0 - x0*k
fn slope_x_from_y(p0: Vec3, p1: Vec3, y: f32) -> (u16, u16) {
    // kx + m = y
    // k = (p1y-p0y)/(p1x-p0x)
    // m = y0 - x0*k
    // x = (y-m)/k = (y- (y0 - x0*k))/k = y/k - y0/k + x0
    // x =  x0 + (y - y0)/k
    // x = p0x + (y - p0.y)*(p1x-p0x) / (p1y-p0y)

    // ZERO DIVISION check
    if 1.0 > libm::fabsf(p1.1 - p0.1) {
        return float_to_unsigned_int_frac(p0.0);
    }

    let x = p0.0 + (y - p0.1) * (p1.0 - p0.0) / (p1.1 - p0.1);

    //panic!("slope_x_from_y\n{}\n{}\ny{}\nx{}\n{:?}",
    //p0, p1,
    //y, x,
    //float_to_int_frac(x));

    float_to_unsigned_int_frac(x)
}

// X coordinate of the intersection of the edge from p0 to p1 and the sub-scanline at (or higher than) p0.y
fn slope_y_next_subpixel_intersection(p0: Vec3, p1: Vec3) -> (u16, u16) {
    let y = libm::ceilf(p0.1 * 4.0) / 4.0;

    slope_x_from_y(p0, p1, y)
}

fn slope_y_prev_scanline_intersection(p0: Vec3, p1: Vec3) -> (u16, u16) {
    let y = libm::floorf(p0.1);

    slope_x_from_y(p0, p1, y)
}

fn int_frac_greater(a_integer: u16, a_fraction: u16, b_integer: u16, b_fraction: u16) -> bool {
    if a_integer == b_integer {
        a_fraction > b_fraction
    } else {
        a_integer > b_integer
    }
}

// p0  y postive down
// p1
// p2
// p2 - p0 slope vs p1-p0 slope.
// 2_0 slope > 1_0 slope => left major
// 2_0 slope = (p2x-p0x)/(p2_y-p0_y)
// 1_0 slope = (p1x-p0x)/(p1_y-p0_y)
//   p2_y-p0_y > 0 && p1_y-p0_y > 0
// (p2x-p0x)/(p2_y-p0_y) > (p1x-p0x)/(p1_y-p0_y)
// if and only if (since denominators are positive)
//   (p2x-p0x)*(p1_y-p0_y) > (p1x-p0x)*(p2_y-p0_y)
fn is_triangle_right_major(p0: Vec3, p1: Vec3, p2: Vec3) -> bool {
    //if p1.1 == p2.1 {
    //    // Flat bottom
    //    return false;
    //}

    // Counter clockwise order?
    // (p0 - p1)x(p2 - p1) > 0 (area)
    // (p0x - p1x)   (p2x - p1x)    0
    // (p0y - p1y) x (p2y - p1y)  = 0
    //      0             0         Z

    // Z = (p0x - p1x)*(p2y - p1y) - (p2x - p1x)*(p0y - p1y);
    // Z > 0 => (p0x - p1x)*(p2y - p1y) > (p2x - p1x)*(p0y - p1y)

    return (p0.0 - p1.0) * (p2.1 - p1.1) < (p2.0 - p1.0) * (p0.1 - p1.1);
    //return (p2.0-p0.0)*(p1.1-p0.1) < (p1.0-p0.0)*(p2.1-p0.1);
}

// Sort so that v0.1 <= v1.1 <= v2.1
fn sorted_triangle(v0: Vec3, v1: Vec3, v2: Vec3) -> (Vec3, Vec3, Vec3) {
    if v0.1 > v1.1 {
        sorted_triangle(v1, v0, v2)
    } else if v0.1 > v2.1 {
        sorted_triangle(v2, v0, v1)
    } else if v1.1 > v2.1 {
        sorted_triangle(v0, v2, v1)
    } else {
        (v0, v1, v2)
        //(v2, v1, v0) // Reverse order
    }
}

pub struct CommandBufferCache {
    rdp: RdpCommandBuilder,
}

impl CommandBufferCache {
    pub fn new() -> Self {
        Self {
            rdp: RdpCommandBuilder::new(),
        }
    }
}

pub struct CommandBuffer<'a> {
    out_tex: &'a mut TextureMut<'a>,
    colored_rect_count: u32,
    textured_rect_count: u32,
    cache: &'a mut CommandBufferCache,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(out_tex: &'a mut TextureMut<'a>, cache: &'a mut CommandBufferCache) -> Self {
        cache.rdp.clear();

        cache
            .rdp
            .set_color_image(
                FORMAT_RGBA,
                SIZE_OF_PIXEL_16B,
                out_tex.width as u16,
                out_tex.data.as_mut_ptr() as *mut u16,
            )
            .set_scissor(
                Vec2::zero(),
                Vec2::new((out_tex.width - 1) as f32, (out_tex.height - 1) as f32),
            )
            .set_combine_mode(&[0, 0, 0, 0, 6, 1, 0, 15, 1, 0, 0, 0, 0, 7, 7, 7]);

        CommandBuffer {
            out_tex,
            colored_rect_count: 0,
            textured_rect_count: 0,
            cache,
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.cache
            .rdp
            .set_other_modes(
                OTHER_MODE_CYCLE_TYPE_FILL
                    | OTHER_MODE_CYCLE_TYPE_COPY
                    | OTHER_MODE_CYCLE_TYPE_2_CYCLE
                    | OTHER_MODE_RGB_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_FORCE_BLEND,
            )
            .set_fill_color(Color::new(0b00000_00000_00000_1))
            .fill_rectangle(
                Vec2::new(0.0, 0.0),
                Vec2::new(
                    (self.out_tex.width - 1) as f32,
                    (self.out_tex.height - 1) as f32,
                ),
            );

        self
    }

    pub fn add_colored_rect(
        &mut self,
        upper_left: Vec2,
        lower_right: Vec2,
        color: Color,
    ) -> &mut Self {
        self.colored_rect_count += 1;
        self.cache
            .rdp
            .sync_pipe()
            .set_other_modes(
                OTHER_MODE_CYCLE_TYPE_FILL
                    | OTHER_MODE_CYCLE_TYPE_COPY
                    | OTHER_MODE_CYCLE_TYPE_1_CYCLE
                    | OTHER_MODE_RGB_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_FORCE_BLEND,
            )
            .set_combine_mode(&[0, 0, 0, 0, 6, 1, 0, 15, 1, 0, 0, 0, 0, 7, 7, 7])
            .set_fill_color(color)
            .fill_rectangle(upper_left, lower_right - Vec2::new(1.0, 1.0));

        self
    }

    pub fn add_textured_rect(
        &mut self,
        upper_left: Vec2,
        lower_right: Vec2,
        texture: Texture<'static>,
        blend_color: Option<u32>,
    ) -> &mut Self {
        self.textured_rect_count += 1;
        self.cache.rdp.sync_tile().set_other_modes(
            OTHER_MODE_SAMPLE_TYPE
                | OTHER_MODE_BI_LERP_0
                | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER
                | OTHER_MODE_B_M2A_0_1
                | if let Some(_) = blend_color {
                    OTHER_MODE_B_M1A_0_2
                } else {
                    0
                }
                | OTHER_MODE_FORCE_BLEND
                | OTHER_MODE_IMAGE_READ_EN,
        );

        if let Some(blend_color) = blend_color {
            self.cache.rdp.set_blend_color(blend_color);
        }

        self.cache
            .rdp
            .set_texture_image(
                FORMAT_RGBA,
                SIZE_OF_PIXEL_16B,
                texture.width as u16,
                texture.data.as_ptr() as *const u16,
            )
            .set_tile(
                FORMAT_RGBA,
                SIZE_OF_PIXEL_16B,
                texture.width as u16,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            )
            .load_tile(
                Vec2::new((texture.width) as f32, (texture.height) as f32),
                Vec2::new(0.0, 0.0),
                0,
            )
            .texture_rectangle(
                upper_left,
                lower_right,
                0,
                Vec2::new(0.0, 0.0),
                Vec2::new(32.0, 32.0),
            );
        self
    }

    pub fn add_mesh_indexed(
        &mut self,
        verts: &[Vec3],
        uvs: &[Vec2],
        colors: &[u32],
        indices: &[[u8; 3]],
        transform: &[[f32; 4]; 4],
        texture: Option<Texture<'static>>,
    ) -> &mut Self {
        self.cache
            .rdp
            .set_combine_mode(&[0, 0, 0, 0, 6, 1, 0, 15, 1, 0, 0, 0, 0, 7, 7, 7])
            .set_fill_color(Color::new(0b10000_00011_00011_1));

        // Set triangle mode fill
        self.cache
            .rdp
            //.set_other_modes(3u64 <<52);
            .set_other_modes(
                OTHER_MODE_CYCLE_TYPE_FILL
                    | OTHER_MODE_RGB_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER,
            );
        for triangle in indices {
            // TODO: Transform before sort
            let mut v0 = verts[triangle[0] as usize];
            let mut v1 = verts[triangle[1] as usize];
            let mut v2 = verts[triangle[2] as usize];

            let x_limit = 320.0;
            let y_limit = 240.0;

            if false {
                //let scale = 64.0;
                //
                //v0.0 = libm::fmaxf(libm::fminf( 0.5 * x_limit * (1.0 + v0.0), x_limit), 0.0);
                //v1.0 = libm::fmaxf(libm::fminf( 0.5 * x_limit * (1.0 + v1.0), x_limit), 0.0);
                //v2.0 = libm::fmaxf(libm::fminf( 0.5 * x_limit * (1.0 + v2.0), x_limit), 0.0);
                //v0.1 = libm::fmaxf(libm::fminf( 0.5 * y_limit * (1.0 + v0.1), y_limit), 0.0);
                //v1.1 = libm::fmaxf(libm::fminf( 0.5 * y_limit * (1.0 + v1.1), y_limit), 0.0);
                //v2.1 = libm::fmaxf(libm::fminf( 0.5 * y_limit * (1.0 + v2.1), y_limit), 0.0);

                v0.0 = libm::fmaxf(libm::fminf(0.5 * 32.0 * (1.0 + v0.0), x_limit), 0.0);
                v1.0 = libm::fmaxf(libm::fminf(0.5 * 32.0 * (1.0 + v1.0), x_limit), 0.0);
                v2.0 = libm::fmaxf(libm::fminf(0.5 * 32.0 * (1.0 + v2.0), x_limit), 0.0);
                v0.1 = libm::fmaxf(libm::fminf(0.5 * 32.0 * (1.0 + v0.1), y_limit), 0.0);
                v1.1 = libm::fmaxf(libm::fminf(0.5 * 32.0 * (1.0 + v1.1), y_limit), 0.0);
                v2.1 = libm::fmaxf(libm::fminf(0.5 * 32.0 * (1.0 + v2.1), y_limit), 0.0);
            } else {
                v0.0 = libm::fmaxf(libm::fminf(v0.0, x_limit), 0.0);
                v1.0 = libm::fmaxf(libm::fminf(v1.0, x_limit), 0.0);
                v2.0 = libm::fmaxf(libm::fminf(v2.0, x_limit), 0.0);
                v0.1 = libm::fmaxf(libm::fminf(v0.1, y_limit), 0.0);
                v1.1 = libm::fmaxf(libm::fminf(v1.1, y_limit), 0.0);
                v2.1 = libm::fmaxf(libm::fminf(v2.1, y_limit), 0.0);
            }

            // Vh is the highest point (smallest y value)
            // Vl is the lowest point (largest y value)
            //let (vh, vm, vl) = sorted_triangle(v0, v1, v2);
            let (vh, vm, vl) = sorted_triangle(v0, v1, v2);

            debugln!("V012\n{}\n{}\n{}\nVLMH\n{}\n{}\n{}", v0, v1, v2, vl, vm, vh);
            //n64_macros::debugflush();

            //TODO: Actual intersections (low with subpixel, mid & high with previous scanline)
            //
            let (l_int, l_frac) = slope_y_next_subpixel_intersection(vm, vl);
            let (m_int, m_frac) = slope_y_prev_scanline_intersection(vh, vm);
            let (h_int, h_frac) = slope_y_prev_scanline_intersection(vh, vl);

            // panic!("{}\n{}\n{}\n{}\n{}\n{}", l_int, l_frac, m_int, m_frac, h_int, h_frac);

            //panic!("x{:x?}\n{}\nx{:x?}\n{}\nx{:x?}\n{}", l_slope_int, l_slope_frac, m_slope_int, m_slope_frac, h_slope_int, h_slope_frac);

            let mut l_slope = edge_slope(vl, vm);
            let mut m_slope = edge_slope(vm, vh);
            let mut h_slope = edge_slope(vl, vh);

            //l_slope = (l_slope & ((1 << 30) - 1));
            //m_slope = (m_slope & ((1 << 30) - 1));
            //h_slope = (h_slope & ((1<<30)-1));

            //panic!("{}.{}>{}.{}\n{}", m_int, m_frac, h_int, h_frac, int_frac_greater(m_int, m_frac, h_int, h_frac));

            let right_major = is_triangle_right_major(vh, vm, vl);

            //if !right_major {
            //   return self;
            // }

            //self.cache.rdp.sync_full();
            //self.cache.rdp.sync_pipe(); // Should not be needed.

            //if m_frac == h_frac {
            // TODO Handle!
            //    continue;
            //}

            if true {
                self.cache.rdp.edge_coefficients(
                    false,
                    false,
                    false,
                    right_major,
                    0,
                    0,
                    vl.1,
                    vm.1,
                    vh.1,
                    l_int,
                    l_frac,
                    m_int,
                    m_frac,
                    h_int,
                    h_frac,
                    l_slope, // l_slope
                    m_slope, //m_slope,
                    h_slope, //h_slope,
                );
            }

            /*panic!(
                "Vl {} {}\nVm {} {}\nVh {} {}\nLY,X {} {} {}\nMY,X {} {} {}\nHY,X {} {} {}",
            vl.1, vl.0,
            vm.1, vm.0,
            vh.1, vh.0,
            vl.1, l_int, l_frac,
            vm.1, m_int, m_frac,
            vh.1, h_int, h_frac
            );*/
        }
        self
    }

    pub fn run(mut self, _graphics: &mut Graphics) -> (i32, i32, usize) {
        self.cache.rdp.sync_full();

        let status = unsafe {
            self.cache.rdp.commands =
                Some(rdp::swap_commands(self.cache.rdp.commands.take().unwrap()));
            rdp::run_command_buffer()
        };

        (
            self.colored_rect_count as i32,
            self.textured_rect_count as i32,
            status,
        )
    }
}
