use super::{FillPipeline, Pipeline, TextureMut};
use crate::graphics::Graphics;
use n64_math::{vec2, Color, Mat4, Vec2, Vec3};
use n64_sys::rdp;
use rdp_command_builder::*;
use rdp_state::RdpState;

mod rdp_command_builder;
mod rdp_state;

// Note: Primitive color, g*DPSetPrimColor( ), primitive depth, g*DPSetPrimDepth( ), and scissor, g*DPSetScissor( ), are attributes that do not require any syncs.

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
    out_tex: TextureMut<'a>,
    z_buffer: &'a mut [u16],
    colored_rect_count: u32,
    textured_rect_count: u32,
    mesh_count: u32,
    current_state: RdpState,
    cache: &'a mut CommandBufferCache,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(
        (out_tex, z_buffer): (TextureMut<'a>, &'a mut [u16]),
        cache: &'a mut CommandBufferCache,
    ) -> Self {
        cache.rdp.clear();

        cache
            .rdp
            .set_color_image(
                FORMAT_RGBA,
                SIZE_OF_PIXEL_16B,
                out_tex.width as u16,
                out_tex.data.as_mut_ptr() as *mut u16,
            )
            .set_z_image(z_buffer.as_mut_ptr() as *mut u16)
            .set_scissor(
                Vec2::ZERO,
                vec2((out_tex.width - 1) as f32, (out_tex.height - 1) as f32),
            );

        CommandBuffer {
            out_tex,
            z_buffer,
            colored_rect_count: 0,
            textured_rect_count: 0,
            mesh_count: 0,
            current_state: RdpState::default(),
            cache,
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        rdp_state::apply_fill_pipeline(
            &mut self.cache.rdp,
            &mut self.current_state,
            &FillPipeline {
                fill_color: Color::new(0b00000_00000_00000_1),
                ..FillPipeline::default()
            },
        );

        self.cache.rdp.fill_rectangle(
            vec2(0.0, 0.0),
            vec2(
                (self.out_tex.width - 1) as f32,
                (self.out_tex.height - 1) as f32,
            ),
        );

        self.cache.rdp.set_color_image(
            FORMAT_RGBA,
            SIZE_OF_PIXEL_16B,
            self.out_tex.width as u16,
            self.z_buffer.as_mut_ptr() as *mut u16,
        );

        rdp_state::apply_fill_pipeline(
            &mut self.cache.rdp,
            &mut self.current_state,
            &FillPipeline {
                fill_color: Color::new(0x7fff),
                ..FillPipeline::default()
            },
        );

        self.cache.rdp.fill_rectangle(
            vec2(0.0, 0.0),
            vec2(
                (self.out_tex.width - 1) as f32,
                (self.out_tex.height - 1) as f32,
            ),
        );

        self.cache.rdp.set_color_image(
            FORMAT_RGBA,
            SIZE_OF_PIXEL_16B,
            self.out_tex.width as u16,
            self.out_tex.data.as_mut_ptr() as *mut u16,
        );

        self
    }

    pub fn set_fill_pipeline(&mut self, pipeline: &FillPipeline) -> &mut Self {
        rdp_state::apply_fill_pipeline(&mut self.cache.rdp, &mut self.current_state, pipeline);
        self
    }

    pub fn set_pipeline(&mut self, pipeline: &Pipeline) -> &mut Self {
        rdp_state::apply_pipeline(&mut self.cache.rdp, &mut self.current_state, pipeline);
        self
    }

    pub fn add_colored_rect(&mut self, upper_left: Vec2, lower_right: Vec2) -> &mut Self {
        self.colored_rect_count += 1;
        self.cache
            .rdp
            .fill_rectangle(upper_left, lower_right - vec2(1.0, 1.0));

        self
    }

    pub fn add_textured_rect(&mut self, upper_left: Vec2, lower_right: Vec2) -> &mut Self {
        self.textured_rect_count += 1;

        self.cache.rdp.texture_rectangle(
            upper_left,
            lower_right,
            0,
            vec2(0.0, 0.0),
            vec2(32.0, 32.0),
        );
        self
    }

    pub fn add_mesh_indexed(
        &mut self,
        verts: &[[f32; 3]],
        uvs: &[[f32; 2]],
        colors: &[u32],
        indices: &[[u8; 3]],
        transform: &[[f32; 4]; 4],
    ) -> &mut Self {
        self.mesh_count += 1;

        let transform = Mat4::from_cols_array_2d(transform);

        for triangle in indices {
            let mut v0 = transform.transform_point3(Vec3::from(verts[triangle[0] as usize]));
            let mut v1 = transform.transform_point3(Vec3::from(verts[triangle[1] as usize]));
            let mut v2 = transform.transform_point3(Vec3::from(verts[triangle[2] as usize]));

            let x_limit = 320.0;
            let y_limit = 240.0;

            v0.x = libm::fmaxf(libm::fminf(v0.x, x_limit), 0.0);
            v1.x = libm::fmaxf(libm::fminf(v1.x, x_limit), 0.0);
            v2.x = libm::fmaxf(libm::fminf(v2.x, x_limit), 0.0);
            v0.y = libm::fmaxf(libm::fminf(v0.y, y_limit), 0.0);
            v1.y = libm::fmaxf(libm::fminf(v1.y, y_limit), 0.0);
            v2.y = libm::fmaxf(libm::fminf(v2.y, y_limit), 0.0);

            if triangle_has_zero_area(v0, v1, v2) {
                continue;
            }
            // Vh is the highest point (smallest y value)
            // Vl is the lowest point (largest y value)
            let (vh, vm, vl) = sorted_triangle(v0, v1, v2);

            let (l_int, l_frac) = slope_y_next_subpixel_intersection(vm, vl);
            let (m_int, m_frac) = slope_y_prev_scanline_intersection(vh, vm);
            let (h_int, h_frac) = slope_y_prev_scanline_intersection(vh, vl);

            let l_slope = edge_slope(vl, vm);
            let m_slope = edge_slope(vm, vh);
            let h_slope = edge_slope(vl, vh);

            let right_major = is_triangle_right_major(vh, vm, vl);

            let is_shaded = true;

            self.cache.rdp.edge_coefficients(
                is_shaded,
                false,
                false,
                right_major,
                0,
                0,
                vl.y,
                vm.y,
                vh.y,
                l_int,
                l_frac,
                m_int,
                m_frac,
                h_int,
                h_frac,
                l_slope,
                m_slope,
                h_slope,
            );

            if is_shaded {
                let (vhi, vmi, vli) = sorted_triangle_indices(v0, v1, v2);
                //let color_tri : [[i32; 3]; 3] = [[255<<16, 0, 0], [0, 255<<16, 0], [0, 0, 255<<16]];

                //n64_macros::debugln!("Sorted indices {} {} {}: {} {} {}", vhi, vmi, vli,
                //    verts[triangle[vhi as usize] as usize][1],
                //    verts[triangle[vmi as usize] as usize][1],
                //    verts[triangle[vli as usize] as usize][1]);

                if true {
                    // TODO: Calc nz once
                    //let color_tri: [[i32; 3]; 3] = [[255, 0, 0], [0, 255, 0], [0, 0, 255]];
                    //let color_h = color_tri[vhi as usize];
                    //let color_m = color_tri[vmi as usize];
                    //let color_l = color_tri[vli as usize];
                    let color_h = color_to_i32(colors[triangle[vhi as usize] as usize]);
                    let color_m = color_to_i32(colors[triangle[vmi as usize] as usize]);
                    let color_l = color_to_i32(colors[triangle[vli as usize] as usize]);

                    let (r_dx, r_dy, r_de, r_off) = shaded_triangle_coeff(
                        vh,
                        vm,
                        vl,
                        color_h[0] as f32,
                        color_m[0] as f32,
                        color_l[0] as f32,
                    );
                    let (g_dx, g_dy, g_de, g_off) = shaded_triangle_coeff(
                        vh,
                        vm,
                        vl,
                        color_h[1] as f32,
                        color_m[1] as f32,
                        color_l[1] as f32,
                    );
                    let (b_dx, b_dy, b_de, b_off) = shaded_triangle_coeff(
                        vh,
                        vm,
                        vl,
                        color_h[2] as f32,
                        color_m[2] as f32,
                        color_l[2] as f32,
                    );
                    let red = color_h[0] << 16; //r_off;
                    let green = color_h[1] << 16; //g_off;
                    let blue = color_h[2] << 16; //b_off;

                    //n64_macros::debugln!("off {:?} {:?}",  (red, green, blue), (red>>16, green>>16, blue>>16));
                    //n64_macros::debugln!("dx {:?} {:?}", (r_dx, g_dx, b_dx), (r_dx>>16, g_dx>>16, b_dx>>16));
                    //n64_macros::debugln!("dy {:?} {:?}", (r_dy, g_dy, b_dy), (r_dy>>16, g_dy>>16, b_dy>>16));

                    self.cache.rdp.shade_coefficients(
                        red, green, blue, 0, // Color
                        r_dx, g_dx, b_dx, 0, // Delta color X
                        r_de, g_de, b_de, 0, // Delta color Edge
                        r_dy, g_dy, b_dy, 0, // Delta color y
                    );

                    //let a = 16<<16|16;//16<<16;
                    //self.cache.rdp.shade_coefficients(
                    //          0,   0,  0,    0, // Color
                    //          0,   0,  0,    0, // Delta color X
                    //          a,   0,  0,    0, // Delta color Edge
                    //          0,   0,  0,    0, // Delta color y
                    // );
                    ////n64_macros::debugln!("vec h, m, l {:?} {:?} {:?}", vh, vm, vl);
                    //n64_macros::debugln!("rgb h, m, l {:?} {:?} {:?}", color_h, color_m, color_l);
                } else {
                    let color_tri: [[i32; 3]; 3] =
                        [[255 << 16, 0, 0], [0, 255 << 16, 0], [0, 0, 255 << 16]];
                    let color_h = color_tri[vhi as usize];
                    let color_m = color_tri[vmi as usize];
                    let color_l = color_tri[vli as usize];

                    let xMax = libm::fmaxf(libm::fmaxf(v0.x, v1.x), v2.x);
                    let xMin = libm::fminf(libm::fminf(v0.x, v1.x), v2.x);
                    let yMax = libm::fmaxf(libm::fmaxf(v0.y, v1.y), v2.y);
                    let yMin = libm::fminf(libm::fminf(v0.y, v1.y), v2.y);
                    let red = color_h[0];
                    let green = color_h[1];
                    let blue = color_h[2];
                    let rDe = find_color_d(color_l[0], color_h[0], vl.y, vh.y);
                    let gDe = find_color_d(color_l[1], color_h[1], vl.y, vh.y);
                    let bDe = find_color_d(color_l[2], color_h[2], vl.y, vh.y);

                    let edge_color_m_r = ((rDe as f32) * (vm.y - vh.y)) as i32 + red;
                    let edge_color_m_g = ((gDe as f32) * (vm.y - vh.y)) as i32 + green;
                    let edge_color_m_b = ((bDe as f32) * (vm.y - vh.y)) as i32 + blue;

                    let rDx = find_color_d(color_h[0], color_m[0] - edge_color_m_r, vh.x, vm.x);
                    let gDx = find_color_d(color_h[1], color_m[1] - edge_color_m_g, vh.x, vm.x);
                    let bDx = find_color_d(color_h[2], color_m[2] - edge_color_m_b, vh.x, vm.x);
                    let rDy = 0; //find_color_d(color_h[0], color_m[0] - edge_color_m_r, vh.y, vm.y)/2;
                    let gDy = 0; //find_color_d(color_h[1], color_m[1] - edge_color_m_g, vh.y, vm.y)/2;
                    let bDy = 0; //find_color_d(color_h[2], color_m[2] - edge_color_m_b, vh.y, vm.y)/2;
                    self.cache.rdp.shade_coefficients(
                        red, green, blue, 0, // Color
                        rDx, gDx, bDx, 0, // Delta color X
                        rDe, gDe, bDe, 0, // Delta color Edge
                        rDy, gDy, bDy, 0, // Delta color y
                    );
                }
            }
        }
        self
    }

    pub fn run(mut self, _graphics: &mut Graphics) -> (i32, i32, i32) {
        self.cache.rdp.sync_full();

        unsafe {
            self.cache.rdp.commands =
                Some(rdp::swap_commands(self.cache.rdp.commands.take().unwrap()));
            rdp::run_command_buffer();
        };

        (
            self.colored_rect_count as i32,
            self.textured_rect_count as i32,
            self.mesh_count as i32,
        )
    }
}

impl Default for CommandBufferCache {
    fn default() -> Self {
        Self::new()
    }
}

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
    if 1.0 > libm::fabsf(p1.y - p0.y) {
        return f32_to_fixed_16_16(p1.x - p0.x);
    }
    f32_to_fixed_16_16((p1.x - p0.x) / (p1.y - p0.y))
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
    if 1.0 > libm::fabsf(p1.y - p0.y) {
        return float_to_unsigned_int_frac(p0.x);
    }

    let x = p0.x + (y - p0.y) * (p1.x - p0.x) / (p1.y - p0.y);

    float_to_unsigned_int_frac(x)
}

// X coordinate of the intersection of the edge from p0 to p1 and the sub-scanline at (or higher than) p0.y
fn slope_y_next_subpixel_intersection(p0: Vec3, p1: Vec3) -> (u16, u16) {
    let y = libm::ceilf(p0.y * 4.0) / 4.0;

    slope_x_from_y(p0, p1, y)
}

fn slope_y_prev_scanline_intersection(p0: Vec3, p1: Vec3) -> (u16, u16) {
    let y = libm::floorf(p0.y);

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
    // Counter clockwise order?
    // (p0 - p1)x(p2 - p1) > 0 (area)
    // (p0x - p1x)   (p2x - p1x)    0
    // (p0y - p1y) x (p2y - p1y)  = 0
    //      0             0         Z

    // Z = (p0x - p1x)*(p2y - p1y) - (p2x - p1x)*(p0y - p1y);
    // Z > 0 => (p0x - p1x)*(p2y - p1y) > (p2x - p1x)*(p0y - p1y)

    (p0.x - p1.x) * (p2.y - p1.y) < (p2.x - p1.x) * (p0.y - p1.y)
}

// Sort so that v0.y <= v1.y <= v2.y
fn sorted_triangle(v0: Vec3, v1: Vec3, v2: Vec3) -> (Vec3, Vec3, Vec3) {
    if v0.y > v1.y {
        sorted_triangle(v1, v0, v2)
    } else if v0.y > v2.y {
        sorted_triangle(v2, v0, v1)
    } else if v1.y > v2.y {
        sorted_triangle(v0, v2, v1)
    } else {
        (v0, v1, v2)
    }
}

// Sort so that v0.y <= v1.y <= v2.y
fn sorted_triangle_indices(v0: Vec3, v1: Vec3, v2: Vec3) -> (u8, u8, u8) {
    if v0.y > v1.y {
        if v1.y > v2.y {
            // V0 > v1, V1 > V2
            (2, 1, 0)
        } else if v0.y > v2.y {
            // V0 > V1, V2 > V1, V0 > V2
            (1, 2, 0)
        } else {
            // V0 > V1, V2 > V1, V2 > V0
            (1, 0, 2)
        }
    } else if v0.y > v2.y {
        // V1 > V0, V0 > V2
        (2, 0, 1)
    } else if v1.y > v2.y {
        // V1 > v0, V2 > v0, V1 > V2
        (0, 2, 1)
    } else {
        //
        (0, 1, 2)
    }
}

fn find_color_d(ch: i32, ci: i32, dh: f32, di: f32) -> i32 {
    let ad = di - dh;
    if libm::fabsf(ad) < 1.0 {
        return 0;
    }

    // ch + res*(di - dh) = ci
    (((ci - ch) as f32) / ad) as i32
}

fn triangle_has_zero_area(v0: Vec3, v1: Vec3, v2: Vec3) -> bool {
    let cross_a = f32_to_fixed_16_16((v0.x - v1.x) * (v2.y - v1.y));
    let cross_b = f32_to_fixed_16_16((v0.y - v1.y) * (v2.x - v1.x));

    //n64_macros::debugln!("v0 {:?}", v0);
    //n64_macros::debugln!("v1 {:?}", v1);
    //n64_macros::debugln!("v2 {:?}", v2);
    //n64_macros::debugln!("crossA {:?}", crossA);
    //n64_macros::debugln!("crossB {:?}", crossB);

    cross_a == cross_b
}

// TODO: Take nz and va-vb & vc-vb instead
fn shaded_triangle_coeff(
    vb: Vec3,
    va: Vec3,
    vc: Vec3,
    bi: f32,
    ai: f32,
    ci: f32,
) -> (i32, i32, i32, i32) {
    // Already checked for nz = 0
    //let nx = ai * (vb.y - vc.y) + bi * (vc.y - va.y) + ci * (va.y - vb.y);
    //let ny = ai * (vc.x - vb.x) + bi * (va.x - vc.x) + ci * (vb.x - va.x);

    let nx = (va.y - vb.y) * (ci - bi) - (ai - bi) * (vc.y - vb.y);
    let ny = (ai - bi) * (vc.x - vb.x) - (va.x - vb.x) * (ci - bi);
    let nz = (va.x - vb.x) * (vc.y - vb.y) - (va.y - vb.y) * (vc.x - vb.x);

    //n64_macros::debugln!("vb.y - vc.y {}", vb.y - vc.y);
    //n64_macros::debugln!("vc.y - va.y {}", vc.y - va.y);
    //n64_macros::debugln!("va.y - vb.y {}", va.y - vb.y);
    //n64_macros::debugln!("n xyz {:?}, i abc {:?}", (nx, ny, nz), (ai, bi, ci));
    //n64_macros::debugln!("n xyz {:?}, i abc {:?}", (nx, ny, nz), (ai, bi, ci));
    //return (f32_to_fixed_16_16(nx/nz), f32_to_fixed_16_16(ny/nz));
    // Color already in f16.16
    //let pOff = -(vb.x*nx + vb.y*ny + bi*nz);

    //n64_macros::debugln!("n dot a {:?}", (va.x) * nx + (va.y) * ny + (ai) * nz);
    //n64_macros::debugln!("n dot b {:?}", (vb.x) * nx + (vb.y) * ny + (bi) * nz);
    //n64_macros::debugln!("n dot c {:?}", (vc.x) * nx + (vc.y) * ny + (ci) * nz);

    //return (((-nx/nz) as i32)<<16, ((-ny/nz) as i32)<<16, (pOff as i32)<<16);
    //return ((-nx as i32)<<16, (-ny as i32)<<16, (pOff as i32)<<16);
    //return (((-nx/nz) as i32)<<16, ((-ny/nz) as i32)<<16, ((-pOff/nz) as i32)<<16);

    //return (((-nx/nz) as i32)<<16, ((-ny/nz) as i32)<<16, (bi as i32)<<16);

    //let ne = (nx*(vc.x - vb.x) + ny*(vc.y - vb.y))/(libm::sqrtf((vc.x - vb.x)*(vc.x - vb.x) + (vc.y - vb.y)*(vc.y - vb.y)));
    //let ne = (- ny*(vc.x - vb.x) + nx*(vc.y - vb.y))/(libm::sqrtf((vc.x - vb.x)*(vc.x - vb.x) + (vc.y - vb.y)*(vc.y - vb.y)));

    let ne = ny + nx * (vc.x - vb.x) / (libm::fmaxf(1.0, vc.y - vb.y));

    //let ne = (nx*(vc.x - vb.x) + ny*(vc.y - vb.y));//

    //n64_macros::debugln!("ne {}", ne);
    let norm = -((1 << 16) as f32) / nz;
    // return ((nx*norm) as i32, (ne*norm) as i32, (bi*norm) as i32);
    //return ((ne*norm) as i32, (ny*norm) as i32, (bi*norm) as i32);
    //return ((ne*norm) as i32, (ny*norm) as i32, (bi*norm) as i32);
    //return ((ne*norm) as i32, (ny*norm) as i32, (((bi as i32)<<16) as f32) as i32);

    let dcdx = (nx * norm) as i32;
    let dcdy = (ny * norm) as i32;
    let dcde = (ne * norm) as i32;
    let color = (((bi as i32) << 16) as f32) as i32;

    (dcdx, dcdy, dcde, color)

    //return (((-nx/nz) as i32)<<16, 0, (bi as i32)<<16);
    //return (0, 0, (bi as i32)<<16);
    //return (0, ((-ny/nz) as i32)<<16, (bi as i32)<<16);
}

fn color_to_i32(color: u32) -> [i32; 3] {
    [
        ((color >> 24) & 0xff) as i32,
        ((color >> 16) & 0xff) as i32,
        ((color >> 8) & 0xff) as i32,
    ]
}
