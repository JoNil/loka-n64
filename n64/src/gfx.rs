pub use command_buffer::CommandBuffer;
pub use texture::{Texture, TextureMut, StaticTexture};

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {
        mod command_buffer;

    } else {
        mod command_buffer_emu;
        use command_buffer_emu as command_buffer;
    }
}

mod texture;