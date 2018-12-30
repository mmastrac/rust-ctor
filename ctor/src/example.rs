extern crate ctor;
extern crate libc_print;

use ctor::*;
use libc_print::*;

#[ctor]
fn ctor() {
    libc_eprintln!("ctor");
}

#[ctor]
unsafe fn ctor_unsafe() {
    libc_eprintln!("ctor_unsafe");
}

#[dtor]
fn dtor() {
    libc_eprintln!("dtor");
}

#[dtor]
unsafe fn dtor_unsafe() {
    libc_eprintln!("dtor_unsafe");
}

pub fn main() {
    libc_eprintln!("main!");
}
