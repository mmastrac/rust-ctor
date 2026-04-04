#!/usr/bin/env bash
set -euo pipefail

cargo build

if [[ "${RUNNER_OS:-}" == "Linux" ]]; then
  RUSTFLAGS="-C target-feature=+crt-static" \
    cargo run --example example --target x86_64-unknown-linux-gnu --verbose
fi
