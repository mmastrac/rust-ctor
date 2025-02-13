#![cfg_attr(feature = "used_linker", feature(used_with_arg))]

use ctor::ctor;

#[ctor]
#[allow(unsafe_code)]
unsafe fn foo() {
    println!("foo");
}

fn main() {
    println!("main");
}
