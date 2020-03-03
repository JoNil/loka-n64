use lazy_static::lazy_static;
use n64_math::Color;
use std::collections::HashSet;
use std::mem;
use std::process::exit;
use std::sync::Mutex;
use std::thread_local;
use winit::{
    event::{self, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::desktop::EventLoopExtDesktop,
    window::Window,
};
use zerocopy::{AsBytes, FromBytes};

pub const WIDTH: i32 = 320;
pub const HEIGHT: i32 = 240;

const FRAME_BUFFER_SIZE: usize = (WIDTH * HEIGHT) as usize;
const SCALE: i32 = 4;

#[repr(C)]
#[derive(Clone, Copy, Debug, AsBytes, FromBytes)]
pub(crate) struct ColoredRectUniforms {
    pub color: [f32; 4],
    pub offset: [f32; 2],
    pub scale: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes)]
struct Vertex {
    pos: [f32; 3],
}

static QUAD_VERTEX_DATA: &'static [Vertex] = &[
    Vertex {
        pos: [-1.0, -1.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0],
    },
];

pub(crate) static QUAD_INDEX_DATA: &'static [u16] = &[0, 1, 2, 2, 3, 0];

thread_local! {
    static EVENT_LOOP: Mutex<EventLoop<()>> = Mutex::new(EventLoop::new());
}

lazy_static! {
    pub(crate) static ref FRAMEBUFFER_STATE: Mutex<FramebufferState> =
        Mutex::new(FramebufferState::new());
}

pub(crate) struct FramebufferState {
    pub using_framebuffer_a: bool,
    pub framebuffer_a: Box<[Color]>,
    pub framebuffer_b: Box<[Color]>,
}

impl FramebufferState {
    fn new() -> FramebufferState {
        FramebufferState {
            using_framebuffer_a: false,
            framebuffer_a: {
                let mut buffer = Vec::new();
                buffer.resize_with(FRAME_BUFFER_SIZE, || Color::new(0x0001));
                buffer.into_boxed_slice()
            },
            framebuffer_b: {
                let mut buffer = Vec::new();
                buffer.resize_with(FRAME_BUFFER_SIZE, || Color::new(0x0001));
                buffer.into_boxed_slice()
            },
        }
    }

    pub(crate) fn next_buffer(&mut self) -> &mut [Color] {
        if self.using_framebuffer_a {
            &mut self.framebuffer_a[..]
        } else {
            &mut self.framebuffer_b[..]
        }
    }

    pub(crate) fn swap_buffer(&mut self) {
        self.using_framebuffer_a = !self.using_framebuffer_a;
    }
}

lazy_static! {
    pub(crate) static ref GFX_EMU_STATE: Mutex<GfxEmuState> = Mutex::new(GfxEmuState::new());
}

pub(crate) struct GfxEmuState {
    pub window: Window,
    pub keys_down: HashSet<VirtualKeyCode>,

    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,

    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,

    pub colored_rect_dst_buffer: wgpu::Buffer,
    pub colored_rect_dst_tex_format: wgpu::TextureFormat,
    pub colored_rect_dst_tex_extent: wgpu::Extent3d,
    pub colored_rect_dst_tex: wgpu::Texture,
    pub colored_rect_dst_tex_view: wgpu::TextureView,
    pub colored_rect_bind_group_layout: wgpu::BindGroupLayout,
    pub colored_rect_pipeline_layout: wgpu::PipelineLayout,
    pub colored_rect_vs_module: wgpu::ShaderModule,
    pub colored_rect_fs_module: wgpu::ShaderModule,
    pub colored_rect_pipeline: wgpu::RenderPipeline,

    pub copy_tex_src_buffer: Box<[u8]>,
    pub copy_tex_src_tex_extent: wgpu::Extent3d,
    pub copy_tex_src_tex: wgpu::Texture,
    pub copy_tex_src_tex_view: wgpu::TextureView,
    pub copy_tex_src_sampler: wgpu::Sampler,
    pub copy_tex_bind_group_layout: wgpu::BindGroupLayout,
    pub copy_tex_bind_group: wgpu::BindGroup,
    pub copy_tex_pipeline_layout: wgpu::PipelineLayout,
    pub copy_tex_vs_module: wgpu::ShaderModule,
    pub copy_tex_fs_module: wgpu::ShaderModule,
    pub copy_tex_pipeline: wgpu::RenderPipeline,
}

impl GfxEmuState {
    fn new() -> GfxEmuState {
        let window = {
            let mut builder = winit::window::WindowBuilder::new();
            builder = builder.with_title("N64");
            builder = builder
                .with_inner_size(winit::dpi::LogicalSize::new(SCALE * WIDTH, SCALE * HEIGHT));
            builder = builder.with_visible(false);
            EVENT_LOOP.with(|event_loop| builder.build(&event_loop.lock().unwrap()).unwrap())
        };

        let keys_down = HashSet::new();

        let size = window.inner_size();

        let surface = wgpu::Surface::create(&window);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
            },
            wgpu::BackendBit::PRIMARY,
        )
        .unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Vsync,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let vertex_buf =
            device.create_buffer_with_data(QUAD_VERTEX_DATA.as_bytes(), wgpu::BufferUsage::VERTEX);

        let index_buf =
            device.create_buffer_with_data(QUAD_INDEX_DATA.as_bytes(), wgpu::BufferUsage::INDEX);

        let colored_rect_dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: (4 * WIDTH * HEIGHT) as u64,
            usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
        });

        let colored_rect_dst_tex_format = wgpu::TextureFormat::Rgba8Unorm;

        let colored_rect_dst_tex_extent = wgpu::Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth: 1,
        };
        let colored_rect_dst_tex = device.create_texture(&wgpu::TextureDescriptor {
            size: colored_rect_dst_tex_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: colored_rect_dst_tex_format,
            usage: wgpu::TextureUsage::COPY_DST
                | wgpu::TextureUsage::COPY_SRC
                | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        });
        let colored_rect_dst_tex_view = colored_rect_dst_tex.create_default_view();

        let colored_rect_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT | wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                }],
            });

        let colored_rect_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&colored_rect_bind_group_layout],
            });

        let colored_rect_vs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("gfx/shaders/colored_rect.vert"),
                glsl_to_spirv::ShaderType::Vertex,
            )
            .unwrap(),
        )
        .unwrap();

        let colored_rect_fs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("gfx/shaders/colored_rect.frag"),
                glsl_to_spirv::ShaderType::Fragment,
            )
            .unwrap(),
        )
        .unwrap();

        let colored_rect_vs_module = device.create_shader_module(&colored_rect_vs_bytes);
        let colored_rect_fs_module = device.create_shader_module(&colored_rect_fs_bytes);

        let colored_rect_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &colored_rect_pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &colored_rect_vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &colored_rect_fs_module,
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
                    format: colored_rect_dst_tex_format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                depth_stencil_state: None,
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
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        let copy_tex_src_buffer = {
            let mut buffer = Vec::new();
            buffer.resize_with((4 * WIDTH * HEIGHT) as usize, || 0);
            buffer.into_boxed_slice()
        };

        let copy_tex_src_tex_extent = wgpu::Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth: 1,
        };
        let copy_tex_src_tex = device.create_texture(&wgpu::TextureDescriptor {
            size: copy_tex_src_tex_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });
        let copy_tex_src_tex_view = copy_tex_src_tex.create_default_view();

        let copy_tex_src_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare_function: wgpu::CompareFunction::Always,
        });

        let copy_tex_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutBinding {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                        },
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler,
                    },
                ],
            });

        let copy_tex_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &copy_tex_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&copy_tex_src_tex_view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&copy_tex_src_sampler),
                },
            ],
        });

        let copy_tex_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&copy_tex_bind_group_layout],
            });

        let copy_tex_vs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("gfx/shaders/copy_tex.vert"),
                glsl_to_spirv::ShaderType::Vertex,
            )
            .unwrap(),
        )
        .unwrap();

        let copy_tex_fs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("gfx/shaders/copy_tex.frag"),
                glsl_to_spirv::ShaderType::Fragment,
            )
            .unwrap(),
        )
        .unwrap();

        let copy_tex_vs_module = { device.create_shader_module(&copy_tex_vs_bytes) };
        let copy_tex_fs_module = device.create_shader_module(&copy_tex_fs_bytes);

        let copy_tex_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &copy_tex_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &copy_tex_vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &copy_tex_fs_module,
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
                format: swap_chain_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
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
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        window.set_visible(true);

        GfxEmuState {
            window,
            keys_down,

            surface,
            adapter,
            device,
            queue,
            swap_chain_desc,
            swap_chain,

            vertex_buf,
            index_buf,

            colored_rect_dst_buffer,
            colored_rect_dst_tex_format,
            colored_rect_dst_tex_extent,
            colored_rect_dst_tex,
            colored_rect_dst_tex_view,
            colored_rect_bind_group_layout,
            colored_rect_pipeline_layout,
            colored_rect_vs_module,
            colored_rect_fs_module,
            colored_rect_pipeline,

            copy_tex_src_buffer,
            copy_tex_src_tex_extent,
            copy_tex_src_tex,
            copy_tex_src_tex_view,
            copy_tex_src_sampler,
            copy_tex_bind_group_layout,
            copy_tex_bind_group,
            copy_tex_pipeline_layout,
            copy_tex_vs_module,
            copy_tex_fs_module,
            copy_tex_pipeline,
        }
    }

    pub(crate) fn poll_events(&mut self, fb: &mut [Color]) {
        EVENT_LOOP.with(|event_loop| {
            event_loop
                .lock()
                .unwrap()
                .run_return(move |event, _, control_flow| {
                    *control_flow = ControlFlow::Exit;
                    match event {
                        event::Event::WindowEvent {
                            event: WindowEvent::Resized(size),
                            ..
                        } => {
                            self.swap_chain_desc.width = size.width;
                            self.swap_chain_desc.height = size.height;
                            self.swap_chain = self
                                .device
                                .create_swap_chain(&self.surface, &self.swap_chain_desc);
                        }
                        event::Event::WindowEvent { event, .. } => match event {
                            WindowEvent::KeyboardInput {
                                input:
                                    event::KeyboardInput {
                                        virtual_keycode: Some(event::VirtualKeyCode::Escape),
                                        state: event::ElementState::Pressed,
                                        ..
                                    },
                                ..
                            }
                            | WindowEvent::CloseRequested => {
                                exit(0);
                            }
                            WindowEvent::KeyboardInput {
                                input:
                                    event::KeyboardInput {
                                        virtual_keycode: Some(keycode),
                                        state: event::ElementState::Pressed,
                                        ..
                                    },
                                ..
                            } => {
                                self.keys_down.insert(keycode);
                            }
                            WindowEvent::KeyboardInput {
                                input:
                                    event::KeyboardInput {
                                        virtual_keycode: Some(keycode),
                                        state: event::ElementState::Released,
                                        ..
                                    },
                                ..
                            } => {
                                self.keys_down.remove(&keycode);
                            }
                            _ => {}
                        },
                        event::Event::RedrawRequested(_) => {
                            self.render_cpu_buffer(fb);
                        }
                        _ => {}
                    }
                });
        });
    }

    pub(crate) fn render_cpu_buffer(&mut self, fb: &mut [Color]) {

        for (pixel, data) in fb.iter().zip(self.copy_tex_src_buffer.chunks_mut(4)) {
            let rgba = pixel.to_rgba();

            data[0] = (rgba[0] * 255.0) as u8;
            data[1] = (rgba[1] * 255.0) as u8;
            data[2] = (rgba[2] * 255.0) as u8;
            data[3] = (rgba[3] * 255.0) as u8;
        }

        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("Timeout when acquiring next swap chain texture");

        let temp_buf = self
            .device
            .create_buffer_with_data(&self.copy_tex_src_buffer, wgpu::BufferUsage::COPY_SRC);

        let render_command_buf = {
            let mut encoder = self.device.create_command_encoder(&Default::default());

            encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &temp_buf,
                    offset: 0,
                    row_pitch: 4 * WIDTH as u32,
                    image_height: HEIGHT as u32,
                },
                wgpu::TextureCopyView {
                    texture: &self.copy_tex_src_tex,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                },
                self.copy_tex_src_tex_extent,
            );

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
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
                render_pass.set_pipeline(&self.copy_tex_pipeline);
                render_pass.set_bind_group(0, &self.copy_tex_bind_group, &[]);
                render_pass.set_index_buffer(&self.index_buf, 0);
                render_pass.set_vertex_buffers(0, &[(&self.vertex_buf, 0)]);
                render_pass.draw_indexed(0..(QUAD_INDEX_DATA.len() as u32), 0, 0..1);
            }

            encoder.finish()
        };
        self.queue.submit(&[render_command_buf]);
    }
}

pub(crate) fn init() {
    let _ = GFX_EMU_STATE.lock().unwrap();
}

pub fn swap_buffers() {
    let mut state = GFX_EMU_STATE.lock().unwrap();

    with_framebuffer(|fb| {
        state.poll_events(fb);
        state.render_cpu_buffer(fb);
    });

    FRAMEBUFFER_STATE.lock().unwrap().swap_buffer();
}

pub fn with_framebuffer<F: FnOnce(&mut [Color])>(f: F) {
    f(FRAMEBUFFER_STATE.lock().unwrap().next_buffer());
}

#[inline]
pub fn slow_cpu_clear() {
    with_framebuffer(|fb| {
        fb.iter_mut()
            .for_each(|v| *v = Color::new(0b00001_00001_00001_1));
    });
}
