#!/usr/bin/env bash
set -xeuo pipefail

cargo zigbuild --workspace --bins --examples --target "$TARGET"

# Same smoke test as build.sh: run the ctor example (runtime ctor registration).
find target/
exec "target/${TARGET}/debug/examples/example"
