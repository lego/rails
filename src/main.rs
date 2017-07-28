#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(no_core)]

#![allow(unused_imports)]
#![allow(improper_ctypes)]

#![no_core]
#![no_main]

extern crate core;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn memset() {}

#[no_mangle]
pub extern "C" fn __libc_init_array() {}

#[no_mangle]
pub extern "C" fn exit() {}

// I'm not 100% sure what this function does, but references to it are compiled
// into the program by the Rust compiler. I think it would be called in the case
// of a program panic.
#[no_mangle]
pub fn __aeabi_unwind_cpp_pr0() {
    loop {}
}

#[no_mangle]
pub fn main() -> i32 {
    return 0;
    // println!("Hello, world!");
}
