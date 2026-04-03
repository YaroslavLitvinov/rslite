#!/usr/bin/env bash
set -euo pipefail

PROJ=$(pwd)
# get from env var or use default
SRC="${SQLITE_SRC:-/sqlite}"

mkdir -p "$PROJ/target"
mkdir -p "$PROJ/sqlite-shell"

CARGO_TARGET_DIR="$PROJ/sqlite-shell" cargo build -q --release

echo "Building crust-sqlite-shell (C2Rust transpiled shell)..."
cargo build -q --release -p crust-sqlite-shell

echo "Installing binary to $SRC/sqlite3..."
# The binary is built to the workspace target directory, not the crate's target dir
if [ -f "$PROJ/target/release/sqlite3" ]; then
    cp "$PROJ/target/release/sqlite3" "$SRC/sqlite3"
elif [ -f "$PROJ/crust-sqlite-shell/target/release/sqlite3" ]; then
    cp "$PROJ/crust-sqlite-shell/target/release/sqlite3" "$SRC/sqlite3"
else
    echo "ERROR: sqlite3 binary not found in target directories"
    exit 1
fi
chmod +x "$SRC/sqlite3"

echo "shell_build.sh complete: $SRC/sqlite3"
