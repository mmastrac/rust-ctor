#!/usr/bin/env bash
set -xeuo pipefail

cargo clippy --examples --bins --all --target $TARGET -- ${CLIPPY_LINTS}
cargo fmt --check --all

