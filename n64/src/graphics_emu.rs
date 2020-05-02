use colored_rect::ColoredRect;
use copy_tex::CopyTex;
use lazy_static::lazy_static;
use n64_math::Color;
use std::collections::HashSet;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread_local;
use textured_rect::TexturedRect;
use winit::{
    event::{self, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::desktop::EventLoopExtDesktop,
    window::Window,
};
use zerocopy::{AsBytes, FromBytes};

pub(crate) mod colored_rect;
pub(crate) mod copy_tex;
pub(crate) mod textured_rect;

pub const WIDTH: i32 = 320;
pub const HEIGHT: i32 = 240;

const FRAME_BUFFER_SIZE: usize = (WIDTH * HEIGHT) as usize;
const SCALE: i32 = 4;

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes)]
pub(crate) struct Vertex {
    pos: [f32; 3],
    tex_coord: [f32; 2],
}

static QUAD_VERTEX_DATA: &'static [Vertex] = &[
    Vertex {
        pos: [-1.0, -1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0],
        tex_coord: [0.0, 0.0],
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

pub(crate) struct CommandBufferDst {
    pub buffer: wgpu::Buffer,
    pub tex_format: wgpu::TextureFormat,
    pub tex_extent: wgpu::Extent3d,
    pub tex: wgpu::Texture,
    pub tex_view: wgpu::TextureView,
}

impl CommandBufferDst {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (4 * WIDTH * HEIGHT) as u64,
            usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
        });

        let tex_format = wgpu::TextureFormat::Rgba8Unorm;

        let tex_extent = wgpu::Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
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
            usage: wgpu::TextureUsage::COPY_DST
                | wgpu::TextureUsage::COPY_SRC
                | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        });
        let tex_view = tex.create_default_view();

        Self {
            buffer,
            tex_format,
            tex_extent,
            tex,
            tex_view,
        }
    }
}

pub(crate) struct GfxEmuState {
    pub window: Window,
    pub keys_down: HashSet<VirtualKeyCode>,

    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub swap_chain_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,

    pub quad_vertex_buf: wgpu::Buffer,
    pub quad_index_buf: wgpu::Buffer,

    pub copy_tex: CopyTex,

    pub command_buffer_dst: CommandBufferDst,
    pub colored_rect: ColoredRect,
    pub textured_rect: TexturedRect,

    pub device_poll_thread_run: Arc<AtomicBool>,
    pub device_poll_thread: Option<thread::JoinHandle<()>>,
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

        let (adapter, device, queue) = futures_executor::block_on(async {
            let adapter = wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::BackendBit::PRIMARY,
            )
            .await
            .unwrap();

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: wgpu::Limits::default(),
                })
                .await;

            (adapter, Arc::new(device), queue)
        });

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let quad_vertex_buf =
            device.create_buffer_with_data(QUAD_VERTEX_DATA.as_bytes(), wgpu::BufferUsage::VERTEX);

        let quad_index_buf =
            device.create_buffer_with_data(QUAD_INDEX_DATA.as_bytes(), wgpu::BufferUsage::INDEX);

        let copy_tex = CopyTex::new(&device, &swap_chain_desc);
        let command_buffer_dst = CommandBufferDst::new(&device);
        let colored_rect = ColoredRect::new(&device, command_buffer_dst.tex_format);
        let textured_rect = TexturedRect::new(&device, command_buffer_dst.tex_format);

        window.set_visible(true);

        let device_poll_thread_run = Arc::new(AtomicBool::new(true));
        let device_poll_thread = {
            let run = device_poll_thread_run.clone();
            let device = device.clone();

            Some(thread::spawn(move || {
                while run.load(Ordering::SeqCst) {
                    device.poll(wgpu::Maintain::Poll);
                }
            }))
        };

        GfxEmuState {
            window,
            keys_down,

            surface,
            adapter,
            device,
            queue,
            swap_chain_desc,
            swap_chain,

            quad_vertex_buf,
            quad_index_buf,

            copy_tex,
            command_buffer_dst,
            colored_rect,
            textured_rect,

            device_poll_thread_run,
            device_poll_thread,
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
        for (pixel, data) in fb.iter().zip(self.copy_tex.src_buffer.chunks_mut(4)) {
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
            .create_buffer_with_data(&self.copy_tex.src_buffer, wgpu::BufferUsage::COPY_SRC);

        let render_command_buf = {
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &temp_buf,
                    offset: 0,
                    bytes_per_row: 4 * WIDTH as u32,
                    rows_per_image: HEIGHT as u32,
                },
                wgpu::TextureCopyView {
                    texture: &self.copy_tex.src_tex,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                },
                self.copy_tex.src_tex_extent,
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
                render_pass.set_pipeline(&self.copy_tex.pipeline);
                render_pass.set_bind_group(0, &self.copy_tex.bind_group, &[]);
                render_pass.set_index_buffer(&self.quad_index_buf, 0, 0);
                render_pass.set_vertex_buffer(0, &self.quad_vertex_buf, 0, 0);
                render_pass.draw_indexed(0..(QUAD_INDEX_DATA.len() as u32), 0, 0..1);
            }

            encoder.finish()
        };
        self.queue.submit(&[render_command_buf]);
    }
}

impl Drop for GfxEmuState {
    fn drop(&mut self) {
        self.device_poll_thread_run.store(false, Ordering::SeqCst);

        if let Some(join_handle) = self.device_poll_thread.take() {
            join_handle.join().unwrap();
        }
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
            .for_each(|v| *v = Color::new(0b00000_00000_00000_1));
    });
}
