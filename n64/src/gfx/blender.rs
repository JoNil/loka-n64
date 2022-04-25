#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
pub struct Blender {}

impl Blender {
    pub const fn default() -> Self {
        Self {}
    }
}

impl Blender {
    pub fn to_command(&self) -> u64 {
        0
    }
}

impl Default for Blender {
    fn default() -> Self {
        Self::default()
    }
}
