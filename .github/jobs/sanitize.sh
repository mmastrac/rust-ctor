#!/usr/bin/env bash
set -xeuo pipefail

RUSTFLAGS="-Z sanitizer=address" cargo +nightly run -p ctor --example example --target $TARGET
RUSTFLAGS="-Z sanitizer=address" cargo +nightly run -p link-section --example example --target $TARGET
