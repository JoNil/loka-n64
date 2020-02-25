use n64_math::{Vec2, Color};
use crate::graphics::{WIDTH, HEIGHT, GFX_EMU_STATE, GfxEmuState};
use core::marker::PhantomData;
use std::sync::MutexGuard;

pub struct CommandBuffer<'a> {
    marker: PhantomData<&'a mut [Color]>,

    //state: MutexGuard<'a, GfxEmuState>,

    //encoder: Option<wgpu::CommandEncoder>,
    //render_pass: Option<wgpu::RenderPass<'a>>, 

}

impl<'a> CommandBuffer<'a> {
    pub fn new(framebuffer: &'a mut [Color]) -> Self {

        //let state = GFX_EMU_STATE.lock().unwrap();

        CommandBuffer {
            marker: PhantomData,
            //state,
            //encoder: None,
            //render_pass: None,
        }
    }

    pub fn clear(&mut self) -> &mut Self {

        /*self.encoder = Some(self.state.device.create_command_encoder(&Default::default()));
        self.render_pass = Some(self.encoder.as_mut().unwrap().begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.state.frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            }],
            depth_stencil_attachment: None,
        }));*/

        self
    }

    pub fn add_rect(&mut self, upper_left: Vec2, lower_right: Vec2, color: Color) -> &mut Self {
        self
    }

    pub fn run(mut self) {
        
    }
}