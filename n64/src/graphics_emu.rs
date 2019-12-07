use minifb::{Window, Key};
use rayon;
use std::cell::RefCell;
use std::thread_local;
use std::thread;
use std::time::{Instant, Duration};

pub const WIDTH: i32 = 320;
pub const HEIGHT: i32 = 240;

const SCALE: i32 = 4;

struct WindowData {
    framebuffer_is_a: bool,
    framebuffer_a: Vec<u16>,
    framebuffer_b: Vec<u16>,
    window: Window,
    frame_start: Instant,
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
    res.resize_with((SCALE * WIDTH * SCALE * HEIGHT) as usize, Default::default);

    rayon::scope(|s| {

        let mut rest = &mut res[..];

        for y in 0..HEIGHT {

            let (lines, new_rest) = rest.split_at_mut((SCALE * SCALE * WIDTH) as usize);

            rest = new_rest;

            s.spawn(move |_| {
                for x in 0..WIDTH {
                    let color = convert_5551_to_8888(framebuffer[(x + y * WIDTH) as usize]);

                    for i in 0..SCALE {
                        for j in 0..SCALE {
                            lines[(SCALE * x + j + i*SCALE*WIDTH) as usize] = color;
                        }
                    }
                }
            });
        }
    });

    res
}

pub(crate) fn get_keys() -> Vec<Key> {
    
    WINDOW_DATA.with(|wd| {
        if let Some(ref window_data) = &*wd.borrow() {
            window_data.window.get_keys().unwrap_or(Vec::new())
        } else {
            Vec::new()
        }
    })
}

pub(crate) fn init() {
    WINDOW_DATA.with(|wd| {
        let mut window_data = wd.borrow_mut();

        *window_data = Some(WindowData {
            framebuffer_is_a: true,
            framebuffer_a: vec![0; (WIDTH * HEIGHT) as usize],
            framebuffer_b: vec![0; (WIDTH * HEIGHT) as usize],
            window: Window::new(
                "Nintendo 64",
                (SCALE * WIDTH) as usize,
                (SCALE * HEIGHT) as usize,
                Default::default(),
            ).unwrap(),
            frame_start: Instant::now(),
        });
    });
}

pub fn swap_buffers() {
    WINDOW_DATA.with(|window_data| {
        if let Some(ref mut wd) = &mut *window_data.borrow_mut() {
            if wd.framebuffer_is_a {
                wd
                    .window
                    .update_with_buffer_size(
                        &framebuffer_to_rgba(&wd.framebuffer_a),
                        WIDTH as usize,
                        HEIGHT as usize,
                    )
                    .unwrap();
            } else {
                wd
                    .window
                    .update_with_buffer_size(
                        &framebuffer_to_rgba(&wd.framebuffer_b),
                        WIDTH as usize,
                        HEIGHT as usize,
                    )
                    .unwrap();
            }

            wd.framebuffer_is_a = !wd.framebuffer_is_a;

            if !wd.window.is_open() {
                std::process::exit(0);
            }

            {
                while (wd.frame_start.elapsed() + Duration::from_micros(2500)).as_secs_f64() < 1.0 / 60.0 {
                    thread::sleep(Duration::from_millis(1));
                }
    
                while wd.frame_start.elapsed().as_secs_f64() < 1.0 / 60.0 {
                    thread::yield_now();
                }

                wd.frame_start = Instant::now();
            }
        }
    });
}

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

pub fn clear_buffer() {
    with_framebuffer(|fb| {
        fb.iter_mut().for_each(|v| *v = 0b00001_00001_00001_1);
    });
}
