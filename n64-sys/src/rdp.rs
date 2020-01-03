#![allow(dead_code)]

use alloc::boxed::Box;
use core::ptr::{read_volatile, write_volatile};
use crate::rdp_command_builder::Command;
use crate::sys::{data_cache_hit_writeback_invalidate, memory_barrier, virtual_to_physical};

const RDP_BASE: usize = 0xA410_0000;

const RDP_COMMAND_BUFFER_START: *mut usize = (RDP_BASE + 0x00) as _;
const RDP_COMMAND_BUFFER_END: *mut usize = (RDP_BASE + 0x04) as _;
const RDP_COMMAND_BUFFER_CURRENT: *const usize = (RDP_BASE + 0x08) as _;
const RDP_STATUS: *mut usize = (RDP_BASE + 0x0C) as _;
const RDP_CLOCK_COUNTER: *const usize = (RDP_BASE + 0x10) as _;
const RDP_COMMAND_BUFFER_BUSY: *const usize = (RDP_BASE + 0x14) as _;
const RDP_PIPE_BUSY: *const usize = (RDP_BASE + 0x18) as _;
const RDP_TMEM_BUSY: *const usize = (RDP_BASE + 0x1C) as _;

// RDP Status Read Flags:
const RDP_STATUS_XBS: usize = 0x001; // RDP_STATUS: Use XBUS DMEM DMA Or DRAM DMA (Bit 0)
const RDP_STATUS_FRZ: usize = 0x002; // RDP_STATUS: RDP Frozen (Bit 1)
const RDP_STATUS_FLS: usize = 0x004; // RDP_STATUS: RDP Flushed (Bit 2)
const RDP_STATUS_GCL: usize = 0x008; // RDP_STATUS: GCLK Alive (Bit 3)
const RDP_STATUS_TMB: usize = 0x010; // RDP_STATUS: TMEM Busy (Bit 4)
const RDP_STATUS_PLB: usize = 0x020; // RDP_STATUS: RDP PIPELINE Busy (Bit 5)
const RDP_STATUS_CMB: usize = 0x040; // RDP_STATUS: RDP COMMAND Unit Busy (Bit 6)
const RDP_STATUS_CMR: usize = 0x080; // RDP_STATUS: RDP COMMAND Buffer Ready (Bit 7)
const RDP_STATUS_DMA: usize = 0x100; // RDP_STATUS: RDP DMA Busy (Bit 8)
const RDP_STATUS_CME: usize = 0x200; // RDP_STATUS: RDP COMMAND END Register Valid (Bit 9)
const RDP_STATUS_CMS: usize = 0x400; // RDP_STATUS: RDP COMMAND START Register Valid (Bit 10)

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
fn wait_for_done() {
    while unsafe { read_volatile(RDP_COMMAND_BUFFER_BUSY) } > 0 {}
}

static mut COMMANDS: Option<Box<[Command]>> = None;

#[inline]
pub unsafe fn run_command_buffer(commands_in: Box<[Command]>) {
    if commands_in.len() == 0 {
        return;
    }

    COMMANDS = Some(commands_in);

    if let Some(commands) = &COMMANDS {

        data_cache_hit_writeback_invalidate(&commands);

        write_volatile(
            RDP_STATUS,
            RDP_STATUS_CLR_XBS | RDP_STATUS_CLR_FRZ | RDP_STATUS_CLR_FLS,
        );
        memory_barrier();

        write_volatile(
            RDP_COMMAND_BUFFER_START,
            virtual_to_physical(commands.as_ptr()),
        );
        memory_barrier();
        write_volatile(
            RDP_COMMAND_BUFFER_END,
            virtual_to_physical(commands.as_ptr().offset(commands.len() as isize)),
        );
        memory_barrier();

        wait_for_done();
    }
}
