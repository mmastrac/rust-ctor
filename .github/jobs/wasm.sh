#!/usr/bin/env bash
set -xeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cd "$ROOT/tests/ctor/wasm"
cargo build --target wasm32-unknown-unknown
wasmtime run target/wasm32-unknown-unknown/debug/tests_wasm.wasm
cargo build --target wasm32-wasip1
wasmtime run target/wasm32-wasip1/debug/tests_wasm.wasm || echo "WASI failed"

# We don't have a way to initialize the runtime yet...
cd "$ROOT/tests/link_section/wasm"
cargo build --target wasm32-unknown-unknown
# wasmtime run target/wasm32-unknown-unknown/debug/tests_link_section_wasm.wasm
cargo build --target wasm32-wasip1
# wasmtime run target/wasm32-wasip1/debug/tests_link_section_wasm.wasm || echo "WASI failed"

echo "Done."
