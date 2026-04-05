# link-section

A crate for defining link sections in Rust.

Sections are defined using the `#[section]` macro. This creates an associated
`data` and `text` section, and items decorated with the `#[in_section]` macro are
placed into the associated section.

## Platform Support

| Platform                 | Support                                         |
| ------------------------ | ----------------------------------------------- |
| Linux                    | ✅ Supported, uses orphan section handling (§1) |
| macOS                    | ✅ Fully supported                              |
| Windows                  | ✅ Fully supported                              |
| WASM                     | ✅ Fully supported                              |
| Other LLVM/GCC platforms | ✅ Supported, uses orphan section handling (§1) |

(§1) Orphan section handling is a feature of the linker that allows sections to
be defined without a pre-defined name.

## Usage

```rust
use link_section::{in_section, section};

#[section]
pub static CODE_SECTION: link_section::Section;

#[in_section(CODE_SECTION)]
pub fn link_section_function() {
    println!("link_section_function");
}
```
