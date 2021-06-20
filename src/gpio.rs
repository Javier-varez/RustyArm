use core::sync::atomic::{AtomicBool, Ordering};
use register::{mmio::*, register_bitfields, register_structs};

static TAKEN: AtomicBool = AtomicBool::new(false);

register_bitfields! {
    u32,

    /// Gpio function select register
    GPFSEL1 [
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Uart0 = 0b100
        ],

        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Uart0 = 0b100
        ]
    ],

    /// Gpio pull up/down control
    GPPUD [
        PUD OFFSET(0) NUMBITS(2) [
            Off = 0b00,
            PullDown = 0b01,
            PullUp = 0b10
        ]
    ],

    /// Gpio pull up/down clock register
    GPPUDCLK0 [
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0b0,
            AssertClock = 0b1
        ],

        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0b0,
            AssertClock = 0b1
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub GpioRegisters {
        (0x00 => _reserved1),
        (0x04 => gpfsel1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _reserved2),
        (0x94 => gppud: ReadWrite<u32, GPPUD::Register>),
        (0x98 => gppudclk0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0xB0 => @END),
    }
}

pub struct Gpio {
    registers: &'static mut GpioRegisters,
}

impl Gpio {
    #[allow(dead_code)]
    pub fn take() -> Option<Self> {
        if !TAKEN.swap(true, Ordering::Relaxed) {
            return Some(Self {
                registers: unsafe { &mut *(0x3F200000 as *mut GpioRegisters) },
            });
        }
        None
    }

    pub unsafe fn steal() -> Self {
        Self {
            registers: &mut *(0x3F200000 as *mut GpioRegisters),
        }
    }

    pub fn configure_uart_alternate_function(&mut self) {
        self.registers
            .gpfsel1
            .modify(GPFSEL1::FSEL14::Uart0 + GPFSEL1::FSEL15::Uart0);

        self.registers.gppud.write(GPPUD::PUD::Off);

        for _ in 0..2000 {
            cortex_a::asm::nop();
        }

        self.registers
            .gppudclk0
            .write(GPPUDCLK0::PUDCLK14::AssertClock + GPPUDCLK0::PUDCLK15::AssertClock);

        for _ in 0..2000 {
            cortex_a::asm::nop();
        }

        self.registers.gppud.write(GPPUD::PUD::Off);
        self.registers.gppudclk0.set(0);
    }
}
