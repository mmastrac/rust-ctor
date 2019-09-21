#!/bin/bash
set -euf -o pipefail
cargo test --all
cargo test --all --release

if [[ $TRAVIS_OS_NAME == "linux" && $TRAVIS_RUST_VERSION == "nightly" ]]; then
    echo "Testing '-Z sanitizer=address'..."
    # Target needed to avoid "Only executables, staticlibs, cdylibs, dylibs and rlibs can be compiled..."
    RUSTFLAGS="-Z sanitizer=address" cargo run --example example --target x86_64-unknown-linux-gnu
fi
