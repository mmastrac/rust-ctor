#!/usr/bin/env bash
set -xeuo pipefail

cargo test --target $TARGET
