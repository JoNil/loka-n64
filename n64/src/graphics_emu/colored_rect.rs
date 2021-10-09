use crate::graphics_emu::Vertex;
use assert_into::AssertInto;
use std::{io::Read, mem};
use zerocopy::{AsBytes, FromBytes};

pub const MAX_COLORED_RECTS: u64 = 4096;

#[repr(C)]
#[derive(Clone, Copy, Debug, AsBytes, FromBytes)]
pub(crate) struct ColoredRectUniforms {
    pub color: [f32; 4],
    pub offset: [f32; 2],
    pub scale: [f32; 2],
}

pub(crate) struct ColoredRect {
    pub pipeline: wgpu::RenderPipeline,
    pub shader_storage_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl ColoredRect {
    pub(crate) fn new(device: &wgpu::Device, dst_tex_format: wgpu::TextureFormat) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vs_bytes = {
            let mut buffer = Vec::new();
            let mut file = glsl_to_spirv::compile(
                include_str!("shaders/colored_rect.vert"),
                glsl_to_spirv::ShaderType::Vertex,
            )
            .map_err(|e| {
                println!("{}", e);
                "Unable to compile shaders/colored_rect.vert"
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
                include_str!("shaders/colored_rect.frag"),
                glsl_to_spirv::ShaderType::Fragment,
            )
            .map_err(|e| {
                println!("{}", e);
                "Unable to compile shaders/colored_rect.frag"
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
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    }],
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
                    format: dst_tex_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
        });

        let shader_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: MAX_COLORED_RECTS * mem::size_of::<ColoredRectUniforms>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    shader_storage_buffer.as_entire_buffer_binding(),
                ),
            }],
        });

        Self {
            pipeline,
            shader_storage_buffer,
            bind_group,
        }
    }
}
