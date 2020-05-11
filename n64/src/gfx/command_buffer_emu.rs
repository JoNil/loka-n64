use super::TextureMut;
use crate::gfx::Texture;
use crate::{
    graphics::QUAD_INDEX_DATA,
    graphics_emu::{
        colored_rect::ColoredRectUniforms,
        dst_texture::DstTexture,
        textured_rect::{TexturedRectUniforms, UploadedTexture},
        Graphics,
    },
};
use futures_executor;
use n64_math::{Color, Vec2};
use std::convert::TryInto;
use std::mem;
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
        texture: Texture<'static>,
    },
}

pub struct CommandBuffer<'a> {
    out_tex: &'a mut TextureMut<'a>,
    clear: bool,
    colored_rect_count: u32,
    textured_rect_count: u32,
    commands: Vec<Command>,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(out_tex: &'a mut TextureMut<'a>) -> Self {
        CommandBuffer {
            out_tex,
            clear: false,
            colored_rect_count: 0,
            textured_rect_count: 0,
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
        self.colored_rect_count += 1;
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
        texture: Texture<'static>,
    ) -> &mut Self {
        self.textured_rect_count += 1;
        self.commands.push(Command::TexturedRect {
            upper_left,
            lower_right,
            texture,
        });

        self
    }

    pub fn run(self, graphics: &mut Graphics) {
        let dst = DstTexture::new(&graphics.device, self.out_tex.width, self.out_tex.height);
        let window_size = Vec2::new(self.out_tex.width as f32, self.out_tex.height as f32);

        {
            let old_size = graphics.colored_rect.draw_data.len();
            let extra_colored_rect = (self.colored_rect_count as i32) - (old_size as i32);

            graphics
                .colored_rect
                .draw_data
                .resize_with(self.colored_rect_count as usize, Default::default);

            if extra_colored_rect > 0 {
                for colored_rect_draw_data in graphics.colored_rect.draw_data[old_size..].iter_mut()
                {
                    colored_rect_draw_data
                        .init(&graphics.device, &graphics.colored_rect.bind_group_layout);
                }
            }
        }

        {
            let old_size = graphics.textured_rect.draw_data.len();
            let extra_textured_rect = (self.textured_rect_count as i32) - (old_size as i32);

            graphics
                .textured_rect
                .draw_data
                .resize_with(self.textured_rect_count as usize, Default::default);

            if extra_textured_rect > 0 {
                for textured_rect_draw_data in
                    graphics.textured_rect.draw_data[old_size..].iter_mut()
                {
                    textured_rect_draw_data.init(&graphics.device);
                }
            }
        }

        let command_buf = {
            let mut encoder = graphics
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut colored_rect_index = 0;
                let mut textured_rect_index = 0;

                for command in &self.commands {
                    match command {
                        Command::ColoredRect {
                            upper_left,
                            lower_right,
                            color,
                        } => {
                            let size = *lower_right - *upper_left;
                            let scale = size / window_size;
                            let offset_x = 2.0 * upper_left.x() / window_size.x() - 1.0 + scale.x();
                            let offset_y =
                                -2.0 * upper_left.y() / window_size.y() + 1.0 - scale.y();

                            let uniforms = ColoredRectUniforms {
                                color: color.to_rgba(),
                                offset: [offset_x, offset_y],
                                scale: [scale.x(), scale.y()],
                            };

                            let temp_buffer = graphics.device.create_buffer_with_data(
                                uniforms.as_bytes(),
                                wgpu::BufferUsage::COPY_SRC,
                            );

                            encoder.copy_buffer_to_buffer(
                                &temp_buffer,
                                0,
                                graphics.colored_rect.draw_data[colored_rect_index]
                                    .uniform_buffer
                                    .as_ref()
                                    .unwrap(),
                                0,
                                mem::size_of::<ColoredRectUniforms>() as u64,
                            );

                            colored_rect_index += 1;
                        }
                        Command::TexturedRect {
                            upper_left,
                            lower_right,
                            texture,
                        } => {
                            if !graphics
                                .textured_rect
                                .texture_cache
                                .contains_key(&(texture.data as *const _))
                            {
                                graphics.textured_rect.texture_cache.insert(
                                    texture.data as *const _,
                                    UploadedTexture::new(&graphics.device, &mut encoder, texture),
                                );
                            }

                            let size = *lower_right - *upper_left;
                            let scale = size / window_size;
                            let offset_x = 2.0 * upper_left.x() / window_size.x() - 1.0 + scale.x();
                            let offset_y =
                                -2.0 * upper_left.y() / window_size.y() + 1.0 - scale.y();

                            let uniforms = TexturedRectUniforms {
                                offset: [offset_x, offset_y],
                                scale: [scale.x(), scale.y()],
                            };

                            let temp_buffer = graphics.device.create_buffer_with_data(
                                uniforms.as_bytes(),
                                wgpu::BufferUsage::COPY_SRC,
                            );

                            encoder.copy_buffer_to_buffer(
                                &temp_buffer,
                                0,
                                graphics.textured_rect.draw_data[textured_rect_index]
                                    .uniform_buffer
                                    .as_ref()
                                    .unwrap(),
                                0,
                                mem::size_of::<TexturedRectUniforms>() as u64,
                            );

                            graphics.textured_rect.draw_data[textured_rect_index].bind_group = Some(
                                graphics
                                    .device
                                    .create_bind_group(&wgpu::BindGroupDescriptor {
                                        layout: &graphics.textured_rect.bind_group_layout,
                                        bindings: &[
                                            wgpu::Binding {
                                                binding: 0,
                                                resource: wgpu::BindingResource::Buffer {
                                                    buffer: graphics.textured_rect.draw_data
                                                        [textured_rect_index]
                                                        .uniform_buffer
                                                        .as_ref()
                                                        .unwrap(),
                                                    range: 0
                                                        ..(mem::size_of::<TexturedRectUniforms>()
                                                            as u64),
                                                },
                                            },
                                            wgpu::Binding {
                                                binding: 1,
                                                resource: wgpu::BindingResource::TextureView(
                                                    &graphics
                                                        .textured_rect
                                                        .texture_cache
                                                        .get(&(texture.data as *const _))
                                                        .unwrap()
                                                        .tex_view,
                                                ),
                                            },
                                            wgpu::Binding {
                                                binding: 2,
                                                resource: wgpu::BindingResource::Sampler(
                                                    &graphics.textured_rect.sampler,
                                                ),
                                            },
                                        ],
                                        label: None,
                                    }),
                            );

                            textured_rect_index += 1;
                        }
                    }
                }
            }

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &dst.tex_view,
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

                render_pass.set_index_buffer(&graphics.quad_index_buf, 0, 0);
                render_pass.set_vertex_buffer(0, &graphics.quad_vertex_buf, 0, 0);

                {
                    let mut colored_rect_index = 0;
                    let mut textured_rect_index = 0;

                    for command in &self.commands {
                        match command {
                            Command::ColoredRect { .. } => {
                                render_pass.set_pipeline(&graphics.colored_rect.pipeline);
                                render_pass.set_bind_group(
                                    0,
                                    graphics.colored_rect.draw_data[colored_rect_index]
                                        .bind_group
                                        .as_ref()
                                        .unwrap(),
                                    &[],
                                );
                                render_pass.draw_indexed(
                                    0..(QUAD_INDEX_DATA.len() as u32),
                                    0,
                                    0..1,
                                );
                                colored_rect_index += 1;
                            }
                            Command::TexturedRect { .. } => {
                                render_pass.set_pipeline(&graphics.textured_rect.pipeline);
                                render_pass.set_bind_group(
                                    0,
                                    graphics.textured_rect.draw_data[textured_rect_index]
                                        .bind_group
                                        .as_ref()
                                        .unwrap(),
                                    &[],
                                );
                                render_pass.draw_indexed(
                                    0..(QUAD_INDEX_DATA.len() as u32),
                                    0,
                                    0..1,
                                );
                                textured_rect_index += 1;
                            }
                        }
                    }
                }
            }

            encoder.copy_texture_to_buffer(
                wgpu::TextureCopyView {
                    texture: &dst.tex,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                },
                wgpu::BufferCopyView {
                    buffer: &dst.buffer,
                    offset: 0,
                    bytes_per_row: 4 * self.out_tex.width as u32,
                    rows_per_image: self.out_tex.height as u32,
                },
                dst.tex_extent,
            );

            encoder.finish()
        };

        graphics.queue.submit(&[command_buf]);

        futures_executor::block_on(async {
            let mapped_colored_rect_dst_buffer = dst
                .buffer
                .map_read(0, (4 * self.out_tex.width * self.out_tex.height) as u64)
                .await
                .unwrap();

            for (fb_color, mapped_color) in self
                .out_tex
                .data
                .iter_mut()
                .zip(mapped_colored_rect_dst_buffer.as_slice().chunks(4))
            {
                *fb_color = Color::from_bytes(mapped_color.try_into().unwrap());
            }
        });
    }
}
