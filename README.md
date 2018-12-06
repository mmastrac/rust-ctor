# rust-ctor

`__attribute__((constructor))` for Rust

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
