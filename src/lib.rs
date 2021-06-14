#![cfg_attr(feature = "libcore", no_std)]

/// The S8 is compatible with the Std Rust Library, and Libcore / Liballoc
#[cfg(feature = "libcore")]
    pub(crate) use core as std;

#[cfg(not(feature = "libcore"))]
    pub(crate) use std as std;

#[cfg(feature = "liballoc")]
    pub(crate) extern crate alloc;

#[cfg(feature = "liballoc")]
    pub(crate) use alloc as mem;

#[cfg(not(feature = "liballoc"))]
    pub(crate) use std as mem;


pub mod assembler;
pub use assembler as asm;


