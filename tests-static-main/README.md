# Static linking example

Example showing how static linking may be complicated. Without the `import` feature,
the `#[ctor]` function will not be run.

```
cargo run -p tests-static-main --features=import
```

On MacOS, you may need to link with `rust-lld`:

```
RUSTFLAGS="-Clinker=rust-lld" cargo run -p tests-static-main --features=import
```

See https://github.com/rust-lang/rust/issues/133491 for more information.
