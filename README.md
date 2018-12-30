# rust-ctor

[![Build Status](https://api.travis-ci.org/mmastrac/rust-ctor.svg?branch=master)](https://travis-ci.org/mmastrac/rust-ctor)
[![docs.rs](https://docs.rs/ctor/badge.svg)](https://docs.rs/ctor)
[![crates.io](https://img.shields.io/crates/v/ctor.svg)](https://crates.io/crates/ctor)

Module initialization/teardown functions for Rust (like `__attribute__((constructor))` in C/C++) for Linux, OSX, and Windows.

This library currently requires Rust > `1.31.0` at a minimum for the
procedural macro support.

Idea inspired by [this code](https://github.com/neon-bindings/neon/blob/2277e943a619579c144c1da543874f4a7ec39879/src/lib.rs#L42) in the Neon project.

## Support

This library works and has been tested for Linux, OSX and Windows. This
library will also work as expected in both `bin` and `cdylib` outputs, 
ie: the `ctor` and `dtor` will run at executable or library 
startup/shutdown respectively.

## Warnings

Rust's philosophy is that nothing happens before or after main and 
this library explicitly subverts that. The code that runs in the `ctor`
and `dtor` functions should be careful to limit itself to `libc` 
functions and code that does not rely on Rust's stdlib services.

For example, using stdout in a `dtor` function is a guaranteed panic.

In most cases, `sys_common::at_exit` is a better choice than `#[dtor]`. Caveat emptor!

On some platforms, unloading of shared libraries may not actually 
happen until process exit, even if explicitly unloaded. The rules for 
this are arcane and difficult to understand. For example, thread-local
storage on OSX will affect this (see [this comment](https://github.com/rust-lang/rust/issues/28794#issuecomment-368693049)).

## Examples

Marks the function `foo` as a module constructor, called when a static
library is loaded or an executable is started:

```
    static INITED: AtomicBool = ATOMIC_BOOL_INIT;

    #[ctor]
    fn foo() {
        INITED.store(true, Ordering::SeqCst);
    }
```

Print a message at shutdown time. Note that Rust may have shut down
some stdlib services at this time.

```
    #[dtor]
    unsafe fn shutdown() {
        // Using println or eprintln here will panic as Rust has shut down
        libc::printf("Shutting down!\n\0".as_ptr() as *const i8);
    }
```

## Under the Hood

The `#[ctor]` macro makes use of linker sections to ensure that a 
function is run at startup time.

The above example translates into the following Rust code (approximately):

```
    #[used]
    #[cfg_attr(target_os = "linux", link_section = ".ctors")]
    #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
    #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
    pub static foo: extern fn() = { 
        extern fn foo() { ... };
        foo 
    }
```

The `#[dtor]` macro effectively creates a constructor that calls `libc::atexit` with the provided function, ie roughly equivalent to:

```
    #[ctor]
    fn dtor_atexit() {
        libc::atexit(dtor);
    }
```
