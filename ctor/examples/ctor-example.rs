//! This example demonstrates the various types of ctor/dtor in an executable
//! context.

#![cfg_attr(feature = "used_linker", feature(used_with_arg))]

use ctor::ctor;
use libc_print::*;
use std::collections::HashMap;

#[ctor]
/// This is an immutable static, evaluated at init time
static STATIC_CTOR: HashMap<u32, &'static str> = unsafe {
    let mut m = HashMap::new();
    _ = m.insert(0, "foo");
    _ = m.insert(1, "bar");
    _ = m.insert(2, "baz");
    libc_println!("STATIC_CTOR");
    m
};

#[ctor(anonymous)]
unsafe fn anonymous_ctor() {
    libc_println!("ctor_anonymous (#1)");
    // We can still reference the function itself
    let _f = anonymous_ctor;
}

#[ctor(anonymous)]
unsafe fn anonymous_ctor() {
    libc_println!("ctor_anonymous (#2)");
}

const _: () = {
    #[ctor]
    unsafe fn anonymous_ctor() {
        libc_println!("ctor_anonymous (#3)");
        let _f = anonymous_ctor;
    }
};

#[ctor]
#[allow(unsafe_code)]
unsafe fn ctor() {
    libc_println!("ctor");
    // We can still reference the function itself
    let _f = ctor;
}

#[ctor(priority = 1)]
#[allow(unsafe_code)]
unsafe fn ctor_priority_one() {
    libc_println!("ctor_priority_one");
    // We can still reference the function itself
    let _f = ctor_priority_one;
}

#[ctor]
#[allow(unsafe_code)]
unsafe fn ctor_unsafe() {
    libc_println!("ctor_unsafe");
}

/// A module with a static ctor/dtor
pub mod module {
    use ctor::*;
    use libc_print::*;

    #[ctor]
    pub(crate) static STATIC_CTOR: u8 = unsafe {
        libc_println!("module::STATIC_CTOR");
        42
    };
}

/// Executable main which demonstrates the various types of ctor/dtor.
pub fn main() {
    use libc_print::*;
    libc_println!("main!");
    libc_println!("STATIC_CTOR = {:?}", *STATIC_CTOR);
    libc_println!("module::STATIC_CTOR = {:?}", *module::STATIC_CTOR);
}
