#![no_std]

pub mod controller;
pub mod graphics;
pub mod ipl3font;

use n64_sys::vi;

pub fn init() {
    vi::init();
}
