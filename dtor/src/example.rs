//! This example demonstrates the various types of ctor/dtor in an executable
//! context.

#![cfg_attr(feature = "used_linker", feature(used_with_arg))]

use dtor::dtor;
use libc_print::*;

#[dtor]
#[allow(unsafe_code)]
unsafe fn dtor() {
    libc_eprintln!("dtor");
}

/// Doc
#[dtor]
#[allow(unsafe_code)]
unsafe fn dtor_unsafe() {
    libc_eprintln!("dtor_unsafe");
}

/// Executable main which demonstrates the various types of ctor/dtor.
pub fn main() {
    use libc_print::*;
    libc_eprintln!("main!");
}
