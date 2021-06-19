use core::sync::atomic::{AtomicBool, Ordering};
use register::{mmio::*, register_bitfields, register_structs};

static TAKEN: AtomicBool = AtomicBool::new(false);

register_bitfields! {
    u32,

    /// Flags register
    FR [
        /// Transmit FIFO empty
        TXFE OFFSET(7) NUMBITS(1) [],
        /// Transmit FIFO full
        TXFF OFFSET(5) NUMBITS(1) [],
        /// Receive FIFO empty
        RXFE OFFSET(4) NUMBITS(1) [],
        /// Busy flag
        BUSY OFFSET(3) NUMBITS(1) []
    ],

    /// Integer baudrate register
    IBRD [
        /// Baudrate integer division
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional baudrate register
    FBRD [
        /// Baudrate fractional division
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line control register
    LCRH [
        /// Word length
        WL OFFSET(5) NUMBITS(2) [],
        /// FIFO enable
        FEN OFFSET(4) NUMBITS(1) []
    ],

    /// Control register
    CR [
        /// Uart enable
        UARTEN OFFSET(0) NUMBITS(1) [],
        /// Transmitter enable
        TXE OFFSET(8) NUMBITS(1) [],
        /// Receiver enable
        RXE OFFSET(9) NUMBITS(1) []
    ],

    /// Interrupt clear register
    ICR [
        /// All interrupts
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub UartRegisters {
        (0x00 => dr: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => fr: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => ibrd: WriteOnly<u32, IBRD::Register>),
        (0x28 => fbrd: WriteOnly<u32, IBRD::Register>),
        (0x2C => lcrh: WriteOnly<u32, LCRH::Register>),
        (0x30 => cr: WriteOnly<u32, CR::Register>),
        (0x34 => _reserved3),
        (0x44 => icr: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
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
        self.registers.fr.read(FR::TXFF) == 1
    }

    fn writec(&mut self, c: u8) {
        while self.transmit_fifo_full() {}
        self.registers.dr.set(c as u32);
    }

    pub fn write(&mut self, msg: &str) {
        for c in msg.chars() {
            self.writec(c as u8)
        }
    }

    pub fn writeln(&mut self, msg: &str) {
        self.write(msg);
        self.writec(b'\n')
    }
}
