#![no_std]

pub use profiler::{ProfilerMessageBuffer, ScopeData};
pub use rdp_command::{RdpBlock, RdpCommand};
pub use video_mode::VideoMode;

mod profiler;
mod rdp_command;
mod video_mode;

pub const MESSAGE_MAGIC_PROFILER: u8 = 0x1c;
pub const MESSAGE_MAGIC_PRINT: u8 = 0x1d;

#[macro_export]
macro_rules! static_assert {
    ($cond:expr) => {
        $crate::static_assert!($cond, concat!("assertion failed: ", stringify!($cond)));
    };
    ($cond:expr, $($t:tt)+) => {
        const _: () = {
            if !$cond {
                core::panic!($($t)+)
            }
        };
    };
}
