use vu_emu::{regs::*, Vu};

fn main() {
    let mut vu = Vu::new();

    vu.load_fix_point(v1.e0(), v2.e0(), 3 << 15);
    vu.load_fix_point(v1.e2(), v2.e2(), 5 << 15);
    vu.load_fix_point(v1.e4(), v2.e4(), 7 << 15);
    vu.load_fix_point(v1.e6(), v2.e6(), 9 << 15);

    vu.load_fix_point(v3.e0(), v4.e0(), 3 << 16);
    vu.load_fix_point(v3.e2(), v4.e2(), 4 << 16);
    vu.load_fix_point(v3.e4(), v4.e4(), 5 << 16);
    vu.load_fix_point(v3.e6(), v4.e6(), 6 << 16);

    println!("{vu}");

    vu.vmudl(v5, v1, v3);
    vu.vmadm(v6, v2, v3);
    vu.vmadn(v7, v1, v4);
    vu.vmadh(v8, v2, v4);

    println!("{vu}");
    //println!("{}", vu.asm());
}
