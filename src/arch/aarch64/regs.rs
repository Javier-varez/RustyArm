pub fn vbar_el2() -> u64 {
    let x: u64;
    unsafe {
        asm!("mrs {}, vbar_el2", out(reg) x);
    }
    x
}

pub fn sctlr_el2() -> u64 {
    let x: u64;
    unsafe {
        asm!("mrs {}, sctlr_el2", out(reg) x);
    }
    x
}

pub fn sctlr_el1() -> u64 {
    let x: u64;
    unsafe {
        asm!("mrs {}, sctlr_el1", out(reg) x);
    }
    x
}
