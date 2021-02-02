#![allow(unused_imports)]

use ctor::*;
use dlopen::raw::Library;
use libc_print::*;

#[ctor]
#[cfg(not(test))]
unsafe fn ctor() {
    sleep(1);
    libc_ewriteln!("+ ctor bin");
}

#[dtor]
#[cfg(not(test))]
unsafe fn dtor() {
    sleep(1);
    libc_ewriteln!("- dtor bin");
}

#[cfg(target_os = "macos")]
fn lib_extension() -> &'static str {
    "dylib"
}

#[cfg(target_os = "linux")]
fn lib_extension() -> &'static str {
    "so"
}

#[cfg(target_os = "freebsd")]
fn lib_extension() -> &'static str {
    "so"
}

#[cfg(target_os = "netbsd")]
fn lib_extension() -> &'static str {
    "so"
}

#[cfg(target_os = "openbsd")]
fn lib_extension() -> &'static str {
    "so"
}

#[cfg(windows)]
fn lib_extension() -> &'static str {
    "dll"
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
unsafe fn sleep(seconds: u32) {
    Sleep(seconds * 1000);
}

#[cfg(not(windows))]
unsafe fn sleep(seconds: u32) {
    libc::sleep(seconds);
}

pub fn main() {
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
