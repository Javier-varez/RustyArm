#![no_std]
#![no_main]

mod gpio;
mod uart;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut uart = unsafe { uart::Uart::steal() };
    let mut gpio = unsafe { gpio::Gpio::steal() };

    gpio.configure_uart_alternate_function();

    uart.writeln("Hi there! This should print a message in your shell!");
    panic!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
