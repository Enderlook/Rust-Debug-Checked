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

/// Replaces the value in `reference` with a new one produced in `closure`.
///
/// # Abort (Debug)
///
/// It aborts if `closure` panics.
///
/// # Safety
///
/// `closure` shouldn't panic.
///
/// Failing this produces undefined behavior on Release.
#[cfg(debug_assertions)]
#[inline(always)]
pub unsafe fn replace_with_dc<T>(reference: &mut T, closure: impl FnOnce(T) -> T) {
    use std::{panic, ptr};

    let old_value = ptr::read(reference);
    let new_value = panic::catch_unwind(panic::AssertUnwindSafe(|| closure(old_value)))
        .unwrap_or_else(|_| ::std::process::abort());
    ptr::write(reference, new_value);
}

/// Replaces the value in `reference` with a new one produced in `closure`.
///
/// # Abort (Debug)
///
/// It aborts if `closure` panics.
///
/// # Safety
///
/// `closure` shouldn't panic.
///
/// Failing this produces undefined behavior on Release.
#[cfg(not(debug_assertions))]
#[inline(always)]
pub unsafe fn replace_with_dc<T>(reference: &mut T, closure: impl FnOnce(T) -> T) {
    use std::{panic, ptr};

    let old_value = ptr::read(reference);
    let new_value = closure(old_value);
    ptr::write(reference, new_value);
}

/// Informs the compiler this method is never reached.
///
/// # Panics (Debug)
///
/// Panics if the function is called.
///
/// # Safety
///
/// Function should never be reached.
///
/// Failing this produces undefined behavior on Release.
#[cfg(debug_assertions)]
#[track_caller]
pub unsafe fn unreachable_dc() -> ! {
    unreachable!();
}

/// Informs the compiler this method is never reached.
///
/// # Panics (Debug)
///
/// Panics with the specified message if the function is called.
///
/// # Safety
///
/// Function should never be reached.
///
/// Failing this produces undefined behavior on Release.
#[cfg(debug_assertions)]
#[track_caller]
pub unsafe fn expect_unreachable_dc(msg: &str) -> ! {
    unreachable!("{}", msg);
}

/// Informs the compiler this method is never reached.
///
/// # Panics (Debug)
///
/// Panics if the function is called.
///
/// # Safety
///
/// Function should never be reached.
///
/// Failing this produces undefined behavior on Release.
#[cfg(not(debug_assertions))]
#[inline(always)]
pub unsafe fn unreachable_dc() -> ! {
    unsafe {
        std::hint::unreachable_unchecked();
    }
}

/// Informs the compiler this method is never reached.
///
/// # Panics (Debug)
///
/// Panics with the specified message if the function is called.
///
/// # Safety
///
/// Function should never be reached.
///
/// Failing this produces undefined behavior on Release.
#[cfg(not(debug_assertions))]
#[inline(always)]
pub unsafe fn expect_unreachable_dc(_msg: &str) -> ! {
    unsafe {
        std::hint::unreachable_unchecked();
    }
}