extern crate ctor;
extern crate libc;

use ctor::*;

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
    eprintln!("ctor");
}

#[ctor]
fn ctor_unsafe() {
    eprintln!("ctor_unsafe");
}

#[dtor]
fn dtor() {
    shutdown_println("dtor");
}

#[dtor]
unsafe fn dtor_unsafe() {
    shutdown_println("dtor_unsafe");
}

pub fn main() {
    eprintln!("main!");
}
