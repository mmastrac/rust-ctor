# link-section

A crate for defining link sections in Rust.

## Usage

```rust
use link_section::{in_section, section};

#[section(code)]
pub static CODE_SECTION: link_section::Section;

#[in_section(CODE_SECTION)]
pub fn link_section_function() {
    println!("link_section_function");
}
```
