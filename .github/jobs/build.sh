#!/usr/bin/env bash
set -xeuo pipefail

# Remove Cargo.lock for testing down-level Rust versions
rm Cargo.lock
# May need to rebuild when beta/nightly changes
cargo build || (cargo clean && cargo build)
cargo run -p ctor --example example
