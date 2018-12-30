#![no_std]

use ctor::*;
use libc_print::*;

#[ctor]
unsafe fn ctor() {
    // libc::sleep(1);
    libc_ewriteln!("+++ ctor lib");
}

#[dtor]
unsafe fn dtor() {
    // libc::sleep(1);
    libc_ewriteln!("--- dtor lib");
}
