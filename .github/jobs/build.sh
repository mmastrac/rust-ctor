#!/usr/bin/env bash
set -euo pipefail

rm Cargo.lock && cargo build
