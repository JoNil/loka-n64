mod texture;

pub(crate) mod rdp_command_builder;

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {
        pub mod command_buffer;
    } else {
        pub mod command_buffer_emu;
        pub use command_buffer_emu as command_buffer;
    }
}

pub use command_buffer::CommandBuffer;
pub use texture::Texture;
