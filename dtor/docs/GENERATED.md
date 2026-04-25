Module teardown functions for Rust (like `__attribute__((destructor))` in C/C++)
for Linux, OSX, FreeBSD, NetBSD, Illumos, OpenBSD, DragonFlyBSD, Android, iOS,
WASM, and Windows.

# Examples

Print a message at shutdown time. Note that Rust may have shut down some stdlib
services at this time.

```rust,ignore
#[dtor]
unsafe fn shutdown() {
    // Using println or eprintln here will panic as Rust has shut down
    libc::printf("Shutting down!\n\0".as_ptr() as *const i8);
}
```

# Under the Hood

The `#[dtor]` macro effectively creates a constructor that calls `libc::atexit`
with the provided function, ie roughly equivalent to:

```rust,ignore
#[ctor]
fn dtor_atexit() {
    libc::atexit(dtor);
}
```
# Features
<code>"no_warn_on_missing_unsafe"</code>: 

<code>"proc_macro"</code>: 

<code>"std"</code>: 

<code>"used_linker"</code>: 

