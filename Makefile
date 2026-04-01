# Makefile for SQLite Shell C-to-Rust Migration using c2rust
# Automates project setup, transpilation, building, and testing

.PHONY: help setup install-tools create-project transpile build test clean analyze benchmark help-targets

# Color output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m # No Color

# Configuration
SHELL_C := shell.c
RUST_PROJECT := sqlite-shell-rs
RUST_BIN := sqlite3
CARGO_PROFILE ?= release
C2RUST_VERSION ?= latest

# Paths
PROJECT_DIR := $(PWD)
RUST_SRC := $(RUST_PROJECT)/src
GENERATED_DIR := $(RUST_SRC)/generated
BUILD_DIR := $(RUST_PROJECT)/target/$(CARGO_PROFILE)

help:
	@echo "$(BLUE)=== SQLite Shell C-to-Rust Migration Makefile ===$(NC)"
	@echo ""
	@echo "$(GREEN)Setup Targets:$(NC)"
	@echo "  $(YELLOW)make setup$(NC)              - Complete setup (install tools, create project)"
	@echo "  $(YELLOW)make install-tools$(NC)     - Install c2rust and dependencies"
	@echo "  $(YELLOW)make create-project$(NC)    - Create Rust project structure"
	@echo "  $(YELLOW)make create-config$(NC)     - Create c2rust.toml configuration"
	@echo ""
	@echo "$(GREEN)Transpilation & Build Targets:$(NC)"
	@echo "  $(YELLOW)make transpile$(NC)         - Transpile shell.c to Rust"
	@echo "  $(YELLOW)make build$(NC)             - Build Rust project"
	@echo "  $(YELLOW)make build-release$(NC)     - Build optimized release binary"
	@echo "  $(YELLOW)make build-debug$(NC)       - Build debug binary with symbols"
	@echo ""
	@echo "$(GREEN)Testing Targets:$(NC)"
	@echo "  $(YELLOW)make test$(NC)              - Run all tests"
	@echo "  $(YELLOW)make test-unit$(NC)         - Run unit tests only"
	@echo "  $(YELLOW)make test-integration$(NC)  - Run integration tests only"
	@echo "  $(YELLOW)make test-suite$(NC)        - Run SQLite test suite"
	@echo ""
	@echo "$(GREEN)Analysis & Benchmarking:$(NC)"
	@echo "  $(YELLOW)make analyze$(NC)           - Analyze shell.c for migration scope"
	@echo "  $(YELLOW)make benchmark$(NC)         - Compare C vs Rust performance"
	@echo "  $(YELLOW)make clippy$(NC)            - Run clippy linter"
	@echo "  $(YELLOW)make check-unsafe$(NC)      - Check for unsafe code"
	@echo ""
	@echo "$(GREEN)Cleanup:$(NC)"
	@echo "  $(YELLOW)make clean$(NC)             - Clean build artifacts"
	@echo "  $(YELLOW)make clean-project$(NC)     - Remove entire Rust project"
	@echo "  $(YELLOW)make distclean$(NC)         - Full cleanup (tools, project, artifacts)"
	@echo ""

# ============================================================================
# SETUP TARGETS
# ============================================================================

setup: install-tools create-project create-config
	@echo "$(GREEN)✓ Setup complete!$(NC)"
	@echo "$(BLUE)Next steps:$(NC)"
	@echo "  1. Review $(YELLOW)c2rust.toml$(NC) configuration"
	@echo "  2. Run $(YELLOW)make transpile$(NC) to transpile shell.c"
	@echo "  3. Run $(YELLOW)make build$(NC) to build the project"

install-tools:
	@echo "$(BLUE)Installing c2rust...$(NC)"
	@command -v c2rust >/dev/null 2>&1 || cargo install c2rust
	@c2rust --version
	@echo "$(GREEN)✓ c2rust installed$(NC)"

create-project:
	@echo "$(BLUE)Creating Rust project structure...$(NC)"
	@if [ -d "$(RUST_PROJECT)" ]; then \
		echo "$(YELLOW)Project $(RUST_PROJECT) already exists, skipping creation$(NC)"; \
	else \
		cargo new --name $(RUST_BIN) $(RUST_PROJECT); \
		echo "$(GREEN)✓ Project created$(NC)"; \
	fi
	@mkdir -p $(GENERATED_DIR)
	@mkdir -p $(RUST_PROJECT)/tests
	@mkdir -p $(RUST_PROJECT)/tests/fixtures
	@echo "$(GREEN)✓ Directory structure created$(NC)"

create-config:
	@echo "$(BLUE)Creating c2rust.toml...$(NC)"
	@test -f c2rust.toml || (cat > c2rust.toml << 'EOF'
[transpilation]
# Emit Rust code that compiles
emit_modules = true
emit_extern_crates = true
use_std = true

# Paths
source_files = ["shell.c"]
output_dir = "$(RUST_PROJECT)/src/generated/"

# Compilation settings
compile_flags = [
  "-DSQLITE_HAVE_ZLIB=1",
  "-DSQLITE_ENABLE_FTS4",
  "-DSQLITE_ENABLE_RTREE",
  "-DSQLITE_ENABLE_EXPLAIN_COMMENTS",
  "-I.",
  "-I./src",
]

[c2rust.options]
simplify_extern_crates = true
preserve_comments = true
emit_panic_on_fail = true
EOF
)
	@echo "$(GREEN)✓ c2rust.toml created$(NC)"

# ============================================================================
# TRANSPILATION TARGETS
# ============================================================================

transpile: $(SHELL_C)
	@echo "$(BLUE)Transpiling $(SHELL_C) to Rust...$(NC)"
	@if [ ! -f "$(SHELL_C)" ]; then \
		echo "$(RED)Error: $(SHELL_C) not found$(NC)"; \
		exit 1; \
	fi
	@c2rust transpile c2rust.toml
	@echo "$(GREEN)✓ Transpilation complete$(NC)"
	@echo "$(YELLOW)⚠ Review generated code in $(GENERATED_DIR)$(NC)"

transpile-verbose:
	@echo "$(BLUE)Transpiling with verbose output...$(NC)"
	c2rust transpile --verbose c2rust.toml

transpile-dry-run:
	@echo "$(BLUE)Transpile dry-run (shows what would be transpiled)...$(NC)"
	c2rust transpile --dry-run c2rust.toml

# ============================================================================
# BUILD TARGETS
# ============================================================================

build: build-$(CARGO_PROFILE)

build-release:
	@echo "$(BLUE)Building optimized release binary...$(NC)"
	cd $(RUST_PROJECT) && cargo build --release
	@echo "$(GREEN)✓ Release build complete$(NC)"
	@echo "Binary: $(BUILD_DIR)/$(RUST_BIN)"

build-debug:
	@echo "$(BLUE)Building debug binary...$(NC)"
	cd $(RUST_PROJECT) && cargo build
	@echo "$(GREEN)✓ Debug build complete$(NC)"
	@echo "Binary: $(RUST_PROJECT)/target/debug/$(RUST_BIN)"

build-check:
	@echo "$(BLUE)Checking if code compiles...$(NC)"
	cd $(RUST_PROJECT) && cargo check
	@echo "$(GREEN)✓ Code compiles$(NC)"

# ============================================================================
# TESTING TARGETS
# ============================================================================

test: test-unit test-integration
	@echo "$(GREEN)✓ All tests passed$(NC)"

test-unit:
	@echo "$(BLUE)Running unit tests...$(NC)"
	cd $(RUST_PROJECT) && cargo test --lib
	@echo "$(GREEN)✓ Unit tests passed$(NC)"

test-integration:
	@echo "$(BLUE)Running integration tests...$(NC)"
	cd $(RUST_PROJECT) && cargo test --test '*'
	@echo "$(GREEN)✓ Integration tests passed$(NC)"

test-verbose:
	@echo "$(BLUE)Running tests with output...$(NC)"
	cd $(RUST_PROJECT) && cargo test -- --nocapture --test-threads=1

test-suite:
	@echo "$(BLUE)Running SQLite test suite...$(NC)"
	@if [ ! -d "sqlite-shell-src/test" ]; then \
		echo "$(YELLOW)SQLite test suite not found at sqlite-shell-src/test$(NC)"; \
		echo "$(YELLOW)Download SQLite source to enable full test suite$(NC)"; \
		exit 0; \
	fi
	@echo "Running tests (this may take a while)..."
	cd sqlite-shell-src/test && \
		tclsh testrunner.tcl \
			--shell ../../$(BUILD_DIR)/$(RUST_BIN) \
			--timeout 10000
	@echo "$(GREEN)✓ SQLite test suite passed$(NC)"

test-quick:
	@echo "$(BLUE)Quick smoke test...$(NC)"
	@if [ -f "$(BUILD_DIR)/$(RUST_BIN)" ]; then \
		$(BUILD_DIR)/$(RUST_BIN) :memory: "SELECT 1;" && echo "$(GREEN)✓ Quick test passed$(NC)"; \
	else \
		echo "$(RED)Binary not found, build first with 'make build'$(NC)"; \
	fi

# ============================================================================
# ANALYSIS & BENCHMARKING
# ============================================================================

analyze:
	@echo "$(BLUE)Analyzing shell.c for migration scope...$(NC)"
	@echo ""
	@echo "$(YELLOW)File Statistics:$(NC)"
	@wc -l $(SHELL_C)
	@echo ""
	@echo "$(YELLOW)Function Count:$(NC)"
	@grep -c "^[a-z_]*(" $(SHELL_C) || echo "0 (or pattern not found)"
	@echo ""
	@echo "$(YELLOW)Macro Count:$(NC)"
	@grep -c "#define" $(SHELL_C) || echo "0"
	@echo ""
	@echo "$(YELLOW)Platform-specific Sections:$(NC)"
	@grep -c "#ifdef" $(SHELL_C) || echo "0"
	@echo ""
	@echo "$(YELLOW)Unique SQLite API Functions:$(NC)"
	@grep -oE "sqlite3_[a-z_0-9]+" $(SHELL_C) | sort -u | wc -l
	@echo ""
	@echo "$(YELLOW)Top 10 Most Used SQLite Functions:$(NC)"
	@grep -oE "sqlite3_[a-z_0-9]+" $(SHELL_C) | sort | uniq -c | sort -rn | head -10

analyze-generated:
	@echo "$(BLUE)Analyzing generated Rust code...$(NC)"
	@if [ ! -d "$(GENERATED_DIR)" ]; then \
		echo "$(RED)Generated directory not found. Run 'make transpile' first.$(NC)"; \
		exit 1; \
	fi
	@echo "$(YELLOW)Generated files:$(NC)"
	@find $(GENERATED_DIR) -name "*.rs" | wc -l
	@echo ""
	@echo "$(YELLOW)Total lines of generated code:$(NC)"
	@find $(GENERATED_DIR) -name "*.rs" -exec wc -l {} + | tail -1
	@echo ""
	@echo "$(YELLOW)Unsafe blocks:$(NC)"
	@grep -r "unsafe" $(GENERATED_DIR) | wc -l || echo "0"

benchmark:
	@echo "$(BLUE)Benchmarking C vs Rust implementations...$(NC)"
	@if [ ! -f "$(BUILD_DIR)/$(RUST_BIN)" ]; then \
		echo "$(RED)Rust binary not found. Build with 'make build-release' first.$(NC)"; \
		exit 1; \
	fi
	@if [ ! -f "sqlite3" ]; then \
		echo "$(YELLOW)C binary (sqlite3) not found in current directory$(NC)"; \
		echo "$(YELLOW)Skipping C comparison$(NC)"; \
	else \
		echo "$(YELLOW)C Implementation:$(NC)"; \
		time ./sqlite3 :memory: "SELECT 1;" > /dev/null 2>&1; \
	fi
	@echo ""
	@echo "$(YELLOW)Rust Implementation:$(NC)"
	@time $(BUILD_DIR)/$(RUST_BIN) :memory: "SELECT 1;" > /dev/null 2>&1

benchmark-detailed:
	@echo "$(BLUE)Detailed benchmark with memory profiling...$(NC)"
	@echo "$(YELLOW)Rust startup time (10 iterations):$(NC)"
	@for i in 1 2 3 4 5 6 7 8 9 10; do \
		time $(BUILD_DIR)/$(RUST_BIN) :memory: "SELECT 1;" 2>&1 | grep real; \
	done | awk '{sum += $$2} END {print "Average:", sum/NR}'

clippy:
	@echo "$(BLUE)Running clippy linter...$(NC)"
	cd $(RUST_PROJECT) && cargo clippy -- -W clippy::all -D warnings
	@echo "$(GREEN)✓ Clippy checks passed$(NC)"

check-unsafe:
	@echo "$(BLUE)Checking for unsafe code...$(NC)"
	@echo "$(YELLOW)Unsafe blocks in source:$(NC)"
	@find $(RUST_SRC) -name "*.rs" -not -path "$(GENERATED_DIR)/*" \
		-exec grep -l "unsafe" {} \; | wc -l || echo "0"
	@echo "$(YELLOW)Unsafe blocks in generated code:$(NC)"
	@find $(GENERATED_DIR) -name "*.rs" -exec grep -c "unsafe" {} + | awk '{s+=$1} END {print s}' || echo "0"
	@echo ""
	@echo "$(YELLOW)Files with unsafe code:$(NC)"
	@find $(RUST_SRC) -name "*.rs" -exec grep -l "unsafe" {} \;

fmt-check:
	@echo "$(BLUE)Checking code formatting...$(NC)"
	cd $(RUST_PROJECT) && cargo fmt -- --check
	@echo "$(GREEN)✓ Code is properly formatted$(NC)"

fmt-fix:
	@echo "$(BLUE)Fixing code formatting...$(NC)"
	cd $(RUST_PROJECT) && cargo fmt
	@echo "$(GREEN)✓ Code formatting fixed$(NC)"

# ============================================================================
# CLEANUP TARGETS
# ============================================================================

clean:
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	@if [ -d "$(RUST_PROJECT)" ]; then \
		cd $(RUST_PROJECT) && cargo clean; \
	fi
	@rm -f *.o *.so
	@echo "$(GREEN)✓ Clean complete$(NC)"

clean-project:
	@echo "$(BLUE)Removing Rust project...$(NC)"
	@rm -rf $(RUST_PROJECT)
	@echo "$(GREEN)✓ Project removed$(NC)"

clean-config:
	@echo "$(BLUE)Removing c2rust configuration...$(NC)"
	@rm -f c2rust.toml
	@echo "$(GREEN)✓ Configuration removed$(NC)"

distclean: clean clean-project clean-config
	@echo "$(BLUE)Uninstalling c2rust...$(NC)"
	@cargo uninstall c2rust 2>/dev/null || true
	@echo "$(GREEN)✓ Full cleanup complete$(NC)"

# ============================================================================
# DEVELOPMENT HELPERS
# ============================================================================

watch-build:
	@echo "$(BLUE)Watching for changes and rebuilding...$(NC)"
	cd $(RUST_PROJECT) && cargo watch -x build

watch-test:
	@echo "$(BLUE)Watching for changes and running tests...$(NC)"
	cd $(RUST_PROJECT) && cargo watch -x test

doc:
	@echo "$(BLUE)Generating documentation...$(NC)"
	cd $(RUST_PROJECT) && cargo doc --no-deps --open
	@echo "$(GREEN)✓ Documentation generated$(NC)"

info:
	@echo "$(BLUE)=== Project Information ===$(NC)"
	@echo "$(YELLOW)Project Name:$(NC) $(RUST_PROJECT)"
	@echo "$(YELLOW)Binary Name:$(NC) $(RUST_BIN)"
	@echo "$(YELLOW)Source File:$(NC) $(SHELL_C)"
	@echo "$(YELLOW)Rust Project Dir:$(NC) $(RUST_PROJECT)"
	@echo "$(YELLOW)Generated Code Dir:$(NC) $(GENERATED_DIR)"
	@echo "$(YELLOW)Build Directory:$(NC) $(BUILD_DIR)"
	@echo ""
	@echo "$(YELLOW)Cargo Version:$(NC)"
	@cargo --version
	@echo "$(YELLOW)Rustc Version:$(NC)"
	@rustc --version
	@echo "$(YELLOW)c2rust Version:$(NC)"
	@c2rust --version 2>/dev/null || echo "Not installed"

# ============================================================================
# PHONY DECLARATIONS
# ============================================================================

.PHONY: help setup install-tools create-project create-config transpile transpile-verbose transpile-dry-run
.PHONY: build build-release build-debug build-check
.PHONY: test test-unit test-integration test-verbose test-suite test-quick
.PHONY: analyze analyze-generated benchmark benchmark-detailed
.PHONY: clippy check-unsafe fmt-check fmt-fix
.PHONY: clean clean-project clean-config distclean
.PHONY: watch-build watch-test doc info

.DEFAULT_GOAL := help
