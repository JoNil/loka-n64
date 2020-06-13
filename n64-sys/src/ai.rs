use crate::sys::{memory_barrier, uncached_addr, virtual_to_physical, data_cache_hit_writeback};
use core::ptr::{read_volatile, write_volatile};

const AI_BASE: usize = 0xA4500000;

const AI_ADDR: *mut usize = (AI_BASE + 0x00) as _;
const AI_LENGTH: *mut usize = (AI_BASE + 0x04) as _;
const AI_CONTROL: *mut usize = (AI_BASE + 0x08) as _;
const AI_STATUS: *mut usize = (AI_BASE + 0x0C) as _;
const AI_DACRATE: *mut usize = (AI_BASE + 0x10) as _;
const AI_SAMPLESIZE: *mut usize = (AI_BASE + 0x14) as _;

const AI_NTSC_DACRATE: usize = 48681812;
const AI_PAL_DACRATE: usize = 49656530;
const AI_MPAL_DACRATE: usize = 48628316;

const AI_STATUS_BUSY: usize = 1 << 30;
const AI_STATUS_FULL: usize = 1 << 31;

const TV_TYPE_LOC: usize = 0x80000300;

const FREQUENCY: usize = 22050;

#[inline]
pub fn init() {
    unsafe {
        let clockrate = match read_volatile(TV_TYPE_LOC as *const usize) {
            0 => AI_PAL_DACRATE,
            2 => AI_MPAL_DACRATE,
            _ => AI_NTSC_DACRATE,
        };

        write_volatile(AI_DACRATE, ((2 * clockrate / FREQUENCY) + 1) / 2 - 1);
        write_volatile(AI_SAMPLESIZE, 15);
    }
}

#[inline]
pub fn busy() -> bool {
    unsafe { read_volatile(AI_STATUS) & AI_STATUS_BUSY > 0 }
}

#[inline]
pub fn full() -> bool {
    unsafe { read_volatile(AI_STATUS) & AI_STATUS_FULL > 0 }
}

#[inline]
pub fn submit_audio_data_to_dac(buffer: &[i16]) {
    unsafe {
        data_cache_hit_writeback(buffer);
        write_volatile(AI_ADDR,virtual_to_physical(buffer.as_ptr()));
        write_volatile(AI_LENGTH, buffer.len() & !7);
        write_volatile(AI_CONTROL, 1);
    }
}
