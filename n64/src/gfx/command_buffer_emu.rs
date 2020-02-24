use n64_math::{Vec2, Color};
use crate::graphics::{WIDTH, HEIGHT};
use core::marker::PhantomData;

pub struct CommandBuffer<'a> {
    marker: PhantomData<&'a mut [Color]>,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(framebuffer: &'a mut [Color]) -> Self {
        CommandBuffer {
            marker: PhantomData,
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        self
    }

    pub fn add_rect(&mut self, upper_left: Vec2, lower_right: Vec2, color: Color) -> &mut Self {
        self
    }

    pub fn run(mut self) {
        
    }
}