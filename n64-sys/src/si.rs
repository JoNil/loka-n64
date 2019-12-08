use crate::sys::{
    data_cache_hit_writeback_invalidate, disable_interrupts, enable_interrupts, memory_barrier,
    uncached_addr_mut, uncached_addr,
};
use core::intrinsics::{volatile_copy_nonoverlapping_memory};
use core::ptr::{read_volatile, write_volatile};

const SI_BASE: usize = 0xA480_0000;

const SI_ADDR: *mut usize = (SI_BASE + 0x00) as _;
const SI_START_READ: *mut usize = (SI_BASE + 0x04) as _;
const SI_START_WRITE: *mut usize = (SI_BASE + 0x10) as _;
const SI_STATUS: *mut usize = (SI_BASE + 0x18) as _;

const SI_STATUS_DMA_BUSY: usize = 0x0001;
const SI_STATUS_IO_BUSY: usize = 0x0002;

const PIF_RAM: usize = 0x1fc007c0;

fn dma_wait() {
    while unsafe { read_volatile(SI_STATUS) } & (SI_STATUS_DMA_BUSY | SI_STATUS_IO_BUSY) > 0 {}
}

fn dma_pif_block(inblock: &[u64; 8], outblock: &mut [u64; 8]) -> u32 {
    
    let res;

    unsafe {
        let mut inblock_temp: [u64; 8] = [0; 8];
        let mut outblock_temp: [u64; 8] = [0; 8];

        data_cache_hit_writeback_invalidate(&mut inblock_temp);
        volatile_copy_nonoverlapping_memory(
            uncached_addr_mut(inblock_temp.as_mut_ptr()),
            inblock.as_ptr(),
            inblock.len(),
        );

        disable_interrupts();

        dma_wait();

        memory_barrier();
        write_volatile(SI_ADDR, inblock_temp.as_mut_ptr() as usize);
        memory_barrier();
        write_volatile(SI_START_WRITE, PIF_RAM);
        memory_barrier();

        dma_wait();

        data_cache_hit_writeback_invalidate(&mut outblock_temp);

        memory_barrier();
        write_volatile(SI_ADDR, uncached_addr_mut(outblock_temp.as_mut_ptr()) as usize);
        memory_barrier();
        write_volatile(SI_START_READ, PIF_RAM);
        memory_barrier();

        dma_wait();

        res = (0..16).fold(0, |count, i| count + read_volatile(uncached_addr_mut((outblock_temp.as_mut_ptr() as *mut u32).offset(i))) as u32);
        //res = read_volatile(uncached_addr_mut((outblock_temp.as_mut_ptr() as *mut u32).offset(4))) as u32;

        enable_interrupts();

        volatile_copy_nonoverlapping_memory(
            outblock.as_mut_ptr(),
            uncached_addr_mut(outblock_temp.as_mut_ptr()),
            outblock.len(),
        );
    }

    res
}

pub fn read_controllers(outblock: &mut [u64; 8]) -> u32 {
    static READ_CON_BLOCK: [u64; 8] = [
        0xff010401ffffffff,
        0xff010401ffffffff,
        0xff010401ffffffff,
        0xff010401ffffffff,
        0xfe00000000000000,
        0,
        0,
        1,
    ];

    dma_pif_block(&READ_CON_BLOCK, outblock)
}
