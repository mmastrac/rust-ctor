# link-section

A crate for defining link sections in Rust.

Sections are defined using the `#[section]` macro. This creates an associated
`data` and `text` section, and items decorated with the `#[in_section]` macro
are placed into the associated section.

## Platform Support

| Platform                 | Support                                                                                       |
| ------------------------ | --------------------------------------------------------------------------------------------- |
| Linux                    | ✅ Supported, uses orphan section handling (§1)                                               |
| \*BSD                    | ✅ Supported, uses orphan section handling (§1)                                               |
| macOS                    | ✅ Fully supported                                                                            |
| Windows                  | ✅ Fully supported                                                                            |
| WASM                     | ⚠️ Only integers can be stored in sections and may require host environment support (§2) (§3) |
| Other LLVM/GCC platforms | ✅ Supported, uses orphan section handling (§1)                                               |

(§1) Orphan section handling is a feature of the linker that allows sections to
be defined without a pre-defined name.

(§2) Wasm only allows plain bytes in `#[link_section]` statics (no pointers).

(§3) Host environment support (by calling the exported `register_link_section`
function) is required to register each section with the runtime. As a
consequence, the functions available on the `Section` and `TypedSection` types
are not `const`.

## Typed Sections

Typed sections provide a section where all items are of a specific, sized type.
The typed section may be accessed as a slice of the type at zero cost if
desired.

`fn` items are special-cased and stored as function pointers in the typed
section.

## Usage

Create an untyped section using the `#[section]` macro that keeps related items
in close proximity:

```rust
use link_section::{in_section, section};

#[section]
pub static CODE_SECTION: link_section::Section;

#[in_section(CODE_SECTION)]
pub fn link_section_function() {
    println!("link_section_function");
}
```

Create a typed section using the `#[section]` macro that stores items of a
specific, sized type:

```rust
mod my_registry {
    use link_section::{in_section, section};

    pub struct MyStruct {
        name: &'static str,
    }

    #[section]
    pub static MY_REGISTRY: link_section::TypedSection<MyStruct>;

    mod a {
        use super::*;

        #[in_section(MY_REGISTRY)]
        pub static LINKED_MY_STRUCT: MyStruct = MyStruct { name: "my_struct" };
    }

    mod b {
        use super::*;

        #[in_section(MY_REGISTRY)]
        pub static LINKED_MY_STRUCT: MyStruct = MyStruct { name: "my_struct_2" };
    }
}
```
