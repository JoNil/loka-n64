#![allow(clippy::inconsistent_digit_grouping)]

use crate::{gfx::Texture, graphics_emu::shader};
use n64_math::Color;
use std::{collections::HashMap, mem, num::NonZeroU32};
use zerocopy::{AsBytes, FromBytes};

pub const MAX_MESHES: u64 = 4096;

#[repr(C)]
#[derive(Clone, Copy, Debug, AsBytes, FromBytes)]
pub(crate) struct MeshUniforms {
    pub transform: [[f32; 4]; 4],
    pub screen_size_and_pad: [f32; 4],
    pub combine_mode: [u32; 2],
    pub blend_mode: [u32; 2],
    pub prim_color: [f32; 4],
    pub env_color: [f32; 4],
    pub blend_color: [f32; 4],
    pub fog_color: [f32; 4],
}

pub(crate) struct UploadedTexture {
    pub bind_group: wgpu::BindGroup,
}

pub(crate) struct Mesh {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline_with_depth_compare_and_depth_write: wgpu::RenderPipeline,
    pub pipeline_with_depth_compare: wgpu::RenderPipeline,
    pub pipeline_with_depth_write: wgpu::RenderPipeline,
    pub pipeline_with_no_depth: wgpu::RenderPipeline,
    pub shader_storage_buffer: wgpu::Buffer,
    pub sampler: wgpu::Sampler,
    pub texture_cache: HashMap<usize, UploadedTexture>,
}

impl Mesh {
    pub(crate) fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        dst_tex_format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
    ) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let (vs_module, fs_module) = shader::compile(
            device,
            include_str!("shaders/mesh.vert"),
            include_str!("shaders/mesh.frag"),
        );

        let target_desc = &[wgpu::ColorTargetState {
            format: dst_tex_format,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::Zero,
                    operation: wgpu::BlendOperation::Add,
                },
            }),
            write_mask: wgpu::ColorWrites::ALL,
        }];

        let mut pipeline_desc = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 9 * mem::size_of::<f32>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 3 * mem::size_of::<f32>() as u64,
                            shader_location: 1,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 5 * mem::size_of::<f32>() as u64,
                            shader_location: 2,
                        },
                    ],
                }],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: true,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: target_desc,
            }),
            multiview: None,
        };

        let pipeline_with_depth_compare_and_depth_write =
            device.create_render_pipeline(&pipeline_desc);

        pipeline_desc.depth_stencil = Some(wgpu::DepthStencilState {
            format: depth_format,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        let pipeline_with_depth_compare = device.create_render_pipeline(&pipeline_desc);

        pipeline_desc.depth_stencil = Some(wgpu::DepthStencilState {
            format: depth_format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Always,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        let pipeline_with_depth_write = device.create_render_pipeline(&pipeline_desc);

        pipeline_desc.depth_stencil = Some(wgpu::DepthStencilState {
            format: depth_format,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::Always,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        let pipeline_with_no_depth = device.create_render_pipeline(&pipeline_desc);

        let shader_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: MAX_MESHES * mem::size_of::<MeshUniforms>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        });

        let texture_cache = HashMap::new();

        let mut mesh = Self {
            bind_group_layout,
            pipeline_with_depth_compare_and_depth_write,
            pipeline_with_depth_compare,
            pipeline_with_depth_write,
            pipeline_with_no_depth,
            shader_storage_buffer,
            sampler,
            texture_cache,
        };

        {
            #[allow(clippy::unusual_byte_groupings)]
            let data = [Color::new(0b11111_11111_11111_1)];

            let texture = Texture {
                width: 1,
                height: 1,
                data: &data,
            };

            mesh.upload_texture_data_internal(device, queue, 0, &texture);
        }

        mesh
    }

    pub(crate) fn upload_texture_data_internal(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        key: usize,
        texture: &Texture,
    ) {
        if self.texture_cache.contains_key(&key) {
            return;
        }

        let tex_format = wgpu::TextureFormat::Rgba8Unorm;

        let tex_extent = wgpu::Extent3d {
            width: texture.width as u32,
            height: texture.height as u32,
            depth_or_array_layers: 1,
        };
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: tex_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: tex_format,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        });
        let tex_view = tex.create_view(&Default::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        self.shader_storage_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&tex_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
            label: None,
        });

        let mut buffer = Vec::new();
        buffer.resize_with(4 * texture.data.len(), Default::default);

        for (pixel, data) in texture.data.iter().zip(buffer.chunks_exact_mut(4_usize)) {
            let rgba = pixel.be_to_le().to_rgba();

            data[0] = (rgba[0] * 255.0) as u8;
            data[1] = (rgba[1] * 255.0) as u8;
            data[2] = (rgba[2] * 255.0) as u8;
            data[3] = (rgba[3] * 255.0) as u8;
        }

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            &buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * texture.width as u32),
                rows_per_image: NonZeroU32::new(texture.height as u32),
            },
            tex_extent,
        );

        self.texture_cache
            .insert(key, UploadedTexture { bind_group });
    }

    pub(crate) fn upload_texture_data(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &Texture,
    ) {
        self.upload_texture_data_internal(device, queue, texture.data.as_ptr() as _, texture)
    }
}
