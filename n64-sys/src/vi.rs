extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem::size_of;
use core::ptr::{read_volatile, write_volatile};

const VI_BASE: usize = 0xA440_0000;

const VI_STATUS: *mut u32 = VI_BASE as *mut u32;
const VI_DRAM_ADDR: *mut usize = (VI_BASE + 0x04) as *mut usize;
const VI_H_WIDTH: *mut u32 = (VI_BASE + 0x08) as *mut u32;
const VI_V_INTR: *mut u32 = (VI_BASE + 0x0C) as *mut u32;
const VI_CURRENT: *const u32 = (VI_BASE + 0x10) as *const u32;
const VI_TIMING: *mut u32 = (VI_BASE + 0x14) as *mut u32;
const VI_V_SYNC: *mut u32 = (VI_BASE + 0x18) as *mut u32;
const VI_H_SYNC: *mut u32 = (VI_BASE + 0x1C) as *mut u32;
const VI_H_SYNC_LEAP: *mut u32 = (VI_BASE + 0x20) as *mut u32;
const VI_H_VIDEO: *mut u32 = (VI_BASE + 0x24) as *mut u32;
const VI_V_VIDEO: *mut u32 = (VI_BASE + 0x28) as *mut u32;
const VI_V_BURST: *mut u32 = (VI_BASE + 0x2C) as *mut u32;
const VI_X_SCALE: *mut u32 = (VI_BASE + 0x30) as *mut u32;
const VI_Y_SCALE: *mut u32 = (VI_BASE + 0x34) as *mut u32;

const FRAME_BUFFER_SIZE: usize = WIDTH as usize * HEIGHT as usize * 2;
static mut FRAME_BUFFER: Option<Box<[u16]>> = None;

pub const WIDTH: i32 = 320;
pub const HEIGHT: i32 = 240;

#[inline]
pub fn init() {
    unsafe {
        let mut buffer = Vec::new();
        buffer.resize_with(FRAME_BUFFER_SIZE, || 0x0001);
        FRAME_BUFFER = Some(buffer.into_boxed_slice())
    };

    unsafe {
        write_volatile(VI_STATUS, 0x0000_320E);
        write_volatile(VI_DRAM_ADDR, FRAME_BUFFER.as_mut().unwrap().as_mut_ptr() as usize);
        write_volatile(VI_H_WIDTH, WIDTH as u32);
        write_volatile(VI_V_INTR, 2);
        write_volatile(VI_TIMING, 0x03E5_2239);
        write_volatile(VI_V_SYNC, 0x0000_020D);
        write_volatile(VI_H_SYNC, 0x0000_0C15);
        write_volatile(VI_H_SYNC_LEAP, 0x0C15_0C15);
        write_volatile(VI_H_VIDEO, 0x006C_02EC);
        write_volatile(VI_V_VIDEO, 0x0025_01FF);
        write_volatile(VI_V_BURST, 0x000E_0204);
        write_volatile(VI_X_SCALE, 0x0000_0200);
        write_volatile(VI_Y_SCALE, 0x0000_0400);
    }
}

#[inline]
pub fn wait_for_vblank() {
    loop {
        let current_halfline = unsafe { read_volatile(VI_CURRENT) };
        if current_halfline <= 10 {
            break;
        }
    }
}

#[inline]
pub unsafe fn next_buffer() -> *mut u16 {
    let current_fb = read_volatile(VI_DRAM_ADDR);
    let frame_buffer = FRAME_BUFFER.as_mut().unwrap().as_mut_ptr();

    if current_fb != frame_buffer as usize {
        frame_buffer
    } else {
        (frame_buffer as usize + size_of::<i16>()*FRAME_BUFFER_SIZE) as *mut u16
    }
}

#[inline]
pub fn swap_buffers() {
    unsafe {
        write_volatile(VI_DRAM_ADDR, next_buffer() as usize);
    }
}
