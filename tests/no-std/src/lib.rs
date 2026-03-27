#![no_std]

use ctor::ctor;

#[ctor]
unsafe fn init() {}

pub fn initialized() -> bool {
    true
}
