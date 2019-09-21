#!/bin/bash
set -euf -o pipefail

if [ -z "${MUSL:-}" ]; then

cargo test --all
cargo test --all --release

if [[ $TRAVIS_OS_NAME == "linux" && $TRAVIS_RUST_VERSION == "nightly" ]]; then
    echo "Testing '-Z sanitizer=address'..."
    # Target needed to avoid "Only executables, staticlibs, cdylibs, dylibs and rlibs can be compiled..."
    RUSTFLAGS="-Z sanitizer=address" cargo run --example example --target x86_64-unknown-linux-gnu
fi

else

rustup target add x86_64-unknown-linux-musl

# We can't test dynamic linking on musl
echo "Testing --target x86_64-unknown-linux-musl..."
cargo run --example example --target x86_64-unknown-linux-musl

fi
