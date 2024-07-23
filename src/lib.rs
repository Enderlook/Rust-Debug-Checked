//! This crate defines types, traits and extension methods which are used to perform operations that are checked on release but not in debug.
//!
//! That is, when the compiler flag `debug_assertions` is on, these functions are safe and panic if used incorrectly.
//!
//! But when compiled without such flag, these functions are unsafe and doesn't perform any check.
//!
//! For these reason, the functions are always `unsafe`.
//!
//! For the purposes of the documentation of the crate, "Debug" means having the compiler flag `debug_assertions` and "Release" means not having it.
//!
//! That is, if you compile "Release" using `debug_assertions`, you could use it to check for errors in your release.

#![feature(coerce_unsized)]
#![feature(negative_impls)]
#![feature(must_not_suspend)]
#![feature(unsize)]

mod dc_ref_cell;
mod dc_option;
mod dc_result;
mod dc_slice;

pub use dc_ref_cell::*;
pub use dc_option::*;
pub use dc_result::*;
pub use dc_slice::*;