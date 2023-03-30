pub use command_buffer::{CommandBuffer, CommandBufferCache};
pub use pipeline::{CycleType, FillPipeline, Pipeline, ZMode, ZSrc};
pub use texture::{StaticTexture, Texture, TextureAlignment, TextureMut};

mod command_buffer_n64;

#[cfg(not(target_vendor = "nintendo64"))]
mod command_buffer_emu;

#[cfg(target_vendor = "nintendo64")]
use command_buffer_n64 as command_buffer;

#[cfg(not(target_vendor = "nintendo64"))]
use command_buffer_emu as command_buffer;

pub mod blend_mode;
pub mod color_combiner_mode;
mod pipeline;
mod texture;
