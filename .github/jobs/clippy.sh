#!/usr/bin/env bash
set -euo pipefail

cargo clippy --examples --bins --all -- ${CLIPPY_LINTS}

rustup toolchain add nightly
rustup component add clippy --toolchain nightly
cargo +nightly clippy --all-features --examples --bins --all -- ${CLIPPY_LINTS}
