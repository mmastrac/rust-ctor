#!/usr/bin/env bash
set -xeuo pipefail

# https://blog.rust-lang.org/2022/09/15/const-eval-safety-rule-revision/
export RUSTFLAGS="-Z extra-const-ub-checks"

cargo miri test

cd tests/ctor/edition-2018
cargo miri run
cd ../../..

cd tests/link_section/basic
cargo miri run
cd ../../..
