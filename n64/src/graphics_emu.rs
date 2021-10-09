use crate::{current_time_us, framebuffer::Framebuffer, VideoMode};
use colored_rect::ColoredRect;
use copy_tex::CopyTex;
use mesh::Mesh;
use std::collections::HashSet;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread_local;
use textured_rect::TexturedRect;
use wgpu::util::DeviceExt;
use winit::{
    event::{self, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
};
use zerocopy::{AsBytes, FromBytes};

pub(crate) mod colored_rect;
pub(crate) mod copy_tex;
pub(crate) mod dst_texture;
pub(crate) mod mesh;
pub(crate) mod textured_rect;

const SCALE: i32 = 2;

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes)]
pub(crate) struct Vertex {
    pos: [f32; 3],
    tex_coord: [f32; 2],
}

static QUAD_VERTEX_DATA: &[Vertex] = &[
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

pub(crate) static QUAD_INDEX_DATA: &[u16] = &[0, 1, 2, 2, 3, 0];

thread_local! {
    static EVENT_LOOP: Mutex<EventLoop<()>> = Mutex::new(EventLoop::new());
}

pub struct Graphics {
    pub(crate) video_mode: VideoMode,
    pub(crate) keys_down: HashSet<VirtualKeyCode>,

    _window: Window,
    _instance: wgpu::Instance,
    _adapter: wgpu::Adapter,

    pub(crate) surface: wgpu::Surface,
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: wgpu::Queue,
    pub(crate) swap_chain_desc: wgpu::SwapChainDescriptor,
    pub(crate) swap_chain: wgpu::SwapChain,

    pub(crate) quad_vertex_buf: wgpu::Buffer,
    pub(crate) quad_index_buf: wgpu::Buffer,

    pub(crate) copy_tex: CopyTex,
    pub(crate) colored_rect: ColoredRect,
    pub(crate) textured_rect: TexturedRect,
    pub(crate) mesh: Mesh,

    pub(crate) device_poll_thread_run: Arc<AtomicBool>,
    pub(crate) device_poll_thread: Option<thread::JoinHandle<()>>,
}

impl Graphics {
    pub(crate) fn new(video_mode: VideoMode, _framebuffer: &mut Framebuffer) -> Self {
        let window = {
            let mut builder = winit::window::WindowBuilder::new();
            builder = builder.with_title("N64");
            builder = builder.with_inner_size(winit::dpi::LogicalSize::new(
                SCALE * video_mode.width(),
                SCALE * video_mode.height(),
            ));
            builder = builder.with_visible(false);
            EVENT_LOOP.with(|event_loop| builder.build(&event_loop.lock().unwrap()).unwrap())
        };

        let keys_down = HashSet::new();

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(&window);
            (size, surface)
        };

        let (adapter, device, queue) = futures_executor::block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                    },
                    None,
                )
                .await
                .unwrap();

            (adapter, Arc::new(device), queue)
        });

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsages::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let quad_vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: QUAD_VERTEX_DATA.as_bytes(),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let quad_index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: QUAD_INDEX_DATA.as_bytes(),
            usage: wgpu::BufferUsages::INDEX,
        });

        let copy_tex = CopyTex::new(&device, &swap_chain_desc, video_mode);
        let colored_rect = ColoredRect::new(&device, dst_texture::TEXUTRE_FORMAT);
        let textured_rect = TexturedRect::new(&device, dst_texture::TEXUTRE_FORMAT);
        let mesh = Mesh::new(&device, &queue, dst_texture::TEXUTRE_FORMAT);

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

        Self {
            video_mode,
            keys_down,

            _window: window,
            _instance: instance,
            _adapter: adapter,

            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,

            quad_vertex_buf,
            quad_index_buf,

            copy_tex,
            colored_rect,
            textured_rect,
            mesh,

            device_poll_thread_run,
            device_poll_thread,
        }
    }

    pub(crate) fn poll_events(&mut self, framebuffer: &mut Framebuffer) {
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
                            self.render_cpu_buffer(framebuffer);
                        }
                        _ => {}
                    }
                });
        });
    }

    pub(crate) fn render_cpu_buffer(&mut self, framebuffer: &mut Framebuffer) -> i64 {
        let fb = framebuffer.next_buffer();

        for (pixel, data) in fb.data.iter().zip(self.copy_tex.src_buffer.chunks_mut(4)) {
            let rgba = pixel.to_rgba();

            data[0] = (rgba[0] * 255.0) as u8;
            data[1] = (rgba[1] * 255.0) as u8;
            data[2] = (rgba[2] * 255.0) as u8;
            data[3] = (rgba[3] * 255.0) as u8;
        }

        let frame = self
            .swap_chain
            .get_current_frame()
            .expect("Timeout when acquiring next swap chain texture");

        let temp_buf = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &self.copy_tex.src_buffer,
                usage: wgpu::BufferUsages::COPY_SRC,
            });

        let render_command_buf = {
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &temp_buf,
                    layout: wgpu::TextureDataLayout {
                        offset: 0,
                        bytes_per_row: 4 * self.video_mode.width() as u32,
                        rows_per_image: self.video_mode.height() as u32,
                    },
                },
                wgpu::TextureCopyView {
                    texture: &self.copy_tex.src_tex,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                },
                self.copy_tex.src_tex_extent,
            );

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &frame.output.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&self.copy_tex.pipeline);
                render_pass.set_bind_group(0, &self.copy_tex.bind_group, &[]);
                render_pass
                    .set_index_buffer(self.quad_index_buf.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.set_vertex_buffer(0, self.quad_vertex_buf.slice(..));
                render_pass.draw_indexed(0..(QUAD_INDEX_DATA.len() as u32), 0, 0..1);
            }

            encoder.finish()
        };

        let frame_end_time = current_time_us();

        self.queue.submit(Some(render_command_buf));

        frame_end_time
    }

    pub fn swap_buffers(&mut self, framebuffer: &mut Framebuffer) -> i64 {
        self.poll_events(framebuffer);
        let frame_end_time = self.render_cpu_buffer(framebuffer);
        framebuffer.swap_buffer();
        frame_end_time
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        self.device_poll_thread_run.store(false, Ordering::SeqCst);

        if let Some(join_handle) = self.device_poll_thread.take() {
            join_handle.join().unwrap();
        }
    }
}
