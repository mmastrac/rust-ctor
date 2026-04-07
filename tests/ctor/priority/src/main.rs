#![cfg_attr(feature = "used_linker", feature(used_with_arg))]
//! Edition 2024 test.

use ctor::ctor;

#[ctor(unsafe, priority = 01)]
unsafe fn priority_1() {
    println!("1");
}

#[ctor(unsafe, priority = 2)]
unsafe fn priority_b() {
    println!("2");
}

#[ctor(unsafe, priority = 3)]
unsafe fn priority_three() {
    println!("3");
}

#[ctor(unsafe, priority = 7)]
unsafe fn z_priority_7() {
    println!("7");
}

#[ctor(unsafe, priority = 4)]
unsafe fn priority_four() {
    println!("4");
}

#[ctor(unsafe, priority = 10)]
unsafe fn priority_10() {
    println!("10");
}

#[ctor(unsafe, priority = 5)]
unsafe fn priority_five() {
    println!("5");
}

#[ctor(unsafe, priority = 6)]
unsafe fn a_priority_6() {
    println!("6");
}

#[ctor(unsafe, priority = 8)]
unsafe fn priority_eight() {
    println!("8");
}

#[ctor(unsafe, priority = 9)]
unsafe fn priority_nine() {
    println!("9");
}

fn main() {
    println!("main");
}
