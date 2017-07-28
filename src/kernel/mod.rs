use util::io;

#[no_mangle]
pub extern "C" fn kernel_main() -> i32 {
    unsafe {
        let x = true;
        if x {
            io::io_init();
            io::putstr("Hello world!\n");
        }
    }
    return 0;
}
