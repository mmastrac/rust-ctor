#!/usr/bin/env bash
set -euo pipefail

cargo clippy --examples --bins --all -- ${CLIPPY_LINTS}

cargo +nightly clippy --all-features --examples --bins --all -- ${CLIPPY_LINTS}
