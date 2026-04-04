#!/usr/bin/env bash
set -euo pipefail

rustup component add miri --toolchain nightly
cd tests/edition-2018
cargo +nightly miri run
