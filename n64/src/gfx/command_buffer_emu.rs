use crate::graphics::{ColoredRectUniforms, GfxEmuState, GFX_EMU_STATE, HEIGHT, INDEX_DATA, WIDTH};
use core::marker::PhantomData;
use core::mem;
use n64_math::{Color, Vec2};
use std::sync::MutexGuard;
use zerocopy::{AsBytes, FromBytes};

enum Command {
    Rect {
        upper_left: Vec2,
        lower_right: Vec2,
        color: Color,
    },
}

pub struct CommandBuffer<'a> {
    marker: PhantomData<&'a mut [Color]>,

    clear: bool,
    commands: Vec<Command>,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(framebuffer: &'a mut [Color]) -> Self {
        CommandBuffer {
            marker: PhantomData,
            clear: false,
            commands: Vec::new(),
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.clear = true;
        self.commands.clear();

        self
    }

    pub fn add_rect(&mut self, upper_left: Vec2, lower_right: Vec2, color: Color) -> &mut Self {
        self.commands.push(Command::Rect {
            upper_left,
            lower_right,
            color,
        });

        self
    }

    pub fn run(mut self) {
        /*let state = &mut *GFX_EMU_STATE.lock().unwrap();

        let frame = state
            .swap_chain
            .get_next_texture()
            .expect("Timeout when acquiring next swap chain texture");

        let command_buf = {
            let mut encoder = state.device.create_command_encoder(&Default::default());

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: if self.clear {
                            wgpu::LoadOp::Clear
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                render_pass.set_index_buffer(&state.index_buf, 0);
                render_pass.set_vertex_buffers(0, &[(&state.vertex_buf, 0)]);
                render_pass.set_pipeline(&state.colored_rect_pipeline);

                for command in self.commands {
                    match command {
                        Command::Rect {
                            upper_left,
                            lower_right,
                            color,
                        } => {
                            let uniforms = ColoredRectUniforms {
                                color: color.to_rgba(),
                                offset: [0.0, 0.0, 0.0],
                                scale: 1.0,
                            };

                            let uniform_buf = state.device.create_buffer_with_data(
                                uniforms.as_bytes(),
                                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                            );

                            let bind_group =
                                state.device.create_bind_group(&wgpu::BindGroupDescriptor {
                                    layout: &state.colored_rect_bind_group_layout,
                                    bindings: &[wgpu::Binding {
                                        binding: 0,
                                        resource: wgpu::BindingResource::Buffer {
                                            buffer: &uniform_buf,
                                            range: 0..mem::size_of::<ColoredRectUniforms>() as u64,
                                        },
                                    }],
                                });

                            //render_pass.set_bind_group(0, &bind_group, &[]);
                            render_pass.draw_indexed(0..(INDEX_DATA.len() as u32), 0, 0..1);
                        }
                    }
                }
            }

            encoder.finish()
        };

        state.queue.submit(&[command_buf]);*/
    }
}
