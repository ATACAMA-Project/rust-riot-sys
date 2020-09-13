//! # C2Rust transpiled header contents (static inline functions
//!
//! Types in here are distinct from those created in the main module (using bindgen); unifying
//! those will be part of [bindgen's #1334], but it's a long way there.
//!
//! [bindgen's #1334]: https://github.com/rust-lang/rust-bindgen/issues/1344
//!
//! Use these functions through the re-export in the main module, for the C headers may flip-flop
//! between static inline and linked.
//!
//! ---
//!
//! Some special treatment has been applied in the course of the transpilation process:
//!
//! * All functions were made `pub`
//! * Some functions (currently only `mutex_init`) were changed to be `const`
//!
// While it'd be tempting to clean them all up in RIOT by a large constification haul, now is not
// the time for that
#![allow(unused_mut)]
// Probably __attribute__((used)) doesn't get translated
#![allow(unused)]

use crate::libc;

include!(concat!(env!("OUT_DIR"), "/riot_c2rust_replaced.rs"));
