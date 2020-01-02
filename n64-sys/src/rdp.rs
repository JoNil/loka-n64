#![allow(dead_code)]

const RDP_BASE: usize = 0xA410_0000;

const RDP_COMMAND_BUFFER_START: *mut u32 = (RDP_BASE + 0x00) as _;
const RDP_COMMAND_BUFFER_END: *mut u32 = (RDP_BASE + 0x04) as _;
const RDP_COMMAND_BUFFER_CURRENT: *const u32 = (RDP_BASE + 0x08) as _;
const RDP_STATUS: *mut u32 = (RDP_BASE + 0x0C) as _;
const RDP_CLOCK_COUNTER: *const u32 = (RDP_BASE + 0x10) as _;
const RDP_COMMAND_BUFFER_BUSY: *const u32 = (RDP_BASE + 0x14) as _;
const RDP_PIPE_BUSY: *const u32 = (RDP_BASE + 0x18) as _;
const RDP_TMEM_BUSY: *const u32 = (RDP_BASE + 0x1C) as _;