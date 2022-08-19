#![allow(dead_code, unused_imports)]

use ctor::{ctor, dtor};

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
pub static STATIC_INT: u8 = {
    eprintln!("+++ ctor STATIC_INT");
    200
};

#[ctor]
#[cfg(not(test))]
#[cfg(target_feature = "crt-static")]
unsafe fn ctor() {
    sleep(1);
    eprintln!("+++ ctor lib (+crt-static)");
}

#[ctor]
#[cfg(not(test))]
#[cfg(not(target_feature = "crt-static"))]
unsafe fn ctor() {
    sleep(1);
    eprintln!("+++ ctor lib (-crt-static)");
}

#[dtor]
#[cfg(not(test))]
unsafe fn dtor() {
    sleep(1);
    eprintln!("--- dtor lib");
}
