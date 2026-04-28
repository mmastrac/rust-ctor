Shutdown functions for Rust for Linux, OSX, Windows, mobile (iOS/Android), WASM,
BSD/BSD-likes and many other platforms.

Like `__attribute__((destructor))` in C/C++, but for Rust.

# Examples

Print a message at shutdown time.

```rust
# use dtor::dtor;
# use libc_print::*;
#[dtor(unsafe)]
fn shutdown() {
    // Using println! or eprintln! here may panic as Rust may have
    // shut down some stdlib services at this time.
    libc_println!("Shutting down!");
}
```

# Platform Support

| Platform | Link Section | at_binary_exit | at_module_exit |
| --- | --- | --- | --- |
| Linux | `.fini_array` | Yes (`atexit`) | Yes (`__cxa_atexit`) |
| MacOS | `.mod_term_func` <sup><sup>🍎</sup></sup> | Yes (`atexit`) | Yes (`__cxa_atexit`) |
| Windows | `.CRT$XPU` <sup><sup>🪟</sup></sup> | No | Yes (`atexit`) |
| AIX | No <sup><sup>🔵</sup></sup> | Yes | Yes |
| Other POSIX-like platforms | `.fini_array`/`.dtors` | Yes (`atexit`) | Yes (`__cxa_atexit`) |

Notes:
 - <sup><sup>🍎</sup></sup>: Not recommended. Apple platforms no longer call `mod_term_func` functions.
 - <sup><sup>🪟</sup></sup>: Not recommended. Windows platforms may not reliably call functions in link sections, unless a binary is built with a static CRT.
 - <sup><sup>🔵</sup></sup>: Link sections are not supported on AIX, but the platform calls functions with the prefix `__sinit` and `__sterm` at startup and shutdown respectively.

# Under the Hood

The `#[dtor]` macro effectively creates a constructor that calls `libc::atexit`
with the provided function, i.e. roughly equivalent to:

```rust,ignore
#[ctor]
fn dtor_atexit() {
    libc::atexit(dtor);
}
```

# Crate Features

| Cargo feature | Description |
| --- | --- |
| `no_warn_on_missing_unsafe` |  Do not warn when a ctor or dtor is missing the `unsafe` keyword. |
| `proc_macro` |  Enable support for the proc-macro `#[dtor]` attribute. The declarative form (`dtor!(...)`) is always available. It is recommended that crates re-exporting the `dtor` macro disable this feature and only use the declarative form. |
| `std` |  Enable support for the standard library. |
| `used_linker` |  Applies `used(linker)` to all `dtor`-generated functions. Requires nightly and `feature(used_with_arg)`. |

# Macro Attributes

<table><tr><th>Attribute</th><th>Description</th></tr>
<tr><td><code>anonymous</code></td><td>

 Make the ctor function anonymous.


</td></tr>
<tr><td><code>crate_path = $path : pat</code></td><td>

 Specify a custom crate path for the `dtor` crate. Used when re-exporting the dtor macro.


</td></tr>
<tr><td><code>ctor(link_section = $ctor_link_section_name : literal)</code></td><td>

 Place the initialization function pointer in a custom link section.


</td></tr>
<tr><td><code>link_section = $section : literal</code></td><td>

 Place the destructor function pointer in a custom link section.


</td></tr>
<tr><td><code>method = $method_id : ident</code></td><td>

 Specify the dtor method.

  - `term`: Run the dtor on binary termination. Not recommended as code
    may be unloaded before the dtor is called.
  - `unload`: Run the dtor on module unload (library or binary).
  - `at_module_exit`: Run the dtor using `__cxa_atexit`.
  - `at_binary_exit`: Run the dtor using `atexit` (unsupported on Windows
    platforms).
  - `link_section`: Run the dtor using a custom link section (unsupported
    on Apple platforms).


</td></tr>
<tr><td><code>unsafe</code></td><td>

 Marks a ctor/dtor as unsafe.


</td></tr>
<tr><td><code>used(linker)</code></td><td>

 Mark generated functions for this `dtor` as `used(linker)`. Requires nightly and `feature(used_with_arg)`.


</td></tr>
</table>

# Defaults

## `ctor_link_section`

 ```rust
 # #[cfg(false)] {
#[cfg(target_vendor = "apple")]
 # const _: () = { let
ctor_link_section = "__DATA,__mod_init_func,mod_init_funcs"
 # ; };

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd",
target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly",
target_os = "illumos", target_os = "haiku", target_os = "vxworks", target_os =
"nto", target_family = "wasm"))]
 # const _: () = { let
ctor_link_section = ".init_array"
 # ; };

#[cfg(target_os = "none")]
 # const _: () = { let
ctor_link_section = ".init_array"
 # ; };

#[cfg(target_arch = "xtensa")]
 # const _: () = { let
ctor_link_section = ".ctors"
 # ; };

#[cfg(all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc")))]
 # const _: () = { let
ctor_link_section = ".CRT$XCU"
 # ; };

#[cfg(all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc"))))]
 # const _: () = { let
ctor_link_section = ".ctors"
 # ; };

#[cfg(all(target_os = "aix"))]
 # const _: () = { let
ctor_link_section = ()
 # ; };

 // default
ctor_link_section = (compile_error! ("Unsupported target for #[ctor]"))
 # }
 ```

## `default_term_method`

 ```rust
 # #[cfg(false)] {
#[cfg(target_vendor = "pc")]
 # const _: () = { let
default_term_method = at_module_exit
 # ; };

 // default
default_term_method = at_binary_exit
 # }
 ```

## `default_unload_method`

 ```rust
 # #[cfg(false)] {
 // default
default_unload_method = at_module_exit
 # }
 ```

## `link_section`

 ```rust
 # #[cfg(false)] {
#[cfg(target_vendor = "apple")]
 # const _: () = { let
link_section = "__DATA,__mod_term_func,mod_term_funcs"
 # ; };

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd",
target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly",
target_os = "illumos", target_os = "haiku", target_os = "vxworks", target_os =
"nto", target_family = "wasm"))]
 # const _: () = { let
link_section = ".fini_array"
 # ; };

#[cfg(target_os = "none")]
 # const _: () = { let
link_section = ".fini_array"
 # ; };

#[cfg(target_arch = "xtensa")]
 # const _: () = { let
link_section = ".dtors"
 # ; };

#[cfg(all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc")))]
 # const _: () = { let
link_section = ".CRT$XPU"
 # ; };

#[cfg(all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc"))))]
 # const _: () = { let
link_section = ".dtors"
 # ; };

 // default
link_section = (compile_error! ("Unsupported target for #[dtor]"))
 # }
 ```

## `method`

 ```rust
 # #[cfg(false)] {
#[cfg(target_vendor = "apple")]
 # const _: () = { let
method = at_module_exit
 # ; };

#[cfg(target_vendor = "pc")]
 # const _: () = { let
method = at_module_exit
 # ; };

 // default
method = link_section
 # }
 ```
