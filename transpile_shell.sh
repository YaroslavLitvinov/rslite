#!/usr/bin/env bash
# Transpile shell.c using c2rust from /c2rust repository
set -euo pipefail

PROJ_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
C2RUST_BIN="/c2rust/target/release/c2rust"
C2RUST_CONFIG="$PROJ_DIR/c2rust-shell.toml"
SHELL_SRC="$PROJ_DIR/sqlite-shell-src/shell.c"
OUTPUT_DIR="$PROJ_DIR/crust-sqlite-shell/src/generated"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║            SQLite Shell C-to-Rust Transpilation            ║"
echo "║                    Using c2rust 0.22.1                     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Verify prerequisites
echo "Step 1: Verifying prerequisites..."

if [ ! -f "$C2RUST_BIN" ]; then
    echo "ERROR: c2rust binary not found at $C2RUST_BIN"
    echo "Status: c2rust build in progress (check background task)"
    echo "Or build manually: cd /c2rust && cargo build --release"
    exit 1
fi

if [ ! -f "$C2RUST_CONFIG" ]; then
    echo "ERROR: c2rust config not found at $C2RUST_CONFIG"
    exit 1
fi

if [ ! -f "$SHELL_SRC" ]; then
    echo "ERROR: shell.c not found at $SHELL_SRC"
    exit 1
fi

echo "  ✓ c2rust binary: $C2RUST_BIN"
echo "  ✓ Config file: $C2RUST_CONFIG"
echo "  ✓ Shell source: $SHELL_SRC"
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"
echo "Step 2: Running transpilation..."
echo "  Input: $SHELL_SRC (~33,890 lines)"
echo "  Output: $OUTPUT_DIR/"
echo "  Config: $C2RUST_CONFIG"
echo ""

# Run transpilation
cd "$PROJ_DIR"
if "$C2RUST_BIN" transpile "$C2RUST_CONFIG"; then
    echo ""
    echo "✓ Transpilation successful!"
    echo ""
    echo "Step 3: Generated files:"
    find "$OUTPUT_DIR" -name "*.rs" | wc -l | xargs echo "  Generated Rust files:"
    find "$OUTPUT_DIR" -name "*.rs" -exec wc -l {} + | tail -1 | awk '{print "  Total lines: " $1}'
    echo ""
    echo "Next steps:"
    echo "  1. Review generated code: $OUTPUT_DIR/"
    echo "  2. Build: cd crust-sqlite-shell && cargo build --release"
    echo "  3. Test: ./crust-sqlite-shell/target/release/sqlite3 :memory: \"SELECT 1;\""
    exit 0
else
    echo ""
    echo "ERROR: Transpilation failed!"
    echo "See output above for error details."
    exit 1
fi
