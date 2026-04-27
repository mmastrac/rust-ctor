Shutdown functions for Rust for Linux, OSX, Windows, mobile (iOS/Android), WASM,
BSD/BSD-likes and many other platforms.

Like `__attribute__((destructor))` in C/C++, but for Rust.


Print a message at shutdown time.

```rust
#[dtor(unsafe)]
fn shutdown() {
    // Using println! or eprintln! here may panic as Rust may have
    // shut down some stdlib services at this time.
    libc_println!("Shutting down!");
}
```


| Platform | Link Section | at_binary_exit | at_module_exit |
| --- | --- | --- | --- |
| Linux | `.fini_array`/`.dtors` | Yes (`atexit`) | Yes (`__cxa_atexit`) |
| MacOS | `.mod_term_func` 🍎 | Yes (`atexit`) | Yes (`__cxa_atexit`) |
| Windows | `.CRT$XPU` 🪟 | No | Yes (`atexit`) |
| AIX | No 🔵 | Yes | Yes |
| Other POSIX-like platforms | `.fini_array`/`.dtors` | Yes (`atexit`) | Yes (`__cxa_atexit`) |

Notes:
 - 🍎: Not recommended. Apple platforms no longer call `mod_term_func` functions.
 - 🪟: Not recommended. Windows platforms may not reliably call functions in link sections, unless a binary is built with a static CRT.
 - 🔵: Link sections are not supported on AIX, but `__sinit` functions can be used to call `atexit`.


The `#[dtor]` macro effectively creates a constructor that calls `libc::atexit`
with the provided function, i.e. roughly equivalent to:

```rust,ignore
#[ctor]
fn dtor_atexit() {
    libc::atexit(dtor);
}
```


| Cargo feature | Description |
| --- | --- |
| `no_warn_on_missing_unsafe` |  Do not warn when a ctor or dtor is missing the `unsafe` keyword. |
| `proc_macro` |  Enable support for the proc-macro `#[dtor]` attribute. The declarative form (`dtor!(...)`) is always available. It is recommended that crates re-exporting the `dtor` macro disable this feature and only use the declarative form. |
| `std` |  Enable support for the standard library. |
| `used_linker` |  Applies `used(linker)` to all `dtor`-generated functions. Requires nightly and `feature(used_with_arg)`. |


| Attribute | Description |
| --- | --- |
| `anonymous` |  Make the ctor function anonymous. |
| `crate_path = $path : pat` |  Specify a custom crate path for the `dtor` crate. Used when re-exporting the dtor macro. |
| `ctor(link_section = $ctor_link_section_name : literal)` |  Place the initialization function pointer in a custom link section. |
| `link_section = $section : literal` |  Place the destructor function pointer in a custom link section. |
| `method = $method_id : ident` |  Specify the dtor method.  - `term`: Run the dtor on binary termination. Not recommended as code may be unloaded before the dtor is called.  - `unload`: Run the dtor on module unload (library or binary).  - `at_library_exit`: Run the dtor using `__cxa_atexit` (unsupported on Windows platforms).  - `at_binary_exit`: Run the dtor using `atexit`.  - `link_section`: Run the dtor using a custom link section (unsupported on Apple platforms). |
| `unsafe` |  Marks a ctor/dtor as unsafe. |
| `used(linker)` |  Mark generated functions for this `dtor` as `used(linker)`. Requires nightly and `feature(used_with_arg)`. |


## `ctor_link_section`

 ```rust
#[cfg(target_vendor = "apple")]
ctor_link_section = "__DATA,__mod_init_func,mod_init_funcs"

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd",
target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly",
target_os = "illumos", target_os = "haiku", target_family = "wasm"))]
ctor_link_section = ".init_array"

#[cfg(target_arch = "xtensa")]
ctor_link_section = ".ctors"

#[cfg(all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc")))]
ctor_link_section = ".CRT$XPU"

#[cfg(all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc"))))]
ctor_link_section = ".ctors"

 // default
ctor_link_section = (compile_error! ("Unsupported target for #[ctor]"))
 ```

## `default_term_method`

 ```rust
#[cfg(target_vendor = "pc")]
default_term_method = at_library_exit

 // default
default_term_method = at_binary_exit
 ```

## `default_unload_method`

 ```rust
 // default
default_unload_method = at_module_exit
 ```

## `link_section`

 ```rust
#[cfg(target_vendor = "apple")]
link_section = "__DATA,__mod_term_func,mod_term_funcs"

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd",
target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly",
target_os = "illumos", target_os = "haiku", target_family = "wasm"))]
link_section = ".fini_array"

#[cfg(target_arch = "xtensa")]
link_section = ".dtors"

#[cfg(all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc")))]
link_section = ".CRT$XTU"

#[cfg(all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc"))))]
link_section = ".dtors"

 // default
link_section = (compile_error! ("Unsupported target for #[dtor]"))
 ```

## `method`

 ```rust
#[cfg(target_vendor = "apple")]
method = at_library_exit

 // default
method = link_section
 ```
