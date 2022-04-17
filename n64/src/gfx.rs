pub use command_buffer::{CommandBuffer, CommandBufferCache};
pub use texture::{StaticTexture, Texture, TextureMut};

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {
        mod command_buffer;

    } else {
        mod command_buffer_emu;
        use command_buffer_emu as command_buffer;
    }
}

mod color_combiner;
mod texture;
