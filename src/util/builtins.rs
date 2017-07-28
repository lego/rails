/**
 * These are a set of shims for built-ins which we need to provide
 * when building bare metal
 */
// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn rust_eh_unwind_resume() {}

// I'm not 100% sure what this function does, but references to it are compiled
// into the program by the Rust compiler. I think it would be called in the case
// of a program panic.
#[no_mangle]
pub fn __aeabi_unwind_cpp_pr0() {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {}


// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}
