//! Doctest test.

use std::sync::atomic::AtomicBool;

pub static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// ```rust
/// use ctor::ctor;
/// use libc_print::*;
/// use std::sync::atomic::{AtomicBool, Ordering};
///
/// #[ctor(unsafe)]
/// fn foo() {
///     libc_eprintln!("doctest");
///     ::tests_doctest::INITIALIZED.store(true, Ordering::SeqCst);
/// }
/// 
/// assert!(::tests_doctest::INITIALIZED.load(Ordering::SeqCst));
/// ```
#[allow(unused)]
pub fn foo() {
}
