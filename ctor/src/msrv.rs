//! This example demonstrates the features available at the minimum supported
//! Rust version (MSRV) for the `ctor` crate.

use ctor::{ctor, dtor};
use libc_print::*;

#[ctor(anonymous)]
unsafe fn anonymous_ctor() {
    libc_eprintln!("ctor_anonymous (#1)");
    // We can still reference the function itself
    let f = anonymous_ctor;
}

#[ctor(anonymous)]
unsafe fn anonymous_ctor() {
    libc_eprintln!("ctor_anonymous (#2)");
}

const _: () = {
    #[ctor]
    unsafe fn anonymous_ctor() {
        libc_eprintln!("ctor_anonymous (#3)");
        let f = anonymous_ctor;
    }

    #[dtor]
    unsafe fn anonymous_dtor() {
        libc_eprintln!("dtor_anonymous");
        let f = anonymous_dtor;
    }
};

#[ctor]
#[allow(unsafe_code)]
unsafe fn ctor() {
    libc_eprintln!("ctor");
    // We can still reference the function itself
    let f = ctor;
}

#[ctor]
#[allow(unsafe_code)]
unsafe fn ctor_unsafe() {
    libc_eprintln!("ctor_unsafe");
}

#[dtor]
#[allow(unsafe_code)]
unsafe fn dtor() {
    libc_eprintln!("dtor");
    // We can still reference the function itself
    let f = dtor;
}

#[dtor]
#[allow(unsafe_code)]
unsafe fn dtor_unsafe() {
    libc_eprintln!("dtor_unsafe");
}

#[dtor(anonymous)]
unsafe fn anonymous_dtor() {
    libc_eprintln!("dtor_anonymous (#1)");
    let f = anonymous_dtor;
}

#[dtor(anonymous)]
unsafe fn anonymous_dtor() {
    libc_eprintln!("dtor_anonymous (#2");
    let f = anonymous_dtor;
}

/// A module with a static ctor/dtor
pub mod module {
    use ctor::*;
    use libc_print::*;

    #[dtor]
    #[allow(unsafe_code)]
    unsafe fn dtor_module() {
        libc_eprintln!("module::dtor_module");
    }
}

/// Executable main which demonstrates the various types of ctor/dtor.
pub fn main() {
    use libc_print::*;
    libc_eprintln!("main!");
}
