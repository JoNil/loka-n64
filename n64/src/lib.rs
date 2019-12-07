#![cfg_attr(target_vendor = "nintendo64", no_std)]

pub mod controllers;
pub mod ipl3font;

#[cfg_attr(target_vendor = "nintendo64", path = "graphics.rs")]
#[cfg_attr(not(target_vendor = "nintendo64"), path = "graphics_emu.rs")]
pub mod graphics;

pub fn init() {
    graphics::init();
}
