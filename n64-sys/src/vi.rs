#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};
use n64_math::Color;
use n64_types::VideoMode;

const VI_STATUS_BPP0: usize = 0x0000;  // VI Status/Control: Color Depth Blank (No Data Or Sync) (Bit 0..1)
const VI_STATUS_BPP16: usize = 0x0002; // VI Status/Control: Color Depth 16BPP R5/G5/B5/A1 (Bit 0..1)
const VI_STATUS_BPP32: usize = 0x0003; // VI Status/Control: Color Depth 32BPP R8/G8/B8/A8 (Bit 0..1)
const VI_STATUS_GAMMA_DITHER_EN: usize = 0x00004; // VI Status/Control: Gamma Dither Enable (Requires: Gamma Enable) (Bit 2)
const VI_STATUS_GAMMA_EN: usize = 0x00008;        // VI Status/Control: Gamma Enable (Gamma Boost For YUV Images) (Bit 3)
const VI_STATUS_DIVOT_EN: usize = 0x00010;    // VI Status/Control: Divot Enable (Used With Anti-alias) (Bit 4)
const VI_STATUS_VBUS_CLK_EN: usize = 0x00020; // VI Status/Control: Video Bus Clock Enable (Bit 5)
const VI_STATUS_INTERLACE: usize = 0x00040; // VI Status/Control: Interlace/Serrate (Used With Interlaced Display) (Bit 6)
const VI_STATUS_TST_MODE: usize = 0x00080;  // VI Status/Control: Test Mode (Bit 7)
const VI_STATUS_AA_MODE_0: usize = 0x00000; // VI Status/Control: AA Mode 0 = Anti­-alias & Resample (Always Fetch Extra Lines) (Bit 8..9)
const VI_STATUS_AA_MODE_1: usize = 0x00100; // VI Status/Control: AA Mode 1 = Anti­-alias & Resample (Fetch Extra Lines When Needed) (Bit 8..9)
const VI_STATUS_AA_MODE_2: usize = 0x00200; // VI Status/Control: AA Mode 2 = Resample Only (Bit 8..9)
const VI_STATUS_AA_MODE_3: usize = 0x00300; // VI Status/Control: AA Mode 3 = Replicate Pixels & No Interpolation (Bit 8..9)
const VI_STATUS_DIAG_0: usize = 0x00400; // VI Status/Control: Diagnotic 0 (Bit 10..11)
const VI_STATUS_DIAG_1: usize = 0x00800; // VI Status/Control: Diagnotic 1 (Bit 10..11)
const VI_STATUS_PIXEL_ADV_0: usize = 0x00000; // VI Status/Control: Pixel Advance 0 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_1: usize = 0x01000; // VI Status/Control: Pixel Advance 1 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_2: usize = 0x02000; // VI Status/Control: Pixel Advance 2 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_3: usize = 0x03000; // VI Status/Control: Pixel Advance 3 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_4: usize = 0x04000; // VI Status/Control: Pixel Advance 4 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_5: usize = 0x05000; // VI Status/Control: Pixel Advance 5 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_6: usize = 0x06000; // VI Status/Control: Pixel Advance 6 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_7: usize = 0x07000; // VI Status/Control: Pixel Advance 7 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_8: usize = 0x08000; // VI Status/Control: Pixel Advance 8 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_9: usize = 0x09000; // VI Status/Control: Pixel Advance 9 (Bit 12..15)
const VI_STATUS_PIXEL_ADV_A: usize = 0x0A000; // VI Status/Control: Pixel Advance A (Bit 12..15)
const VI_STATUS_PIXEL_ADV_B: usize = 0x0B000; // VI Status/Control: Pixel Advance B (Bit 12..15)
const VI_STATUS_PIXEL_ADV_C: usize = 0x0C000; // VI Status/Control: Pixel Advance C (Bit 12..15)
const VI_STATUS_PIXEL_ADV_D: usize = 0x0D000; // VI Status/Control: Pixel Advance D (Bit 12..15)
const VI_STATUS_PIXEL_ADV_E: usize = 0x0E000; // VI Status/Control: Pixel Advance E (Bit 12..15)
const VI_STATUS_PIXEL_ADV_F: usize = 0x0F000; // VI Status/Control: Pixel Advance F (Bit 12..15)
const VI_STATUS_DITHER_FILTER_EN: usize = 0x10000; // VI Status/Control: Dither Filter Enable (Used With 16BPP Display) (Bit 16)

const VI_BASE: usize = 0xA440_0000;

const VI_STATUS: *mut usize = VI_BASE as _;
const VI_DRAM_ADDR: *mut usize = (VI_BASE + 0x04) as _;
const VI_H_WIDTH: *mut usize = (VI_BASE + 0x08) as _;
const VI_V_INTR: *mut usize = (VI_BASE + 0x0C) as _;
const VI_CURRENT: *const usize = (VI_BASE + 0x10) as _;
const VI_TIMING: *mut usize = (VI_BASE + 0x14) as _;
const VI_V_SYNC: *mut usize = (VI_BASE + 0x18) as _;
const VI_H_SYNC: *mut usize = (VI_BASE + 0x1C) as _;
const VI_H_SYNC_LEAP: *mut usize = (VI_BASE + 0x20) as _;
const VI_H_VIDEO: *mut usize = (VI_BASE + 0x24) as _;
const VI_V_VIDEO: *mut usize = (VI_BASE + 0x28) as _;
const VI_V_BURST: *mut usize = (VI_BASE + 0x2C) as _;
const VI_X_SCALE: *mut usize = (VI_BASE + 0x30) as _;
const VI_Y_SCALE: *mut usize = (VI_BASE + 0x34) as _;

static mut LAST_BUFFER: Option<*mut Color> = None;

#[inline]
pub fn init(video_mode: VideoMode, fb: &mut [Color]) {
    match video_mode {
        VideoMode::Ntsc { .. } => unsafe {
            write_volatile(VI_STATUS, VI_STATUS_PIXEL_ADV_3 | VI_STATUS_AA_MODE_2 | VI_STATUS_BPP16);
            write_volatile(VI_DRAM_ADDR, fb.as_mut_ptr() as usize);
            write_volatile(VI_H_WIDTH, video_mode.width() as usize);
            write_volatile(VI_V_INTR, 2);
            write_volatile(VI_TIMING, 0x03E5_2239);
            write_volatile(VI_V_SYNC, 0x0000_020D);
            write_volatile(VI_H_SYNC, 0x0000_0C15);
            write_volatile(VI_H_SYNC_LEAP, 0x0C15_0C15);
            write_volatile(VI_H_VIDEO, 0x006C_02EC);
            write_volatile(VI_V_VIDEO, 0x0025_01FF);
            write_volatile(VI_V_BURST, 0x000E_0204);
            write_volatile(VI_X_SCALE, 0x100 * video_mode.width() as usize / 160);
            write_volatile(VI_Y_SCALE, 0x100 * video_mode.height() as usize / 60);
        },
        VideoMode::Pal { .. } => unsafe {
            write_volatile(VI_STATUS, VI_STATUS_PIXEL_ADV_3 | VI_STATUS_AA_MODE_2 | VI_STATUS_BPP16);
            write_volatile(VI_DRAM_ADDR, fb.as_mut_ptr() as usize);
            write_volatile(VI_H_WIDTH, video_mode.width() as usize);
            write_volatile(VI_V_INTR, 0x200);
            write_volatile(VI_TIMING, 0x0040_4233A);
            write_volatile(VI_V_SYNC, 0x0000_0271);
            write_volatile(VI_H_SYNC, 0x0015_0C69);
            write_volatile(VI_H_SYNC_LEAP, 0x0C6F_0C6E);
            write_volatile(VI_H_VIDEO, 0x0080_0300);
            write_volatile(VI_V_VIDEO, 0x005F_0239);
            write_volatile(VI_V_BURST, 0x0009_026B);
            write_volatile(VI_X_SCALE, 0x100 * video_mode.width() as usize / 160);
            write_volatile(VI_Y_SCALE, 0x100 * video_mode.height() as usize / 60);
        },
    }
}

#[inline]
pub fn wait_for_vblank() {
    loop {
        let current_halfline = unsafe { read_volatile(VI_CURRENT) };
        if current_halfline <= 1 {
            break;
        }
    }
}

#[inline]
pub unsafe fn set_vi_buffer(fb: &mut [Color]) {
    LAST_BUFFER = Some(fb.as_mut_ptr());
    write_volatile(VI_DRAM_ADDR, fb.as_mut_ptr() as usize);
}

#[inline]
pub unsafe fn get_vi_buffer() -> *mut Color {
    LAST_BUFFER.unwrap()
}
