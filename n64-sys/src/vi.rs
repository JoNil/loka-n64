#![allow(dead_code)]

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem::size_of;
use core::ptr::{read_volatile, write_volatile};

const VI_BASE: usize = 0xA440_0000;

const VI_STATUS: *mut usize = VI_BASE as _;
const VI_DRAM_ADDR: *mut usize = (VI_BASE + 0x04) as _;
const VI_H_WIDTH: *mut usize = (VI_BASE + 0x08) as _;
const VI_V_INTR: *mut usize = (VI_BASE + 0x0C) as _;
const VI_CURRENT: *const usize = (VI_BASE + 0x10) as _;
const VI_TIMING: *mut usize = (VI_BASE + 0x14) as _;
const VI_V_SYNC: *mut usize = (VI_BASE + 0x18) as _;
const VI_H_SYNC: *mut usize = (VI_BASE + 0x1C) as _;
const VI_H_SYNC_LEAP: *mut usize = (VI_BASE + 0x20) as _;
const VI_H_VIDEO: *mut usize = (VI_BASE + 0x24) as _;
const VI_V_VIDEO: *mut usize = (VI_BASE + 0x28) as _;
const VI_V_BURST: *mut usize = (VI_BASE + 0x2C) as _;
const VI_X_SCALE: *mut usize = (VI_BASE + 0x30) as _;
const VI_Y_SCALE: *mut usize = (VI_BASE + 0x34) as _;

const FRAME_BUFFER_SIZE: usize = (WIDTH * HEIGHT) as usize;
static mut USING_FRAMEBUFFER_A: bool = false;
static mut FRAME_BUFFER_A: Option<Box<[u16]>> = None;
static mut FRAME_BUFFER_B: Option<Box<[u16]>> = None;

pub const WIDTH: i32 = 320;
pub const HEIGHT: i32 = 240;

#[inline]
pub fn init() {
    unsafe {
        let mut buffer = Vec::new();
        buffer.resize_with(FRAME_BUFFER_SIZE, || 0x0001);
        FRAME_BUFFER_A = Some(buffer.into_boxed_slice())
    };

    unsafe {
        let mut buffer = Vec::new();
        buffer.resize_with(FRAME_BUFFER_SIZE, || 0x0001);
        FRAME_BUFFER_B = Some(buffer.into_boxed_slice())
    };

    unsafe {
        write_volatile(VI_STATUS, 0x0000_320E);
        write_volatile(
            VI_DRAM_ADDR,
            FRAME_BUFFER_B.as_mut().unwrap().as_mut_ptr() as usize,
        );
        write_volatile(VI_H_WIDTH, WIDTH as usize);
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
        if current_halfline <= 1 {
            break;
        }
    }
}

#[inline]
pub unsafe fn next_buffer() -> *mut u16 {
    if USING_FRAMEBUFFER_A {
        FRAME_BUFFER_A.as_mut().unwrap().as_mut_ptr()
    } else {
        FRAME_BUFFER_B.as_mut().unwrap().as_mut_ptr()
    }
}

#[inline]
pub fn swap_buffers() {
    unsafe {
        write_volatile(VI_DRAM_ADDR, next_buffer() as usize);
        USING_FRAMEBUFFER_A = !USING_FRAMEBUFFER_A;
    }
}
