use crate::include_bytes_align_as;
use crate::{current_time_us, framebuffer::Framebuffer, VideoMode};
use aligned::{Aligned, A8};
use core::ops::{Deref, DerefMut};
use n64_macros::debugln;
use n64_sys::{rdp, rsp, vi};
use n64_types::RdpCommand;

pub struct Graphics {}

impl Graphics {
    #[inline]
    pub(crate) fn new(video_mode: VideoMode, framebuffer: &mut Framebuffer) -> Self {
        vi::init(video_mode, &mut framebuffer.vi_buffer.0);
        rdp::init();
        rsp::init();
        Self {}
    }

    #[inline]
    pub fn swap_buffers(&mut self, framebuffer: &mut Framebuffer) -> i64 {
        rdp::wait_for_done();

        framebuffer.swap();

        let swap_start = current_time_us();
        vi::wait_for_vblank();
        let swap_end = current_time_us();
        unsafe { vi::set_vi_buffer(&mut framebuffer.vi_buffer.0) };

        swap_end - swap_start
    }

    #[inline]
    pub fn rsp_hello_world(&self, commands: &[RdpCommand]) {
        let code = include_bytes_align_as!(u64, "../../n64-sys/rsp/hello_world.bin");

        let commands_len = commands.len() * core::mem::size_of::<RdpCommand>();

        //debugln!("Commands: {}", commands.len());

        assert!(commands_len < 4096);

        let mut data: Aligned<A8, [u8; 4096]> = Aligned([0xff; 4096]);
        data[..commands_len].copy_from_slice(unsafe {
            core::slice::from_raw_parts(commands.as_ptr() as _, commands_len)
        });

        data[4094..].copy_from_slice((commands.len() as u16).to_be_bytes().as_slice());

        rsp::run(code, Some(data.deref()));

        let mut dmem: Aligned<A8, [u8; 4096]> = Aligned([0x00; 4096]);

        rsp::read_dmem(dmem.deref_mut());

        let should_panic = true;
        if should_panic {
            if true {
                debugln!(
                    "ADDR          : BINARY                                                           : HEX              : DECIMAL");
                for (i, word) in dmem.chunks_exact(8).enumerate() {
                    let a = u64::from_be_bytes(word.try_into().unwrap());

                    debugln!("ADDR {:<8} : {:064b} : {:016x} : {:20}", i * 8, a, a, a);

                    //assert!(a == i as u32);
                }
            } else {
                debugln!("ADDR      : BINARY                           : HEX      : DECIMAL");
                for (i, word) in dmem.chunks_exact(4).enumerate() {
                    let a = u32::from_be_bytes(word.try_into().unwrap());

                    debugln!("ADDR {:<4} : {:032b} : {:08x} : {:10}", i * 4, a, a, a);

                    //assert!(a == i as u32);
                }
            }

            panic!("DONE");
        }
    }
}
