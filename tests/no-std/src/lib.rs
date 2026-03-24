#![no_std]

use ctor::ctor;

#[ctor]
static VALUE: u32 = {
    let value = 42;
    value
};

#[allow(dead_code)]
pub fn value() -> &'static u32 {
    &VALUE
}
