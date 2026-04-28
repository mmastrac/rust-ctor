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

## Usage

`#[ctor]` decorates a function item to be called as a module constructor. Both
free (a global `fn()`) and impl functions (`Self::method()`) are supported.

The example below marks the function `foo` as a module constructor, called when
a static library is loaded or an executable is started:

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use ctor::ctor;

static INITED: AtomicBool = AtomicBool::new(false);

#[ctor(unsafe)]
fn foo() {
    // ... (do something)
    INITED.store(true, Ordering::SeqCst);
}
```

Implementation methods can also be decorated with `#[ctor]`, as long as they
have no `self` parameter:

```rust
use ctor::ctor;

struct MyStruct {
    // ...
}

impl MyStruct {
    /// Ensure the required C library is loaded at startup time.
    #[ctor(unsafe)]
    fn load_required_c_library() {
        // ... (do something)
    }
}
```

### `static` items

The `#[ctor]` macro also supports decorating `static` items, which are
initialized at startup time. `static` items declared in this way must not be
accessed from other threads before the module constructors have run (if this is
done without caution, the initializer may panic).

The below example creates a `HashMap` populated with strings, which would
normally not be possible with `const` items:

```rust
use std::collections::HashMap;
use ctor::ctor;

#[ctor(unsafe)]
/// This is an immutable static, evaluated at init time
static STATIC_CTOR: HashMap<u32, &'static str> = {
    let mut m = HashMap::new();
    m.insert(0, "foo");
    m.insert(1, "bar");
    m.insert(2, "baz");
    m
};
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
