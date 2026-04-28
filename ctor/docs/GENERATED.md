Module initialization functions for Rust (like `__attribute__((constructor))` in
C/C++) for Linux, OSX, Windows, WASM, BSD-likes, and many others.

```rust
use ctor::ctor;

#[ctor(unsafe)]
fn foo() {
    println!("Life before main!");
}
```

## MSRV

For most platforms, this library currently has a MSRV of **Rust >= 1.60**.

MSRV for WASM targets is **Rust >= 1.85**.

## Lightweight

`ctor` has no dependencies other than the `ctor-proc-macro` and `link-section`
crates. The proc-macro is only used to delegate to the declarative macro and
should have minimal effect on compilation time.

## Support

This library works and is regularly tested on Linux, OSX, Windows, and FreeBSD,
with both `+crt-static` and `-crt-static` and `bin`/`cdylib` outputs.

Contributions to support other platforms or improve testing are welcome.

| OS           | Supported | CI Tested |
| ------------ | --------- | --------- |
| Linux        | ✅        | ✅        |
| OSX          | ✅        | ✅        |
| Windows      | ✅        | ✅        |
| FreeBSD      | ✅        | ✅        |
| WASM         | ✅        | ✅        |
| NetBSD       | ✅        | -         |
| OpenBSD      | ✅        | -         |
| DragonFlyBSD | ✅        | -         |
| Illumos      | ✅        | -         |
| Android      | ✅        | -         |
| iOS          | ✅        | -         |
| AIX          | ✅        | -         |
| Haiku        | ✅        | -         |
| VxWorks      | ✅        | -         |
| Xtensa       | ✅        | -         |
| NTO          | ✅        | -         |

## Warnings

Rust's philosophy is that nothing happens before or after main and this library
explicitly subverts that. The code that runs in the `ctor` and `dtor` functions
should be careful to limit itself to `libc` functions and code that does not
rely on Rust's stdlib services.

For example, using stdout in a `dtor` function is a guaranteed panic. Consider
using the [`libc-print` crate](https://crates.io/crates/libc-print) for output
to stderr/stdout during `#[ctor]` and `#[dtor]` methods. Other issues may
involve signal processing or panic handling in that early code.

Some linker configurations may cause `#[ctor]` and `#[dtor]` functions to be
stripped from the final binary. The `used_linker` feature may prevent this, but
is not supported outside of nightly Rust. Often, a simple `use module_with_ctor`
is sufficient to ensure the linker does not strip the function.

On some platforms, unloading of shared libraries may not actually happen until
process exit, even if explicitly unloaded. The rules for this are arcane and
difficult to understand. For example, thread-local storage on OSX will affect
this (see
[this comment](https://github.com/rust-lang/rust/issues/28794#issuecomment-368693049)).

## Examples

Marks the function `foo` as a module constructor, called when a static library
is loaded or an executable is started:

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use ctor::ctor;

static INITED: AtomicBool = AtomicBool::new(false);

#[ctor]
fn foo() {
    INITED.store(true, Ordering::SeqCst);
}
```

Creates a `HashMap` populated with strings when a static library is loaded or an
executable is started (new in `0.1.7`):

`static` items are equivalent to `std::sync::OnceLock`, with an automatic deref
implementation and eager initialization at startup time. `#[ctor]` on `static`
items requires the default `std` feature.

```rust
use std::collections::HashMap;
use ctor::ctor;

#[ctor]
/// This is an immutable static, evaluated at init time
static STATIC_CTOR: HashMap<u32, &'static str> = {
    let mut m = HashMap::new();
    m.insert(0, "foo");
    m.insert(1, "bar");
    m.insert(2, "baz");
    m
};
```

Print a message at shutdown time. Note that Rust may have shut down some stdlib
services at this time.

```rust,ignore
use libc::printf;
use ctor::dtor;

#[dtor]
unsafe fn shutdown() {
    // Using println or eprintln here will panic as Rust has shut down
    libc::printf(c"Shutting down!\n" as _);
}
```

## Under the Hood

The `#[ctor]` macro makes use of linker sections to ensure that a function is
run at startup time.

The above example translates into the following Rust code (approximately):

```rust
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func,mod_init_funcs")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
/* ... other platforms elided ... */
static FOO: extern fn() = {
    extern fn foo() { /* ... */ };
    foo
};
```

## Inspiration

The idea for `ctor` was originally inspired by the Neon project.

# Crate Features

| Cargo feature | Description |
| --- | --- |
| `dtor` |  Enable support for the `#[dtor]` attribute. Deprecated: use the `dtor` crate directly instead. |
| `no_warn_on_missing_unsafe` |  Do not warn when a ctor or dtor is missing the `unsafe` keyword. |
| `priority_enabled` |  Enable support for the priority parameter. |
| `proc_macro` |  Enable support for the proc-macro `#[dtor]` attribute. The declarative form (`dtor!(...)`) is always available. It is recommended that crates re-exporting the `dtor` macro disable this feature and only use the declarative form. |
| `std` |  Enable support for the standard library. |
| `used_linker` |  Applies `used(linker)` to all `dtor`-generated functions. Requires nightly and `feature(used_with_arg)`. |

# Macro Attributes

<table><tr><th>Attribute</th><th>Description</th></tr>
<tr><td><code>anonymous</code></td><td>

 Do not give the constructor a name in the generated code (allows for
 multiple constructors with the same name). Equivalent to wrapping the
 constructor in an anonymous const (i.e.: `const _ = { ... };`).


</td></tr>
<tr><td><code>crate_path = $path : pat</code></td><td>

 The path to the `ctor` crate containing the support macros. If you
 re-export `ctor` items as part of your crate, you can use this to
 redirect the macro’s output to the correct crate.

 Using the declarative [`ctor!`][c] form is
 preferred over this parameter.

 [c]: crate::declarative::ctor!


</td></tr>
<tr><td><code>export_name_prefix = $export_name_prefix_str : literal</code></td><td>

 Specify a custom export name prefix for the constructor function.

 If specified, an export with the given prefix will be generated in the form:

 `<prefix><priority>_<unique_id>`


</td></tr>
<tr><td><code>link_section = $section : literal</code></td><td>

 Place the constructor function pointer in a custom link section. By
 default, this uses the appropriate platform-specific link section.


</td></tr>
<tr><td><code>unsafe</code></td><td>

 Marks a ctor/dtor as unsafe. Recommended.


</td></tr>
<tr><td><code>priority = $priority_value : tt</code></td><td>

 The priority of the constructor. Higher-`N`-priority constructors are
 run last. `N` must be between 0 and 999 for ordering guarantees (`N` >=
 1000 ordering is platform-defined).

 Ordering with respect to constructors without a priority is
 platform-defined.


</td></tr>
<tr><td><code>used(linker)</code></td><td>

 Mark generated functions for this `dtor` as `used(linker)`. Requires nightly and `feature(used_with_arg)`.


</td></tr>
</table>

# Defaults

## `export_name_prefix`

 ```rust
 # #[cfg(false)] {
#[cfg(target_os = "aix")]
 # const _: () = { let
export_name_prefix = "__sinit"
 # ; };

 // default
export_name_prefix = ()
 # }
 ```

## `link_section`

 ```rust
 # #[cfg(false)] {
#[cfg(target_vendor = "apple")]
 # const _: () = { let
link_section = "__DATA,__mod_init_func,mod_init_funcs"
 # ; };

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd",
target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly",
target_os = "illumos", target_os = "haiku", target_os = "vxworks", target_os =
"nto", target_family = "wasm"))]
 # const _: () = { let
link_section = ".init_array"
 # ; };

#[cfg(target_os = "none")]
 # const _: () = { let
link_section = ".init_array"
 # ; };

#[cfg(target_arch = "xtensa")]
 # const _: () = { let
link_section = ".ctors"
 # ; };

#[cfg(all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc")))]
 # const _: () = { let
link_section = ".CRT$XCU"
 # ; };

#[cfg(all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc"))))]
 # const _: () = { let
link_section = ".ctors"
 # ; };

#[cfg(all(target_os = "aix"))]
 # const _: () = { let
link_section = ()
 # ; };

 // default
link_section = (compile_error! ("Unsupported target for #[ctor]"))
 # }
 ```
