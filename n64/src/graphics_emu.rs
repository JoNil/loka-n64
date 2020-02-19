use crate::gfx::Texture;
use lazy_static::lazy_static;
use n64_math::Color;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use wgpu::{self, Surface};
use winit::{
    event,
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub const WIDTH: i32 = 320;
pub const HEIGHT: i32 = 240;

const FRAME_BUFFER_SIZE: usize = (WIDTH * HEIGHT) as usize;
const SCALE: i32 = 4;

lazy_static! {
    static ref GFX_EMU_STATE: Mutex<GfxEmuState> = Mutex::new(GfxEmuState::new());
}

struct WindowThreadShared {
    run: AtomicBool,
}

impl WindowThreadShared {
    fn new() -> WindowThreadShared {
        WindowThreadShared {
            run: AtomicBool::new(true),
        }
    }
}

fn window_thread(shared: Arc<WindowThreadShared>) {
    #[cfg(not(windows))]
    let event_loop = EventLoop::new();

    #[cfg(windows)]
    use winit::platform::windows::EventLoopExtWindows;

    #[cfg(windows)]
    let event_loop = winit::platform::windows::EventLoopExtWindows::new_any_thread();

    let window = {
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_title("N64");
        builder.build(&event_loop).unwrap()
    };

    let surface = wgpu::Surface::create(&window);

    while shared.run.load(Ordering::SeqCst) {
        thread::yield_now();
    }
}

struct GfxEmuState {
    window_shared: Arc<WindowThreadShared>,
    window_thread: JoinHandle<()>,
    using_framebuffer_a: bool,
    framebuffer_a: Box<[Color]>,
    framebuffer_b: Box<[Color]>,
}

impl GfxEmuState {
    fn new() -> GfxEmuState {
        let window_shared = Arc::new(WindowThreadShared::new());

        GfxEmuState {
            window_shared: window_shared.clone(),
            window_thread: thread::spawn(|| window_thread(window_shared)),
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

    fn init(&mut self) {}

    pub fn next_buffer(&mut self) -> &mut [Color] {
        if self.using_framebuffer_a {
            &mut self.framebuffer_a[..]
        } else {
            &mut self.framebuffer_b[..]
        }
    }
}

impl Drop for GfxEmuState {
    fn drop(&mut self) {
        self.window_shared.run.store(false, Ordering::SeqCst);
    }
}

pub(crate) fn get_keys() -> Vec<VirtualKeyCode> {
    Vec::new()
}

pub(crate) fn init() {
    GFX_EMU_STATE.lock().unwrap().init();
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
