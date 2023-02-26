#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use super::rdp_math::{to_fixpoint_10_2_as_integer, to_fixpoint_s_10_5, to_fixpoint_s_11_2};
use alloc::vec::Vec;
use n64_math::{Color, Vec2};
use n64_sys::sys::{virtual_to_physical, virtual_to_physical_mut};
use n64_types::{RdpBlock, RdpCommand};

// RDP Command Docs: http://ultra64.ca/files/documentation/silicon-graphics/SGI_RDP_Command_Summary.pdf

pub const OTHER_MODE_ALPHA_COMPARE_EN: u64 = 0x00_0000_0000_0001; // Set_Other_Modes A: Conditional Color Write On Alpha Compare (Bit 0)
pub const OTHER_MODE_DITHER_ALPHA_EN: u64 = 0x00_0000_0000_0002; // Set_Other_Modes B: Use Random Noise In Alpha Compare, Otherwise Use Blend Alpha In Alpha Compare (Bit 1)
pub const OTHER_MODE_Z_SOURCE_SEL: u64 = 0x00_0000_0000_0004; // Set_Other_Modes C: Choose Between Primitive Z And Pixel Z (Bit 2)
pub const OTHER_MODE_ANTIALIAS_EN: u64 = 0x00_0000_0000_0008; // Set_Other_Modes D: If Not Force Blend, Allow Blend Enable - Use CVG Bits (Bit 3)
pub const OTHER_MODE_Z_COMPARE_EN: u64 = 0x00_0000_0000_0010; // Set_Other_Modes E: Conditional Color Write Enable On Depth Comparison (Bit 4)
pub const OTHER_MODE_Z_UPDATE_EN: u64 = 0x00_0000_0000_0020; // Set_Other_Modes F: Enable Writing Of Z If Color Write Enabled (Bit 5)
pub const OTHER_MODE_IMAGE_READ_EN: u64 = 0x00_0000_0000_0040; // Set_Other_Modes G: Enable Color/CVG Read/Modify/Write Memory Access (Bit 6)
pub const OTHER_MODE_COLOR_ON_CVG: u64 = 0x00_0000_0000_0080; // Set_Other_Modes H: Only Update Color On Coverage Overflow (Transparent Surfaces) (Bit 7)
pub const OTHER_MODE_CVG_DEST_CLAMP: u64 = 0x00_0000_0000_0000; // Set_Other_Modes I: CVG Destination Clamp (Normal) (Bit 8..9)
pub const OTHER_MODE_CVG_DEST_WRAP: u64 = 0x00_0000_0000_0100; // Set_Other_Modes I: CVG Destination Wrap (WAS Assume Full CVG) (Bit 8..9)
pub const OTHER_MODE_CVG_DEST_ZAP: u64 = 0x00_0000_0000_0200; // Set_Other_Modes I: CVG Destination Zap (Force To Full CVG) (Bit 8..9)
pub const OTHER_MODE_CVG_DEST_SAVE: u64 = 0x00_0000_0000_0300; // Set_Other_Modes I: CVG Destination Save (Don't Overwrite Memory CVG) (Bit 8..9)
pub const OTHER_MODE_Z_MODE_OPAQUE: u64 = 0x00_0000_0000_0000; // Set_Other_Modes J: Z Mode Opaque (Bit 10..11)
pub const OTHER_MODE_Z_MODE_INTERPENETRATING: u64 = 0x00_0000_0000_0400; // Set_Other_Modes J: Z Mode Interpenetrating (Bit 10..11)
pub const OTHER_MODE_Z_MODE_TRANSPARENT: u64 = 0x00_0000_0000_0800; // Set_Other_Modes J: Z Mode Transparent (Bit 10..11)
pub const OTHER_MODE_Z_MODE_DECAL: u64 = 0x00_0000_0000_0C00; // Set_Other_Modes J: Z Mode Decal (Bit 10..11)
pub const OTHER_MODE_CVG_TIMES_ALPHA: u64 = 0x00_0000_0000_1000; // Set_Other_Modes K: Use CVG Times Alpha For Pixel Alpha And Coverage (Bit 12)
pub const OTHER_MODE_ALPHA_CVG_SELECT: u64 = 0x00_0000_0000_2000; // Set_Other_Modes L: Use CVG (Or CVG*Alpha) For Pixel Alpha (Bit 13)
pub const OTHER_MODE_FORCE_BLEND: u64 = 0x00_0000_0000_4000; // Set_Other_Modes M: Force Blend Enable (Bit 14)
pub const OTHER_MODE_B_M2B_1_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 0, Cycle 1 (Bit 16..17)
pub const OTHER_MODE_B_M2B_1_1: u64 = 0x00_0000_0001_0000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 1, Cycle 1 (Bit 16..17)
pub const OTHER_MODE_B_M2B_1_2: u64 = 0x00_0000_0002_0000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 2, Cycle 1 (Bit 16..17)
pub const OTHER_MODE_B_M2B_1_3: u64 = 0x00_0000_0003_0000; // Set_Other_Modes O: Blend Modeword, Multiply 2b Input Select 3, Cycle 1 (Bit 16..17)
pub const OTHER_MODE_B_M2B_0_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 0, Cycle 0 (Bit 18..19)
pub const OTHER_MODE_B_M2B_0_1: u64 = 0x00_0000_0004_0000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 1, Cycle 0 (Bit 18..19)
pub const OTHER_MODE_B_M2B_0_2: u64 = 0x00_0000_0008_0000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 2, Cycle 0 (Bit 18..19)
pub const OTHER_MODE_B_M2B_0_3: u64 = 0x00_0000_000C_0000; // Set_Other_Modes P: Blend Modeword, Multiply 2b Input Select 3, Cycle 0 (Bit 18..19)
pub const OTHER_MODE_B_M2A_1_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 0, Cycle 1 (Bit 20..21)
pub const OTHER_MODE_B_M2A_1_1: u64 = 0x00_0000_0010_0000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 1, Cycle 1 (Bit 20..21)
pub const OTHER_MODE_B_M2A_1_2: u64 = 0x00_0000_0020_0000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 2, Cycle 1 (Bit 20..21)
pub const OTHER_MODE_B_M2A_1_3: u64 = 0x00_0000_0030_0000; // Set_Other_Modes Q: Blend Modeword, Multiply 2a Input Select 3, Cycle 1 (Bit 20..21)
pub const OTHER_MODE_B_M2A_0_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 0, Cycle 0 (Bit 22..23)
pub const OTHER_MODE_B_M2A_0_1: u64 = 0x00_0000_0040_0000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 1, Cycle 0 (Bit 22..23)
pub const OTHER_MODE_B_M2A_0_2: u64 = 0x00_0000_0080_0000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 2, Cycle 0 (Bit 22..23)
pub const OTHER_MODE_B_M2A_0_3: u64 = 0x00_0000_00C0_0000; // Set_Other_Modes R: Blend Modeword, Multiply 2a Input Select 3, Cycle 0 (Bit 22..23)
pub const OTHER_MODE_B_M1B_1_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 0, Cycle 1 (Bit 24..25)
pub const OTHER_MODE_B_M1B_1_1: u64 = 0x00_0000_0100_0000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 1, Cycle 1 (Bit 24..25)
pub const OTHER_MODE_B_M1B_1_2: u64 = 0x00_0000_0200_0000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 2, Cycle 1 (Bit 24..25)
pub const OTHER_MODE_B_M1B_1_3: u64 = 0x00_0000_0300_0000; // Set_Other_Modes S: Blend Modeword, Multiply 1b Input Select 3, Cycle 1 (Bit 24..25)
pub const OTHER_MODE_B_M1B_0_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 0, Cycle 0 (Bit 26..27)
pub const OTHER_MODE_B_M1B_0_1: u64 = 0x00_0000_0400_0000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 1, Cycle 0 (Bit 26..27)
pub const OTHER_MODE_B_M1B_0_2: u64 = 0x00_0000_0800_0000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 2, Cycle 0 (Bit 26..27)
pub const OTHER_MODE_B_M1B_0_3: u64 = 0x00_0000_0C00_0000; // Set_Other_Modes T: Blend Modeword, Multiply 1b Input Select 3, Cycle 0 (Bit 26..27)
pub const OTHER_MODE_B_M1A_1_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 0, Cycle 1 (Bit 28..29)
pub const OTHER_MODE_B_M1A_1_1: u64 = 0x00_0000_1000_0000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 1, Cycle 1 (Bit 28..29)
pub const OTHER_MODE_B_M1A_1_2: u64 = 0x00_0000_2000_0000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 2, Cycle 1 (Bit 28..29)
pub const OTHER_MODE_B_M1A_1_3: u64 = 0x00_0000_3000_0000; // Set_Other_Modes U: Blend Modeword, Multiply 1a Input Select 3, Cycle 1 (Bit 28..29)
pub const OTHER_MODE_B_M1A_0_0: u64 = 0x00_0000_0000_0000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 0, Cycle 0 (Bit 30..31)
pub const OTHER_MODE_B_M1A_0_1: u64 = 0x00_0000_4000_0000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 1, Cycle 0 (Bit 30..31)
pub const OTHER_MODE_B_M1A_0_2: u64 = 0x00_0000_8000_0000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 2, Cycle 0 (Bit 30..31)
pub const OTHER_MODE_B_M1A_0_3: u64 = 0x00_0000_C000_0000; // Set_Other_Modes V: Blend Modeword, Multiply 1a Input Select 3, Cycle 0 (Bit 30..31)
pub const OTHER_MODE_ALPHA_DITHER_SEL_PATTERN: u64 = 0x00_0000_0000_0000; // Set_Other_Modes V1: Alpha Dither Selection Pattern (Bit 36..37)
pub const OTHER_MODE_ALPHA_DITHER_SEL_PATTERNB: u64 = 0x00_0010_0000_0000; // Set_Other_Modes V1: Alpha Dither Selection ~Pattern (Bit 36..37)
pub const OTHER_MODE_ALPHA_DITHER_SEL_NOISE: u64 = 0x00_0020_0000_0000; // Set_Other_Modes V1: Alpha Dither Selection Noise (Bit 36..37)
pub const OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER: u64 = 0x00_0030_0000_0000; // Set_Other_Modes V1: Alpha Dither Selection No Dither (Bit 36..37)
pub const OTHER_MODE_RGB_DITHER_SEL_MAGIC_SQUARE_MATRIX: u64 = 0x00_0000_0000_0000; // Set_Other_Modes V2: RGB Dither Selection Magic Square Matrix (Preferred If Filtered) (Bit 38..39)
pub const OTHER_MODE_RGB_DITHER_SEL_STANDARD_BAYER_MATRIX: u64 = 0x00_0040_0000_0000; // Set_Other_Modes V2: RGB Dither Selection Standard Bayer Matrix (Preferred If Not Filtered) (Bit 38..39)
pub const OTHER_MODE_RGB_DITHER_SEL_NOISE: u64 = 0x00_0080_0000_0000; // Set_Other_Modes V2: RGB Dither Selection Noise (As Before) (Bit 38..39)
pub const OTHER_MODE_RGB_DITHER_SEL_NO_DITHER: u64 = 0x00_00C0_0000_0000; // Set_Other_Modes V2: RGB Dither Selection No Dither (Bit 38..39)
pub const OTHER_MODE_KEY_EN: u64 = 0x00_0100_0000_0000; // Set_Other_Modes W: Enables Chroma Keying (Bit 40)
pub const OTHER_MODE_CONVERT_ONE: u64 = 0x00_0200_0000_0000; // Set_Other_Modes X: Color Convert Texel That Was The Ouput Of The Texture Filter On Cycle0, Used To Qualify BI_LERP_1 (Bit 41)
pub const OTHER_MODE_BI_LERP_1: u64 = 0x00_0400_0000_0000; // Set_Other_Modes Y: 1=BI_LERP, 0=Color Convert Operation In Texture Filter. Used In Cycle 1 (Bit 42)
pub const OTHER_MODE_BI_LERP_0: u64 = 0x00_0800_0000_0000; // Set_Other_Modes Z: 1=BI_LERP, 0=Color Convert Operation In Texture Filter. Used In Cycle 0 (Bit 43)
pub const OTHER_MODE_MID_TEXEL: u64 = 0x00_1000_0000_0000; // Set_Other_Modes a: Indicates Texture Filter Should Do A 2x2 Half Texel Interpolation, Primarily Used For MPEG Motion Compensation Processing (Bit 44)
pub const OTHER_MODE_SAMPLE_TYPE: u64 = 0x00_2000_0000_0000; // Set_Other_Modes b: Determines How Textures Are Sampled: 0=1x1 (Point Sample), 1=2x2. Note That Copy (Point Sample 4 Horizontally Adjacent Texels) Mode Is Indicated By CYCLE_TYPE (Bit 45)
pub const OTHER_MODE_TLUT_TYPE: u64 = 0x00_4000_0000_0000; // Set_Other_Modes c: Type Of Texels In Table, 0=16b RGBA(5/5/5/1), 1=IA(8/8) (Bit 46)
pub const OTHER_MODE_EN_TLUT: u64 = 0x00_8000_0000_0000; // Set_Other_Modes d: Enable Lookup Of Texel Values From TLUT. Meaningful If Texture Type Is Index, Tile Is In Low TMEM, TLUT Is In High TMEM, And Color Image Is RGB (Bit 47)
pub const OTHER_MODE_TEX_LOD_EN: u64 = 0x01_0000_0000_0000; // Set_Other_Modes e: Enable Texture Level Of Detail (LOD) (Bit 48)
pub const OTHER_MODE_SHARPEN_TEX_EN: u64 = 0x02_0000_0000_0000; // Set_Other_Modes f: Enable Sharpened Texture (Bit 49)
pub const OTHER_MODE_DETAIL_TEX_EN: u64 = 0x04_0000_0000_0000; // Set_Other_Modes g: Enable Detail Texture (Bit 50)
pub const OTHER_MODE_PERSP_TEX_EN: u64 = 0x08_0000_0000_0000; // Set_Other_Modes h: Enable Perspective Correction On Texture (Bit 51)
pub const OTHER_MODE_CYCLE_TYPE_1_CYCLE: u64 = 0x00_0000_0000_0000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode 1 Cycle (Bit 52..53)
pub const OTHER_MODE_CYCLE_TYPE_2_CYCLE: u64 = 0x10_0000_0000_0000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode 2 Cycle (Bit 52..53)
pub const OTHER_MODE_CYCLE_TYPE_COPY: u64 = 0x20_0000_0000_0000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode Copy (Bit 52..53)
pub const OTHER_MODE_CYCLE_TYPE_FILL: u64 = 0x30_0000_0000_0000; // Set_Other_Modes i: Display Pipeline Cycle Control Mode Fill (Bit 52..53)
pub const OTHER_MODE_ATOMIC_PRIM: u64 = 0x80_0000_0000_0000; // Set_Other_Modes k: Force Primitive To Be Written To Frame Buffer Before Read Of Following

pub const SIZE_OF_PIXEL_4B: u8 = 0; // Set_Tile/Set_Texture_Image/Set_Color_Image: Size Of Pixel/Texel Color Element 4B (Bit 51..52)
pub const SIZE_OF_PIXEL_8B: u8 = 1; // Set_Tile/Set_Texture_Image/Set_Color_Image: Size Of Pixel/Texel Color Element 8B (Bit 51..52)
pub const SIZE_OF_PIXEL_16B: u8 = 2; // Set_Tile/Set_Texture_Image/Set_Color_Image: Size Of Pixel/Texel Color Element 16B (Bit 51..52)
pub const SIZE_OF_PIXEL_32B: u8 = 3; // Set_Tile/Set_Texture_Image/Set_Color_Image: Size Of Pixel/Texel Color Element 32B (Bit 51..52)
pub const FORMAT_RGBA: u8 = 0; // Set_Tile/Set_Texture_Image/Set_Color_Image: Image Data Format RGBA (Bit 53..55)
pub const FORMAT_YUV: u8 = 1; // Set_Tile/Set_Texture_Image/Set_Color_Image: Image Data Format YUV (Bit 53..55)
pub const FORMAT_COLOR_INDX: u8 = 2; // Set_Tile/Set_Texture_Image/Set_Color_Image: Image Data Format COLOR_INDX (Bit 53..55)
pub const FORMAT_IA: u8 = 3; // Set_Tile/Set_Texture_Image/Set_Color_Image: Image Data Format IA (Bit 53..55)
pub const FORMAT_I: u8 = 4; // Set_Tile/Set_Texture_Image/Set_Color_Image: Image Data Format I (Bit 53..55)

pub const COMMAND_SET_COLOR_IMAGE: u64 = 0xff;
pub const COMMAND_SET_Z_IMAGE: u64 = 0xfe;
pub const COMMAND_SET_TEXTURE_IMAGE: u64 = 0xfd;
pub const COMMAND_SET_COMBINE_MODE: u64 = 0xfc;
pub const COMMAND_SET_ENV_COLOR: u64 = 0xfb;
pub const COMMAND_SET_PRIM_COLOR: u64 = 0xfa;
pub const COMMAND_SET_BLEND_COLOR: u64 = 0xf9;
pub const COMMAND_SET_FOG_COLOR: u64 = 0xf8;
pub const COMMAND_SET_FILL_COLOR: u64 = 0xf7;
pub const COMMAND_FILL_RECTANGLE: u64 = 0xf6;
pub const COMMAND_SET_TILE: u64 = 0xf5;
pub const COMMAND_LOAD_TILE: u64 = 0xf4;
pub const COMMAND_SET_OTHER_MODE: u64 = 0xef;
pub const COMMAND_SET_SCISSOR: u64 = 0xed;
pub const COMMAND_SYNC_FULL: u64 = 0xe9;
pub const COMMAND_SYNC_TILE: u64 = 0xe8;
pub const COMMAND_SYNC_PIPE: u64 = 0xe7;
pub const COMMAND_TEXTURE_RECTANGLE: u64 = 0xe4;
pub const COMMAND_EDGE_COEFFICIENTS: u64 = 0xc8;

pub struct RdpCommandBuilder {
    pub(crate) blocks: Vec<RdpBlock>,
    block_index: usize,
    index: usize,
}

impl RdpCommandBuilder {
    #[inline]
    pub fn new() -> RdpCommandBuilder {
        RdpCommandBuilder {
            blocks: Vec::with_capacity(32),
            block_index: 0,
            index: 0,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.blocks.clear();
        self.blocks.push(RdpBlock::default());
        self.index = 0;
    }

    #[inline]
    fn push(&mut self, command: RdpCommand) {
        if self.index == 128 {
            self.blocks.push(RdpBlock::default());
            self.index = 0;
        }

        self.blocks.last_mut().unwrap().rdp_data[self.index] = command;
        self.index += 1;
    }

    #[inline]
    pub fn set_color_image(
        &mut self,
        format: u8,
        size: u8,
        width: u16,
        image: *mut u16,
    ) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(
            (COMMAND_SET_COLOR_IMAGE << 56)
                | (((format & 0b111) as u64) << 53)
                | (((size & 0b11) as u64) << 51)
                | ((width as u64 - 1) << 32)
                | virtual_to_physical_mut(image) as u64,
        ));

        self
    }

    #[inline]
    pub fn set_z_image(&mut self, image: *mut u16) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(
            (COMMAND_SET_Z_IMAGE << 56) | virtual_to_physical_mut(image) as u64,
        ));

        self
    }

    #[inline]
    pub fn set_texture_image(
        &mut self,
        format: u8,
        size: u8,
        width: u16,
        image: *const u16,
    ) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(
            (COMMAND_SET_TEXTURE_IMAGE << 56)
                | (((format & 0b111) as u64) << 53)
                | (((size & 0b11) as u64) << 51)
                | ((width as u64 - 1) << 32)
                | (virtual_to_physical(image) as u64),
        ));
        self
    }

    #[inline]
    pub fn set_combine_mode(&mut self, value: u64) -> &mut RdpCommandBuilder {
        self.push(RdpCommand((COMMAND_SET_COMBINE_MODE << 56) | value));
        self
    }

    #[inline]
    pub fn set_env_color(&mut self, color: u32) -> &mut RdpCommandBuilder {
        self.push(RdpCommand((COMMAND_SET_ENV_COLOR << 56) | (color as u64)));
        self
    }

    #[inline]
    pub fn set_prim_color(&mut self, color: u32) -> &mut RdpCommandBuilder {
        self.push(RdpCommand((COMMAND_SET_PRIM_COLOR << 56) | (color as u64)));
        self
    }

    #[inline]
    pub fn set_blend_color(&mut self, color: u32) -> &mut RdpCommandBuilder {
        self.push(RdpCommand((COMMAND_SET_BLEND_COLOR << 56) | (color as u64)));
        self
    }

    #[inline]
    pub fn set_fog_color(&mut self, color: u32) -> &mut RdpCommandBuilder {
        self.push(RdpCommand((COMMAND_SET_FOG_COLOR << 56) | (color as u64)));
        self
    }

    #[inline]
    pub fn set_fill_color(&mut self, color: Color) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(
            (COMMAND_SET_FILL_COLOR << 56)
                | ((color.value() as u64) << 16)
                | (color.value() as u64),
        ));
        self
    }

    #[inline]
    pub fn fill_rectangle(&mut self, top_left: Vec2, bottom_right: Vec2) -> &mut RdpCommandBuilder {
        let mut l = top_left.x;
        let mut t = top_left.y;
        let r = bottom_right.x;
        let b = bottom_right.y;

        if r < 0.0 || b < 0.0 {
            // Outside drawing area.
            return self;
        }

        if l < 0.0 {
            l = 0.0;
        }

        if t < 0.0 {
            t = 0.0;
        }

        self.push(RdpCommand(
            (COMMAND_FILL_RECTANGLE << 56)
                | (to_fixpoint_10_2_as_integer(r) << (32 + 12))
                | (to_fixpoint_10_2_as_integer(b) << 32)
                | (to_fixpoint_10_2_as_integer(l) << 12)
                | (to_fixpoint_10_2_as_integer(t)),
        ));

        self
    }

    #[inline]
    pub fn set_tile(
        &mut self,
        format: u8,
        size: u8,
        width: u16,
        texture_cache_start_address: u16,
        tile_index: u8,
        clamp_t: u8,
        mirror_t: u8,
        mask_t: u8,
        shift_t: u8,
        clamp_s: u8,
        mirror_s: u8,
        mask_s: u8,
        shift_s: u8,
    ) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(
            (COMMAND_SET_TILE << 56)
                | (((format & 0b111) as u64) << 53)
                | (((size & 0b11) as u64) << 51)
                | (((width >> 2) as u64) << 41)
                | ((texture_cache_start_address as u64) << 32)
                | ((tile_index as u64) << 24)
                | ((clamp_t as u64) << 19)
                | ((mirror_t as u64) << 18)
                | ((mask_t as u64) << 14)
                | ((shift_t as u64) << 10)
                | ((clamp_s as u64) << 9)
                | ((mirror_s as u64) << 8)
                | ((mask_s as u64) << 4)
                | (shift_s as u64),
        ));
        self
    }

    #[inline]
    pub fn load_tile(
        &mut self,
        top_left: Vec2,
        bottom_right: Vec2,
        tile_index: u8,
    ) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(
            (COMMAND_LOAD_TILE << 56)
                | (to_fixpoint_10_2_as_integer(bottom_right.x) << (32 + 12))
                | (to_fixpoint_10_2_as_integer(bottom_right.y) << 32)
                | ((tile_index as u64) << 24)
                | (to_fixpoint_10_2_as_integer(top_left.x) << 12)
                | (to_fixpoint_10_2_as_integer(top_left.y)),
        ));
        self
    }

    #[inline]
    pub fn set_other_modes(&mut self, flags: u64) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(
            (COMMAND_SET_OTHER_MODE << 56) | (flags & ((1 << 56) - 1)) | 0x0000_000F_0000_0000,
        ));
        self
    }

    #[inline]
    pub fn set_scissor(&mut self, top_left: Vec2, bottom_right: Vec2) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(
            (COMMAND_SET_SCISSOR << 56)
                | (to_fixpoint_10_2_as_integer(top_left.x) << (32 + 12))
                | (to_fixpoint_10_2_as_integer(top_left.y) << 32)
                | (to_fixpoint_10_2_as_integer(bottom_right.x) << 12)
                | (to_fixpoint_10_2_as_integer(bottom_right.y)),
        ));

        self
    }

    #[inline]
    pub fn sync_full(&mut self) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(COMMAND_SYNC_FULL << 56));
        self
    }

    #[inline]
    pub fn sync_tile(&mut self) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(COMMAND_SYNC_TILE << 56));
        self
    }

    #[inline]
    pub fn sync_pipe(&mut self) -> &mut RdpCommandBuilder {
        self.push(RdpCommand(COMMAND_SYNC_PIPE << 56));
        self
    }

    #[inline]
    pub fn edge_coefficients(
        &mut self,
        shade: bool,
        texture: bool,
        z_buffer: bool,
        right_major: bool,
        _level: u8,
        _tile: u8,
        y_low_minor: f32,
        y_mid_minor: f32,
        y_high_major: f32,
        x_low_int: u16,
        x_low_frac: u16,
        x_mid_int: u16,
        x_mid_frac: u16,
        x_high_int: u16,
        x_high_frac: u16,
        inv_slope_low: i32,
        inv_slope_mid: i32,
        inv_slope_high: i32,
    ) -> &mut RdpCommandBuilder {
        let mut command = COMMAND_EDGE_COEFFICIENTS;
        if shade {
            command |= 0x4;
        }
        if texture {
            command |= 0x2;
        }
        if z_buffer {
            command |= 0x1;
        }

        self.push(RdpCommand(
            (command << 56)
                | if right_major { 0x1u64 << 55 } else { 0u64 }
                | (to_fixpoint_s_11_2(y_low_minor)) << 32
                | (to_fixpoint_s_11_2(y_mid_minor)) << 16
                | to_fixpoint_s_11_2(y_high_major),
        ));

        // SIC Should be L, H, M order
        self.push(RdpCommand(
            (x_low_int as u64) << 48 | (x_low_frac as u64) << 32 | (inv_slope_low as u32 as u64),
        ));
        self.push(RdpCommand(
            (x_high_int as u64) << 48 | (x_high_frac as u64) << 32 | (inv_slope_high as u32 as u64),
        ));
        self.push(RdpCommand(
            (x_mid_int as u64) << 48 | (x_mid_frac as u64) << 32 | (inv_slope_mid as u32 as u64),
        ));

        self
    }

    #[inline]
    pub fn shade_coefficients(
        &mut self,
        red: i32,
        green: i32,
        blue: i32,
        alpha: i32,
        dr_dx: i32,
        dg_dx: i32,
        db_dx: i32,
        da_dx: i32,
        dr_de: i32,
        dg_de: i32,
        db_de: i32,
        da_de: i32,
        dr_dy: i32,
        dg_dy: i32,
        db_dy: i32,
        da_dy: i32,
    ) -> &mut RdpCommandBuilder {
        // Color
        // Delta Color X
        // Color fraction
        // Delta Color X fraction
        self.push(RdpCommand(
            ((red >> 16) as u16 as u64) << 48
                | ((green >> 16) as u16 as u64) << 32
                | ((blue >> 16) as u16 as u64) << 16
                | ((alpha >> 16) as u16 as u64),
        ));
        self.push(RdpCommand(
            ((dr_dx >> 16) as u16 as u64) << 48
                | ((dg_dx >> 16) as u16 as u64) << 32
                | ((db_dx >> 16) as u16 as u64) << 16
                | ((da_dx >> 16) as u16 as u64),
        ));
        self.push(RdpCommand(
            ((red & 0x0000ffff) as u16 as u64) << 48
                | ((green & 0x0000ffff) as u16 as u64) << 32
                | ((blue & 0x0000ffff) as u16 as u64) << 16
                | ((alpha & 0x0000ffff) as u16 as u64),
        ));
        self.push(RdpCommand(
            ((dr_dx & 0x0000ffff) as u16 as u64) << 48
                | ((dg_dx & 0x0000ffff) as u16 as u64) << 32
                | ((db_dx & 0x0000ffff) as u16 as u64) << 16
                | ((da_dx & 0x0000ffff) as u16 as u64),
        ));

        // Delta Color Edge
        // Delta Color Y
        // Delta Color Edge fraction
        // Delta Color Y fraction
        self.push(RdpCommand(
            ((dr_de >> 16) as u16 as u64) << 48
                | ((dg_de >> 16) as u16 as u64) << 32
                | ((db_de >> 16) as u16 as u64) << 16
                | ((da_de >> 16) as u16 as u64),
        ));
        self.push(RdpCommand(
            ((dr_dy >> 16) as u16 as u64) << 48
                | ((dg_dy >> 16) as u16 as u64) << 32
                | ((db_dy >> 16) as u16 as u64) << 16
                | ((da_dy >> 16) as u16 as u64),
        ));
        self.push(RdpCommand(
            ((dr_de & 0x0000ffff) as u16 as u64) << 48
                | ((dg_de & 0x0000ffff) as u16 as u64) << 32
                | ((db_de & 0x0000ffff) as u16 as u64) << 16
                | ((da_de & 0x0000ffff) as u16 as u64),
        ));
        self.push(RdpCommand(
            ((dr_dy & 0x0000ffff) as u16 as u64) << 48
                | ((dg_dy & 0x0000ffff) as u16 as u64) << 32
                | ((db_dy & 0x0000ffff) as u16 as u64) << 16
                | ((da_dy & 0x0000ffff) as u16 as u64),
        ));

        self
    }

    #[inline]
    pub fn z_buffer_coefficients(
        &mut self,
        z: i32,
        dx: i32,
        de: i32,
        dy: i32,
    ) -> &mut RdpCommandBuilder {
        self.push(RdpCommand((z as u32 as u64) << 32 | (dx as u32 as u64)));
        self.push(RdpCommand((de as u32 as u64) << 32 | (dy as u32 as u64)));

        self
    }

    #[inline]
    pub fn texture_rectangle(
        &mut self,
        top_left: Vec2,
        bottom_right: Vec2,
        tile_index: u8,
        st_top_left: Vec2,
        d_xy_d_st: Vec2,
    ) -> &mut RdpCommandBuilder {
        let mut l = top_left.x;
        let mut t = top_left.y;
        let r = bottom_right.x;
        let b = bottom_right.y;

        if r < 0.0 || b < 0.0 {
            // Outside drawing area.
            return self;
        }

        let mut st_l = st_top_left.x;
        let mut st_t = st_top_left.y;

        if l < 0.0 {
            st_l -= l;
            l = 0.0;
        }

        if t < 0.0 {
            st_t -= t;
            t = 0.0;
        }

        self.push(RdpCommand(
            (COMMAND_TEXTURE_RECTANGLE << 56)
                | (to_fixpoint_10_2_as_integer(r) << (32 + 12))
                | (to_fixpoint_10_2_as_integer(b) << 32)
                | ((tile_index as u64) << 24)
                | (to_fixpoint_10_2_as_integer(l) << 12)
                | (to_fixpoint_10_2_as_integer(t)),
        ));

        self.push(RdpCommand(
            (to_fixpoint_s_10_5(st_l) << 48)
                | (to_fixpoint_s_10_5(st_t) << 32)
                | (to_fixpoint_s_10_5(d_xy_d_st.x) << 16)
                | to_fixpoint_s_10_5(d_xy_d_st.y),
        ));
        self
    }
}
