#!/usr/bin/env bash
set -xeuo pipefail

cargo zigbuild --workspace --target "$TARGET"

# Same smoke test as build.sh: run the ctor example (runtime ctor registration).
exec "target/${TARGET}/debug/examples/example"
