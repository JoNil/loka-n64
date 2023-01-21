use parse_display::FromStr;

use crate::{Reg, Vu};

#[derive(Copy, Clone, FromStr)]
pub enum Instruction {
    #[display("vmudl {0},{1},{2}")]
    Vmudl(Reg, Reg, Reg),
    #[display("vmadm {0},{1},{2}")]
    Vmadm(Reg, Reg, Reg),
    #[display("vmadn {0},{1},{2}")]
    Vmadn(Reg, Reg, Reg),
    #[display("vmadh {0},{1},{2}")]
    Vmadh(Reg, Reg, Reg),
}

pub fn execute(vu: &mut Vu, instruction: Instruction) {
    match instruction {
        Instruction::Vmudl(vd, vs, vt) => vu.vmudl(vd, vs, vt),
        Instruction::Vmadm(vd, vs, vt) => vu.vmadm(vd, vs, vt),
        Instruction::Vmadn(vd, vs, vt) => vu.vmadn(vd, vs, vt),
        Instruction::Vmadh(vd, vs, vt) => vu.vmadh(vd, vs, vt),
    }
}

pub fn parse(input: &str) -> Option<Instruction> {
    input.parse().ok()
}
