pub use super::rpi2::bwio::{putc as bwputc, getc as bwgetc};

pub fn bwputstr(s: &str) {
  for c in s.chars() {
    bwputc(c);
  }
}
