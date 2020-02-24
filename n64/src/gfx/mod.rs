mod texture;

#[cfg(target_vendor = "nintendo64")]
pub(crate) mod rdp_command_builder;

#[cfg_attr(target_vendor = "nintendo64", path = "command_buffer.rs")]
#[cfg_attr(not(target_vendor = "nintendo64"), path = "command_buffer_emu.rs")]
mod command_buffer;

pub use command_buffer::CommandBuffer;
pub use texture::Texture;