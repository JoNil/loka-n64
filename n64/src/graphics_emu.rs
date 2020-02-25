use crate::gfx::Texture;
use lazy_static::lazy_static;
use n64_math::Color;
use std::mem;
use std::process::exit;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::thread_local;
use std::time::Instant;
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
#[derive(Clone, Copy, AsBytes, FromBytes)]
struct Vertex {
    pos: [f32; 3],
}

static VERTEX_DATA: &'static [Vertex] = &[
    Vertex {
        pos: [-0.5, -0.5, 1.0],
    },
    Vertex {
        pos: [0.5, -0.5, 1.0],
    },
    Vertex {
        pos: [0.5, 0.5, 1.0],
    },
    Vertex {
        pos: [-0.5, 0.5, 1.0],
    },
];

static INDEX_DATA: &'static [u16] = &[0, 1, 2, 2, 3, 0];

thread_local! {
    static EVENT_LOOP: Mutex<EventLoop<()>> = Mutex::new(EventLoop::new());
}

lazy_static! {
    pub(crate) static ref GFX_EMU_STATE: Mutex<GfxEmuState> = Mutex::new(GfxEmuState::new());
}

pub(crate) struct GfxEmuState {
    pub window: Window,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,

    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub uniform_buf: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,

    pub using_framebuffer_a: bool,
    pub framebuffer_a: Box<[Color]>,
    pub framebuffer_b: Box<[Color]>,
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

        let mut swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Vsync,
        };
        let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let vertex_buf =
            device.create_buffer_with_data(VERTEX_DATA.as_bytes(), wgpu::BufferUsage::VERTEX);

        let index_buf =
            device.create_buffer_with_data(INDEX_DATA.as_bytes(), wgpu::BufferUsage::INDEX);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let color: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let uniform_buf = device.create_buffer_with_data(
            color.as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &uniform_buf,
                    range: 0..color.as_bytes().len() as u64,
                },
            }],
        });

        let vs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("gfx/shaders/colored_rect.vert"),
                glsl_to_spirv::ShaderType::Vertex,
            )
            .unwrap(),
        )
        .unwrap();

        let fs_bytes = wgpu::read_spirv(
            glsl_to_spirv::compile(
                include_str!("gfx/shaders/colored_rect.frag"),
                glsl_to_spirv::ShaderType::Fragment,
            )
            .unwrap(),
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
            surface,
            adapter,
            device,
            queue,
            swap_chain_desc,
            swap_chain,

            vertex_buf,
            index_buf,
            bind_group_layout,
            pipeline_layout,
            uniform_buf,
            bind_group,
            vs_module,
            fs_module,
            pipeline,

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

    pub(crate) fn poll_events(&mut self) {
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
                            _ => {}
                        },
                        event::Event::RedrawRequested(_) => {
                            self.render_cpu_buffer();
                        }
                        _ => {}
                    }
                });
        });
    }

    pub(crate) fn render_cpu_buffer(&mut self) {
        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("Timeout when acquiring next swap chain texture");

        let command_buf = {
            let mut encoder = self.device.create_command_encoder(&Default::default());
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.4,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.set_index_buffer(&self.index_buf, 0);
                render_pass.set_vertex_buffers(0, &[(&self.vertex_buf, 0)]);
                render_pass.draw_indexed(0..(INDEX_DATA.len() as u32), 0, 0..1);
            }

            encoder.finish()
        };
        self.queue.submit(&[command_buf]);
    }

    pub(crate) fn next_buffer(&mut self) -> &mut [Color] {
        if self.using_framebuffer_a {
            &mut self.framebuffer_a[..]
        } else {
            &mut self.framebuffer_b[..]
        }
    }
}

pub(crate) fn init() {
    let mut state = GFX_EMU_STATE.lock().unwrap();
}

pub fn swap_buffers() {
    let mut state = GFX_EMU_STATE.lock().unwrap();

    state.using_framebuffer_a = !state.using_framebuffer_a;

    state.poll_events();
    state.render_cpu_buffer();
}

pub fn with_framebuffer<F: FnOnce(&mut [Color])>(f: F) {
    f(GFX_EMU_STATE.lock().unwrap().next_buffer());
}

#[inline]
pub fn slow_cpu_clear() {
    with_framebuffer(|fb| {
        fb.iter_mut()
            .for_each(|v| *v = Color::new(0b00001_00001_00001_1));
    });
}
