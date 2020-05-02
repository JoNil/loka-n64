mod texture;

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {
        mod command_buffer;

        pub(crate) mod rdp_command_builder;

    } else {
        mod command_buffer_emu;
        use command_buffer_emu as command_buffer;

        pub(crate) mod pipelines;
    }
}

pub use command_buffer::CommandBuffer;
pub use texture::Texture;
