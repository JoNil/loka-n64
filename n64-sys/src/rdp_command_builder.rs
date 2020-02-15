use crate::sys::virtual_to_physical_mut;
use crate::vi::WIDTH;
use alloc::boxed::Box;
use alloc::vec::Vec;
use n64_math::{Color, Vec2};

// RDP Command Docs: http://ultra64.ca/files/documentation/silicon-graphics/SGI_RDP_Command_Summary.pdf

pub mod other_modes {
    pub const ALPHA_COMPARE_EN: u64 = 0x00_0000_0000_0001; // Set_Other_Modes A: Conditional Color Write On Alpha Compare (Bit 0)
    pub const DITHER_ALPHA_EN: u64 = 0x00_0000_0000_0002; // Set_Other_Modes B: Use Random Noise In Alpha Compare, Otherwise Use Blend Alpha In Alpha Compare (Bit 1)
    pub const Z_SOURCE_SEL: u64 = 0x00_0000_0000_0004; // Set_Other_Modes C: Choose Between Primitive Z And Pixel Z (Bit 2)
    pub const ANTIALIAS_EN: u64 = 0x00_0000_0000_0008; // Set_Other_Modes D: If Not Force Blend, Allow Blend Enable - Use CVG Bits (Bit 3)
    pub const Z_COMPARE_EN: u64 = 0x00_0000_0000_0010; // Set_Other_Modes E: Conditional Color Write Enable On Depth Comparison (Bit 4)
    pub const Z_UPDATE_EN: u64 = 0x00_0000_0000_0020; // Set_Other_Modes F: Enable Writing Of Z If Color Write Enabled (Bit 5)
    pub const IMAGE_READ_EN: u64 = 0x00_0000_0000_0040; // Set_Other_Modes G: Enable Color/CVG Read/Modify/Write Memory Access (Bit 6)
    pub const COLOR_ON_CVG: u64 = 0x00_0000_0000_0080; // Set_Other_Modes H: Only Update Color On Coverage Overflow (Transparent Surfaces) (Bit 7)
    pub const CVG_DEST_CLAMP: u64 = 0x00_0000_0000_0000; // Set_Other_Modes I: CVG Destination Clamp (Normal) (Bit 8..9)
    pub const CVG_DEST_WRAP: u64 = 0x00_0000_0000_0100; // Set_Other_Modes I: CVG Destination Wrap (WAS Assume Full CVG) (Bit 8..9)
    pub const CVG_DEST_ZAP: u64 = 0x00_0000_0000_0200; // Set_Other_Modes I: CVG Destination Zap (Force To Full CVG) (Bit 8..9)
    pub const CVG_DEST_SAVE: u64 = 0x00_0000_0000_0300; // Set_Other_Modes I: CVG Destination Save (Don't Overwrite Memory CVG) (Bit 8..9)
    pub const Z_MODE_OPAQUE: u64 = 0x00_0000_0000_0000; // Set_Other_Modes J: Z Mode Opaque (Bit 10..11)
    pub const Z_MODE_INTERPENETRATING: u64 = 0x00_0000_0000_0400; // Set_Other_Modes J: Z Mode Interpenetrating (Bit 10..11)
    pub const Z_MODE_TRANSPARENT: u64 = 0x00_0000_0000_0800; // Set_Other_Modes J: Z Mode Transparent (Bit 10..11)
    pub const Z_MODE_DECAL: u64 = 0x00_0000_0000_0C00; // Set_Other_Modes J: Z Mode Decal (Bit 10..11)
    pub const CVG_TIMES_ALPHA: u64 = 0x00_0000_0000_1000; // Set_Other_Modes K: Use CVG Times Alpha For Pixel Alpha And Coverage (Bit 12)
    pub const ALPHA_CVG_SELECT: u64 = 0x00_0000_0000_2000; // Set_Other_Modes L: Use CVG (Or CVG*Alpha) For Pixel Alpha (Bit 13)
    pub const FORCE_BLEND: u64 = 0x00_0000_0000_4000; // Set_Other_Modes M: Force Blend Enable (Bit 14)
    pub const B_M2B_1_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 0, Cycle 1 (Bit 16..17)
    pub const B_M2B_1_1: u64 = 0x00_0000_0001_0000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 1, Cycle 1 (Bit 16..17)
    pub const B_M2B_1_2: u64 = 0x00_0000_0002_0000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 2, Cycle 1 (Bit 16..17)
    pub const B_M2B_1_3: u64 = 0x00_0000_0003_0000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 3, Cycle 1 (Bit 16..17)
    pub const B_M2B_0_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 0, Cycle 0 (Bit 18..19)
    pub const B_M2B_0_1: u64 = 0x00_0000_0004_0000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 1, Cycle 0 (Bit 18..19)
    pub const B_M2B_0_2: u64 = 0x00_0000_0008_0000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 2, Cycle 0 (Bit 18..19)
    pub const B_M2B_0_3: u64 = 0x00_0000_000C_0000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 3, Cycle 0 (Bit 18..19)
    pub const B_M2A_1_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 0, Cycle 1 (Bit 20..21)
    pub const B_M2A_1_1: u64 = 0x00_0000_0010_0000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 1, Cycle 1 (Bit 20..21)
    pub const B_M2A_1_2: u64 = 0x00_0000_0020_0000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 2, Cycle 1 (Bit 20..21)
    pub const B_M2A_1_3: u64 = 0x00_0000_0030_0000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 3, Cycle 1 (Bit 20..21)
    pub const B_M2A_0_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 0, Cycle 0 (Bit 22..23)
    pub const B_M2A_0_1: u64 = 0x00_0000_0040_0000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 1, Cycle 0 (Bit 22..23)
    pub const B_M2A_0_2: u64 = 0x00_0000_0080_0000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 2, Cycle 0 (Bit 22..23)
    pub const B_M2A_0_3: u64 = 0x00_0000_00C0_0000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 3, Cycle 0 (Bit 22..23)
    pub const B_M1B_1_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 0, Cycle 1 (Bit 24..25)
    pub const B_M1B_1_1: u64 = 0x00_0000_0100_0000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 1, Cycle 1 (Bit 24..25)
    pub const B_M1B_1_2: u64 = 0x00_0000_0200_0000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 2, Cycle 1 (Bit 24..25)
    pub const B_M1B_1_3: u64 = 0x00_0000_0300_0000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 3, Cycle 1 (Bit 24..25)
    pub const B_M1B_0_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 0, Cycle 0 (Bit 26..27)
    pub const B_M1B_0_1: u64 = 0x00_0000_0400_0000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 1, Cycle 0 (Bit 26..27)
    pub const B_M1B_0_2: u64 = 0x00_0000_0800_0000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 2, Cycle 0 (Bit 26..27)
    pub const B_M1B_0_3: u64 = 0x00_0000_0C00_0000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 3, Cycle 0 (Bit 26..27)
    pub const B_M1A_1_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 0, Cycle 1 (Bit 28..29)
    pub const B_M1A_1_1: u64 = 0x00_0000_1000_0000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 1, Cycle 1 (Bit 28..29)
    pub const B_M1A_1_2: u64 = 0x00_0000_2000_0000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 2, Cycle 1 (Bit 28..29)
    pub const B_M1A_1_3: u64 = 0x00_0000_3000_0000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 3, Cycle 1 (Bit 28..29)
    pub const B_M1A_0_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 0, Cycle 0 (Bit 30..31)
    pub const B_M1A_0_1: u64 = 0x00_0000_4000_0000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 1, Cycle 0 (Bit 30..31)
    pub const B_M1A_0_2: u64 = 0x00_0000_8000_0000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 2, Cycle 0 (Bit 30..31)
    pub const B_M1A_0_3: u64 = 0x00_0000_C000_0000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 3, Cycle 0 (Bit 30..31)
    pub const ALPHA_DITHER_SEL_PATTERN: u64 = 0x00_0000_0000_0000; // Set_Other_Modes V1: Alpha Dither Selection Pattern (Bit 36..37)
    pub const ALPHA_DITHER_SEL_PATTERNB: u64 = 0x00_0010_0000_0000; // Set_Other_Modes V1: Alpha Dither Selection ~Pattern (Bit 36..37)
    pub const ALPHA_DITHER_SEL_NOISE: u64 = 0x00_0020_0000_0000; // Set_Other_Modes V1: Alpha Dither Selection Noise (Bit 36..37)
    pub const ALPHA_DITHER_SEL_NO_DITHER: u64 = 0x00_0030_0000_0000; // Set_Other_Modes V1: Alpha Dither Selection No Dither (Bit 36..37)
    pub const RGB_DITHER_SEL_MAGIC_SQUARE_MATRIX: u64 = 0x00_0000_0000_0000; // Set_Other_Modes V2: RGB Dither Selection Magic Square Matrix (Preferred If Filtered) (Bit 38..39)
    pub const RGB_DITHER_SEL_STANDARD_BAYER_MATRIX: u64 = 0x00_0040_0000_0000; // Set_Other_Modes V2: RGB Dither Selection Standard Bayer Matrix (Preferred If Not Filtered) (Bit 38..39)
    pub const RGB_DITHER_SEL_NOISE: u64 = 0x00_0080_0000_0000; // Set_Other_Modes V2: RGB Dither Selection Noise (As Before) (Bit 38..39)
    pub const RGB_DITHER_SEL_NO_DITHER: u64 = 0x00_00C0_0000_0000; // Set_Other_Modes V2: RGB Dither Selection No Dither (Bit 38..39)
    pub const KEY_EN: u64 = 0x00_0100_0000_0000; // Set_Other_Modes W: Enables Chroma Keying (Bit 40)
    pub const CONVERT_ONE: u64 = 0x00_0200_0000_0000; // Set_Other_Modes X: Color Convert Texel That Was The Ouput Of The Texture Filter On Cycle0, Used To Qualify BI_LERP_1 (Bit 41)
    pub const BI_LERP_1: u64 = 0x00_0400_0000_0000; // Set_Other_Modes Y: 1=BI_LERP, 0=Color Convert Operation In Texture Filter. Used In Cycle 1 (Bit 42)
    pub const BI_LERP_0: u64 = 0x00_0800_0000_0000; // Set_Other_Modes Z: 1=BI_LERP, 0=Color Convert Operation In Texture Filter. Used In Cycle 0 (Bit 43)
    pub const MID_TEXEL: u64 = 0x00_1000_0000_0000; // Set_Other_Modes a: Indicates Texture Filter Should Do A 2x2 Half Texel Interpolation, Primarily Used For MPEG Motion Compensation Processing (Bit 44)
    pub const SAMPLE_TYPE: u64 = 0x00_2000_0000_0000; // Set_Other_Modes b: Determines How Textures Are Sampled: 0=1x1 (Point Sample), 1=2x2. Note That Copy (Point Sample 4 Horizontally Adjacent Texels) Mode Is Indicated By CYCLE_TYPE (Bit 45)
    pub const TLUT_TYPE: u64 = 0x00_4000_0000_0000; // Set_Other_Modes c: Type Of Texels In Table, 0=16b RGBA(5/5/5/1), 1=IA(8/8) (Bit 46)
    pub const EN_TLUT: u64 = 0x00_8000_0000_0000; // Set_Other_Modes d: Enable Lookup Of Texel Values From TLUT. Meaningful If Texture Type Is Index, Tile Is In Low TMEM, TLUT Is In High TMEM, And Color Image Is RGB (Bit 47)
    pub const TEX_LOD_EN: u64 = 0x01_0000_0000_0000; // Set_Other_Modes e: Enable Texture Level Of Detail (LOD) (Bit 48)
    pub const SHARPEN_TEX_EN: u64 = 0x02_0000_0000_0000; // Set_Other_Modes f: Enable Sharpened Texture (Bit 49)
    pub const DETAIL_TEX_EN: u64 = 0x04_0000_0000_0000; // Set_Other_Modes g: Enable Detail Texture (Bit 50)
    pub const PERSP_TEX_EN: u64 = 0x08_0000_0000_0000; // Set_Other_Modes h: Enable Perspective Correction On Texture (Bit 51)
    pub const CYCLE_TYPE_1_CYCLE: u64 = 0x00_0000_0000_0000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode 1 Cycle (Bit 52..53)
    pub const CYCLE_TYPE_2_CYCLE: u64 = 0x10_0000_0000_0000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode 2 Cycle (Bit 52..53)
    pub const CYCLE_TYPE_COPY: u64 = 0x20_0000_0000_0000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode Copy (Bit 52..53)
    pub const CYCLE_TYPE_FILL: u64 = 0x30_0000_0000_0000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode Fill (Bit 52..53)
    pub const ATOMIC_PRIM: u64 = 0x80_0000_0000_0000; // Set_Other_Modes k: Force Primitive To Be Written To Frame Buffer Before Read Of Following
}

const COMMAND_SET_COLOR_IMAGE: u64 = 0xff;
const COMMAND_SET_SCISSOR: u64 = 0xed;
const COMMAND_SET_OTHER_MODE: u64 = 0xef;
const COMMAND_SET_FILL_COLOR: u64 = 0xf7;
const COMMAND_FILL_RECTANGLE: u64 = 0xf6;
const COMMAND_SYNC_FULL: u64 = 0xe9;
const COMMAND_SYNC_PIPE: u64 = 0xe7;

#[repr(C, align(8))]
pub struct Command(u64);

pub struct RdpCommandBuilder {
    commands: Vec<Command>,
}

impl RdpCommandBuilder {
    #[inline]
    pub fn new() -> RdpCommandBuilder {
        RdpCommandBuilder {
            commands: Vec::with_capacity(1024),
        }
    }

    #[inline]
    pub fn set_color_image(&mut self, image: *mut u16) -> &mut RdpCommandBuilder {
        self.commands.push(Command(
            (COMMAND_SET_COLOR_IMAGE << 56)
                | (0x2 << 51) // RGBA 16 bits per channel
                | ((WIDTH as u64 - 1) << 32)
                | (virtual_to_physical_mut(image) as u64),
        ));

        self
    }

    #[inline]
    pub fn set_scissor(&mut self, top_left: Vec2, bottom_right: Vec2) -> &mut RdpCommandBuilder {
        self.commands.push(Command(
            (COMMAND_SET_SCISSOR << 56)
                | (to_fixpoint_10_2(top_left.x()) << (32 + 12))
                | (to_fixpoint_10_2(top_left.y()) << 32)
                | (to_fixpoint_10_2(bottom_right.x()) << 12)
                | (to_fixpoint_10_2(bottom_right.y())),
        ));

        self
    }

    #[inline]
    pub fn set_other_modes(&mut self, flags: u64) -> &mut RdpCommandBuilder {
        self.commands.push(Command(
            (COMMAND_SET_OTHER_MODE << 56) | (flags & ((1 << 56) - 1)) | 0x0000_000F_0000_0000,
        ));
        self
    }

    #[inline]
    pub fn set_fill_color(&mut self, color: Color) -> &mut RdpCommandBuilder {
        self.commands.push(Command(
            (COMMAND_SET_FILL_COLOR << 56)
                | ((color.value() as u64) << 16)
                | (color.value() as u64),
        ));
        self
    }

    #[inline]
    pub fn fill_rectangle(&mut self, top_left: Vec2, bottom_right: Vec2) -> &mut RdpCommandBuilder {
        self.commands.push(Command(
            (COMMAND_FILL_RECTANGLE << 56)
                | (to_fixpoint_10_2(bottom_right.x()) << (32 + 12))
                | (to_fixpoint_10_2(bottom_right.y()) << 32)
                | (to_fixpoint_10_2(top_left.x()) << 12)
                | (to_fixpoint_10_2(top_left.y())),
        ));
        self
    }

    #[inline]
    pub fn sync_full(&mut self) -> &mut RdpCommandBuilder {
        self.commands.push(Command(COMMAND_SYNC_FULL << 56));
        self
    }

    #[inline]
    pub fn sync_pipe(&mut self) -> &mut RdpCommandBuilder {
        self.commands.push(Command(COMMAND_SYNC_PIPE << 56));
        self
    }

    #[inline]
    pub fn build(self) -> Box<[Command]> {
        self.commands.into_boxed_slice()
    }
}

fn to_fixpoint_10_2(val: f32) -> u64 {
    (((val * (1 << 2) as f32) as i16) & 0xfff) as u64
}
