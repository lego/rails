use mach::mmio::{mmio_write, mmio_read};
use super::uart;

pub fn getc() -> char {
  // Wait for UART to have received something.
  while mmio_read(uart::UART0_FR) & (1 << 4) != 0 {}
  mmio_read(uart::UART0_DR) as u8 as char
}

pub fn putc(c: char) {
  // Wait for UART to become ready to transmit.
  while mmio_read(uart::UART0_FR) & (1 << 5) != 0 {}
  mmio_write(uart::UART0_DR, c as u32);
}
