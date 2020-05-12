use crate::{
    gfx::Texture,
    graphics_emu::Vertex,
};
use std::{collections::HashMap, mem};
use zerocopy::{AsBytes, FromBytes};
use n64_math::Color;

pub const MAX_TEXTURED_RECTS: u64 = 4096;
pub const MAX_TEXTURED_RECT_TEXTURES: u64 = 4096;
pub const TEX_HEIGHT: u32 = 32;
pub const TEX_WIDTH: u32 = 32;

#[repr(C)]
#[derive(Clone, Copy, Debug, AsBytes, FromBytes)]
pub(crate) struct TexturedRectUniforms {
    pub texture: u32,
    pub padding: [u32; 3],
    pub offset: [f32; 2],
    pub scale: [f32; 2],
}

pub(crate) struct TexturedRect {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
    pub sampler: wgpu::Sampler,
    pub tex_format: wgpu::TextureFormat,
    pub tex_extent: wgpu::Extent3d,
    pub tex: wgpu::Texture,
    pub tex_view: wgpu::TextureView,
    pub shader_storage_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub texture_cache: HashMap<*const [Color], u32>,
}

impl TexturedRect {
    pub(crate) fn new(device: &wgpu::Device, dst_tex_format: wgpu::TextureFormat) -> Self {

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        component_type: wgpu::TextureComponentType::Float,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let vs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("shaders/textured_rect.vert"),
                glsl_to_spirv::ShaderType::Vertex,
            )
            .map_err(|e| { println!("{}", e); "Unable to compile shaders/textured_rect.vert" }).unwrap(),
        )
        .unwrap();

        let fs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("shaders/textured_rect.frag"),
                glsl_to_spirv::ShaderType::Fragment,
            )
            .map_err(|e| { println!("{}", e); "Unable to compile shaders/textured_rect.frag" }).unwrap(),
        )
        .unwrap();

        let vs_module = device.create_shader_module(&vs_bytes);
        let fs_module = device.create_shader_module(&fs_bytes);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
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
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: dst_tex_format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
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

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Undefined,
        });

        let tex_format = wgpu::TextureFormat::Rgba8Unorm;

        let tex_extent = wgpu::Extent3d {
            width: TEX_WIDTH,
            height: TEX_HEIGHT,
            depth: 1,
        };
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: tex_extent,
            array_layer_count: MAX_TEXTURED_RECT_TEXTURES,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: tex_format,
            usage: wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
        });
        let tex_view = tex.create_default_view();

        let shader_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: MAX_TEXTURED_RECTS * mem::size_of::<TexturedRectUniforms>() as u64,
            usage: wgpu::BufferUsage::STORAGE_READ | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group = 
            device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &shader_storage_buffer,
                            range: 0..(MAX_TEXTURED_RECTS * mem::size_of::<TexturedRectUniforms>() as u64),
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&tex_view),
                    },
                    wgpu::Binding {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
                label: None,
            });

        let texture_cache = HashMap::new();

        Self {
            bind_group_layout,
            pipeline_layout,
            vs_module,
            fs_module,
            pipeline,
            sampler,
            tex_format,
            tex_extent,
            tex,
            tex_view,
            shader_storage_buffer,
            bind_group,
            texture_cache,
        }
    }

    pub(crate) fn upload_texture_data(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        texture: &Texture,
    ) -> u32 {

        if self.texture_cache.contains_key(&(texture.data as *const _)) {
            return self.texture_cache.get(&(texture.data as *const _));
        }

        assert!(texture_data.width == TEX_WIDTH);
        assert!(texture_data.height == TEX_HEIGHT);

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
            .iter()
            .zip(temp_buffer.chunks_exact_mut(4 as usize))
        {            
            let rgba = pixel.be_to_le().to_rgba();

            data[0] = (rgba[0] * 255.0) as u8;
            data[1] = (rgba[1] * 255.0) as u8;
            data[2] = (rgba[2] * 255.0) as u8;
            data[3] = (rgba[3] * 255.0) as u8;
        }

        let temp_buf = device.create_buffer_with_data(&temp_buffer, wgpu::BufferUsage::COPY_SRC);

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
    }
}
