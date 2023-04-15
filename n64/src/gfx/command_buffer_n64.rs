#![cfg_attr(not(target_vendor = "nintendo64"), allow(unused))]

use super::{FillPipeline, Pipeline};
use crate::{
    framebuffer::ViBufferToken, graphics_n64::Graphics, ipl3font, slow_cpu_clear, VideoMode,
};
use alloc::{boxed::Box, vec::Vec};
use n64_math::{vec2, Color, Mat4, Vec2, Vec3};
use rdp_command_builder::*;
use n64_macros::debugln;
use rdp_math::{
    color_to_i32, edge_slope, is_triangle_right_major, shaded_triangle_coeff,
    slope_y_next_subpixel_intersection, slope_y_prev_scanline_intersection, sorted_triangle,
    sorted_triangle_indices, triangle_is_too_small, truncate_to_pixel, z_triangle_coeff,
};
use rdp_state::RdpState;
use n64_sys::rsp;

mod rdp_command_builder;
mod rdp_math;
mod rdp_state;

// Note: Primitive color, g*DPSetPrimColor( ), primitive depth, g*DPSetPrimDepth( ), and scissor, g*DPSetScissor( ), are attributes that do not require any syncs.

pub struct CommandBufferCache {
    video_mode: VideoMode,
    rdp: RdpCommandBuilder,
    depth_buffer: Box<[u16]>,
    vertex_cache: Box<[(Vec3, i32); 256]>,
    vertex_cache_generation: i32,
}

impl CommandBufferCache {
    pub fn new(video_mode: VideoMode) -> Self {
        Self {
            video_mode,
            rdp: RdpCommandBuilder::new(),
            depth_buffer: {
                let mut buffer = Vec::new();
                buffer.resize_with(video_mode.size() as usize, || 0);
                buffer.into_boxed_slice()
            },
            vertex_cache: Box::new([(Vec3::ZERO, 0); 256]),
            vertex_cache_generation: 0,
        }
    }

    fn get(&mut self, index: u8, f: impl FnOnce() -> Vec3) -> Vec3 {
        // Transform every vertex to cache first
        // No need for generation

        let cached = &self.vertex_cache[index as usize];
        if cached.1 == self.vertex_cache_generation {
            return cached.0;
        }

        let v = f();

        self.vertex_cache[index as usize] = (v, self.vertex_cache_generation);

        v
    }
}

pub struct CommandBuffer<'a> {
    out_tex: ViBufferToken,
    colored_rect_count: u32,
    textured_rect_count: u32,
    mesh_count: u32,
    current_state: RdpState,
    cache: &'a mut CommandBufferCache,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(out_tex: ViBufferToken, cache: &'a mut CommandBufferCache) -> Self {
        cache.rdp.clear();

        cache
            .rdp
            .sync_pipe()
            .set_color_image(
                FORMAT_RGBA,
                SIZE_OF_PIXEL_16B,
                cache.video_mode.width() as u16,
                out_tex.0 as *mut u16,
            )
            .set_z_image(cache.depth_buffer.as_mut_ptr())
            .set_scissor(
                Vec2::ZERO,
                vec2(
                    (cache.video_mode.width() - 1) as f32,
                    (cache.video_mode.height() - 1) as f32,
                ),
            );

        CommandBuffer {
            out_tex,
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
                (self.cache.video_mode.width() - 1) as f32,
                (self.cache.video_mode.height() - 1) as f32,
            ),
        );

        self.cache.rdp.set_color_image(
            FORMAT_RGBA,
            SIZE_OF_PIXEL_16B,
            self.cache.video_mode.width() as u16,
            self.cache.depth_buffer.as_mut_ptr(),
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
                (self.cache.video_mode.width() - 1) as f32,
                (self.cache.video_mode.height() - 1) as f32,
            ),
        );

        self.cache.rdp.set_color_image(
            FORMAT_RGBA,
            SIZE_OF_PIXEL_16B,
            self.cache.video_mode.width() as u16,
            self.out_tex.0 as *mut u16,
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
        _uvs: &[[f32; 2]],
        colors: &[u32],
        indices: &[[u8; 3]],
        transform: &[[f32; 4]; 4],
    ) -> &mut Self {
        self.mesh_count += 1;

        let transform = Mat4::from_cols_array_2d(transform);

        self.cache.vertex_cache_generation = self.cache.vertex_cache_generation.wrapping_add(1);

        for triangle in indices {
            let mut v0 = self.cache.get(triangle[0], || {
                truncate_to_pixel(transform.project_point3(Vec3::from(verts[triangle[0] as usize])))
            });
            let mut v1 = self.cache.get(triangle[1], || {
                truncate_to_pixel(transform.project_point3(Vec3::from(verts[triangle[1] as usize])))
            });
            let mut v2 = self.cache.get(triangle[2], || {
                truncate_to_pixel(transform.project_point3(Vec3::from(verts[triangle[2] as usize])))
            });

            let x_limit = self.cache.video_mode.width() as f32;
            let y_limit = self.cache.video_mode.height() as f32;

            v0.x = libm::fmaxf(libm::fminf(v0.x, x_limit), 0.0);
            v1.x = libm::fmaxf(libm::fminf(v1.x, x_limit), 0.0);
            v2.x = libm::fmaxf(libm::fminf(v2.x, x_limit), 0.0);
            v0.y = libm::fmaxf(libm::fminf(v0.y, y_limit), 0.0);
            v1.y = libm::fmaxf(libm::fminf(v1.y, y_limit), 0.0);
            v2.y = libm::fmaxf(libm::fminf(v2.y, y_limit), 0.0);

            if triangle_is_too_small(v0, v1, v2) {
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
            let is_texured = false;
            let is_z_buffered = true;

            self.cache.rdp.edge_coefficients(
                is_shaded,
                is_texured,
                is_z_buffered,
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

                let color_h = color_to_i32(colors[triangle[vhi as usize] as usize]);
                let color_m = color_to_i32(colors[triangle[vmi as usize] as usize]);
                let color_l = color_to_i32(colors[triangle[vli as usize] as usize]);

                let (r_dx, r_dy, r_de, _r_off) = shaded_triangle_coeff(
                    vh,
                    vm,
                    vl,
                    color_h[0] as f32,
                    color_m[0] as f32,
                    color_l[0] as f32,
                );
                let (g_dx, g_dy, g_de, _g_off) = shaded_triangle_coeff(
                    vh,
                    vm,
                    vl,
                    color_h[1] as f32,
                    color_m[1] as f32,
                    color_l[1] as f32,
                );
                let (b_dx, b_dy, b_de, _b_off) = shaded_triangle_coeff(
                    vh,
                    vm,
                    vl,
                    color_h[2] as f32,
                    color_m[2] as f32,
                    color_l[2] as f32,
                );
                let red = color_h[0] << 16; // r_off;
                let green = color_h[1] << 16; // g_off;
                let blue = color_h[2] << 16; // b_off;

                self.cache.rdp.shade_coefficients(
                    red, green, blue, 0, // Color
                    r_dx, g_dx, b_dx, 0, // Delta color X
                    r_de, g_de, b_de, 0, // Delta color Edge
                    r_dy, g_dy, b_dy, 0, // Delta color y
                );
            }

            if is_z_buffered {
                let (z, dx, de, dy) = z_triangle_coeff(vh, vm, vl);
                self.cache.rdp.z_buffer_coefficients(z, dx, de, dy);
            }
        }
        self
    }

    pub fn submit(self, graphics: &mut Graphics, step: bool) -> (i32, i32, i32, i32) {
        self.cache.rdp.sync_full();

        let use_single_step = false;

        {
            n64_profiler::scope!("Rsp Job");
            if !use_single_step {
                graphics.rsp_start(&mut self.cache.rdp.blocks, false);
            }
            else {
                if !graphics.buffer_started {
                    graphics.rsp_start(&mut self.cache.rdp.blocks, true);
                    graphics.buffer_started = true;
                }

                if true {
                    let mut i = 0;
                    debugln!("Status          PC");
                    loop {
                        let (status, pc) = graphics.rsp_step(true);
                        let code = graphics.code();
                        
                        debugln!("{:015b} {:08x} : {}", status, pc, code[pc / 4]);

                        if i > 1024 {
                            graphics.rsp_dump_mem();
                            panic!("To many steps in rsp");
                        }
                        i = i + 1;
                    }
                }
                else {

                    let (status, pc) = graphics.rsp_step(step);

                    let code = graphics.code();

                    const GREEN: Color = Color::new(0b00011_10000_00011_1);
                    const RED: Color = Color::new(0b10000_00011_00011_1);

                    let mut out_tex = crate::gfx::TextureMut::new(
                        self.cache.video_mode.width(),
                        self.cache.video_mode.height(),
                        unsafe {
                            core::slice::from_raw_parts_mut(
                                self.out_tex.0,
                                self.cache.video_mode.size() as usize,
                            )
                        },
                    );

                    slow_cpu_clear(out_tex.data);

                    ipl3font::draw_str(
                        &mut out_tex,
                        15,
                        15,
                        RED,
                        alloc::format!("PC: {:04x}, Status {:04x}", pc, status).as_bytes(),
                    );

                    ipl3font::draw_str(&mut out_tex, 15, 15 + 20, RED, code[pc / 4].as_bytes());
                }
            }
        }

        debugln!("clk {}", rsp::clock_from_signals());
        (
            self.colored_rect_count as i32,
            self.textured_rect_count as i32,
            self.mesh_count as i32,
            rsp::clock_from_signals() as i32,
        )
    }
}
