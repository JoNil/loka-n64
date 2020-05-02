use super::{pipelines::colored_rect::ColoredRectUniforms, texture::Texture};
use crate::graphics::{GFX_EMU_STATE, HEIGHT, QUAD_INDEX_DATA, WIDTH};
use core::mem;
use futures_executor;
use n64_math::{Color, Vec2};
use std::convert::TryInto;
use zerocopy::AsBytes;

struct CachedTexture {
    pub tex_format: wgpu::TextureFormat,
    pub tex_extent: wgpu::Extent3d,
    pub tex: wgpu::Texture,
    pub tex_view: wgpu::TextureView,
}

impl CachedTexture {
    pub(crate) fn new(device: &wgpu::Device, encoder: &wgpu::CommandEncoder, texture_data: &Texture) -> Self {
        let mut temp_buffer: Box<[u8]> = {
            let mut temp_buffer = Vec::new();
            temp_buffer.resize_with(
                (4 * texture_data.width * texture_data.height) as usize,
                Default::default,
            );
            temp_buffer.into_boxed_slice()
        };

        for (pixel, data) in texture_data
            .data
            .chunks_exact(2)
            .map(|chunk| Color::new(u16::from_le_bytes(chunk.try_into().unwrap())))
            .zip(temp_buffer.chunks_exact_mut(4 * WIDTH as usize))
        {
            let rgba = pixel.to_rgba();

            data[0] = (rgba[0] * 255.0) as u8;
            data[1] = (rgba[1] * 255.0) as u8;
            data[2] = (rgba[2] * 255.0) as u8;
            data[3] = (rgba[3] * 255.0) as u8;
        }

        let temp_buf = device
            .create_buffer_with_data(&temp_buffer, wgpu::BufferUsage::COPY_SRC);

        let tex_format = wgpu::TextureFormat::Rgba8Unorm;

        let tex_extent = wgpu::Extent3d {
            width: texture_data.width as u32,
            height: texture_data.height as u32,
            depth: 1,
        };
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: tex_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: tex_format,
            usage: wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
        });
        let tex_view = tex.create_default_view();

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &temp_buf,
                offset: 0,
                bytes_per_row: 4 * texture_data.width as u32,
                rows_per_image: texture_data.height as u32,
            },
            wgpu::TextureCopyView {
                texture: &tex,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
            },
            tex_extent,
        );

        Self {
            tex_format,
            tex_extent,
            tex,
            tex_view,
        }
    }
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

#[derive(Default)]
struct DrawData {
    
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
        let mut uploaded_textures = Vec::new();

        let draw_data = {
            let mut draw_data = Vec::new();
            draw_data.resize_with(self.commands.len(), Default::default);
            draw_data.into_boxed_slice()
        };
        
        let command_buf = {
            let mut encoder = state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

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
                render_pass.set_pipeline(&state.colored_rect.pipeline);

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
                            layout: &state.colored_rect.bind_group_layout,
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

        let op = async {
            let mapped_colored_rect_dst_buffer = state
                .command_buffer_dst
                .buffer
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
