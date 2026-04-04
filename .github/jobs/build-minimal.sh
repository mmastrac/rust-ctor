#!/usr/bin/env bash
set -xeuo pipefail

# Remove Cargo.lock for testing down-level Rust versions
rm Cargo.lock || true

# libc added a minimum 1.65 which makes it difficult to test
cargo update -p libc-print --precise 0.1.10
cargo update -p libc --precise 0.2.96

# Only run the example
cargo clean
cargo run -p ctor --example example
