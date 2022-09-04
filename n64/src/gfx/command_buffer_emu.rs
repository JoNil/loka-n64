use super::{FillPipeline, Pipeline};
use crate::{
    framebuffer::ViBufferToken,
    graphics::QUAD_INDEX_DATA,
    graphics_emu::{
        colored_rect::{ColoredRectUniforms, MAX_COLORED_RECTS},
        dst_texture::DstTexture,
        textured_rect::{TexturedRectUniforms, MAX_TEXTURED_RECTS},
        Graphics,
    },
};
use crate::{graphics_emu::mesh::MeshUniforms, graphics_emu::mesh::MAX_MESHES};
use alloc::sync::Arc;
use assert_into::AssertInto;
use core::{
    slice,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use n64_math::{Color, Vec2};
use n64_types::VideoMode;
use std::mem;
use std::num::NonZeroU32;
use wgpu::util::DeviceExt;
use zerocopy::{AsBytes, FromBytes};

enum Command {
    ColoredRect {
        upper_left: Vec2,
        lower_right: Vec2,
        pipeline: EmuPipeline,
    },
    TexturedRect {
        upper_left: Vec2,
        lower_right: Vec2,
        pipeline: Pipeline,
    },
    Mesh {
        verts: Vec<[f32; 3]>,
        uvs: Vec<[f32; 2]>,
        colors: Vec<u32>,
        indices: Vec<u8>,
        transform: [[f32; 4]; 4],
        pipeline: Pipeline,
        buffer_index: usize,
    },
}

pub struct CommandBufferCache {
    video_mode: VideoMode,
    commands: Vec<Command>,
}

impl CommandBufferCache {
    pub fn new(video_mode: VideoMode) -> Self {
        Self {
            video_mode,
            commands: Vec::new(),
        }
    }
}

#[derive(Copy, Clone)]
enum EmuPipeline {
    Pipeline(Pipeline),
    FillPipeline(FillPipeline),
}

impl EmuPipeline {
    fn as_pipeline(&self) -> &Pipeline {
        match self {
            EmuPipeline::Pipeline(pipeline) => pipeline,
            _ => panic!("Pipeline is not valid"),
        }
    }

    fn as_fill_pipeline(&self) -> &FillPipeline {
        match self {
            EmuPipeline::FillPipeline(pipeline) => pipeline,
            _ => panic!("FillPipeline is not valid"),
        }
    }
}

pub struct CommandBuffer<'a> {
    out_tex: ViBufferToken,
    clear: bool,
    colored_rect_count: u32,
    textured_rect_count: u32,
    mesh_count: u32,
    current_pipeline: Option<EmuPipeline>,
    cache: &'a mut CommandBufferCache,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(out_tex: ViBufferToken, cache: &'a mut CommandBufferCache) -> Self {
        Self {
            out_tex,
            clear: false,
            colored_rect_count: 0,
            textured_rect_count: 0,
            mesh_count: 0,
            current_pipeline: None,
            cache,
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.clear = true;
        self.cache.commands.clear();
        self
    }

    pub fn set_fill_pipeline(&mut self, pipeline: &FillPipeline) -> &mut Self {
        self.current_pipeline = Some(EmuPipeline::FillPipeline(*pipeline));
        self
    }

    pub fn set_pipeline(&mut self, pipeline: &Pipeline) -> &mut Self {
        self.current_pipeline = Some(EmuPipeline::Pipeline(*pipeline));
        self
    }

    pub fn add_colored_rect(&mut self, upper_left: Vec2, lower_right: Vec2) -> &mut Self {
        self.colored_rect_count += 1;
        self.cache.commands.push(Command::ColoredRect {
            upper_left,
            lower_right,
            pipeline: self.current_pipeline.expect("No pipelien set"),
        });

        self
    }

    pub fn add_textured_rect(&mut self, upper_left: Vec2, lower_right: Vec2) -> &mut Self {
        self.textured_rect_count += 1;
        self.cache.commands.push(Command::TexturedRect {
            upper_left,
            lower_right,
            pipeline: *self
                .current_pipeline
                .expect("No pipeline has been set on the command buffer")
                .as_pipeline(),
        });

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

        self.cache.commands.push(Command::Mesh {
            verts: verts.to_owned(),
            uvs: uvs.to_owned(),
            colors: colors.to_owned(),
            indices: indices.iter().flatten().copied().collect(),
            transform: *transform,
            pipeline: *self
                .current_pipeline
                .expect("No pipeline has been set on the command buffer")
                .as_pipeline(),
            buffer_index: 0,
        });

        self
    }

    pub fn submit(self, graphics: &mut Graphics) -> (i32, i32, i32) {
        let dst = DstTexture::new(
            &graphics.device,
            self.cache.video_mode.width(),
            self.cache.video_mode.height(),
        );
        let window_size = Vec2::new(
            self.cache.video_mode.width() as f32,
            self.cache.video_mode.height() as f32,
        );

        assert!(self.colored_rect_count <= MAX_COLORED_RECTS as u32);
        assert!(self.textured_rect_count <= MAX_TEXTURED_RECTS as u32);
        assert!(self.mesh_count <= MAX_MESHES as u32);

        let command_buf = {
            let mut encoder = graphics
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            let mut render_pass_vertex_buffers = Vec::new();
            let mut render_pass_index_buffers = Vec::new();

            {
                let mut colored_rect_uniforms =
                    Vec::with_capacity(self.colored_rect_count as usize);
                let mut textured_rect_uniforms =
                    Vec::with_capacity(self.textured_rect_count as usize);
                let mut mesh_uniforms = Vec::with_capacity(self.mesh_count as usize);

                for command in &mut self.cache.commands {
                    match command {
                        Command::ColoredRect {
                            upper_left,
                            lower_right,
                            pipeline,
                        } => {
                            let size = *lower_right - *upper_left;
                            let scale = size / window_size;
                            let offset_x = 2.0 * upper_left.x / window_size.x - 1.0 + scale.x;
                            let offset_y = 2.0 * upper_left.y / window_size.y - 1.0 + scale.y;

                            let color = pipeline.as_fill_pipeline().fill_color;

                            colored_rect_uniforms.push(ColoredRectUniforms {
                                color: color.to_rgba(),
                                offset: [offset_x, offset_y],
                                scale: [scale.x, scale.y],
                            });
                        }
                        Command::TexturedRect {
                            upper_left,
                            lower_right,
                            pipeline,
                        } => {
                            let texture = pipeline.texture.expect("Invalid pipeline");

                            graphics.textured_rect.upload_texture_data(
                                &graphics.device,
                                &graphics.queue,
                                &texture,
                            );

                            let size = *lower_right - *upper_left;
                            let scale = size / window_size;
                            let offset_x = 2.0 * upper_left.x / window_size.x - 1.0 + scale.x;
                            let offset_y = 2.0 * upper_left.y / window_size.y - 1.0 + scale.y;

                            let color_combiner_mode = pipeline.color_combiner_mode.to_command();
                            let blend_mode = pipeline.blend_mode.to_command();
                            let prim_color = pipeline.prim_color.unwrap_or(0);
                            let env_color = pipeline.env_color.unwrap_or(0);
                            let blend_color = pipeline.blend_color.unwrap_or(0);
                            let fog_color = pipeline.fog_color.unwrap_or(0);

                            textured_rect_uniforms.push(TexturedRectUniforms {
                                offset: [offset_x, offset_y],
                                scale: [scale.x, scale.y],
                                combine_mode: [
                                    ((color_combiner_mode >> 32) & u32::MAX as u64) as u32,
                                    (color_combiner_mode & u32::MAX as u64) as u32,
                                ],
                                blend_mode: [
                                    ((blend_mode >> 32) & u32::MAX as u64) as u32,
                                    (blend_mode & u32::MAX as u64) as u32,
                                ],
                                prim_color: [
                                    ((prim_color >> 24) & 0xff) as f32 / 255.0,
                                    ((prim_color >> 16) & 0xff) as f32 / 255.0,
                                    ((prim_color >> 8) & 0xff) as f32 / 255.0,
                                    (prim_color & 0xff) as f32 / 255.0,
                                ],
                                env_color: [
                                    ((env_color >> 24) & 0xff) as f32 / 255.0,
                                    ((env_color >> 16) & 0xff) as f32 / 255.0,
                                    ((env_color >> 8) & 0xff) as f32 / 255.0,
                                    (env_color & 0xff) as f32 / 255.0,
                                ],
                                blend_color: [
                                    ((blend_color >> 24) & 0xff) as f32 / 255.0,
                                    ((blend_color >> 16) & 0xff) as f32 / 255.0,
                                    ((blend_color >> 8) & 0xff) as f32 / 255.0,
                                    (blend_color & 0xff) as f32 / 255.0,
                                ],
                                fog_color: [
                                    ((fog_color >> 24) & 0xff) as f32 / 255.0,
                                    ((fog_color >> 16) & 0xff) as f32 / 255.0,
                                    ((fog_color >> 8) & 0xff) as f32 / 255.0,
                                    (fog_color & 0xff) as f32 / 255.0,
                                ],
                            });
                        }
                        Command::Mesh {
                            verts,
                            uvs,
                            colors,
                            indices,
                            transform,
                            pipeline,
                            buffer_index,
                            ..
                        } => {
                            #[repr(C)]
                            #[derive(Clone, Copy, AsBytes, FromBytes)]
                            pub struct MeshVertex {
                                pos: [f32; 3],
                                tex_coord: [f32; 2],
                                color: [f32; 4],
                            }

                            assert!(verts.len() == uvs.len());
                            assert!(verts.len() == colors.len());

                            let mut vertices = Vec::with_capacity(verts.len());

                            for (v, (t, c)) in verts.iter().zip(uvs.iter().zip(colors.iter())) {
                                vertices.push(MeshVertex {
                                    pos: *v,
                                    tex_coord: *t,
                                    color: [
                                        ((*c >> 24) & 0xff) as f32 / 255.0,
                                        ((*c >> 16) & 0xff) as f32 / 255.0,
                                        ((*c >> 8) & 0xff) as f32 / 255.0,
                                        (*c & 0xff) as f32 / 255.0,
                                    ],
                                });
                            }

                            render_pass_vertex_buffers.push(graphics.device.create_buffer_init(
                                &wgpu::util::BufferInitDescriptor {
                                    label: None,
                                    contents: vertices.as_bytes(),
                                    usage: wgpu::BufferUsages::VERTEX,
                                },
                            ));

                            render_pass_index_buffers.push(
                                graphics.device.create_buffer_init(
                                    &wgpu::util::BufferInitDescriptor {
                                        label: None,
                                        contents: indices
                                            .iter()
                                            .map(|v| *v as u16)
                                            .collect::<Vec<u16>>()
                                            .as_bytes(),
                                        usage: wgpu::BufferUsages::INDEX,
                                    },
                                ),
                            );

                            assert!(!render_pass_vertex_buffers.is_empty());
                            assert!(
                                render_pass_vertex_buffers.len() == render_pass_index_buffers.len()
                            );

                            *buffer_index = render_pass_vertex_buffers.len() - 1;

                            if let Some(texture) = &pipeline.texture {
                                graphics.mesh.upload_texture_data(
                                    &graphics.device,
                                    &graphics.queue,
                                    texture,
                                );
                            }

                            let color_combiner_mode = pipeline.color_combiner_mode.to_command();
                            let blend_mode = pipeline.blend_mode.to_command();
                            let prim_color = pipeline.prim_color.unwrap_or(0);
                            let env_color = pipeline.env_color.unwrap_or(0);
                            let blend_color = pipeline.blend_color.unwrap_or(0);
                            let fog_color = pipeline.fog_color.unwrap_or(0);

                            mesh_uniforms.push(MeshUniforms {
                                transform: *transform,
                                screen_size_and_pad: [
                                    graphics.video_mode.width() as f32,
                                    graphics.video_mode.height() as f32,
                                    0.0,
                                    0.0,
                                ],
                                combine_mode: [
                                    ((color_combiner_mode >> 32) & u32::MAX as u64) as u32,
                                    (color_combiner_mode & u32::MAX as u64) as u32,
                                ],
                                blend_mode: [
                                    ((blend_mode >> 32) & u32::MAX as u64) as u32,
                                    (blend_mode & u32::MAX as u64) as u32,
                                ],
                                prim_color: [
                                    ((prim_color >> 24) & 0xff) as f32 / 255.0,
                                    ((prim_color >> 16) & 0xff) as f32 / 255.0,
                                    ((prim_color >> 8) & 0xff) as f32 / 255.0,
                                    (prim_color & 0xff) as f32 / 255.0,
                                ],
                                env_color: [
                                    ((env_color >> 24) & 0xff) as f32 / 255.0,
                                    ((env_color >> 16) & 0xff) as f32 / 255.0,
                                    ((env_color >> 8) & 0xff) as f32 / 255.0,
                                    (env_color & 0xff) as f32 / 255.0,
                                ],
                                blend_color: [
                                    ((blend_color >> 24) & 0xff) as f32 / 255.0,
                                    ((blend_color >> 16) & 0xff) as f32 / 255.0,
                                    ((blend_color >> 8) & 0xff) as f32 / 255.0,
                                    (blend_color & 0xff) as f32 / 255.0,
                                ],
                                fog_color: [
                                    ((fog_color >> 24) & 0xff) as f32 / 255.0,
                                    ((fog_color >> 16) & 0xff) as f32 / 255.0,
                                    ((fog_color >> 8) & 0xff) as f32 / 255.0,
                                    (fog_color & 0xff) as f32 / 255.0,
                                ],
                            });
                        }
                    }
                }

                if !colored_rect_uniforms.is_empty() {
                    let temp_buffer =
                        graphics
                            .device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: colored_rect_uniforms.as_bytes(),
                                usage: wgpu::BufferUsages::COPY_SRC,
                            });

                    encoder.copy_buffer_to_buffer(
                        &temp_buffer,
                        0,
                        &graphics.colored_rect.shader_storage_buffer,
                        0,
                        (colored_rect_uniforms.len() * mem::size_of::<ColoredRectUniforms>())
                            as u64,
                    );
                }

                if !textured_rect_uniforms.is_empty() {
                    let temp_buffer =
                        graphics
                            .device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: textured_rect_uniforms.as_bytes(),
                                usage: wgpu::BufferUsages::COPY_SRC,
                            });

                    encoder.copy_buffer_to_buffer(
                        &temp_buffer,
                        0,
                        &graphics.textured_rect.shader_storage_buffer,
                        0,
                        (textured_rect_uniforms.len() * mem::size_of::<TexturedRectUniforms>())
                            as u64,
                    );
                }

                if !mesh_uniforms.is_empty() {
                    let temp_buffer =
                        graphics
                            .device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: mesh_uniforms.as_bytes(),
                                usage: wgpu::BufferUsages::COPY_SRC,
                            });

                    encoder.copy_buffer_to_buffer(
                        &temp_buffer,
                        0,
                        &graphics.mesh.shader_storage_buffer,
                        0,
                        (mesh_uniforms.len() * mem::size_of::<MeshUniforms>()) as u64,
                    );
                }
            }

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &dst.tex_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: if self.clear {
                                wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 1.0,
                                })
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &dst.depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: if self.clear {
                                wgpu::LoadOp::Clear(1.0)
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                });

                {
                    let mut colored_rect_index = 0;
                    let mut textured_rect_index = 0;
                    let mut mesh_index = 0;

                    for command in &self.cache.commands {
                        match command {
                            Command::ColoredRect { .. } => {
                                render_pass.set_index_buffer(
                                    graphics.quad_index_buf.slice(..),
                                    wgpu::IndexFormat::Uint16,
                                );
                                render_pass
                                    .set_vertex_buffer(0, graphics.quad_vertex_buf.slice(..));
                                render_pass.set_pipeline(&graphics.colored_rect.pipeline);
                                render_pass.set_bind_group(
                                    0,
                                    &graphics.colored_rect.bind_group,
                                    &[],
                                );
                                render_pass.draw_indexed(
                                    0..(QUAD_INDEX_DATA.len() as u32),
                                    0,
                                    colored_rect_index..(colored_rect_index + 1),
                                );
                                colored_rect_index += 1;
                            }
                            Command::TexturedRect { pipeline, .. } => {
                                render_pass.set_index_buffer(
                                    graphics.quad_index_buf.slice(..),
                                    wgpu::IndexFormat::Uint16,
                                );
                                render_pass
                                    .set_vertex_buffer(0, graphics.quad_vertex_buf.slice(..));
                                render_pass.set_pipeline(&graphics.textured_rect.pipeline);
                                render_pass.set_bind_group(
                                    0,
                                    &graphics
                                        .textured_rect
                                        .texture_cache
                                        .get(
                                            &(pipeline
                                                .texture
                                                .expect("Invalid pipeline")
                                                .data
                                                .as_ptr()
                                                as _),
                                        )
                                        .unwrap()
                                        .bind_group,
                                    &[],
                                );
                                render_pass.draw_indexed(
                                    0..(QUAD_INDEX_DATA.len() as u32),
                                    0,
                                    textured_rect_index..(textured_rect_index + 1),
                                );
                                textured_rect_index += 1;
                            }
                            Command::Mesh {
                                indices,
                                pipeline,
                                buffer_index,
                                ..
                            } => {
                                let tex_key = if let Some(texture) = &pipeline.texture {
                                    texture.data.as_ptr() as _
                                } else {
                                    0
                                };

                                let pipeline = match (pipeline.z_compare, pipeline.z_update) {
                                    (true, true) => {
                                        &graphics.mesh.pipeline_with_depth_compare_and_depth_write
                                    }
                                    (true, false) => &graphics.mesh.pipeline_with_depth_compare,
                                    (false, true) => &graphics.mesh.pipeline_with_depth_write,
                                    (false, false) => &graphics.mesh.pipeline_with_no_depth,
                                };

                                render_pass.set_index_buffer(
                                    render_pass_index_buffers[*buffer_index].slice(..),
                                    wgpu::IndexFormat::Uint16,
                                );
                                render_pass.set_vertex_buffer(
                                    0,
                                    render_pass_vertex_buffers[*buffer_index].slice(..),
                                );
                                render_pass.set_pipeline(pipeline);
                                render_pass.set_bind_group(
                                    0,
                                    &graphics.mesh.shader_storage_buffer_bind_group,
                                    &[],
                                );
                                render_pass.set_bind_group(
                                    1,
                                    &graphics
                                        .mesh
                                        .texture_cache
                                        .get(&tex_key)
                                        .unwrap()
                                        .bind_group,
                                    &[],
                                );
                                render_pass.draw_indexed(
                                    0..(indices.len() as u32),
                                    0,
                                    mesh_index..(mesh_index + 1),
                                );
                                mesh_index += 1;
                            }
                        }
                    }
                }
            }

            encoder.copy_texture_to_texture(
                wgpu::ImageCopyTexture {
                    texture: &dst.tex,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::ImageCopyTexture {
                    texture: &dst.download_tex,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                dst.tex_extent,
            );

            encoder.copy_texture_to_buffer(
                wgpu::ImageCopyTexture {
                    texture: &dst.download_tex,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::ImageCopyBuffer {
                    buffer: &dst.buffer,
                    layout: wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: NonZeroU32::new(4 * self.cache.video_mode.width() as u32),
                        rows_per_image: NonZeroU32::new(self.cache.video_mode.height() as u32),
                    },
                },
                dst.tex_extent,
            );

            encoder.finish()
        };

        graphics.queue.submit(Some(command_buf));

        {
            let mapped = Arc::new(AtomicBool::new(false));

            dst.buffer
                .slice(
                    0..((4 * self.cache.video_mode.width() * self.cache.video_mode.height())
                        as u64),
                )
                .map_async(wgpu::MapMode::Read, {
                    let mapped = mapped.clone();
                    move |mapped_slice| {
                        assert!(mapped_slice.is_ok());
                        mapped.store(true, Ordering::SeqCst)
                    }
                });

            while !mapped.load(Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(1));
            }

            let mapped_colored_rect_dst_buffer = dst
                .buffer
                .slice(
                    0..((4 * self.cache.video_mode.width() * self.cache.video_mode.height())
                        as u64),
                )
                .get_mapped_range();

            for (fb_color, mapped_color) in unsafe {
                slice::from_raw_parts_mut(self.out_tex.0, self.cache.video_mode.size() as usize)
            }
            .iter_mut()
            .zip(mapped_colored_rect_dst_buffer.chunks(4))
            {
                *fb_color = Color::from_bytes(mapped_color.assert_into());
            }
        }

        (
            self.colored_rect_count as i32,
            self.textured_rect_count as i32,
            self.mesh_count as i32,
        )
    }
}
