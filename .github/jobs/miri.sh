#!/usr/bin/env bash
set -xeuo pipefail

cd tests/ctor/edition-2018
cargo +nightly miri run
