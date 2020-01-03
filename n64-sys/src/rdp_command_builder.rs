use crate::sys::virtual_to_physical_mut;
use crate::vi::WIDTH;
use alloc::boxed::Box;
use alloc::vec::Vec;
use n64_math::{Color, Vec2};

// RDP Command Docs: http://ultra64.ca/files/documentation/silicon-graphics/SGI_RDP_Command_Summary.pdf

pub const OTHER_MODES_ALPHA_COMPARE_EN: u64 = 0x00000000000001; // Set_Other_Modes A: Conditional Color Write On Alpha Compare (Bit 0)
pub const OTHER_MODES_DITHER_ALPHA_EN: u64 = 0x00000000000002; // Set_Other_Modes B: Use Random Noise In Alpha Compare, Otherwise Use Blend Alpha In Alpha Compare (Bit 1)
pub const OTHER_MODES_Z_SOURCE_SEL: u64 = 0x00000000000004; // Set_Other_Modes C: Choose Between Primitive Z And Pixel Z (Bit 2)
pub const OTHER_MODES_ANTIALIAS_EN: u64 = 0x00000000000008; // Set_Other_Modes D: If Not Force Blend, Allow Blend Enable - Use CVG Bits (Bit 3)
pub const OTHER_MODES_Z_COMPARE_EN: u64 = 0x00000000000010; // Set_Other_Modes E: Conditional Color Write Enable On Depth Comparison (Bit 4)
pub const OTHER_MODES_Z_UPDATE_EN: u64 = 0x00000000000020; // Set_Other_Modes F: Enable Writing Of Z If Color Write Enabled (Bit 5)
pub const OTHER_MODES_IMAGE_READ_EN: u64 = 0x00000000000040; // Set_Other_Modes G: Enable Color/CVG Read/Modify/Write Memory Access (Bit 6)
pub const OTHER_MODES_COLOR_ON_CVG: u64 = 0x00000000000080; // Set_Other_Modes H: Only Update Color On Coverage Overflow (Transparent Surfaces) (Bit 7)
pub const OTHER_MODES_CVG_DEST_CLAMP: u64 = 0x00000000000000; // Set_Other_Modes I: CVG Destination Clamp (Normal) (Bit 8..9)
pub const OTHER_MODES_CVG_DEST_WRAP: u64 = 0x00000000000100; // Set_Other_Modes I: CVG Destination Wrap (WAS Assume Full CVG) (Bit 8..9)
pub const OTHER_MODES_CVG_DEST_ZAP: u64 = 0x00000000000200; // Set_Other_Modes I: CVG Destination Zap (Force To Full CVG) (Bit 8..9)
pub const OTHER_MODES_CVG_DEST_SAVE: u64 = 0x00000000000300; // Set_Other_Modes I: CVG Destination Save (Don't Overwrite Memory CVG) (Bit 8..9)
pub const OTHER_MODES_Z_MODE_OPAQUE: u64 = 0x00000000000000; // Set_Other_Modes J: Z Mode Opaque (Bit 10..11)
pub const OTHER_MODES_Z_MODE_INTERPENETRATING: u64 = 0x00000000000400; // Set_Other_Modes J: Z Mode Interpenetrating (Bit 10..11)
pub const OTHER_MODES_Z_MODE_TRANSPARENT: u64 = 0x00000000000800; // Set_Other_Modes J: Z Mode Transparent (Bit 10..11)
pub const OTHER_MODES_Z_MODE_DECAL: u64 = 0x00000000000C00; // Set_Other_Modes J: Z Mode Decal (Bit 10..11)
pub const OTHER_MODES_CVG_TIMES_ALPHA: u64 = 0x00000000001000; // Set_Other_Modes K: Use CVG Times Alpha For Pixel Alpha And Coverage (Bit 12)
pub const OTHER_MODES_ALPHA_CVG_SELECT: u64 = 0x00000000002000; // Set_Other_Modes L: Use CVG (Or CVG*Alpha) For Pixel Alpha (Bit 13)
pub const OTHER_MODES_FORCE_BLEND: u64 = 0x00000000004000; // Set_Other_Modes M: Force Blend Enable (Bit 14)
pub const OTHER_MODES_B_M2B_1_0: u64 = 0x00000000000000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 0, Cycle 1 (Bit 16..17)
pub const OTHER_MODES_B_M2B_1_1: u64 = 0x00000000010000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 1, Cycle 1 (Bit 16..17)
pub const OTHER_MODES_B_M2B_1_2: u64 = 0x00000000020000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 2, Cycle 1 (Bit 16..17)
pub const OTHER_MODES_B_M2B_1_3: u64 = 0x00000000030000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 3, Cycle 1 (Bit 16..17)
pub const OTHER_MODES_B_M2B_0_0: u64 = 0x00000000000000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 0, Cycle 0 (Bit 18..19)
pub const OTHER_MODES_B_M2B_0_1: u64 = 0x00000000040000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 1, Cycle 0 (Bit 18..19)
pub const OTHER_MODES_B_M2B_0_2: u64 = 0x00000000080000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 2, Cycle 0 (Bit 18..19)
pub const OTHER_MODES_B_M2B_0_3: u64 = 0x000000000C0000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 3, Cycle 0 (Bit 18..19)
pub const OTHER_MODES_B_M2A_1_0: u64 = 0x00000000000000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 0, Cycle 1 (Bit 20..21)
pub const OTHER_MODES_B_M2A_1_1: u64 = 0x00000000100000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 1, Cycle 1 (Bit 20..21)
pub const OTHER_MODES_B_M2A_1_2: u64 = 0x00000000200000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 2, Cycle 1 (Bit 20..21)
pub const OTHER_MODES_B_M2A_1_3: u64 = 0x00000000300000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 3, Cycle 1 (Bit 20..21)
pub const OTHER_MODES_B_M2A_0_0: u64 = 0x00000000000000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 0, Cycle 0 (Bit 22..23)
pub const OTHER_MODES_B_M2A_0_1: u64 = 0x00000000400000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 1, Cycle 0 (Bit 22..23)
pub const OTHER_MODES_B_M2A_0_2: u64 = 0x00000000800000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 2, Cycle 0 (Bit 22..23)
pub const OTHER_MODES_B_M2A_0_3: u64 = 0x00000000C00000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 3, Cycle 0 (Bit 22..23)
pub const OTHER_MODES_B_M1B_1_0: u64 = 0x00000000000000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 0, Cycle 1 (Bit 24..25)
pub const OTHER_MODES_B_M1B_1_1: u64 = 0x00000001000000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 1, Cycle 1 (Bit 24..25)
pub const OTHER_MODES_B_M1B_1_2: u64 = 0x00000002000000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 2, Cycle 1 (Bit 24..25)
pub const OTHER_MODES_B_M1B_1_3: u64 = 0x00000003000000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 3, Cycle 1 (Bit 24..25)
pub const OTHER_MODES_B_M1B_0_0: u64 = 0x00000000000000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 0, Cycle 0 (Bit 26..27)
pub const OTHER_MODES_B_M1B_0_1: u64 = 0x00000004000000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 1, Cycle 0 (Bit 26..27)
pub const OTHER_MODES_B_M1B_0_2: u64 = 0x00000008000000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 2, Cycle 0 (Bit 26..27)
pub const OTHER_MODES_B_M1B_0_3: u64 = 0x0000000C000000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 3, Cycle 0 (Bit 26..27)
pub const OTHER_MODES_B_M1A_1_0: u64 = 0x00000000000000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 0, Cycle 1 (Bit 28..29)
pub const OTHER_MODES_B_M1A_1_1: u64 = 0x00000010000000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 1, Cycle 1 (Bit 28..29)
pub const OTHER_MODES_B_M1A_1_2: u64 = 0x00000020000000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 2, Cycle 1 (Bit 28..29)
pub const OTHER_MODES_B_M1A_1_3: u64 = 0x00000030000000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 3, Cycle 1 (Bit 28..29)
pub const OTHER_MODES_B_M1A_0_0: u64 = 0x00000000000000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 0, Cycle 0 (Bit 30..31)
pub const OTHER_MODES_B_M1A_0_1: u64 = 0x00000040000000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 1, Cycle 0 (Bit 30..31)
pub const OTHER_MODES_B_M1A_0_2: u64 = 0x00000080000000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 2, Cycle 0 (Bit 30..31)
pub const OTHER_MODES_B_M1A_0_3: u64 = 0x000000C0000000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 3, Cycle 0 (Bit 30..31)
pub const OTHER_MODES_ALPHA_DITHER_SEL_PATTERN: u64 = 0x00000000000000; // Set_Other_Modes V1: Alpha Dither Selection Pattern (Bit 36..37)
pub const OTHER_MODES_ALPHA_DITHER_SEL_PATTERNB: u64 = 0x00001000000000; // Set_Other_Modes V1: Alpha Dither Selection ~Pattern (Bit 36..37)
pub const OTHER_MODES_ALPHA_DITHER_SEL_NOISE: u64 = 0x00002000000000; // Set_Other_Modes V1: Alpha Dither Selection Noise (Bit 36..37)
pub const OTHER_MODES_ALPHA_DITHER_SEL_NO_DITHER: u64 = 0x00003000000000; // Set_Other_Modes V1: Alpha Dither Selection No Dither (Bit 36..37)
pub const OTHER_MODES_RGB_DITHER_SEL_MAGIC_SQUARE_MATRIX: u64 = 0x00000000000000; // Set_Other_Modes V2: RGB Dither Selection Magic Square Matrix (Preferred If Filtered) (Bit 38..39)
pub const OTHER_MODES_RGB_DITHER_SEL_STANDARD_BAYER_MATRIX: u64 = 0x00004000000000; // Set_Other_Modes V2: RGB Dither Selection Standard Bayer Matrix (Preferred If Not Filtered) (Bit 38..39)
pub const OTHER_MODES_RGB_DITHER_SEL_NOISE: u64 = 0x00008000000000; // Set_Other_Modes V2: RGB Dither Selection Noise (As Before) (Bit 38..39)
pub const OTHER_MODES_RGB_DITHER_SEL_NO_DITHER: u64 = 0x0000C000000000; // Set_Other_Modes V2: RGB Dither Selection No Dither (Bit 38..39)
pub const OTHER_MODES_KEY_EN: u64 = 0x00010000000000; // Set_Other_Modes W: Enables Chroma Keying (Bit 40)
pub const OTHER_MODES_CONVERT_ONE: u64 = 0x00020000000000; // Set_Other_Modes X: Color Convert Texel That Was The Ouput Of The Texture Filter On Cycle0, Used To Qualify BI_LERP_1 (Bit 41)
pub const OTHER_MODES_BI_LERP_1: u64 = 0x00040000000000; // Set_Other_Modes Y: 1=BI_LERP, 0=Color Convert Operation In Texture Filter. Used In Cycle 1 (Bit 42)
pub const OTHER_MODES_BI_LERP_0: u64 = 0x00080000000000; // Set_Other_Modes Z: 1=BI_LERP, 0=Color Convert Operation In Texture Filter. Used In Cycle 0 (Bit 43)
pub const OTHER_MODES_MID_TEXEL: u64 = 0x00100000000000; // Set_Other_Modes a: Indicates Texture Filter Should Do A 2x2 Half Texel Interpolation, Primarily Used For MPEG Motion Compensation Processing (Bit 44)
pub const OTHER_MODES_SAMPLE_TYPE: u64 = 0x00200000000000; // Set_Other_Modes b: Determines How Textures Are Sampled: 0=1x1 (Point Sample), 1=2x2. Note That Copy (Point Sample 4 Horizontally Adjacent Texels) Mode Is Indicated By CYCLE_TYPE (Bit 45)
pub const OTHER_MODES_TLUT_TYPE: u64 = 0x00400000000000; // Set_Other_Modes c: Type Of Texels In Table, 0=16b RGBA(5/5/5/1), 1=IA(8/8) (Bit 46)
pub const OTHER_MODES_EN_TLUT: u64 = 0x00800000000000; // Set_Other_Modes d: Enable Lookup Of Texel Values From TLUT. Meaningful If Texture Type Is Index, Tile Is In Low TMEM, TLUT Is In High TMEM, And Color Image Is RGB (Bit 47)
pub const OTHER_MODES_TEX_LOD_EN: u64 = 0x01000000000000; // Set_Other_Modes e: Enable Texture Level Of Detail (LOD) (Bit 48)
pub const OTHER_MODES_SHARPEN_TEX_EN: u64 = 0x02000000000000; // Set_Other_Modes f: Enable Sharpened Texture (Bit 49)
pub const OTHER_MODES_DETAIL_TEX_EN: u64 = 0x04000000000000; // Set_Other_Modes g: Enable Detail Texture (Bit 50)
pub const OTHER_MODES_PERSP_TEX_EN: u64 = 0x08000000000000; // Set_Other_Modes h: Enable Perspective Correction On Texture (Bit 51)
pub const OTHER_MODES_CYCLE_TYPE_1_CYCLE: u64 = 0x00000000000000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode 1 Cycle (Bit 52..53)
pub const OTHER_MODES_CYCLE_TYPE_2_CYCLE: u64 = 0x10000000000000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode 2 Cycle (Bit 52..53)
pub const OTHER_MODES_CYCLE_TYPE_COPY: u64 = 0x20000000000000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode Copy (Bit 52..53)
pub const OTHER_MODES_CYCLE_TYPE_FILL: u64 = 0x30000000000000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode Fill (Bit 52..53)
pub const OTHER_MODES_ATOMIC_PRIM: u64 = 0x80000000000000; // Set_Other_Modes k: Force Primitive To Be Written To Frame Buffer Before Read Of Following

const COMMAND_SET_COLOR_IMAGE: u64 = 0x3f;
const COMMAND_SET_SCISSOR: u64 = 0x2d;
const COMMAND_SET_OTHER_MODE: u64 = 0x2f;
const COMMAND_SET_FILL_COLOR: u64 = 0x37;
const COMMAND_FILL_RECTANGLE: u64 = 0x36;
const COMMAND_SYNC_FULL: u64 = 0x29;

#[repr(C, align(8))]
pub struct Command(u64);

pub struct RdpCommandBuilder {
    commands: Vec<Command>,
}

impl RdpCommandBuilder {
    #[inline]
    pub fn new() -> RdpCommandBuilder {
        RdpCommandBuilder {
            commands: Vec::new(),
        }
    }

    #[inline]
    pub fn set_color_image(mut self, image: *mut u16) -> RdpCommandBuilder {
        self.commands.push(Command(
            (COMMAND_SET_COLOR_IMAGE << 56)
                | (0x2 << 51) // RGBA 16 bits per channel
                | ((WIDTH as u64 - 1) << 32)
                | (virtual_to_physical_mut(image) as u64),
        ));

        self
    }

    #[inline]
    pub fn set_scissor(mut self, top_left: Vec2, bottom_right: Vec2) -> RdpCommandBuilder {
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
    pub fn set_other_modes(mut self, flags: u64) -> RdpCommandBuilder {
        self.commands.push(Command(
            (COMMAND_SET_OTHER_MODE << 56) | (flags & ((1 << 56) - 1)),
        ));
        self
    }

    #[inline]
    pub fn set_fill_color(mut self, color: Color) -> RdpCommandBuilder {
        self.commands.push(Command(
            (COMMAND_SET_FILL_COLOR << 56)
                | ((color.value() as u64) << 16)
                | (color.value() as u64),
        ));
        self
    }

    #[inline]
    pub fn fill_rectangle(mut self, top_left: Vec2, bottom_right: Vec2) -> RdpCommandBuilder {
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
    pub fn sync_full(mut self) -> RdpCommandBuilder {
        self.commands.push(Command(COMMAND_SYNC_FULL << 56));
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
