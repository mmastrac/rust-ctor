use ctor::*;
use dlopen::raw::Library;
use libc_print::*;

#[ctor]
unsafe fn ctor() {
    libc::sleep(1);
    libc_ewriteln!("+ ctor bin");
}

#[dtor]
unsafe fn dtor() {
    libc::sleep(1);
    libc_ewriteln!("- dtor bin");
}

#[cfg(target_os = "macos")]
fn extension() -> &'static str {
    "dylib"
}

#[cfg(target_os = "linux")]
fn extension() -> &'static str {
    "so"
}

#[cfg(windows)]
fn extension() -> &'static str {
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

pub fn main() {
    unsafe {
        libc::sleep(1);
        libc_ewriteln!("++ main start");
        let lib = Library::open(format!(
            "target/debug/examples/{}dylib.{}",
            prefix(),
            extension()
        ))
        .unwrap();
        drop(lib);
        libc::sleep(1);
        libc_ewriteln!("-- main end");
    }
}
