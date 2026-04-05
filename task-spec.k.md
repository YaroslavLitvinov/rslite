# Specification

## Overview

Verify c_variadic feature isolation: only in printf_c_variadic.rs

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: build_all](#build_all)
      - [constraint_build_all](#constraint_build_all)
    - [Feature: fts5_no_libc_global](#fts5_no_libc_global)
      - [fts5_no_libc_calls](#fts5_no_libc_calls)
    - [Feature: libs](#libs)
      - [constraint_appendf_c_source_exists](#constraint_appendf_c_source_exists)
      - [constraint_appendf_cdylib_export](#constraint_appendf_cdylib_export)
      - [constraint_appendf_not_in_rust_variadic](#constraint_appendf_not_in_rust_variadic)
      - [constraint_appendf_shell_rlib](#constraint_appendf_shell_rlib)
      - [constraint_appendf_tclsqlite_rlib](#constraint_appendf_tclsqlite_rlib)
    - [Feature: toolchain_version](#toolchain_version)
      - [c6](#c6)

## Features

### Feature: build_all
**Check that our rust codebase is healthy**

#### constraint_build_all
**Description:** Ensure our rust codebase is healthy
**Command:** `cd $WORKSPACE_ROOT && ./build_all.sh`

### Feature: fts5_no_libc_global
**Ensure fts5.rs contains no libc:: calls (only external C FFI allowed)**

**Goals:**
- fts5.rs must not contain libc:: direct calls
- All C interop must go through safe wrappers or external C FFI only

#### fts5_no_libc_calls
**Description:** fts5.rs must not contain libc:: calls - use safe wrappers or external C FFI instead
**Command:** `grep -q "libc::" "$PROJECT_ROOT/src/fts5.rs" && exit 1 || exit 0`

### Feature: libs
**sqlite3_str_appendf correctly exported in rlib and cdylib (.so)**

**Goals:**
- sqlite3_str_appendf (C FFI in c_code/printf_c.c) must be global symbol (T) in shell binary (rlib)
- sqlite3_str_appendf must appear in dynamic symbol table (T) of libsqlite_noamalgam.so (cdylib)
- sqlite3_str_appendf must be global symbol (T) in rustfixture binary (rlib)
- sqlite3_str_appendf must NOT be a Rust variadic function — only in C
- Depends on build_all — assumes release artifacts already built

#### constraint_appendf_c_source_exists
**Description:** Structural: the C implementation of sqlite3_str_appendf must exist in c_code/printf_c.c
**Command:** `grep -q "void sqlite3_str_appendf" $PROJECT_ROOT/c_code/printf_c.c`

#### constraint_appendf_cdylib_export
**Description:** Behavioral: sqlite3_str_appendf must appear in the dynamic symbol table (T) of libsqlite_noamalgam.so for external C clients
**Command:** `cd $PROJECT_ROOT && cargo build --release --lib 2>/dev/null && nm -D target/release/libsqlite_noamalgam.so | grep -q "T sqlite3_str_appendf$"`

#### constraint_appendf_not_in_rust_variadic
**Description:** Structural: sqlite3_str_appendf must NOT be defined as a Rust variadic function — it lives in c_code/printf_c.c
**Command:** `grep -q "void sqlite3_str_appendf" $PROJECT_ROOT/c_code/printf_c.c && ! grep -q "pub unsafe extern.*fn sqlite3_str_appendf" $PROJECT_ROOT/src/printf_c_variadic.rs`

#### constraint_appendf_shell_rlib
**Description:** Behavioral: sqlite3_str_appendf must be a global (T) symbol in the shell binary (rlib linkage)
**Command:** `nm $PROJECT_ROOT/target/release/sqlite3 | grep -q "T sqlite3_str_appendf$"`

#### constraint_appendf_tclsqlite_rlib
**Description:** Behavioral: sqlite3_str_appendf must be a global (T) symbol in the tclsqlite rustfixture binary (rlib linkage)
**Command:** `nm $PROJECT_ROOT/target/release/rustfixture | grep -q "T sqlite3_str_appendf$"`

### Feature: toolchain_version
**Enforce Rust toolchain version nightly-2026-03-26**

**Goals:**
- rust-toolchain.toml must specify channel nightly-2026-03-26

#### toolchain_nightly_version
**Description:** rust-toolchain.toml must use channel nightly-2026-03-26
**Command:** `grep -q "nightly-2026-03-26" "$PROJECT_ROOT/rust-toolchain.toml" && exit 0 || exit 1`