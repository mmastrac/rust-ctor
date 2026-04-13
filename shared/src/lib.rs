//! Shared macros and utilities for the `ctor` and `dtor` crates.

pub mod macros;

pub use macros::__support::ctor_parse;
pub use macros::__support::dtor_parse_impl as dtor_parse;

pub mod __support {
    pub use crate::macros::__support::*;
    #[allow(unused)]
    pub fn at_library_exit(_: unsafe extern "C" fn()) {
    }
    #[allow(unused)]
    pub fn at_binary_exit(_: unsafe extern "C" fn()) {
    }
}
