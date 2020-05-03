use crate::gfx::Texture;
use crate::{
    graphics::{QUAD_INDEX_DATA},
    graphics_emu::{
        colored_rect::ColoredRectUniforms,
        textured_rect::{TexturedRectUniforms, UploadedTexture}, Graphics, dst_texture::DstTexture,
    },
};
use core::mem;
use futures_executor;
use n64_math::{Color, Vec2};
use std::{collections::HashMap, convert::TryInto};
use zerocopy::AsBytes;
use super::TextureMut;

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
        texture: Texture<'static>,
    },
}

pub struct CommandBuffer<'a> {
    out_tex: &'a mut TextureMut<'a>,
    clear: bool,
    commands: Vec<Command>,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(out_tex: &'a mut TextureMut<'a>) -> Self {
        CommandBuffer {
            out_tex, 
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
        texture: Texture<'static>,
    ) -> &mut Self {
        self.commands.push(Command::TexturedRect {
            upper_left,
            lower_right,
            texture,
        });

        self
    }

    pub fn run(self, graphics: &mut Graphics) {

        let dst = DstTexture::new(&graphics.device, self.out_tex.width, self.out_tex.height);

        let mut draw_data: Box<[DrawData]> = {
            let mut draw_data = Vec::new();
            draw_data.resize_with(self.commands.len(), Default::default);
            draw_data.into_boxed_slice()
        };

        let mut texture_data: HashMap<*const [Color], UploadedTexture> = HashMap::new();

        let command_buf = {
            let mut encoder = graphics
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            for command in &self.commands {
                if let Command::TexturedRect { texture, .. } = command {
                    texture_data.insert(
                        texture.data as *const _,
                        UploadedTexture::new(&graphics.device, &mut encoder, texture),
                    );
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

                let window_size = Vec2::new(self.out_tex.width as f32, self.out_tex.height as f32);

                for (command, data) in self.commands.iter().zip(draw_data.iter_mut()) {
                    match command {
                        Command::ColoredRect {
                            upper_left,
                            lower_right,
                            color,
                        } => {
                            let size = *lower_right - *upper_left;
                            let scale = size / window_size;
                            let offset_x = 2.0*upper_left.x()/window_size.x() - 1.0 + scale.x();
                            let offset_y = -2.0*upper_left.y()/window_size.y() + 1.0 - scale.y();

                            let uniforms = ColoredRectUniforms {
                                color: color.to_rgba(),
                                offset: [offset_x, offset_y],
                                scale: [scale.x(), scale.y()],
                            };

                            render_pass.set_pipeline(&graphics.colored_rect.pipeline);

                            data.uniform_buffer = Some(graphics.device.create_buffer_with_data(
                                uniforms.as_bytes(),
                                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                            ));

                            data.bind_group =
                                Some(graphics.device.create_bind_group(&wgpu::BindGroupDescriptor {
                                    layout: &graphics.colored_rect.bind_group_layout,
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
                            let offset_x = 2.0*upper_left.x()/window_size.x() - 1.0 + scale.x();
                            let offset_y = -2.0*upper_left.y()/window_size.y() + 1.0 - scale.y();

                            let uniforms = TexturedRectUniforms {
                                offset: [offset_x, offset_y],
                                scale: [scale.x(), scale.y()],
                            };

                            render_pass.set_pipeline(&graphics.textured_rect.pipeline);

                            data.uniform_buffer = Some(graphics.device.create_buffer_with_data(
                                uniforms.as_bytes(),
                                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                            ));

                            data.bind_group = Some(
                                graphics.device.create_bind_group(&wgpu::BindGroupDescriptor {
                                    layout: &graphics.textured_rect.bind_group_layout,
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

                            render_pass.set_bind_group(0, data.bind_group.as_ref().unwrap(), &[]);
                            render_pass.draw_indexed(0..(QUAD_INDEX_DATA.len() as u32), 0, 0..1);
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
