#!/usr/bin/env bash
set -xeuo pipefail

# Remove Cargo.lock for testing down-level Rust versions
rm Cargo.lock || true

cargo clean
cargo build
cargo run -p ctor --example ctor-example --target $TARGET
