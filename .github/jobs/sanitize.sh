#!/usr/bin/env bash
set -xeuo pipefail

RUSTFLAGS="-Z sanitizer=address" cargo +nightly run --example example --target $TARGET
