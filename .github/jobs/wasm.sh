#!/usr/bin/env bash
set -euo pipefail

cd tests/ctor/wasm
cargo build --target wasm32-unknown-unknown
wasmtime run target/wasm32-unknown-unknown/debug/tests_wasm.wasm
