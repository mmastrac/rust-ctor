#!/bin/bash
set
cargo test --all
cargo test --all --release

if [[ $TRAVIS_OS_NAME == "linux" && $TRAVIS_RUST_VERSION == "nightly" ]]; then
    echo "Testing '-Z sanitizer=address'..."
    RUSTFLAGS="-Z sanitizer=address" cargo run --example example
fi
