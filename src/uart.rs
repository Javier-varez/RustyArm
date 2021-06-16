use core::sync::atomic::{AtomicBool, Ordering};
use volatile_register::RW;

static TAKEN: AtomicBool = AtomicBool::new(false);

#[repr(C)]
struct UartRegisters {
    dr: RW<u32>,
    rsrecr: RW<u32>,
    _reserved: [u32; 4],
    fr: RW<u32>,
}

pub struct Uart {
    registers: &'static mut UartRegisters,
}

impl Uart {
    pub fn take() -> Option<Self> {
        if !TAKEN.swap(true, Ordering::Relaxed) {
            return Some(Uart {
                registers: unsafe { &mut *(0x3F201000 as *mut UartRegisters) },
            });
        }
        None
    }

    fn transmit_fifo_full(&self) -> bool {
        let fr = self.registers.fr.read();
        fr & (1 << 5) > 0
    }

    fn writec(&mut self, c: u8) {
        while self.transmit_fifo_full() {}
        unsafe { self.registers.dr.write(c as u32) };
    }

    pub fn write(&mut self, msg: &str) {
        for c in msg.chars() {
            self.writec(c as u8)
        }
    }

    pub fn writeln(&mut self, msg: &str) {
        self.write(msg);
        self.writec('\n' as u8)
    }
}
