#![cfg_attr(not(target_vendor = "nintendo64"), allow(unused))]

use crate::{current_time_us, framebuffer::Framebuffer, include_bytes_align_as, VideoMode};
use aligned::{Aligned, A8};
use alloc::{string::String, vec::Vec};
use core::{ops::DerefMut, slice};
use n64_macros::debugln;
use n64_sys::{
    rdp, rsp,
    sys::{data_cache_hit_invalidate, data_cache_hit_writeback, virtual_to_physical},
    vi,
};
use n64_types::RdpBlock;
use zerocopy::AsBytes;

pub static CODE: &[u8] = include_bytes_align_as!(u64, "../../n64-sys/rsp/rsp.bin");

#[repr(C, align(8))]
#[derive(AsBytes)]
struct RspDmem {
    pointer_count: u32,
    rsp_res_ptr: u32,
    chunk_pointer: [u32; 255],
    padding: u32,
}

#[repr(C, align(8))]
#[derive(AsBytes, Default, Debug)]
struct RspRes {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
}

pub struct Graphics {
    gpu_commands: Vec<RdpBlock>,
    gpu_res: RspRes,
    pub buffer_started: bool,
    pub code: Vec<String>,
    pub pc: usize,
    pub last_pc: usize,
    pub frame_counter: usize,
}

impl Graphics {
    #[inline]
    pub(crate) fn new(video_mode: VideoMode, framebuffer: &mut Framebuffer) -> Self {
        vi::init(video_mode, &mut framebuffer.vi_buffer.0);
        rsp::init();

        // TODO(JoNil): This takes a lot of memory and should only be used in debug builds
        let mips = mipsasm_rsp::Mipsasm::new();
        let code = mips.disassemble(unsafe {
            slice::from_raw_parts(CODE.as_ptr() as *const u32, CODE.len() / 4)
        });

        Self {
            gpu_commands: Vec::with_capacity(32),
            gpu_res: RspRes::default(),
            buffer_started: false,
            code,
            pc: 0,
            last_pc: 0,
            frame_counter: 0,
        }
    }

    pub fn code(&self) -> &[String] {
        &self.code
    }

    #[inline]
    pub fn swap_buffers(&mut self, framebuffer: &mut Framebuffer) -> i64 {
        //rsp::wait(5_000_000);

        framebuffer.swap();

        let swap_start = current_time_us();
        vi::wait_for_vblank();
        let swap_end = current_time_us();
        unsafe { vi::set_vi_buffer(&mut framebuffer.vi_buffer.0) };

        self.frame_counter += 1;

        swap_end - swap_start
    }

    #[inline]
    pub fn frame_counter(&self) -> usize {
        self.frame_counter
    }

    #[inline]
    pub fn rsp_step(&mut self, step: bool) -> (usize, usize, [u8; 4096]) {
        if step {
            rsp::step();
        }

        let mut dmem: Aligned<A8, [u8; 4096]> = Aligned([0x00; 4096]);
        rsp::read_dmem(dmem.deref_mut());
        (rsp::status(), rsp::pc(), *dmem)
    }

    pub fn rsp_single_step_print(&mut self) {
        rsp::activate_single_step();

        let mut i = 0;
        debugln!("Status          PC");
        loop {
            let (status, pc, _) = self.rsp_step(true);
            let code = self.code();

            if pc / 4 > code.len() {
                debugln!("{:015b} {:08x} : OUTSIDE CODE RANGE", status, pc);
                break;
            }

            debugln!("{:015b} {:08x} : {}", status, pc, code[pc / 4]);

            if i > 1024 {
                break;
            }
            i += 1;
        }
    }

    pub fn rsp_dump_mem(&mut self) {
        let print_64bit = true;
        let print_32bit = true;

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
            }
        }
    }

    #[inline]
    pub fn rsp_start(&mut self, commands: &mut Vec<RdpBlock>, single_step: bool) {
        core::mem::swap(&mut self.gpu_commands, commands);

        // TODO(JoNil): Rsp Res needs to be double buffered when the gpu is pipelined
        let mut rsp_dmem = RspDmem {
            pointer_count: self.gpu_commands.len() as u32,
            rsp_res_ptr: virtual_to_physical(&self.gpu_res as *const RspRes) as u32,
            chunk_pointer: [0; 255],
            padding: 0,
        };

        for (index, chunk) in self.gpu_commands.iter().enumerate() {
            unsafe {
                data_cache_hit_writeback(slice::from_raw_parts::<u64>(
                    &chunk.block_len as *const u64,
                    128,
                ))
            };
            rsp_dmem.chunk_pointer[index] = virtual_to_physical(chunk as *const RdpBlock) as u32;
        }

        let mut should_panic = false;

        rsp::run(CODE, Some(rsp_dmem.as_bytes()), single_step);
        if !single_step {
            let (wait_ok, rsp_status) = {
                n64_profiler::scope!("Rsp Wait");
                rsp::wait(5_000_000)
            };
            if !wait_ok {
                debugln!(
                    "RSP TIMEOUT! {:032b} pc {:08x}, fc {}",
                    rsp_status,
                    rsp::pc(),
                    self.frame_counter,
                );
                should_panic = true;
            }
        }

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
            self.rsp_dump_mem();

            self.rsp_single_step_print();

            unsafe {
                data_cache_hit_invalidate(slice::from_raw_parts::<u64>(
                    &self.gpu_res as *const _ as *const u64,
                    4,
                ));
            }

            debugln!("RSP RES {:#?}", self.gpu_res);

            panic!("RSP TIMEOUT PANIC");
        }
    }

    pub fn rdp_clock_count(&self) -> u32 {
        self.gpu_res.a
    }
}

fn float_from_fix_16_16_u32(a: u32) -> f64 {
    (a as f64) / ((1 << 16) as f64)
}

fn float_from_fix_16_16_u64(a: u64) -> f64 {
    (a as f64) / ((1 << 16) as f64)
}
