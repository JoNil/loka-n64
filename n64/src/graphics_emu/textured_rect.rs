use crate::{gfx::Texture, graphics_emu::Vertex};
use assert_into::AssertInto;
use std::{collections::HashMap, io::Read, mem};
use zerocopy::{AsBytes, FromBytes};

pub const MAX_TEXTURED_RECTS: u64 = 4096;

#[repr(C)]
#[derive(Clone, Copy, Debug, AsBytes, FromBytes)]
pub(crate) struct TexturedRectUniforms {
    pub offset: [f32; 2],
    pub scale: [f32; 2],
    pub blend_color: [f32; 4],
}

pub(crate) struct UploadedTexture {
    pub tex_format: wgpu::TextureFormat,
    pub tex_extent: wgpu::Extent3d,
    pub tex: wgpu::Texture,
    pub tex_view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
}

pub(crate) struct TexturedRect {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
    pub shader_storage_buffer: wgpu::Buffer,
    pub sampler: wgpu::Sampler,
    pub texture_cache: HashMap<usize, UploadedTexture>,
}

impl TexturedRect {
    pub(crate) fn new(device: &wgpu::Device, dst_tex_format: wgpu::TextureFormat) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        readonly: true,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        component_type: wgpu::TextureComponentType::Float,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
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

        let vs_bytes = {
            let mut buffer = Vec::new();
            let mut file = glsl_to_spirv::compile(
                include_str!("shaders/textured_rect.vert"),
                glsl_to_spirv::ShaderType::Vertex,
            )
            .map_err(|e| {
                println!("{}", e);
                "Unable to compile shaders/textured_rect.vert"
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
                include_str!("shaders/textured_rect.frag"),
                glsl_to_spirv::ShaderType::Fragment,
            )
            .map_err(|e| {
                println!("{}", e);
                "Unable to compile shaders/textured_rect.frag"
            })
            .unwrap();
            file.read_to_end(&mut buffer).unwrap();
            buffer
                .chunks_exact(4)
                .map(|chunk| u32::from_le_bytes(chunk.assert_into()))
                .collect::<Vec<_>>()
        };

        let vs_module =
            device.create_shader_module(wgpu::ShaderModuleSource::SpirV(vs_bytes.into()));
        let fs_module =
            device.create_shader_module(wgpu::ShaderModuleSource::SpirV(fs_bytes.into()));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                clamp_depth: false,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: dst_tex_format,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::Zero,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float3,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float2,
                            offset: 3 * mem::size_of::<f32>() as u64,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let shader_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: MAX_TEXTURED_RECTS * mem::size_of::<TexturedRectUniforms>() as u64,
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
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
        });

        let texture_cache = HashMap::new();

        Self {
            bind_group_layout,
            pipeline_layout,
            vs_module,
            fs_module,
            pipeline,
            shader_storage_buffer,
            sampler,
            texture_cache,
        }
    }

    pub(crate) fn upload_texture_data(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &Texture,
    ) {
        if self
            .texture_cache
            .contains_key(&(texture.data.as_ptr() as _))
        {
            return;
        }

        let tex_format = wgpu::TextureFormat::Rgba8Unorm;

        let tex_extent = wgpu::Extent3d {
            width: texture.width as u32,
            height: texture.height as u32,
            depth: 1,
        };
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: tex_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: tex_format,
            usage: wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
        });
        let tex_view = tex.create_view(&Default::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(self.shader_storage_buffer.slice(
                        0..(MAX_TEXTURED_RECTS * mem::size_of::<TexturedRectUniforms>() as u64),
                    )),
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

        for (pixel, data) in texture.data.iter().zip(buffer.chunks_exact_mut(4 as usize)) {
            let rgba = pixel.be_to_le().to_rgba();

            data[0] = (rgba[0] * 255.0) as u8;
            data[1] = (rgba[1] * 255.0) as u8;
            data[2] = (rgba[2] * 255.0) as u8;
            data[3] = (rgba[3] * 255.0) as u8;
        }

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
            },
            &buffer,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * texture.width as u32,
                rows_per_image: texture.height as u32,
            },
            tex_extent,
        );

        self.texture_cache.insert(
            texture.data.as_ptr() as _,
            UploadedTexture {
                tex_format,
                tex_extent,
                tex,
                tex_view,
                bind_group,
            },
        );
    }
}
