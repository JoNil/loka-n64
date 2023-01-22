#![cfg_attr(not(feature = "record_asm"), no_std)]

use core::fmt::{self, Display};
use parse_display::FromStr;
use vu_emu_macro::emit_asm;

pub mod instruction;

#[derive(Clone)]
pub struct Vu {
    pub registers: [[u8; 16]; 32],
    pub acc: [[u8; 6]; 8],
    #[cfg(feature = "record_asm")]
    pub asm: String,
}

impl Display for Vu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Registers: [")?;
        for (i, reg) in self.registers.iter().enumerate() {
            let value = u128::from_be_bytes(*reg);

            write!(f, "    {:2}:", i)?;
            for j in 0..8 {
                let v = &value.to_be_bytes()[(2 * j)..=(2 * j + 1)];
                let v = u16::from_be_bytes(v.try_into().unwrap());

                if v == 0 {
                    write!(f, "     ")?;
                } else {
                    write!(f, " {:4x}", v)?;
                }
            }

            write!(f, " |")?;

            for j in 0..8 {
                let v = &value.to_be_bytes()[(2 * j)..=(2 * j + 1)];
                let v = u16::from_be_bytes(v.try_into().unwrap());

                if v == 0 {
                    write!(f, "      ")?;
                } else {
                    write!(f, " {:5}", v)?;
                }
            }

            writeln!(f, "")?;
        }
        writeln!(f, "]",)?;

        write!(f, "Acc: [")?;

        for e in 0..8 {
            let acc = u32::from_be_bytes(self.acc[e][2..6].try_into().unwrap());
            if e != 7 {
                write!(f, "{}, ", acc as f64 / (1 << 16) as f64)?;
            } else {
                write!(f, "{}", acc as f64 / (1 << 16) as f64)?;
            }
        }

        writeln!(f, "]",)?;

        Ok(())
    }
}

impl Vu {
    pub fn new() -> Self {
        Self {
            registers: [[0; 16]; 32],
            acc: [[0; 6]; 8],
            #[cfg(feature = "record_asm")]
            asm: String::new(),
        }
    }

    pub fn load_fix_point(&mut self, rl: Reg, rh: Reg, v: u32) {
        assert!(rh.1.unwrap() == rl.1.unwrap());

        let low = v & 0xffff;
        let high = v >> 16;

        self.lsv(rl, low as u16);
        self.lsv(rh, high as u16);
    }

    pub fn set_reg(&mut self, r: Reg, value: [u8; 16]) {
        for i in 0..8 {
            let j = match r.1 {
                None => i,
                Some(e) => {
                    if e == 0b000 {
                        i
                    } else if (e & 0b1110) == 0b0010 {
                        (e & 0b0001) + (i & 0b1110)
                    } else if (e & 0b1100) == 0b0100 {
                        (e & 0b0011) + (i & 0b1100)
                    } else if (e & 0b1000) == 0b1000 {
                        e & 0b0111
                    } else {
                        panic!("Invalid e");
                    }
                }
            };

            self.registers[r.0][(i * 2)..=(i * 2 + 1)]
                .copy_from_slice(&value[(j * 2)..=(j * 2 + 1)])
        }
    }

    pub fn lbv(&mut self, r: Reg, byte: u8) {
        let i = r.1.unwrap();
        self.registers[r.0][i] = byte;
    }

    pub fn lsv(&mut self, r: Reg, short: u16) {
        let i = r.1.unwrap();
        assert!(i % 2 == 0);
        assert!(i <= 14);
        self.registers[r.0][i..=(i + 1)].copy_from_slice(&short.to_be_bytes());
    }

    fn read_vs_vt_element(&self, vs: Reg, vt: Reg, i: usize) -> (u16, u16) {
        assert!(vs.1.is_none());

        let j = match vt.1 {
            None => i,
            Some(e) => {
                if e == 0b000 {
                    i
                } else if (e & 0b1110) == 0b0010 {
                    (e & 0b0001) + (i & 0b1110)
                } else if (e & 0b1100) == 0b0100 {
                    (e & 0b0011) + (i & 0b1100)
                } else if (e & 0b1000) == 0b1000 {
                    e & 0b0111
                } else {
                    panic!("Invalid e");
                }
            }
        };

        let vs_data = u16::from_be_bytes(
            self.registers[vs.0][(i * 2)..=(i * 2 + 1)]
                .try_into()
                .unwrap(),
        );
        let vt_data = u16::from_be_bytes(
            self.registers[vt.0][(j * 2)..=(j * 2 + 1)]
                .try_into()
                .unwrap(),
        );

        (vs_data, vt_data)
    }

    #[emit_asm]
    pub fn vadd(&mut self, vd: Reg, vs: Reg, vt: Reg) {
        for i in 0..8 {
            let (vs_data, vt_data) = self.read_vs_vt_element(vs, vt, i);

            // TODO: Carry VCO
            let sum = (vs_data as i32 + vt_data as i32) as i16;

            self.registers[vd.0][(i * 2)..=(i * 2 + 1)]
                .copy_from_slice(&(sum as u16).to_be_bytes());
        }
    }

    #[emit_asm]
    pub fn vsub(&mut self, vd: Reg, vs: Reg, vt: Reg) {
        for i in 0..8 {
            let (vs_data, vt_data) = self.read_vs_vt_element(vs, vt, i);

            // TODO: Carry VCO
            let diff = (vs_data as i32 - vt_data as i32) as i16;

            self.registers[vd.0][(i * 2)..=(i * 2 + 1)]
                .copy_from_slice(&(diff as u16).to_be_bytes());
        }
    }

    #[emit_asm]
    pub fn vmudl(&mut self, vd: Reg, vs: Reg, vt: Reg) {
        for i in 0..8 {
            let (vs_data, vt_data) = self.read_vs_vt_element(vs, vt, i);

            let product = vs_data as u32 * vt_data as u32;

            let acc = (product >> 16) as i16 as u32;
            self.acc[i][2..6].copy_from_slice(&acc.to_be_bytes());

            self.registers[vd.0][(i * 2)..=(i * 2 + 1)]
                .copy_from_slice(&((acc & 0xffff) as u16).to_be_bytes());
        }
    }

    #[emit_asm]
    pub fn vmadm(&mut self, vd: Reg, vs: Reg, vt: Reg) {
        for i in 0..8 {
            let (vs_data, vt_data) = self.read_vs_vt_element(vs, vt, i);

            let product = vs_data as u32 * vt_data as u32;
            let acc = u32::from_be_bytes(self.acc[i][2..6].try_into().unwrap());

            let acc = acc + product;
            self.acc[i][2..6].copy_from_slice(&acc.to_be_bytes());

            self.registers[vd.0][(i * 2)..=(i * 2 + 1)]
                .copy_from_slice(&((acc >> 16) as u16).to_be_bytes());
        }
    }

    #[emit_asm]
    pub fn vmadn(&mut self, vd: Reg, vs: Reg, vt: Reg) {
        for i in 0..8 {
            let (vs_data, vt_data) = self.read_vs_vt_element(vs, vt, i);

            let product = vs_data as u32 * vt_data as u32;
            let acc = u32::from_be_bytes(self.acc[i][2..6].try_into().unwrap());

            let acc = acc + product;
            self.acc[i][2..6].copy_from_slice(&acc.to_be_bytes());

            self.registers[vd.0][(i * 2)..=(i * 2 + 1)]
                .copy_from_slice(&((acc & 0xffff) as u16).to_be_bytes());
        }
    }

    #[emit_asm]
    pub fn vmadh(&mut self, vd: Reg, vs: Reg, vt: Reg) {
        for i in 0..8 {
            let (vs_data, vt_data) = self.read_vs_vt_element(vs, vt, i);

            let product = vs_data as u32 * vt_data as u32;
            let acc = u32::from_be_bytes(self.acc[i][2..6].try_into().unwrap());

            let acc = acc + (product << 16);
            self.acc[i][2..6].copy_from_slice(&acc.to_be_bytes());

            self.registers[vd.0][(i * 2)..=(i * 2 + 1)]
                .copy_from_slice(&((acc >> 16) as u16).to_be_bytes());
        }
    }

    #[cfg(feature = "record_asm")]
    pub fn asm(&self) -> &str {
        &self.asm
    }
}

impl Default for Vu {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, FromStr)]
#[display("v{0}")]
pub struct Reg(usize, #[from_str(default)] Option<usize>);

impl Reg {
    pub fn e0(self) -> Self {
        Reg(self.0, Some(0))
    }

    pub fn e1(self) -> Self {
        Reg(self.0, Some(1))
    }

    pub fn e2(self) -> Self {
        Reg(self.0, Some(2))
    }

    pub fn e3(self) -> Self {
        Reg(self.0, Some(3))
    }

    pub fn e4(self) -> Self {
        Reg(self.0, Some(4))
    }

    pub fn e5(self) -> Self {
        Reg(self.0, Some(5))
    }

    pub fn e6(self) -> Self {
        Reg(self.0, Some(6))
    }

    pub fn e7(self) -> Self {
        Reg(self.0, Some(7))
    }

    pub fn e8(self) -> Self {
        Reg(self.0, Some(8))
    }

    pub fn e9(self) -> Self {
        Reg(self.0, Some(9))
    }

    pub fn e10(self) -> Self {
        Reg(self.0, Some(10))
    }

    pub fn e11(self) -> Self {
        Reg(self.0, Some(11))
    }

    pub fn e12(self) -> Self {
        Reg(self.0, Some(12))
    }

    pub fn e13(self) -> Self {
        Reg(self.0, Some(13))
    }

    pub fn e14(self) -> Self {
        Reg(self.0, Some(14))
    }

    pub fn e15(self) -> Self {
        Reg(self.0, Some(15))
    }
}

impl core::fmt::Display for Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Reg(reg, None) => write!(f, "v{reg}"),
            Reg(reg, Some(e)) => write!(f, "v{reg}[e{e}]"),
        }
    }
}

pub mod regs {
    #![allow(non_upper_case_globals)]

    use crate::Reg;

    pub const v0: Reg = Reg(0, None);
    pub const v1: Reg = Reg(1, None);
    pub const v2: Reg = Reg(2, None);
    pub const v3: Reg = Reg(3, None);
    pub const v4: Reg = Reg(4, None);
    pub const v5: Reg = Reg(5, None);
    pub const v6: Reg = Reg(6, None);
    pub const v7: Reg = Reg(7, None);
    pub const v8: Reg = Reg(8, None);
    pub const v9: Reg = Reg(9, None);
    pub const v10: Reg = Reg(10, None);
    pub const v11: Reg = Reg(11, None);
    pub const v12: Reg = Reg(12, None);
    pub const v13: Reg = Reg(13, None);
    pub const v14: Reg = Reg(14, None);
    pub const v15: Reg = Reg(15, None);
    pub const v16: Reg = Reg(16, None);
    pub const v17: Reg = Reg(17, None);
    pub const v18: Reg = Reg(18, None);
    pub const v19: Reg = Reg(19, None);
    pub const v20: Reg = Reg(20, None);
    pub const v21: Reg = Reg(21, None);
    pub const v22: Reg = Reg(22, None);
    pub const v23: Reg = Reg(23, None);
    pub const v24: Reg = Reg(24, None);
    pub const v25: Reg = Reg(25, None);
    pub const v26: Reg = Reg(26, None);
    pub const v27: Reg = Reg(27, None);
    pub const v28: Reg = Reg(28, None);
    pub const v29: Reg = Reg(29, None);
    pub const v30: Reg = Reg(30, None);
    pub const v31: Reg = Reg(31, None);
}
