#!/usr/bin/env bash
set -xeuo pipefail

# Remove Cargo.lock for testing down-level Rust versions
rm Cargo.lock || true

# Only run the example
cargo clean
cargo run -p ctor --example msrv
