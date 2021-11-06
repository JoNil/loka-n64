use crate::sys::{
    data_cache_hit_writeback, data_cache_hit_writeback_invalidate, memory_barrier, uncached_addr,
    uncached_addr_mut, virtual_to_physical, virtual_to_physical_mut,
};
use core::{
    ptr::{read_volatile, write_volatile},
    slice,
};

const PI_BASE: usize = 0xA460_0000;

const PI_RAM_ADDR: *mut usize = (PI_BASE) as _; // Uncached address in RAM where data should be found
const PI_CART_ADDR: *mut usize = (PI_BASE + 0x4) as _; // Address of data on peripheral
const PI_READ_LENGTH: *mut usize = (PI_BASE + 0x8) as _; // How much data to read from RAM into the peripheral
const PI_WRITE_LENGTH: *mut usize = (PI_BASE + 0xC) as _; // brief How much data to write to RAM from the peripheral
const PI_STATUS: *mut usize = (PI_BASE + 0x10) as _; // brief Status of the PI, including DMA busy

const PI_STATUS_DMA_BUSY: usize = 0x0001;
const PI_STATUS_IO_BUSY: usize = 0x0002;

#[inline]
fn dma_wait() {
    while unsafe { read_volatile(PI_STATUS) } & (PI_STATUS_DMA_BUSY | PI_STATUS_IO_BUSY) > 0 {}
}

pub fn read(dst: *mut u8, len: u32, pi_address: usize) {
    unsafe {
        data_cache_hit_writeback_invalidate(slice::from_raw_parts(dst, len as usize));

        dma_wait();

        write_volatile(PI_STATUS, 3);
        memory_barrier();

        write_volatile(PI_RAM_ADDR, uncached_addr_mut(dst as _) as _);
        memory_barrier();

        write_volatile(PI_CART_ADDR, virtual_to_physical(pi_address as *const u8));
        memory_barrier();

        write_volatile(PI_WRITE_LENGTH, (len - 1) as _);
        memory_barrier();

        dma_wait();
    }
}

pub fn write(src: *const u8, len: u32, pi_address: usize) {
    unsafe {
        data_cache_hit_writeback(slice::from_raw_parts(src, len as usize));

        dma_wait();

        write_volatile(PI_STATUS, 3);
        memory_barrier();

        write_volatile(PI_RAM_ADDR, uncached_addr(src as _) as _);
        memory_barrier();

        write_volatile(PI_CART_ADDR, virtual_to_physical_mut(pi_address as *mut u8));
        memory_barrier();

        write_volatile(PI_READ_LENGTH, (len - 1) as _);
        memory_barrier();

        dma_wait();
    }
}
