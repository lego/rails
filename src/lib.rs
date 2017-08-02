#![no_std]
#![feature(lang_items)]

// #![feature(macro_rules, globs, managed_boxes, linkage)]
// #![feature(default_type_params, phase, lang_items, unsafe_destructor)]

// #![allow(deprecated)]
// #![allow(missing_doc)]

// Note: remember to update RUST_LIBS in Makefile when adding more extern
// crates here.

// extern crate alloc;
// extern crate unicode;
// #[phase(plugin, link)] extern crate core;
// extern crate core_collections = "collections";
// extern crate core_rand = "rand";
// #[cfg(not(bootstrap))] extern crate core_sync = "sync";
// extern crate libc;
// extern crate rlibc;
// #[cfg(not(bootstrap))] extern crate rustrt;
//
// pub use core::any;
// pub use core::bool;
// pub use core::cell;
// pub use core::clone;
// pub use core::cmp;
// pub use core::default;
// pub use core::finally;
// pub use core::intrinsics;
// pub use core::iter;
// pub use core::kinds;
// pub use core::mem;
// pub use core::ops;
// pub use core::ptr;
// pub use core::raw;
// pub use core::simd;
// pub use core::tuple;
// pub use core::unit;
// pub use core::ty;
// pub use core::result;
// pub use core::option;


// Pull in utils library.
pub mod util;
pub mod mach;
pub mod arch;
pub mod kernel;
