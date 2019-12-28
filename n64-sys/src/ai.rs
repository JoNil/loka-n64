use crate::sys::{memory_barrier, uncached_addr, uncached_addr_mut, virtual_to_physical};
use core::intrinsics::volatile_copy_nonoverlapping_memory;
use core::ptr::{read_volatile, write_volatile};

const AI_BASE: usize = 0xA4500000;

const AI_ADDR: *mut usize = (AI_BASE + 0x00) as _;
const AI_LENGTH: *mut u32 = (AI_BASE + 0x04) as _;
const AI_CONTROL: *mut u32 = (AI_BASE + 0x08) as _;
const AI_STATUS: *mut u32 = (AI_BASE + 0x0C) as _;
const AI_DACRATE: *mut u32 = (AI_BASE + 0x10) as _;
const AI_SAMPLESIZE: *mut u32 = (AI_BASE + 0x14) as _;

const AI_NTSC_DACRATE: u32 = 48681812;
const AI_PAL_DACRATE: u32 = 49656530;
const AI_MPAL_DACRATE: u32 = 48628316;

const AI_STATUS_BUSY: u32 = 1 << 30;
const AI_STATUS_FULL: u32 = 1 << 31;

const TV_TYPE_LOC: usize = 0x80000300;

const FREQUENCY: u32 = 22050;
const BUFFER_COUNT: usize = 4;

pub const BUFFER_NO_SAMPLES: usize = 2 * 512;

static mut REAL_FREQUENCY: u32 = 0;
static mut BUFFERS: [[i16; BUFFER_NO_SAMPLES]; BUFFER_COUNT] =
    [[0; BUFFER_NO_SAMPLES]; BUFFER_COUNT];

static mut NOW_PLAYING: usize = 0;
static mut NOW_WRITING: usize = 0;
static mut BUFFERS_FULL_BITMASK: usize = 0;

#[inline]
fn ai_busy() -> bool {
    unsafe { read_volatile(AI_STATUS) & AI_STATUS_BUSY > 0 }
}

#[inline]
fn ai_full() -> bool {
    unsafe { read_volatile(AI_STATUS) & AI_STATUS_FULL > 0 }
}

#[inline]
pub fn init() {
    unsafe {
        let clockrate = match read_volatile(TV_TYPE_LOC as *const u32) {
            0 => AI_PAL_DACRATE,
            2 => AI_MPAL_DACRATE,
            _ => AI_NTSC_DACRATE,
        };

        write_volatile(AI_DACRATE, ((2 * clockrate / FREQUENCY) + 1) / 2 - 1);
        write_volatile(AI_SAMPLESIZE, 15);

        REAL_FREQUENCY = 2 * clockrate / ((2 * clockrate / FREQUENCY) + 1);
    }
}

#[inline]
pub fn write_audio_blocking(buffer: &[i16]) {

    if buffer.len() != BUFFER_NO_SAMPLES {
        panic!();
    }

    unsafe {
        let next = (NOW_WRITING + 1) % BUFFER_COUNT;
        while BUFFERS_FULL_BITMASK & (1 << next) > 0 {
            submit_audio_data_to_dac();
        }

        BUFFERS_FULL_BITMASK |= 1 << next;
        NOW_WRITING = next;

        volatile_copy_nonoverlapping_memory(
            uncached_addr_mut(BUFFERS[NOW_WRITING].as_mut_ptr()),
            buffer.as_ptr(),
            buffer.len(),
        );
    }
}

#[inline]
pub fn all_buffers_are_full() -> bool {
    unsafe {
        let next = (NOW_WRITING + 1) % BUFFER_COUNT;
        return BUFFERS_FULL_BITMASK & (1 << next) > 0;
    }
}

#[inline]
pub fn submit_audio_data_to_dac() {
    unsafe {
        while !ai_full() {
            // check if next buffer is full
            let next = (NOW_PLAYING + 1) % BUFFER_COUNT;
            if BUFFERS_FULL_BITMASK & (1 << next) == 0 {
                break;
            }

            // clear buffer full flag
            BUFFERS_FULL_BITMASK &= !(1 << next);

            // Set up DMA
            NOW_PLAYING = next;

            write_volatile(
                AI_ADDR,
                virtual_to_physical(uncached_addr(BUFFERS[NOW_PLAYING].as_ptr())),
            );
            memory_barrier();
            write_volatile(AI_LENGTH, ((BUFFER_NO_SAMPLES * 2) & !7) as u32);
            memory_barrier();

            // Start DMA
            write_volatile(AI_CONTROL, 1);
            memory_barrier();
        }
    }
}