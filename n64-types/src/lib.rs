#![no_std]

pub use video_mode::VideoMode;

mod video_mode;

#[repr(C, align(8))]
pub struct RdpCommand(pub u64);