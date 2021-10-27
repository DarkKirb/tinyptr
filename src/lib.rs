//! Small Pointer support crate
#![no_std]
#![cfg_attr(feature = "alloc", feature(allocator_api))]
#![feature(mixed_integer_ops)]
#![feature(ptr_internals)]
#![feature(ptr_metadata)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod alloc_integration;
pub mod ptr;
mod reference;
pub mod util;

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use alloc_integration::*;
#[doc(inline)]
pub use reference::*;

pub type TinyUSize = u16;

/// Converts a pointer to an u16 without checking for any invariants.
///
/// # Safety
/// This function is unsafe because it does not do any range checking.
///
/// - The caller has to ensure that ptr points into the ram arena, or is a null pointer
pub unsafe fn ptr_to_u16_unchecked<const BASE_ADDR: usize>(ptr: *const ()) -> u16 {
    if ptr.is_null() {
        return 0;
    }
    (ptr as usize - BASE_ADDR) as u16
}

/// Converts a pointer to an u16.
pub fn ptr_to_u16<const BASE_ADDR: usize>(ptr: *const ()) -> Option<u16> {
    if ptr.is_null() {
        return Some(0);
    }
    let ptr_usize = ptr as usize;
    if ptr_usize <= BASE_ADDR || ptr_usize >= BASE_ADDR + 65536 {
        return None;
    }
    Some((ptr_usize - BASE_ADDR) as u16)
}

/// Converts a u16 to a pointer
pub fn u16_to_ptr<const BASE_ADDR: usize>(ptr: u16) -> *const () {
    if ptr == 0 {
        return core::ptr::null();
    }
    (ptr as usize + BASE_ADDR) as *const ()
}
