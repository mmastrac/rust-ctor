#!/usr/bin/env bash
set -xeuo pipefail

cargo zigbuild --workspace --bins --examples --target "$TARGET"

# Same smoke test as build.sh: run the ctor example (runtime ctor registration).
echo "Running basic example..."
target/${TARGET}/debug/examples/basic

echo "Running example..."
target/${TARGET}/debug/examples/example

echo "Running link-section example..."
target/${TARGET}/debug/examples/link-section-example
