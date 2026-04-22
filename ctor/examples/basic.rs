//! Basic example of using the `ctor` crate.

use ctor::ctor;

#[ctor(unsafe)]
fn ctor() {
    println!("ctor");
}

fn main() {
    println!("main");
}
