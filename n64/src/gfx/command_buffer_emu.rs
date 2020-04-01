use super::texture::Texture;
use crate::graphics::{ColoredRectUniforms, GFX_EMU_STATE, HEIGHT, QUAD_INDEX_DATA, WIDTH};
use core::mem;
use futures_executor;
use n64_math::{Color, Vec2};
use std::convert::TryInto;
use zerocopy::AsBytes;

enum Command {
    ColoredRect {
        upper_left: Vec2,
        lower_right: Vec2,
        color: Color,
    },
    TexturedRect {
        upper_left: Vec2,
        lower_right: Vec2,
        texture: &'static Texture,
    },
}

pub struct CommandBuffer<'a> {
    framebuffer: &'a mut [Color],

    clear: bool,
    commands: Vec<Command>,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(framebuffer: &'a mut [Color]) -> Self {
        CommandBuffer {
            framebuffer,
            clear: false,
            commands: Vec::new(),
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.clear = true;
        self.commands.clear();

        self
    }

    pub fn add_colored_rect(
        &mut self,
        upper_left: Vec2,
        lower_right: Vec2,
        color: Color,
    ) -> &mut Self {
        self.commands.push(Command::ColoredRect {
            upper_left,
            lower_right,
            color,
        });

        self
    }

    pub fn add_textured_rect(
        &mut self,
        upper_left: Vec2,
        lower_right: Vec2,
        texture: &'static Texture,
    ) -> &mut Self {
        self.commands.push(Command::TexturedRect {
            upper_left,
            lower_right,
            texture,
        });

        self
    }

    pub fn run(self) {
        let state = &mut *GFX_EMU_STATE.lock().unwrap();

        let mut uniform_buffers = Vec::new();
        let mut bind_groups = Vec::new();

        {
            let command_buf = {
                let mut encoder = state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &state.colored_rect_dst_tex_view,
                            resolve_target: None,
                            load_op: if self.clear {
                                wgpu::LoadOp::Clear
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

                    render_pass.set_index_buffer(&state.index_buf, 0, 0);
                    render_pass.set_vertex_buffer(0, &state.vertex_buf, 0, 0);
                    render_pass.set_pipeline(&state.colored_rect_pipeline);

                    let window_size = Vec2::new(WIDTH as f32, HEIGHT as f32);

                    for command in &self.commands {
                        match command {
                            Command::ColoredRect {
                                upper_left,
                                lower_right,
                                color,
                            } => {
                                let size = *lower_right - *upper_left;
                                let scale = size / window_size;
                                let offset = 2.0 * (*upper_left - window_size / 2.0 + size / 2.0)
                                    / window_size;

                                let uniforms = ColoredRectUniforms {
                                    color: color.to_rgba(),
                                    offset: [offset.x(), offset.y()],
                                    scale: [scale.x(), scale.y()],
                                };

                                uniform_buffers.push(state.device.create_buffer_with_data(
                                    uniforms.as_bytes(),
                                    wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                                ));
                            }
                            Command::TexturedRect {
                                upper_left,
                                lower_right,
                                texture,
                            } => {}
                        }
                    }

                    for uniforms in &uniform_buffers {
                        bind_groups.push(state.device.create_bind_group(
                            &wgpu::BindGroupDescriptor {
                                layout: &state.colored_rect_bind_group_layout,
                                bindings: &[wgpu::Binding {
                                    binding: 0,
                                    resource: wgpu::BindingResource::Buffer {
                                        buffer: &uniforms,
                                        range: 0..(mem::size_of::<ColoredRectUniforms>() as u64),
                                    },
                                }],
                                label: None,
                            },
                        ));
                    }

                    for bind_group in &bind_groups {
                        render_pass.set_bind_group(0, bind_group, &[]);
                        render_pass.draw_indexed(0..(QUAD_INDEX_DATA.len() as u32), 0, 0..1);
                    }
                }

                encoder.copy_texture_to_buffer(
                    wgpu::TextureCopyView {
                        texture: &state.colored_rect_dst_tex,
                        mip_level: 0,
                        array_layer: 0,
                        origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    },
                    wgpu::BufferCopyView {
                        buffer: &state.colored_rect_dst_buffer,
                        offset: 0,
                        bytes_per_row: 4 * WIDTH as u32,
                        rows_per_image: HEIGHT as u32,
                    },
                    state.colored_rect_dst_tex_extent,
                );

                encoder.finish()
            };

            state.queue.submit(&[command_buf]);

            let op = async {
                let mapped_colored_rect_dst_buffer = state
                    .colored_rect_dst_buffer
                    .map_read(0, (4 * WIDTH * HEIGHT) as u64)
                    .await
                    .unwrap();

                for (fb_color_row, mapped_color_row) in
                    self.framebuffer.chunks_exact_mut(WIDTH as usize).zip(
                        mapped_colored_rect_dst_buffer
                            .as_slice()
                            .chunks_exact(4 * WIDTH as usize)
                            .rev(),
                    )
                {
                    for (fb_color, mapped_color) in
                        fb_color_row.iter_mut().zip(mapped_color_row.chunks(4))
                    {
                        *fb_color = Color::from_bytes(mapped_color.try_into().unwrap());
                    }
                }
            };

            futures_executor::block_on(op);
        }
    }
}
