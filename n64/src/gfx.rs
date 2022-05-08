pub use command_buffer::{CommandBuffer, CommandBufferCache};
pub use pipeline::{CycleType, FillPipeline, Pipeline};
pub use texture::{StaticTexture, Texture, TextureAlignment, TextureMut};

#[cfg(target_vendor = "nintendo64")]
mod command_buffer;

#[cfg(not(target_vendor = "nintendo64"))]
mod command_buffer_emu;
#[cfg(not(target_vendor = "nintendo64"))]
use command_buffer_emu as command_buffer;

pub mod blend_mode;
pub mod color_combiner_mode;
mod pipeline;
mod texture;
