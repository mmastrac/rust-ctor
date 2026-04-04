#!/usr/bin/env bash
set -euo pipefail

cd tests/ctor/edition-2018
cargo +nightly miri run
