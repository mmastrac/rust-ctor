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
| Linux | `.fini_array`/`.dtors` | Yes (`atexit`) | Yes (`__cxa_atexit`) |
| MacOS | `.mod_term_func` 🍎 | Yes (`atexit`) | Yes (`__cxa_atexit`) |
| Windows | `.CRT$XPU` 🪟 | No | Yes (`atexit`) |
| AIX | No 🔵 | Yes | Yes |
| Other POSIX-like platforms | `.fini_array`/`.dtors` | Yes (`atexit`) | Yes (`__cxa_atexit`) |

Notes:
 - 🍎: Not recommended. Apple platforms no longer call `mod_term_func` functions.
 - 🪟: Not recommended. Windows platforms may not reliably call functions in link sections, unless a binary is built with a static CRT.
 - 🔵: Link sections are not supported on AIX, but `__sinit` functions can be used to call `atexit`.

# Under the Hood

The `#[dtor]` macro effectively creates a constructor that calls `libc::atexit`
with the provided function, i.e. roughly equivalent to:

```rust,ignore
#[ctor]
fn dtor_atexit() {
    libc::atexit(dtor);
}
```
