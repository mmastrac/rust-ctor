use ctor::*;
use dlopen::raw::Library;

#[cfg(not(windows))]
pub fn shutdown_println(msg: &str) {
    unsafe {
        libc::write(
            2,
            std::mem::transmute(msg.as_ptr()),
            msg.len() as libc::size_t,
        );
        let newline = "\n";
        libc::write(2, std::mem::transmute(newline.as_ptr()), 1);
    }
}

#[cfg(windows)]
pub fn shutdown_println(msg: &str) {
    unsafe {
        libc::write(2, std::mem::transmute(msg.as_ptr()), msg.len() as u32);
        let newline = "\n";
        libc::write(2, std::mem::transmute(newline.as_ptr()), 1);
    }
}

#[ctor]
fn ctor() {
    shutdown_println("+ ctor bin");
}

#[dtor]
fn dtor() {
    shutdown_println("- dtor bin");
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
    shutdown_println("++ main start");
    let lib = Library::open(format!("target/debug/examples/{}dylib.{}", prefix(), extension())).unwrap();
    drop(lib);
    shutdown_println("-- main end");
}
