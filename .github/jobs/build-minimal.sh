#!/usr/bin/env bash
set -euo pipefail

# Remove Cargo.lock for testing down-level Rust versions
rm Cargo.lock && cargo run --example example
