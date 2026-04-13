//! Shared macros and utilities for the `ctor` and `dtor` crates.

pub mod macros;
pub mod __support {
    pub use crate::macros::__support::*;
}

pub use ctor::declarative::ctor as ctor_parse;
pub use dtor::declarative::dtor as dtor_parse;
