#!/bin/bash
set -euf -o pipefail

CLIPPY_OPTS="--all-targets -- -D warnings -D unsafe_code -D clippy::all"

# Test on multiple Rust versions
for version in "nightly" "beta" "stable" "1.74" "nightly-2023-11-16"; do
    echo "Testing on $version..."
    # Install the specified Rust version
    rustup install $version || { echo "Failed to install $version"; exit 1; }
    RUSTFLAGS="-D warnings" cargo +$version test || { echo "$version tests failed"; exit 1; }
    cargo +$version clippy $CLIPPY_OPTS || { echo "$version clippy failed"; exit 1; }
done
