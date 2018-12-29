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

pub fn main() {
    println!("++ main start");
    let lib = Library::open("target/debug/examples/libdylib.dylib").unwrap();
    drop(lib);
    println!("-- main end");
}
