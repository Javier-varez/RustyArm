#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;
use core::panic::PanicInfo;

const UART_DR: u32 = 0x3F201000;
const UART_FR: u32 = 0x3F201018;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

fn writeln(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
    writec('\n' as u8)
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    writeln("Hi there! This should print a message in your shell!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
