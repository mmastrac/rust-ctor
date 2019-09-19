#![no_std]
#![feature(lang_items)]
#![allow(dead_code, unused_imports)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

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
pub static STATIC_INT: u8 = {
    libc_ewriteln!("+++ ctor STATIC_INT");
    200
};

#[ctor]
#[cfg(not(test))]
#[cfg(target_feature = "crt-static")]
unsafe fn ctor() {
    sleep(1);
    libc_ewriteln!("+++ ctor lib (+crt-static)");
}

#[ctor]
#[cfg(not(test))]
#[cfg(not(target_feature = "crt-static"))]
unsafe fn ctor() {
    sleep(1);
    libc_ewriteln!("+++ ctor lib (-crt-static)");
}

#[dtor]
#[cfg(not(test))]
unsafe fn dtor() {
    sleep(1);
    libc_ewriteln!("--- dtor lib");
}
