#![no_std]

pub mod controllers;
pub mod ipl3font;

#[cfg_attr(target_vendor = "nintendo64", path = "graphics.rs")]
#[cfg_attr(not(target_vendor = "nintendo64"), path = "graphics_emu.rs")]
pub mod graphics;

#[cfg(target_vendor = "nintendo64")]
use n64_sys::vi;

pub fn init() {
    #[cfg(target_vendor = "nintendo64")]
    vi::init();
}