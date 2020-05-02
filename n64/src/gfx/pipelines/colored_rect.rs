use crate::graphics_emu::{HEIGHT, WIDTH, Vertex};
use std::mem;

pub(crate) struct ColoredRect {
    pub dst_buffer: wgpu::Buffer,
    pub dst_tex_format: wgpu::TextureFormat,
    pub dst_tex_extent: wgpu::Extent3d,
    pub dst_tex: wgpu::Texture,
    pub dst_tex_view: wgpu::TextureView,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
}

impl ColoredRect {
    pub(crate) fn new(device: &wgpu::Device) -> Self {

        let dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (4 * WIDTH * HEIGHT) as u64,
            usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
        });

        let dst_tex_format = wgpu::TextureFormat::Rgba8Unorm;

        let dst_tex_extent = wgpu::Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth: 1,
        };
        let dst_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: dst_tex_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: dst_tex_format,
            usage: wgpu::TextureUsage::COPY_DST
                | wgpu::TextureUsage::COPY_SRC
                | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        });
        let dst_tex_view = dst_tex.create_default_view();

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                }],
                label: None,
            });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });

        let vs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("shaders/colored_rect.vert"),
                glsl_to_spirv::ShaderType::Vertex,
            )
            .unwrap(),
        )
        .unwrap();

        let fs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("shaders/colored_rect.frag"),
                glsl_to_spirv::ShaderType::Fragment,
            )
            .unwrap(),
        )
        .unwrap();

        let vs_module = device.create_shader_module(&vs_bytes);
        let fs_module = device.create_shader_module(&fs_bytes);

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                        attributes: &[wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float3,
                            offset: 0,
                            shader_location: 0,
                        }],
                    }],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        Self {
            dst_buffer,
            dst_tex_format,
            dst_tex_extent,
            dst_tex,
            dst_tex_view,
            bind_group_layout,
            pipeline_layout,
            vs_module,
            fs_module,
            pipeline,
        }
    }
}
