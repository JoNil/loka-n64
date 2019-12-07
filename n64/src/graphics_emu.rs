use minifb::Window;
use std::cell::RefCell;
use std::thread_local;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;

struct WindowData {
    framebuffer_is_a: bool,
    framebuffer_a: Vec<u16>,
    framebuffer_b: Vec<u16>,
    window: Window,
}

thread_local! {
    static WINDOW_DATA: RefCell<Option<WindowData>> = RefCell::new(None);
}

fn convert_5551_to_8888(input: &u16) -> u32 {
    let r = ((*input >> 11 & 0b11111) + 4) as u8 * 8;
    let g = ((*input >> 6 & 0b11111) + 4) as u8 * 8;
    let b = ((*input >> 1 & 0b11111) + 4) as u8 * 8;

    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

fn framebuffer_to_rgba(framebuffer: &[u16]) -> Vec<u32> {
    framebuffer.iter().map(convert_5551_to_8888).collect()
}

#[inline]
pub(crate) fn init() {
    WINDOW_DATA.with(|wd| {
        let mut window_data = wd.borrow_mut();

        *window_data = Some(WindowData {
            framebuffer_is_a: true,
            framebuffer_a: vec![0; WIDTH * HEIGHT],
            framebuffer_b: vec![0; WIDTH * HEIGHT],
            window: Window::new("Nintendo 64", WIDTH, HEIGHT, Default::default()).unwrap(),
        });
    });
}

#[inline]
pub fn swap_buffers() {
    WINDOW_DATA.with(|wd| {
        if let Some(ref mut window_data) = &mut *wd.borrow_mut() {

            if window_data.framebuffer_is_a {
                window_data.window
                    .update_with_buffer_size(&framebuffer_to_rgba(&window_data.framebuffer_a), WIDTH, HEIGHT)
                    .unwrap();
            } else {
                window_data.window
                    .update_with_buffer_size(&framebuffer_to_rgba(&window_data.framebuffer_b), WIDTH, HEIGHT)
                    .unwrap();
            }

            window_data.framebuffer_is_a = !window_data.framebuffer_is_a;
        }
    });
}

#[inline]
pub fn with_framebuffer<F: FnOnce(&mut [u16])>(f: F) {
    
    WINDOW_DATA.with(|wd| {
        if let Some(ref mut window_data) = &mut *wd.borrow_mut() {

            if window_data.framebuffer_is_a {
                f(&mut window_data.framebuffer_a);
            } else {
                f(&mut window_data.framebuffer_b);
            }
        }
    });
}

#[inline]
pub fn clear_buffer() {
    with_framebuffer(|fb| {
        fb.iter_mut().for_each(|v| *v = 0b00001_00001_00001_1);
    });
}