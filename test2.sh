#!/bin/bash
set -euf -o pipefail

CLIPPY_OPTS="--all-targets -- -D warnings -D unsafe_code -D clippy::all"

# Kill all background processes on exit
trap "echo 'Killing all background processes...'; pkill -P $$; exit 0" EXIT

# Test on multiple Rust versions
for version in "nightly" "beta" "stable" "1.74" "nightly-2023-11-16"; do
    echo "Testing on $version..."
    # Install the specified Rust version
    (
        LOG_FILE="`pwd`/target/system-test/$version.log"

        # Copy source tree to version-specific directory
        rm -rf "target/system-test/$version" 2>/dev/null || true
        mkdir -p "target/system-test/$version"
        rsync -a --exclude 'target' . "target/system-test/$version/"
        rm -f "target/system-test/$version/Cargo.lock"
        cd "target/system-test/$version"

        rustup install $version > "$LOG_FILE" 2>&1 || { echo "Failed to install $version"; exit 1; }
        RUSTFLAGS="-D warnings" cargo +$version test >> "$LOG_FILE" 2>&1 || { echo "$version tests failed"; exit 1; }
        cargo +$version clippy $CLIPPY_OPTS >> "$LOG_FILE" 2>&1 || { echo "$version clippy failed"; exit 1; }
    ) &
done

wait
