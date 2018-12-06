# rust-ctor

`__attribute__((constructor))` for Rust [![Build Status](https://api.travis-ci.org/mmastrac/rust-ctor.svg?branch=master)](https://travis-ci.org/mmastrac/rust-ctor)

Note that this library currently requires the `beta` or `nightly` channel, but should work with `stable` soon (I think!).

## Example

Marks the function `foo` as a module constructor, called when a static
library is loaded or an executable is started:

```
    static INITED: AtomicBool = ATOMIC_BOOL_INIT;

    #[ctor]
    fn foo() {
        INITED.store(true, Ordering::SeqCst);
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