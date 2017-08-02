
extern "C" {
  pub fn c_mmio_write(reg: u32, data: u32);
  pub fn c_mmio_read(reg: u32) -> u32;
}

#[inline]
pub fn mmio_write(reg: u32, data: u32) {
  unsafe {
    c_mmio_write(reg, data);
  }
}

#[inline]
pub fn mmio_read(reg: u32) -> u32 {
  unsafe { c_mmio_read(reg) }
}
