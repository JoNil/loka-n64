use vu_emu::{Vu, V1, V2, V3, V5, V6};

fn main() {
    let mut vu = Vu::new();

    vu.vmudl(V5, V1, V3);
    vu.vmadm(V6, V2, V3);

    println!("{}", vu.asm());
}
