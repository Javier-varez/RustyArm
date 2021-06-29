#![no_std]
#![no_main]
#![feature(asm)]

mod arch;
mod gpio;
mod uart;

use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_a::regs::{CurrentEL, RegisterReadOnly};

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut uart = unsafe { uart::Uart::steal() };
    let mut gpio = unsafe { gpio::Gpio::steal() };

    gpio.configure_uart_alternate_function();

    let exception_level = CurrentEL.read(CurrentEL::EL);
    write!(uart, "Running on exception level: {}\n", exception_level).unwrap();

    panic!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Reinitialize the peripherals if really needed (maybe we paniced before reaching kernel_main)
    let mut uart = unsafe { uart::Uart::steal() };
    let mut gpio = unsafe { gpio::Gpio::steal() };

    gpio.configure_uart_alternate_function();

    write!(uart, "Panic! {:?}", info).unwrap();
    loop {}
}
