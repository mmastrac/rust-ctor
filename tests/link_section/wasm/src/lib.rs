//! Minimal wasm smoke test for `link-section`: one untyped section, one typed section, and prints.
//!
//! Wasm only allows plain bytes in `#[link_section]` statics (no pointers), so we use `u32` items
//! in both sections and call normal Rust functions to print.

use link_section::{in_section, section};

/// Untyped section with plain `u32` data.
#[section]
pub static UNTYPED: link_section::Section;

#[in_section(UNTYPED)]
pub static UNTYPED_MARKER: u32 = 0x1111_2222;

/// Typed `u32` section.
#[section]
pub static TYPED: link_section::TypedSection<u64>;

#[in_section(TYPED)]
pub static TYPED_VAL: u64 = 0x1234_5678_90AB_CDEF;

fn print_untyped_summary() {
    println!("link_section_untyped_summary");
}

fn print_typed_summary() {
    println!("link_section_typed_summary");
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> u32 {
    print_untyped_summary();
    println!("UNTYPED: {:?}", UNTYPED);
    assert_eq!(UNTYPED_MARKER, 0x1111_2222);

    print_typed_summary();
    println!("TYPED: {:?}", TYPED);
    assert_eq!(TYPED.as_slice(), &[0x1234_5678]);

    println!("link_section_wasm_ok");
    42
}
