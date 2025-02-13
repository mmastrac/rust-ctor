#![cfg_attr(feature = "used_linker", feature(used_with_arg))]
#![allow(dead_code, unused_imports)]

use ctor::*;
use libc_print::*;

#[cfg(windows)]
#[allow(unsafe_code)]
unsafe extern "C" {
    #[allow(unused)]
    unsafe fn Sleep(ms: u32);
}

#[cfg(windows)]
#[allow(unsafe_code)]
unsafe fn sleep(seconds: u32) {
    unsafe {
        Sleep(seconds * 1000);
    }
}

#[cfg(not(windows))]
#[allow(unsafe_code)]
unsafe fn sleep(seconds: u32) {
    unsafe {
        libc::sleep(seconds);
    }
}

#[ctor]
pub static STATIC_INT: u8 = {
    libc_ewriteln!("+++ ctor STATIC_INT");
    200
};

#[ctor]
#[cfg(not(test))]
#[cfg(target_feature = "crt-static")]
#[allow(unsafe_code)]
unsafe fn ctor() {
    unsafe {
        sleep(1);
    }
    libc_ewriteln!("+++ ctor lib (+crt-static)");
}

#[ctor]
#[cfg(not(test))]
#[cfg(not(target_feature = "crt-static"))]
#[allow(unsafe_code)]
unsafe fn ctor() {
    unsafe {
        sleep(1);
    }
    libc_ewriteln!("+++ ctor lib (-crt-static)");
}

#[dtor]
#[cfg(not(test))]
#[allow(unsafe_code)]
unsafe fn dtor() {
    unsafe {
        sleep(1);
    }
    libc_ewriteln!("--- dtor lib");
}
