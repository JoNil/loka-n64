#![cfg_attr(not(target_vendor = "nintendo64"), allow(unused))]

use crate::{current_time_us, framebuffer::Framebuffer, include_bytes_align_as, VideoMode};
use aligned::{Aligned, A8};
use alloc::vec::Vec;
use core::{ops::DerefMut, slice};
use n64_macros::debugln;
use n64_sys::{
    rsp,
    sys::{data_cache_hit_writeback, virtual_to_physical},
    vi,
};
use n64_types::RdpBlock;
use zerocopy::AsBytes;

#[repr(C, align(8))]
#[derive(AsBytes)]
struct RspDmem {
    pointer_count: u32,
    chunk_pointer: [u32; 255],
}

pub struct Graphics {
    gpu_commands: Vec<RdpBlock>,
}

impl Graphics {
    #[inline]
    pub(crate) fn new(video_mode: VideoMode, framebuffer: &mut Framebuffer) -> Self {
        vi::init(video_mode, &mut framebuffer.vi_buffer.0);
        rsp::init();
        Self {
            gpu_commands: Vec::with_capacity(32),
        }
    }

    #[inline]
    pub fn swap_buffers(&mut self, framebuffer: &mut Framebuffer) -> i64 {
        //rsp::wait(500);

        framebuffer.swap();

        let swap_start = current_time_us();
        vi::wait_for_vblank();
        let swap_end = current_time_us();
        unsafe { vi::set_vi_buffer(&mut framebuffer.vi_buffer.0) };

        swap_end - swap_start
    }

    #[inline]
    pub fn rsp_start(&mut self, commands: &mut Vec<RdpBlock>) {
        let code = include_bytes_align_as!(u64, "../../n64-sys/rsp/rsp.bin");

        core::mem::swap(&mut self.gpu_commands, commands);

        let mut rsp_dmem = RspDmem {
            pointer_count: self.gpu_commands.len() as u32, //2,
            chunk_pointer: [0; 255],
        };

        static mut FC: usize = 0;

        for (index, chunk) in self.gpu_commands.iter().enumerate() {
            unsafe {
                data_cache_hit_writeback(slice::from_raw_parts::<u64>(
                    &chunk.block_len as *const u64,
                    128,
                ))
            };
            rsp_dmem.chunk_pointer[index] = virtual_to_physical(chunk as *const RdpBlock) as u32;
        }

        unsafe {
            FC += 1;
        }

        let mut should_panic = false;

        rsp::run(code, Some(rsp_dmem.as_bytes()));
        if !rsp::wait(5000) {
            debugln!("RSP TIMEOUT! {:032b} pc {:08x}", rsp::status(), rsp::pc());
            should_panic = true;
        }

        unsafe {
            debugln!("{} status {:032b} pc {:08x}", FC, rsp::status(), rsp::pc());
        }

        //debugln!("Hello");
        let print_64bit = true;
        let print_32bit = true;
        if should_panic {
            for (block_index, block) in self.gpu_commands.iter().enumerate() {
                debugln!("BLOCK {}: {}", block_index, block.block_len);
                for (i, command) in block.rdp_data.iter().enumerate() {
                    debugln!(
                        "ADDR {:<8} : {:064b} : {:016x} : {:20}",
                        i,
                        command.0,
                        command.0,
                        command.0,
                    );
                }
            }

            let mut dmem: Aligned<A8, [u8; 4096]> = Aligned([0x00; 4096]);

            rsp::read_dmem(dmem.deref_mut());

            if print_64bit {
                debugln!(
                    "ADDR          : BINARY                                                           : HEX              : DECIMAL");
                for (i, word) in dmem.chunks_exact(8).enumerate() {
                    let a = u64::from_be_bytes(word.try_into().unwrap());

                    debugln!(
                        "ADDR {:<8} : {:064b} : {:016x} : {:20} : {:24.8}",
                        i * 8,
                        a,
                        a,
                        a,
                        float_from_fix_16_16_u64(a)
                    );

                    //assert!(a == i as u32);
                }
            }
            if print_32bit {
                debugln!("ADDR      : BINARY                           : HEX      : DECIMAL");
                for (i, word) in dmem.chunks_exact(4).enumerate() {
                    let a = u32::from_be_bytes(word.try_into().unwrap());

                    debugln!(
                        "ADDR {:<4} : {:032b} : {:08x} : {:10} : {:14.8}",
                        i * 4,
                        a,
                        a,
                        a,
                        float_from_fix_16_16_u32(a)
                    );

                    //assert!(a == i as u32);
                }
            }

            panic!("DONE");
        }
    }
}

fn float_from_fix_16_16_u32(a: u32) -> f64 {
    (a as f64) / ((1 << 16) as f64)
}

fn float_from_fix_16_16_u64(a: u64) -> f64 {
    (a as f64) / ((1 << 16) as f64)
}
