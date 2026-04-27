#!/usr/bin/env bash
set -xeuo pipefail

# Remove Cargo.lock for testing down-level Rust versions
rm Cargo.lock || true

# Some of the development dependencies are not available for the minimal Rust version
cargo clean
cargo run -p ctor --example ctor-example --target $TARGET
