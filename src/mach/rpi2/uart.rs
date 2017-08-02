
// The GPIO registers base address.
pub const GPIO_BASE: u32 = 0x3F200000; // for raspi2 & 3, 0x20200000 for raspi1

// The offsets for reach register.

// Controls actuation of pull up/down to ALL GPIO pins.
pub const GPPUD: u32 = (GPIO_BASE + 0x94);

// Controls actuation of pull up/down for specific GPIO pin.
pub const GPPUDCLK0: u32 = (GPIO_BASE + 0x98);

// The base address for UART.
pub const UART0_BASE: u32 = 0x3F201000; // for raspi2 & 3, 0x20201000 for raspi1

// The offsets for reach register for the UART.
#[allow(dead_code)]
pub const UART0_DR: u32 = (UART0_BASE + 0x00);
#[allow(dead_code)]
pub const UART0_RSRECR: u32 = (UART0_BASE + 0x04);
#[allow(dead_code)]
pub const UART0_FR: u32 = (UART0_BASE + 0x18);
#[allow(dead_code)]
pub const UART0_ILPR: u32 = (UART0_BASE + 0x20);
pub const UART0_IBRD: u32 = (UART0_BASE + 0x24);
pub const UART0_FBRD: u32 = (UART0_BASE + 0x28);
pub const UART0_LCRH: u32 = (UART0_BASE + 0x2C);
pub const UART0_CR: u32 = (UART0_BASE + 0x30);
#[allow(dead_code)]
pub const UART0_IFLS: u32 = (UART0_BASE + 0x34);
pub const UART0_IMSC: u32 = (UART0_BASE + 0x38);
#[allow(dead_code)]
pub const UART0_RIS: u32 = (UART0_BASE + 0x3C);
#[allow(dead_code)]
pub const UART0_MIS: u32 = (UART0_BASE + 0x40);
pub const UART0_ICR: u32 = (UART0_BASE + 0x44);
#[allow(dead_code)]
pub const UART0_DMACR: u32 = (UART0_BASE + 0x48);
#[allow(dead_code)]
pub const UART0_ITCR: u32 = (UART0_BASE + 0x80);
#[allow(dead_code)]
pub const UART0_ITIP: u32 = (UART0_BASE + 0x84);
#[allow(dead_code)]
pub const UART0_ITOP: u32 = (UART0_BASE + 0x88);
#[allow(dead_code)]
pub const UART0_TDR: u32 = (UART0_BASE + 0x8C);
