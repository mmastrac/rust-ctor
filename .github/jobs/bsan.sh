#!/usr/bin/env bash
set -xeuo pipefail

# BorrowSanitizer (https://borrowsanitizer.com/). Unlike miri, which interprets
# MIR, bsan instruments real machine code, so the link-section/ctor tests that
# miri has to skip do run here.
#
# `cargo bsan` picks the host target itself and forwards a second `--target` if
# we pass one, so none of the commands below specify a target.

# Build the instrumented sysroot up front. It would otherwise happen lazily
# inside the first command, mixing a libstd build into that command's output.
cargo bsan setup

cargo bsan test

cargo bsan run --example "link-section-const"

# Crates outside the workspace, matching the set miri runs.
bsan_crates=(
  tests/ctor/edition-2018
  tests/ctor/priority
  tests/link_section/basic
)
for dir in "${bsan_crates[@]}"; do
  (cd "$dir" && cargo bsan run)
done
