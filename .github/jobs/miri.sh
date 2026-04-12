#!/usr/bin/env bash
set -xeuo pipefail

cargo miri test

cd tests/ctor/edition-2018
cargo miri run
cd ../../..

cd tests/link_section/basic
cargo miri run
cd ../../..
