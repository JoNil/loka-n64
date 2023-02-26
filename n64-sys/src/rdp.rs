#![allow(dead_code)]

use crate::sys::data_cache_hit_writeback;
use alloc::vec::Vec;
use core::ptr::{read_volatile, write_volatile};
use n64_types::RdpCommand;

const RDP_BASE: usize = 0xA410_0000;

const RDP_COMMAND_BUFFER_START: *mut usize = (RDP_BASE) as _;
const RDP_COMMAND_BUFFER_END: *mut usize = (RDP_BASE + 0x04) as _;
const RDP_COMMAND_BUFFER_CURRENT: *const usize = (RDP_BASE + 0x08) as _;
const RDP_STATUS: *mut usize = (RDP_BASE + 0x0C) as _;
const RDP_CLOCK_COUNTER: *const usize = (RDP_BASE + 0x10) as _;
const RDP_COMMAND_BUFFER_BUSY: *const usize = (RDP_BASE + 0x14) as _;
const RDP_PIPE_BUSY: *const usize = (RDP_BASE + 0x18) as _;
const RDP_TMEM_BUSY: *const usize = (RDP_BASE + 0x1C) as _;

// RDP Status Read Flags:
pub const RDP_STATUS_XBS: usize = 0x001; // RDP_STATUS: Use XBUS DMEM DMA Or DRAM DMA (Bit 0)
pub const RDP_STATUS_FRZ: usize = 0x002; // RDP_STATUS: RDP Frozen (Bit 1)
pub const RDP_STATUS_FLS: usize = 0x004; // RDP_STATUS: RDP Flushed (Bit 2)
pub const RDP_STATUS_GCL: usize = 0x008; // RDP_STATUS: GCLK Alive (Bit 3)
pub const RDP_STATUS_TMB: usize = 0x010; // RDP_STATUS: TMEM Busy (Bit 4)
pub const RDP_STATUS_PLB: usize = 0x020; // RDP_STATUS: RDP PIPELINE Busy (Bit 5)
pub const RDP_STATUS_CMB: usize = 0x040; // RDP_STATUS: RDP COMMAND Unit Busy (Bit 6)
pub const RDP_STATUS_CMR: usize = 0x080; // RDP_STATUS: RDP COMMAND Buffer Ready (Bit 7)
pub const RDP_STATUS_DMA: usize = 0x100; // RDP_STATUS: RDP DMA Busy (Bit 8)
pub const RDP_STATUS_CME: usize = 0x200; // RDP_STATUS: RDP COMMAND END Register Valid (Bit 9)
pub const RDP_STATUS_CMS: usize = 0x400; // RDP_STATUS: RDP COMMAND START Register Valid (Bit 10)

// RDP Status Write Flags:
const RDP_STATUS_CLR_XBS: usize = 0x001; // RDP_STATUS: Clear XBUS DMEM DMA (Bit 0)
const RDP_STATUS_SET_XBS: usize = 0x002; // RDP_STATUS:   Set XBUS DMEM DMA (Bit 1)
const RDP_STATUS_CLR_FRZ: usize = 0x004; // RDP_STATUS: Clear FREEZE (Bit 2)
const RDP_STATUS_SET_FRZ: usize = 0x008; // RDP_STATUS:   Set FREEZE (Bit 3)
const RDP_STATUS_CLR_FLS: usize = 0x010; // RDP_STATUS: Clear FLUSH (Bit 4)
const RDP_STATUS_SET_FLS: usize = 0x020; // RDP_STATUS:   Set FLUSH (Bit 5)
const RDP_STATUS_CLR_TMC: usize = 0x040; // RDP_STATUS: Clear TMEM COUNTER (Bit 6)
const RDP_STATUS_CLR_PLC: usize = 0x080; // RDP_STATUS: Clear PIPELINE COUNTER (Bit 7)
const RDP_STATUS_CLR_CMC: usize = 0x100; // RDP_STATUS: Clear COMMAND COUNTER (Bit 8)
const RDP_STATUS_CLR_CLK: usize = 0x200; // RDP_STATUS: Clear CLOCK COUNTER (Bit 9)

#[inline]
pub unsafe fn start_command_buffer(commands: &[RdpCommand]) {
    data_cache_hit_writeback(commands);

    write_volatile(
        RDP_STATUS,
        RDP_STATUS_CLR_XBS | RDP_STATUS_CLR_FRZ | RDP_STATUS_CLR_FLS,
    );
    write_volatile(
        RDP_COMMAND_BUFFER_START,
        (commands.as_ptr() as usize) | 0xa000_0000,
    );
    write_volatile(
        RDP_COMMAND_BUFFER_END,
        (commands.as_ptr().add(commands.len()) as usize) | 0xa000_0000,
    );
}

#[inline]
pub fn wait_for_done() {
    let start = crate::sys::current_time_us();

    loop {
        let status = unsafe { read_volatile(RDP_STATUS) };

        if status & RDP_STATUS_CMB == 0 && status & RDP_STATUS_PLB == 0 {
            return;
        }

        if crate::sys::current_time_us() - start > 1_000_000 {
            return;
        }
    }
}
