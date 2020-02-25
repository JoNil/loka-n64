#![no_std]

#[repr(C, align(8))]
pub struct RdpCommand(pub u64);
