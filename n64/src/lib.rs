#![no_std]

pub mod ipl3font;
pub mod controller;
pub mod graphics;

use n64_sys::vi;

pub fn init() {
    vi::init();
}