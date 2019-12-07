use minifb::Window;
use std::cell::RefCell;
use std::thread_local;

pub const WIDTH: i32 = 320;
pub const HEIGHT: i32 = 240;

struct WindowData {
    framebuffer_is_a: bool,
    framebuffer_a: Vec<u16>,
    framebuffer_b: Vec<u16>,
    window: Window,
}

thread_local! {
    static WINDOW_DATA: RefCell<Option<WindowData>> = RefCell::new(None);
}

fn convert_5551_to_8888(input: u16) -> u32 {
    let r = (input >> 11 & 0b11111) as u8 * 8 + 4;
    let g = (input >> 6 & 0b11111) as u8 * 8 + 4;
    let b = (input >> 1 & 0b11111) as u8 * 8 + 4;

    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

fn framebuffer_to_rgba(framebuffer: &[u16]) -> Vec<u32> {
    let mut res = Vec::new();
    res.resize_with((4 * WIDTH * 4 * HEIGHT) as usize, Default::default);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = convert_5551_to_8888(framebuffer[(x + y * WIDTH) as usize]);

            for i in 0..4 {
                for j in 0..4 {
                    res[(4 * x + j + (4 * y + i) * 4 * WIDTH) as usize] = color;
                }
            }
        }
    }

    res
}

#[inline]
pub(crate) fn init() {
    WINDOW_DATA.with(|wd| {
        let mut window_data = wd.borrow_mut();

        *window_data = Some(WindowData {
            framebuffer_is_a: true,
            framebuffer_a: vec![0; (WIDTH * HEIGHT) as usize],
            framebuffer_b: vec![0; (WIDTH * HEIGHT) as usize],
            window: Window::new(
                "Nintendo 64",
                (4 * WIDTH) as usize,
                (4 * HEIGHT) as usize,
                Default::default(),
            )
            .unwrap(),
        });
    });
}

#[inline]
pub fn swap_buffers() {
    WINDOW_DATA.with(|wd| {
        if let Some(ref mut window_data) = &mut *wd.borrow_mut() {
            if window_data.framebuffer_is_a {
                window_data
                    .window
                    .update_with_buffer_size(
                        &framebuffer_to_rgba(&window_data.framebuffer_a),
                        WIDTH as usize,
                        HEIGHT as usize,
                    )
                    .unwrap();
            } else {
                window_data
                    .window
                    .update_with_buffer_size(
                        &framebuffer_to_rgba(&window_data.framebuffer_b),
                        WIDTH as usize,
                        HEIGHT as usize,
                    )
                    .unwrap();
            }

            window_data.framebuffer_is_a = !window_data.framebuffer_is_a;

            if !window_data.window.is_open() {
                std::process::exit(0);
            }
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
