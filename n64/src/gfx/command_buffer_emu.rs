use crate::gfx::Texture;
use crate::{
    graphics::{GFX_EMU_STATE, HEIGHT, QUAD_INDEX_DATA, WIDTH},
    graphics_emu::{
        colored_rect::ColoredRectUniforms,
        textured_rect::{TexturedRectUniforms, UploadedTexture},
    },
};
use core::mem;
use futures_executor;
use n64_math::{Color, Vec2};
use std::{collections::HashMap, convert::TryInto};
use zerocopy::AsBytes;

#[derive(Default)]
struct DrawData {
    uniform_buffer: Option<wgpu::Buffer>,
    bind_group: Option<wgpu::BindGroup>,
}

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

        let mut draw_data: Box<[DrawData]> = {
            let mut draw_data = Vec::new();
            draw_data.resize_with(self.commands.len(), Default::default);
            draw_data.into_boxed_slice()
        };

        let mut texture_data: HashMap<*const Texture, UploadedTexture> = HashMap::new();

        let command_buf = {
            let mut encoder = state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            for command in &self.commands {
                if let Command::TexturedRect { texture, .. } = command {
                    texture_data.insert(
                        *texture as *const _,
                        UploadedTexture::new(&state.device, &mut encoder, texture),
                    );
                }
            }

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &state.command_buffer_dst.tex_view,
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

                render_pass.set_index_buffer(&state.quad_index_buf, 0, 0);
                render_pass.set_vertex_buffer(0, &state.quad_vertex_buf, 0, 0);

                let window_size = Vec2::new(WIDTH as f32, HEIGHT as f32);

                for (command, data) in self.commands.iter().zip(draw_data.iter_mut()) {
                    match command {
                        Command::ColoredRect {
                            upper_left,
                            lower_right,
                            color,
                        } => {
                            let size = *lower_right - *upper_left;
                            let scale = size / window_size;
                            let offset_x = 2.0 * (upper_left.x() - window_size.x() / 2.0 + size.x() / 2.0) / window_size.x();
                            let offset_y = 2.0 - 2.0 * (upper_left.y() + window_size.y() / 2.0 + size.y() / 2.0) / window_size.y();

                            let uniforms = ColoredRectUniforms {
                                color: color.to_rgba(),
                                offset: [offset_x, offset_y],
                                scale: [scale.x(), scale.y()],
                            };

                            render_pass.set_pipeline(&state.colored_rect.pipeline);

                            data.uniform_buffer = Some(state.device.create_buffer_with_data(
                                uniforms.as_bytes(),
                                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                            ));

                            data.bind_group =
                                Some(state.device.create_bind_group(&wgpu::BindGroupDescriptor {
                                    layout: &state.colored_rect.bind_group_layout,
                                    bindings: &[wgpu::Binding {
                                        binding: 0,
                                        resource: wgpu::BindingResource::Buffer {
                                            buffer: data.uniform_buffer.as_ref().unwrap(),
                                            range: 0..(mem::size_of::<ColoredRectUniforms>()
                                                as u64),
                                        },
                                    }],
                                    label: None,
                                }));

                            render_pass.set_bind_group(0, &data.bind_group.as_ref().unwrap(), &[]);
                            render_pass.draw_indexed(0..(QUAD_INDEX_DATA.len() as u32), 0, 0..1);
                        }
                        Command::TexturedRect {
                            upper_left,
                            lower_right,
                            texture,
                        } => {
                            let size = *lower_right - *upper_left;
                            let scale = size / window_size;
                            let offset_x = 2.0 * (upper_left.x() - window_size.x() / 2.0 + size.x() / 2.0) / window_size.x();
                            let offset_y = 2.0 - 2.0 * (upper_left.y() + window_size.y() / 2.0 + size.y() / 2.0) / window_size.y();

                            let uniforms = TexturedRectUniforms {
                                offset: [offset_x, offset_y],
                                scale: [scale.x(), scale.y()],
                            };

                            render_pass.set_pipeline(&state.textured_rect.pipeline);

                            data.uniform_buffer = Some(state.device.create_buffer_with_data(
                                uniforms.as_bytes(),
                                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                            ));

                            data.bind_group = Some(
                                state.device.create_bind_group(&wgpu::BindGroupDescriptor {
                                    layout: &state.textured_rect.bind_group_layout,
                                    bindings: &[
                                        wgpu::Binding {
                                            binding: 0,
                                            resource: wgpu::BindingResource::Buffer {
                                                buffer: data.uniform_buffer.as_ref().unwrap(),
                                                range: 0..(mem::size_of::<TexturedRectUniforms>()
                                                    as u64),
                                            },
                                        },
                                        wgpu::Binding {
                                            binding: 1,
                                            resource: wgpu::BindingResource::TextureView(
                                                &texture_data
                                                    .get(&(*texture as *const _))
                                                    .unwrap()
                                                    .tex_view,
                                            ),
                                        },
                                        wgpu::Binding {
                                            binding: 2,
                                            resource: wgpu::BindingResource::Sampler(
                                                &state.textured_rect.sampler,
                                            ),
                                        },
                                    ],
                                    label: None,
                                }),
                            );

                            render_pass.set_bind_group(0, data.bind_group.as_ref().unwrap(), &[]);
                            render_pass.draw_indexed(0..(QUAD_INDEX_DATA.len() as u32), 0, 0..1);
                        }
                    }
                }
            }

            encoder.copy_texture_to_buffer(
                wgpu::TextureCopyView {
                    texture: &state.command_buffer_dst.tex,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                },
                wgpu::BufferCopyView {
                    buffer: &state.command_buffer_dst.buffer,
                    offset: 0,
                    bytes_per_row: 4 * WIDTH as u32,
                    rows_per_image: HEIGHT as u32,
                },
                state.command_buffer_dst.tex_extent,
            );

            encoder.finish()
        };

        state.queue.submit(&[command_buf]);

        futures_executor::block_on(async {
            let mapped_colored_rect_dst_buffer = state
                .command_buffer_dst
                .buffer
                .map_read(0, (4 * WIDTH * HEIGHT) as u64)
                .await
                .unwrap();

            for (fb_color, mapped_color) in self
                .framebuffer
                .iter_mut()
                .zip(mapped_colored_rect_dst_buffer.as_slice().chunks(4))
            {
                *fb_color = Color::from_bytes(mapped_color.try_into().unwrap());
            }
        });
    }
}
