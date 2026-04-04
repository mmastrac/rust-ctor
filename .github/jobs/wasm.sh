#!/usr/bin/env bash
set -xeuo pipefail

cd tests/ctor/wasm
echo "wasm32-unknown-unknown..."
cargo build --target wasm32-unknown-unknown
wasmtime run target/wasm32-unknown-unknown/debug/tests_wasm.wasm
echo "wasm32-wasip1..."
cargo build --target wasm32-wasip1
wasmtime run target/wasm32-wasip1/debug/tests_wasm.wasm || echo "WASI failed"
echo "Done."
