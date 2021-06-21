use core::sync::atomic::{AtomicBool, Ordering};
use register::{mmio::*, register_bitfields, register_structs};

static TAKEN: AtomicBool = AtomicBool::new(false);

register_bitfields! {
    u32,

    /// Flags register
    FR [
        /// Transmit FIFO empty
        TXFE OFFSET(7) NUMBITS(1) [
            NotEmpty = 0,
            Empty = 1
        ],
        /// Transmit FIFO full
        TXFF OFFSET(5) NUMBITS(1) [
            NotFull = 0,
            Full = 1
        ],
        /// Receive FIFO empty
        RXFE OFFSET(4) NUMBITS(1) [
            NotEmpty = 0,
            Empty = 1
        ],
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
        WL OFFSET(5) NUMBITS(2) [
            FiveBits = 0b00,
            SixBits = 0b01,
            SevenBits = 0b10,
            EightBits = 0b11
        ],
        /// FIFO enable
        FEN OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Control register
    CR [
        /// Uart enable
        UARTEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Transmitter enable
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Receiver enable
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
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
    #[allow(dead_code)]
    pub fn take() -> Option<Self> {
        if !TAKEN.swap(true, Ordering::Relaxed) {
            let mut device = Self {
                registers: unsafe { &mut *(0x3F201000 as *mut UartRegisters) },
            };
            device.init();
            return Some(device);
        }
        None
    }

    pub unsafe fn steal() -> Self {
        let mut device = Self {
            registers: &mut *(0x3F201000 as *mut UartRegisters),
        };
        device.init();
        device
    }

    fn init(&mut self) {
        self.flush();

        self.registers.cr.write(CR::UARTEN::CLEAR);

        self.registers.icr.write(ICR::ALL::CLEAR);

        // Baudrate is currently fixed to 115200
        // 48 MHz / 16 / 115200 = 26.0416
        // fraction = 0.416 * 64 + 0.5 = 3
        self.registers.ibrd.write(IBRD::BAUD_DIVINT.val(26));
        self.registers.fbrd.write(IBRD::BAUD_DIVINT.val(3));

        self.registers
            .lcrh
            .write(LCRH::WL::EightBits + LCRH::FEN::Enabled);

        self.registers
            .cr
            .write(CR::RXE::Enabled + CR::TXE::Enabled + CR::UARTEN::Enabled)
    }

    fn transmit_fifo_full(&self) -> bool {
        self.registers.fr.matches_all(FR::TXFF::Full)
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

    pub fn flush(&self) {
        while self.registers.fr.matches_all(FR::BUSY::SET) {}
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, msg: &str) -> Result<(), core::fmt::Error> {
        self.write(msg);
        Ok(())
    }
}
