#![no_std]
#![allow(dead_code, unused_imports)]

use ctor::*;
use libc_print::*;

#[cfg(windows)]
extern "C" {
    fn Sleep(ms: u32);
}

#[cfg(windows)]
unsafe fn sleep(seconds: u32) {
    Sleep(seconds * 1000);
}

#[cfg(not(windows))]
unsafe fn sleep(seconds: u32) {
    libc::sleep(seconds);
}

#[ctor]
#[cfg(not(test))]
unsafe fn ctor() {
    sleep(1);
    libc_ewriteln!("+++ ctor lib");
}

#[dtor]
#[cfg(not(test))]
unsafe fn dtor() {
    sleep(1);
    libc_ewriteln!("--- dtor lib");
}
