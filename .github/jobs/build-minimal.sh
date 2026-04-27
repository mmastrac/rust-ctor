#!/usr/bin/env bash
set -xeuo pipefail

# Remove Cargo.lock for testing down-level Rust versions
rm Cargo.lock || true

cd tests/ctor/edition-2018
cargo run --target $TARGET
cd ../../..

cd tests/ctor/edition-2021
cargo run --target $TARGET
cd ../../..
