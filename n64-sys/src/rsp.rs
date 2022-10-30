#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

use crate::sys::{
    data_cache_hit_writeback, data_cache_hit_writeback_invalidate, virtual_to_physical,
};

const RSP_RSP_ADDR: *mut usize = 0xA404_0000_usize as _; // RSP memory address (IMEM/DMEM)
const RSP_DRAM_ADDR: *mut usize = 0xA404_0004_usize as _; // RSP RDRAM memory address
const RSP_READ_LENGTH: *mut usize = 0xA404_0008_usize as _; // RDRAM->RSP DMA length
const RSP_WRITE_LENGTH: *mut usize = 0xA404_000C_usize as _; // RSP->RDRAM DMA length
const RSP_STATUS: *mut usize = 0xA404_0010_usize as _; // RSP status register
const RSP_DMA_FULL: *mut usize = 0xA404_0014_usize as _; // RSP DMA full register
const RSP_DMA_BUSY: *mut usize = 0xA404_0018_usize as _; // RSP DMA busy register
const RSP_SEMAPHORE: *mut usize = 0xA404_001C_usize as _; // RSP semaphore register

const RSP_DMEM: *mut usize = 0xA400_0000_usize as _; // RSP DMEM: 4K of data memory
const RSP_IMEM: *mut usize = 0xA400_1000_usize as _; // RSP IMEM: 4K of instruction memory
const RSP_PC: *mut usize = 0xA408_0000_usize as _; // Current RSP program counter

const RSP_STATUS_HALTED: usize = 1 << 0; // RSP halted
const RSP_STATUS_BROKE: usize = 1 << 1; // RSP executed a break instruction
const RSP_STATUS_DMA_BUSY: usize = 1 << 2; // RSP DMA busy
const RSP_STATUS_DMA_FULL: usize = 1 << 3; // RSP DMA full
const RSP_STATUS_IO_BUSY: usize = 1 << 4; // RSP IO busy
const RSP_STATUS_SSTEP: usize = 1 << 5; // RSP is in single step mode
const RSP_STATUS_INTERRUPT_ON_BREAK: usize = 1 << 6; // RSP generate interrupt when hit a break instruction
const RSP_STATUS_SIG0: usize = 1 << 7; // RSP signal 0 is set
const RSP_STATUS_SIG1: usize = 1 << 8; // RSP signal 1 is set
const RSP_STATUS_SIG2: usize = 1 << 9; // RSP signal 2 is set
const RSP_STATUS_SIG3: usize = 1 << 10; // RSP signal 3 is set
const RSP_STATUS_SIG4: usize = 1 << 11; // RSP signal 4 is set
const RSP_STATUS_SIG5: usize = 1 << 12; // RSP signal 5 is set
const RSP_STATUS_SIG6: usize = 1 << 13; // RSP signal 6 is set
const RSP_STATUS_SIG7: usize = 1 << 14; // RSP signal 7 is set

const RSP_WSTATUS_CLEAR_HALT: usize = 0x00001; // RSP_STATUS write mask: clear RSP_STATUS_HALTED bit
const RSP_WSTATUS_SET_HALT: usize = 0x00002; // RSP_STATUS write mask: set RSP_STATUS_HALTED bit
const RSP_WSTATUS_CLEAR_BROKE: usize = 0x00004; // RSP_STATUS write mask: clear BROKE bit
const RSP_WSTATUS_CLEAR_INTR: usize = 0x00008; // RSP_STATUS write mask: clear INTR bit
const RSP_WSTATUS_SET_INTR: usize = 0x00010; // RSP_STATUS write mask: set HALT bit
const RSP_WSTATUS_CLEAR_SSTEP: usize = 0x00020; // RSP_STATUS write mask: clear SSTEP bit
const RSP_WSTATUS_SET_SSTEP: usize = 0x00040; // RSP_STATUS write mask: set SSTEP bit
const RSP_WSTATUS_CLEAR_INTR_BREAK: usize = 0x00080; // RSP_STATUS write mask: clear #SP_STATUS_INTERRUPT_ON_BREAK bit
const RSP_WSTATUS_SET_INTR_BREAK: usize = 0x00100; // RSP_STATUS write mask: set SSTEP bit
const RSP_WSTATUS_CLEAR_SIG0: usize = 0x00200; // RSP_STATUS write mask: clear SIG0 bit
const RSP_WSTATUS_SET_SIG0: usize = 0x00400; // RSP_STATUS write mask: set SIG0 bit
const RSP_WSTATUS_CLEAR_SIG1: usize = 0x00800; // RSP_STATUS write mask: clear SIG1 bit
const RSP_WSTATUS_SET_SIG1: usize = 0x01000; // RSP_STATUS write mask: set SIG1 bit
const RSP_WSTATUS_CLEAR_SIG2: usize = 0x02000; // RSP_STATUS write mask: clear SIG2 bit
const RSP_WSTATUS_SET_SIG2: usize = 0x04000; // RSP_STATUS write mask: set SIG2 bit
const RSP_WSTATUS_CLEAR_SIG3: usize = 0x08000; // RSP_STATUS write mask: clear SIG3 bit
const RSP_WSTATUS_SET_SIG3: usize = 0x10000; // RSP_STATUS write mask: set SIG3 bit
const RSP_WSTATUS_CLEAR_SIG4: usize = 0x20000; // RSP_STATUS write mask: clear SIG4 bit
const RSP_WSTATUS_SET_SIG4: usize = 0x40000; // RSP_STATUS write mask: set SIG4 bit
const RSP_WSTATUS_CLEAR_SIG5: usize = 0x80000; // RSP_STATUS write mask: clear SIG5 bit
const RSP_WSTATUS_SET_SIG5: usize = 0x100000; // RSP_STATUS write mask: set SIG5 bit
const RSP_WSTATUS_CLEAR_SIG6: usize = 0x200000; // RSP_STATUS write mask: clear SIG6 bit
const RSP_WSTATUS_SET_SIG6: usize = 0x400000; // RSP_STATUS write mask: set SIG6 bit
const RSP_WSTATUS_CLEAR_SIG7: usize = 0x800000; // RSP_STATUS write mask: clear SIG7 bit
const RSP_WSTATUS_SET_SIG7: usize = 0x1000000; // RSP_STATUS write mask: set SIG7 bit

pub struct RspProgram<'a> {
    pub code: &'a [u8],
    pub data: Option<&'a [u8]>,
}

fn dma_wait() {
    while unsafe { read_volatile(RSP_STATUS) } & (RSP_STATUS_DMA_BUSY | RSP_STATUS_IO_BUSY) > 0 {}
}

fn start() {
    unsafe {
        write_volatile(RSP_PC, 0);
        write_volatile(RSP_STATUS, RSP_WSTATUS_CLEAR_HALT | RSP_WSTATUS_CLEAR_BROKE);
    }
}

fn wait() {
    loop {
        // Wait for the RSP to halt and the DMA engine to be idle.
        let status = unsafe { read_volatile(RSP_STATUS) };

        if (status & RSP_STATUS_HALTED) > 0
            && (status & (RSP_STATUS_DMA_BUSY | RSP_STATUS_DMA_FULL)) == 0
        {
            break;
        }
    }
}

pub fn init() {
    unsafe {
        write_volatile(RSP_PC, 0x1000);
        write_volatile(RSP_STATUS, RSP_WSTATUS_SET_HALT);
    }
}

pub fn run(program: RspProgram) {
    write_imem(program.code);
    if let Some(data) = program.data {
        write_dmem(data);
    }

    start();
    wait();
}

pub fn write_imem(data: &[u8]) {
    unsafe {
        assert!(data.len() <= 4096);
        assert!(data.as_ptr() as usize % 8 == 0);

        data_cache_hit_writeback(data);

        dma_wait();

        write_volatile(RSP_DRAM_ADDR, virtual_to_physical(data.as_ptr()));
        write_volatile(RSP_RSP_ADDR, RSP_IMEM as usize);
        write_volatile(RSP_READ_LENGTH, data.len() - 1);

        dma_wait();
    }
}

pub fn read_imem(data: &mut [u8; 4096]) {
    unsafe {
        assert!(data.as_ptr() as usize % 8 == 0);

        data_cache_hit_writeback_invalidate(data.as_slice());

        dma_wait();

        write_volatile(RSP_DRAM_ADDR, virtual_to_physical(data.as_ptr()));
        write_volatile(RSP_RSP_ADDR, RSP_IMEM as usize);
        write_volatile(RSP_WRITE_LENGTH, data.len() - 1);

        dma_wait();
    }
}

pub fn write_dmem(data: &[u8]) {
    unsafe {
        assert!(data.len() <= 4096);
        assert!(data.as_ptr() as usize % 8 == 0);

        data_cache_hit_writeback(data);

        dma_wait();

        write_volatile(RSP_DRAM_ADDR, virtual_to_physical(data.as_ptr()));
        write_volatile(RSP_RSP_ADDR, RSP_DMEM as usize);
        write_volatile(RSP_READ_LENGTH, data.len() - 1);

        dma_wait();
    }
}

pub fn read_dmem(data: &mut [u8; 4096]) {
    unsafe {
        assert!(data.as_ptr() as usize % 8 == 0);

        data_cache_hit_writeback_invalidate(data.as_slice());

        dma_wait();

        write_volatile(RSP_DRAM_ADDR, virtual_to_physical(data.as_ptr()));
        write_volatile(RSP_RSP_ADDR, RSP_DMEM as usize);
        write_volatile(RSP_WRITE_LENGTH, data.len() - 1);

        dma_wait();
    }
}
