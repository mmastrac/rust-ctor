//#![cfg(not(miri))]

//! To overwrite the Linux expansion tests on macOS, run:
//!
//! ```bash
//! docker run --rm -v "$(pwd):/src" -w /src rust:latest \
//!   bash -lc 'export PATH="/usr/local/cargo/bin:$PATH" && cargo install cargo-expand && export MACROTEST=overwrite && cargo test -p shared --test macrotest'
//! ```

use std::path::Path;

/// Macrotest sometimes leaves files empty when things fail to compile.
#[test]
fn ensure_no_empty_files() {
    if ensure_no_empty_files_recurse("tests") {
        panic!("Empty files found");
    }
}

fn ensure_no_empty_files_recurse(path: impl AsRef<Path>) -> bool {
    let mut empty_files = false;
    let files = std::fs::read_dir(path).unwrap();
    for file in files.flatten() {
        if file.file_type().unwrap().is_dir() {
            ensure_no_empty_files_recurse(file.path());
        } else {
            let content = std::fs::read_to_string(file.path()).unwrap();
            if content.trim().is_empty() {
                eprintln!("Empty file found: {}", file.path().display());
                empty_files = true;
            }
        }
    }
    empty_files
}

#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}

#[cfg(target_vendor = "apple")]
#[test]
pub fn pass_darwin() {
    macrotest::expand("tests/expand-darwin/*.rs");
}

#[cfg(target_os = "linux")]
#[test]
pub fn pass_linux() {
    macrotest::expand("tests/expand-linux/*.rs");
}

#[test]
pub fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/errors/*.rs");
    t.pass("tests/pass/*.rs");
}
