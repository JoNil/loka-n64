pub struct Vu {
    registers: [[u8; 16]; 32],
    acc: [[u8; 6]; 8],
}

impl Vu {
    pub fn new() -> Self {
        Self {
            registers: [[0; 16]; 32],
            acc: [[0; 6]; 8],
        }
    }

    pub fn vmudl(&mut self, vd: Reg, vs: Reg, vt: Reg) {
        for i in 0..7 {
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
                self.registers[vs.0][(i * 2)..(i * 2 + 1)]
                    .try_into()
                    .unwrap(),
            );
            let vt_data = u16::from_be_bytes(
                self.registers[vt.0][(j * 2)..(j * 2 + 1)]
                    .try_into()
                    .unwrap(),
            );

            let product = vs_data as u32 * vt_data as u32;

            let acc = (product >> 16) as i16 as u32;
            self.acc[i][2..6].copy_from_slice(&acc.to_be_bytes());

            self.registers[vd.0][(i * 2)..(i * 2 + 1)]
                .copy_from_slice(&((acc & 0xffff) as u16).to_be_bytes());
        }
    }

    pub fn vmadm(&mut self, vd: Reg, vs: Reg, vt: Reg) {
        for i in 0..7 {
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
                self.registers[vs.0][(i * 2)..(i * 2 + 1)]
                    .try_into()
                    .unwrap(),
            );
            let vt_data = u16::from_be_bytes(
                self.registers[vt.0][(j * 2)..(j * 2 + 1)]
                    .try_into()
                    .unwrap(),
            );

            let product = vs_data as u32 * vt_data as u32;
            let acc = u32::from_be_bytes(self.acc[i][2..6].try_into().unwrap());

            let acc = acc + product;
            self.acc[i][2..6].copy_from_slice(&acc.to_be_bytes());

            self.registers[vd.0][(i * 2)..(i * 2 + 1)]
                .copy_from_slice(&((acc >> 16) as u16).to_be_bytes());
        }
    }
}

impl Default for Vu {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone)]
pub struct Reg(usize, Option<usize>);

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

pub const V0: Reg = Reg(0, None);
pub const V1: Reg = Reg(1, None);
pub const V2: Reg = Reg(2, None);
pub const V3: Reg = Reg(3, None);
pub const V4: Reg = Reg(4, None);
pub const V5: Reg = Reg(5, None);
pub const V6: Reg = Reg(6, None);
pub const V7: Reg = Reg(7, None);
pub const V8: Reg = Reg(8, None);
pub const V9: Reg = Reg(9, None);
pub const V10: Reg = Reg(10, None);
pub const V11: Reg = Reg(11, None);
pub const V12: Reg = Reg(12, None);
pub const V13: Reg = Reg(13, None);
pub const V14: Reg = Reg(14, None);
pub const V15: Reg = Reg(15, None);
pub const V16: Reg = Reg(16, None);
pub const V17: Reg = Reg(17, None);
pub const V18: Reg = Reg(18, None);
pub const V19: Reg = Reg(19, None);
pub const V20: Reg = Reg(20, None);
pub const V21: Reg = Reg(21, None);
pub const V22: Reg = Reg(22, None);
pub const V23: Reg = Reg(23, None);
pub const V24: Reg = Reg(24, None);
pub const V25: Reg = Reg(25, None);
pub const V26: Reg = Reg(26, None);
pub const V27: Reg = Reg(27, None);
pub const V28: Reg = Reg(28, None);
pub const V29: Reg = Reg(29, None);
pub const V30: Reg = Reg(30, None);
pub const V31: Reg = Reg(31, None);
