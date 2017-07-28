extern "C" {
    pub fn io_init();
    fn bwputc(c: char);
}


pub fn putstr(s: &str) {
    for c in s.chars() {
        unsafe {
            bwputc(c);
        }
    }
}
