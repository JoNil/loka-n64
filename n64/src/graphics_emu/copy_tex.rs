use assert_into::AssertInto;

use crate::{graphics_emu::Vertex, VideoMode};
use std::{io::Read, mem};

pub(crate) struct CopyTex {
    pub src_buffer: Box<[u8]>,
    pub src_tex_extent: wgpu::Extent3d,
    pub src_tex: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
}

impl CopyTex {
    pub(crate) fn new(
        device: &wgpu::Device,
        swap_chain_desc: &wgpu::SwapChainDescriptor,
        video_mode: VideoMode,
    ) -> Self {
        let src_buffer = {
            let mut buffer = Vec::new();
            buffer.resize_with(
                (4 * video_mode.width() * video_mode.height()) as usize,
                || 0,
            );
            buffer.into_boxed_slice()
        };

        let src_tex_extent = wgpu::Extent3d {
            width: video_mode.width() as u32,
            height: video_mode.height() as u32,
            depth_or_array_layers: 1,
        };
        let src_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: src_tex_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });
        let src_tex_view = src_tex.create_view(&Default::default());

        let src_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(wgpu::CompareFunction::Always),
            anisotropy_clamp: None,
            border_color: None,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&src_tex_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&src_sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vs_bytes = {
            let mut buffer = Vec::new();
            let mut file = glsl_to_spirv::compile(
                include_str!("shaders/copy_tex.vert"),
                glsl_to_spirv::ShaderType::Vertex,
            )
            .map_err(|e| {
                println!("{}", e);
                "Unable to compile shaders/copy_tex.vert"
            })
            .unwrap();
            file.read_to_end(&mut buffer).unwrap();
            buffer
                .chunks_exact(4)
                .map(|chunk| u32::from_le_bytes(chunk.assert_into()))
                .collect::<Vec<_>>()
        };

        let fs_bytes = {
            let mut buffer = Vec::new();
            let mut file = glsl_to_spirv::compile(
                include_str!("shaders/copy_tex.frag"),
                glsl_to_spirv::ShaderType::Fragment,
            )
            .map_err(|e| {
                println!("{}", e);
                "Unable to compile shaders/frag.vert"
            })
            .unwrap();
            file.read_to_end(&mut buffer).unwrap();
            buffer
                .chunks_exact(4)
                .map(|chunk| u32::from_le_bytes(chunk.assert_into()))
                .collect::<Vec<_>>()
        };

        let vs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(vs_bytes.into()),
        });
        let fs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(fs_bytes.into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
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
                    ],
                }],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: swap_chain_desc.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
        });

        Self {
            src_buffer,
            src_tex_extent,
            src_tex,
            bind_group,
            pipeline,
        }
    }
}
