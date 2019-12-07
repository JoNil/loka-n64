#![cfg_attr(target_vendor = "nintendo64", no_std)]

pub mod ipl3font;

#[cfg_attr(target_vendor = "nintendo64", path = "graphics.rs")]
#[cfg_attr(not(target_vendor = "nintendo64"), path = "graphics_emu.rs")]
pub mod graphics;

#[cfg_attr(target_vendor = "nintendo64", path = "controllers.rs")]
#[cfg_attr(not(target_vendor = "nintendo64"), path = "controllers_emu.rs")]
pub mod controllers;

pub fn init() {
    graphics::init();
}
