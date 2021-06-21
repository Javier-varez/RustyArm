use cortex_a::regs::{
    RegisterReadWrite, CNTHCTL_EL2, CNTVOFF_EL2, ELR_EL2, HCR_EL2, SPSR_EL2, SP_EL1,
};

fn transition_to_el1() {
    // Enable timer counter registers for EL1.
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading the counters.
    CNTVOFF_EL2.set(0);

    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::M::EL1h
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked,
    );

    ELR_EL2.set(crate::kernel_main as *const () as u64);
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // This is wrong, we are trashing a bunch of data by doing setting the SP here...
    // But let's try anyway!
    SP_EL1.set(0x80000);

    cortex_a::asm::eret();
}

#[no_mangle]
pub extern "C" fn _start_kernel() {
    transition_to_el1();
}
