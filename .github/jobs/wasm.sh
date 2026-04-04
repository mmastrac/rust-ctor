#!/usr/bin/env bash
set -euo pipefail

rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown -p tests-wasm
wasmtime run target/wasm32-unknown-unknown/debug/tests_wasm.wasm
