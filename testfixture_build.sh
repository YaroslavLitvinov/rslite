#!/usr/bin/env bash
set -euo pipefail

PROJ=$(pwd)
SRC="${SQLITE_SRC:-/sqlite}"

echo "Building rustfixture (crust-tclsqlite)..."
cd "$PROJ/crust-tclsqlite" && cargo build --release -q --features test

cp "$PROJ/crust-tclsqlite/target/release/rustfixture" "$SRC/rustfixture"
chmod +x "$SRC/rustfixture"

echo "testfixture_build.sh complete: $SRC/rustfixture"
