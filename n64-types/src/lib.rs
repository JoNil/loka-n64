#![no_std]

pub use profiler::ScopeData;
pub use rdp_command::RdpCommand;
pub use video_mode::VideoMode;

mod profiler;
mod rdp_command;
mod video_mode;

pub const MESSAGE_MAGIC_PROFILER: u8 = 0x1c;
pub const MESSAGE_MAGIC_PRINT: u8 = 0x1d;
