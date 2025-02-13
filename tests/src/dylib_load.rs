#![cfg_attr(feature = "used_linker", feature(used_with_arg))]
#![allow(unused_imports)]

use ctor::*;
use dlopen::raw::Library;
use libc_print::*;

#[ctor]
#[cfg(not(test))]
#[allow(unsafe_code)]
unsafe fn ctor() {
    unsafe {
        sleep(1);
    }
    libc_ewriteln!("+ ctor bin");
}

#[dtor]
#[cfg(not(test))]
#[allow(unsafe_code)]
unsafe fn dtor() {
    unsafe {
        sleep(1);
    }
    libc_ewriteln!("- dtor bin");
}

#[cfg(target_vendor = "apple")]
fn lib_extension() -> &'static str {
    "dylib"
}

#[cfg(windows)]
fn lib_extension() -> &'static str {
    "dll"
}

#[cfg(all(not(windows), not(target_vendor = "apple")))]
fn lib_extension() -> &'static str {
    "so"
}

#[cfg(not(windows))]
fn prefix() -> &'static str {
    "lib"
}

#[cfg(windows)]
fn prefix() -> &'static str {
    ""
}

#[cfg(windows)]
extern "C" {
    fn Sleep(ms: u32);
}

#[cfg(windows)]
#[allow(unsafe_code)]
unsafe fn sleep(seconds: u32) {
    Sleep(seconds * 1000);
}

#[cfg(not(windows))]
#[allow(unsafe_code)]
unsafe fn sleep(seconds: u32) {
    unsafe {
        libc::sleep(seconds);
    }
}

pub fn main() {
    #[allow(unsafe_code)]
    unsafe {
        sleep(1);
        libc_ewriteln!("++ main start");
        let lib = Library::open(format!(
            "target/debug/examples/{}dylib.{}",
            prefix(),
            lib_extension()
        ))
        .unwrap();
        drop(lib);
        sleep(1);
        libc_ewriteln!("-- main end");
    }
}
