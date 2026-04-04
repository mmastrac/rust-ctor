#!/usr/bin/env bash
set -xeuo pipefail

cargo clippy --examples --bins --all -- ${CLIPPY_LINTS}

cargo +nightly clippy --all-features --examples --bins --all -- ${CLIPPY_LINTS}
