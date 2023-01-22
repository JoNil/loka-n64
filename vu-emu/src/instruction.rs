use parse_display::FromStr;

use crate::{Reg, Vu};

#[derive(Copy, Clone, FromStr)]
pub enum Instruction {
    #[display("vmudl v{0},v{1},v{2}")]
    Vmudl(usize, usize, usize),
    #[display("vmadm v{0},v{1},v{2}")]
    Vmadm(usize, usize, usize),
    #[display("vmadn v{0},v{1},v{2}")]
    Vmadn(usize, usize, usize),
    #[display("vmadh v{0},v{1},v{2}")]
    Vmadh(usize, usize, usize),
    #[display("vmudl v{0},v{1},v{2}[e{3}]")]
    VmudlE(usize, usize, usize, usize),
    #[display("vmadm v{0},v{1},v{2}[e{3}]")]
    VmadmE(usize, usize, usize, usize),
    #[display("vmadn v{0},v{1},v{2}[e{3}]")]
    VmadnE(usize, usize, usize, usize),
    #[display("vmadh v{0},v{1},v{2}[e{3}]")]
    VmadhE(usize, usize, usize, usize),
}

pub fn execute(vu: &mut Vu, instruction: Instruction) {
    match instruction {
        Instruction::Vmudl(vd, vs, vt) => vu.vmudl(Reg(vd & 0x1f, None), Reg(vs & 0x1f, None), Reg(vt & 0x1f, None)),
        Instruction::Vmadm(vd, vs, vt) => vu.vmadm(Reg(vd & 0x1f, None), Reg(vs & 0x1f, None), Reg(vt & 0x1f, None)),
        Instruction::Vmadn(vd, vs, vt) => vu.vmadn(Reg(vd & 0x1f, None), Reg(vs & 0x1f, None), Reg(vt & 0x1f, None)),
        Instruction::Vmadh(vd, vs, vt) => vu.vmadh(Reg(vd & 0x1f, None), Reg(vs & 0x1f, None), Reg(vt & 0x1f, None)),
        Instruction::VmudlE(vd, vs, vt, et) => vu.vmadh(Reg(vd & 0x1f, None), Reg(vs & 0x1f, None), Reg(vt & 0x1f, Some(et))),
        Instruction::VmadmE(vd, vs, vt, et) => vu.vmadh(Reg(vd & 0x1f, None), Reg(vs & 0x1f, None), Reg(vt & 0x1f, Some(et))),
        Instruction::VmadnE(vd, vs, vt, et) => vu.vmadh(Reg(vd & 0x1f, None), Reg(vs & 0x1f, None), Reg(vt & 0x1f, Some(et))),
        Instruction::VmadhE(vd, vs, vt, et) => vu.vmadh(Reg(vd & 0x1f, None), Reg(vs & 0x1f, None), Reg(vt & 0x1f, Some(et))),
    }
}

pub fn parse(input: &str) -> Option<Instruction> {
    input.parse().ok()
}
