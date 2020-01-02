use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::vi::WIDTH;
use crate::sys::virtual_to_physical_mut;

const COMMAND_SET_COLOR_IMAGE: u64 = 0x3f;

pub struct RdpCommandBuilder {
    commands: Vec<u64>,
}

impl RdpCommandBuilder {
    pub fn new() -> RdpCommandBuilder {
        RdpCommandBuilder {
            commands: Vec::new(),
        }
    }

    pub fn set_color_image(mut self, image: *mut u16) -> RdpCommandBuilder {
        self.commands.push(
            COMMAND_SET_COLOR_IMAGE << 56 |
            0x2 << 51 |
            (WIDTH as u64 - 1) << 32 |
            virtual_to_physical_mut(image) as u64
        );

        self
    }

    //Set_Scissor 0<<2,0<<2, 0,0, 320<<2,240<<2 // Set Scissor: XH 0.0,YH 0.0, Scissor Field Enable Off,Field Off, XL 320.0,YL 240.0
    //Set_Other_Modes CYCLE_TYPE_FILL // Set Other Modes
    //Set_Fill_Color $00010001 // Set Fill Color: PACKED COLOR 16B R5G5B5A1 Pixels
    //Fill_Rectangle 319<<2,239<<2, 0<<2,0<<2 // Fill Rectangle: XL 319.0,YL 239.0, XH 0.0,YH 0.0

    pub fn build(self) -> Box<[u64]> {
        self.commands.into_boxed_slice()
    }
}