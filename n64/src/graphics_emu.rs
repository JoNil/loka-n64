use crate::gfx::Texture;
use lazy_static::lazy_static;
use n64_math::Color;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use winit::{event::VirtualKeyCode, event_loop::EventLoop};
use zerocopy::{AsBytes, FromBytes};

pub const WIDTH: i32 = 320;
pub const HEIGHT: i32 = 240;

const FRAME_BUFFER_SIZE: usize = (WIDTH * HEIGHT) as usize;
const SCALE: i32 = 4;

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes)]
struct Vertex {
    pos: [f32; 3],
    tex_coord: [f32; 2],
}

static VERTEX_DATA: &'static [Vertex] = &[
    Vertex {
        pos: [-1.0, -1.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
];

static INDEX_DATA: &'static [u16] = &[0, 1, 2, 2, 3, 0];

lazy_static! {
    static ref GFX_EMU_STATE: Mutex<GfxEmuState> = Mutex::new(GfxEmuState::new());
}

fn gpu_thread(shared: &Mutex<GfxEmuState>) {
    let event_loop = EventLoop::new();

    let window = {
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_title("N64");
        builder.build(&event_loop).unwrap()
    };

    let surface = wgpu::Surface::create(&window);

    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
    },
    wgpu::BackendBit::PRIMARY)
    .unwrap();

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: (SCALE * WIDTH) as u32,
        height: (SCALE * HEIGHT) as u32,
        present_mode: wgpu::PresentMode::Vsync,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let mut init_encoder = device.create_command_encoder(&Default::default());

    let vertex_buf = device.create_buffer_with_data(VERTEX_DATA.as_bytes(), wgpu::BufferUsage::VERTEX);

    let index_buf = device.create_buffer_with_data(INDEX_DATA.as_bytes(), wgpu::BufferUsage::INDEX);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[],
    });

    loop {
        thread::yield_now();
    }
}

struct GfxEmuState {
    using_framebuffer_a: bool,
    framebuffer_a: Box<[Color]>,
    framebuffer_b: Box<[Color]>,
}

impl GfxEmuState {
    fn new() -> GfxEmuState {
        GfxEmuState {
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

    pub fn next_buffer(&mut self) -> &mut [Color] {
        if self.using_framebuffer_a {
            &mut self.framebuffer_a[..]
        } else {
            &mut self.framebuffer_b[..]
        }
    }
}

pub(crate) fn get_keys() -> Vec<VirtualKeyCode> {
    Vec::new()
}

pub(crate) fn init(f: impl FnOnce() + Send + 'static) {
    let state = GFX_EMU_STATE.lock().unwrap();

    thread::spawn(f);

    gpu_thread(&*GFX_EMU_STATE);
}

pub fn swap_buffers() {
    let mut state = GFX_EMU_STATE.lock().unwrap();
    state.using_framebuffer_a = !state.using_framebuffer_a;
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
