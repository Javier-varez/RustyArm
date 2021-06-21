use cortex_a::regs::{
    CurrentEL, RegisterReadOnly, RegisterReadWrite, CNTHCTL_EL2, CNTVOFF_EL2, ELR_EL2, ELR_EL3,
    HCR_EL2, SCR_EL3, SPSR_EL2, SPSR_EL3, SP_EL1,
};

fn transition_to_el2() -> ! {
    SPSR_EL3.write(
        SPSR_EL3::D::Masked
            + SPSR_EL3::M::EL2h
            + SPSR_EL3::A::Masked
            + SPSR_EL3::I::Masked
            + SPSR_EL3::F::Masked,
    );
    SCR_EL3.write(
        SCR_EL3::RW::NextELIsAarch64
            + SCR_EL3::NS::NonSecure
            + SCR_EL3::SMD::SmcDisabled
            + SCR_EL3::HCE::HvcDisabled,
    );

    // Reboot again
    ELR_EL3.set(0x80000);
    cortex_a::asm::eret();
}

fn transition_to_el1() -> ! {
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
    SP_EL1.set(0x80000);

    cortex_a::asm::eret();
}

#[no_mangle]
pub extern "C" fn _start_kernel() -> ! {
    match CurrentEL.read_as_enum(CurrentEL::EL).unwrap() {
        CurrentEL::EL::Value::EL3 => {
            transition_to_el2();
        }
        CurrentEL::EL::Value::EL2 => {
            transition_to_el1();
        }
        _ => {
            crate::kernel_main();
        }
    }
}
