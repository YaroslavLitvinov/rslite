#!/bin/bash
# Updated build_all.sh that uses C2Rust transpiled Rust shell
#
# This version builds the SQLite library + transpiled Rust shell instead of C shell
# Replace the original build_all.sh with this file once Rust shell is fully tested

set -euo pipefail

export SQLITE_SRC="${SQLITE_SRC:-/sqlite}"
# Content-hash build cache: if src/ is identical to a previously passing
# build+test run, skip the expensive recompile and test execution.
_PROJ=$(cd "$(dirname "$0")" && pwd)
_BUILD_CACHE_DIR="/tmp/build_all_cache"
mkdir -p "$_BUILD_CACHE_DIR"

_calc_hash() {
    find "$_PROJ" -name "*.rs" -not -path "*/target/*" -print0 \
        | sort -z \
        | (xargs -0 sha256sum 2>/dev/null || true) \
        | sha256sum \
        | awk '{print $1}'
}

while true; do
    _SRC_HASH=$(_calc_hash)
    _CACHE_FILE="$_BUILD_CACHE_DIR/$_SRC_HASH"

    if [ -f "$_CACHE_FILE" ]; then
        echo "build_all: cache hit — build+tests previously passed for this src/ (${_SRC_HASH:0:8})"
        exit 0
    fi

    echo "build_all: starting build for src/ hash ${_SRC_HASH:0:8}"

    cd "$_PROJ"

    # Step 1: Build Rust library
    echo "Step 1/4: Building Rust SQLite library..."
    mkdir -p "$_PROJ/target"
    mkdir -p "$_PROJ/sqlite-shell"
    CARGO_TARGET_DIR="$_PROJ/sqlite-shell" cargo build -q --release

    # Step 2: Build Rust shell (c2rust transpilation)
    echo "Step 2/4: Building C2Rust transpiled shell..."
    cd "$_PROJ/crust-sqlite-shell"
    cargo build -q --release

    # Copy Rust shell binary to SQLite source directory for tests
    cp target/release/sqlite3 "$SQLITE_SRC/sqlite3"
    echo "  ✓ SQLite shell installed: $SQLITE_SRC/sqlite3"

    # Step 3: Build test fixture
    cd "$_PROJ"
    echo "Step 3/4: Building test fixture..."
    ./testfixture_build.sh

    # Step 4: Run tests
    echo "Step 4/4: Running SQLite test suite..."
    ( cd "$SQLITE_SRC" && ./rustfixture test/testrunner.tcl 2>&1 | tee /tmp/test_output.log )

    # Check that tests passed (must have exactly "0 errors out of")
    if grep -E "^0 errors out of" /tmp/test_output.log > /dev/null 2>&1; then
        _POST_HASH=$(_calc_hash)
        if [ "$_POST_HASH" != "_SRC_HASH" ]; then
            echo "build_all: src/ changed during build (${_SRC_HASH:0:8} → ${_POST_HASH:0:8}), restarting..."
            continue
        fi
        echo "passed" > "$_CACHE_FILE"
        echo "✓ All tests passed"
        exit 0
    else
        echo "FAILURE: Tests failed - found errors in test output"
        echo "See /tmp/test_output.log for details"
        exit 1
    fi
done
