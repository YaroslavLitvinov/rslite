## Table of Contents

- [Constraint Results](#constraint-results)
  - [Feature: build_all](#build-all)
    - [constraint_build_all](#build-all-constraint-build-all)
  - [Feature: clippy_checks](#clippy-checks)
    - [clippy_checks](#clippy-checks-clippy-checks)
    - [no_forbidden_allows_robust](#clippy-checks-no-forbidden-allows-robust)
  - [Feature: fts5_no_libc_global](#fts5-no-libc-global)
    - [fts5_no_libc_calls](#fts5-no-libc-global-fts5-no-libc-calls)
  - [Feature: lib_exports](#lib-exports)
    - [cdylib_symbols](#lib-exports-cdylib-symbols)
    - [no_rust_variadic_exports](#lib-exports-no-rust-variadic-exports)
  - [Feature: toolchain_version](#toolchain-version)
    - [c2rust_nightly](#toolchain-version-c2rust-nightly)
    - [stable_toolchain_or_missing](#toolchain-version-stable-toolchain-or-missing)

## Constraint Results

<a id="build-all"></a>
### Feature: build_all

**✓ 1/1 constraints passed**

<a id="build-all-constraint-build-all"></a>
**✓ PASS** constraint_build_all

- Executed: 2026-04-12T15:09:28.597819
- Duration: 113.46s
- Output: [stdout] sqlite/testrunner.log
0 errors out of 396913 tests in 00:38 on d8392dd691dd Linux 64-bit
SQLite 2026-01-09 17:27:48 b270f8339eb13b504d0b2ba154ebca966b7dde08e40c3ed7d559
✓ C TCL tests (release) passed


<a id="clippy-checks"></a>
### Feature: clippy_checks

**✓ 2/2 constraints passed**

<a id="clippy-checks-clippy-checks"></a>
**✓ PASS** clippy_checks

- Executed: 2026-04-12T15:09:43.176184
- Duration: 14.58s
- Output: [stderr] ust-bitfields-derive v0.22.1
   Compiling strum_macros v0.27.2
    Checking c2rust-bitfields v0.22.1
    Checking snafu v0.7.5
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 14.51s


<a id="clippy-checks-no-forbidden-allows-robust"></a>
**✓ PASS** no_forbidden_allows_robust

- Executed: 2026-04-12T15:09:43.185613
- Duration: 0.01s

<a id="fts5-no-libc-global"></a>
### Feature: fts5_no_libc_global

**✓ 1/1 constraints passed**

<a id="fts5-no-libc-global-fts5-no-libc-calls"></a>
**✓ PASS** fts5_no_libc_calls

- Executed: 2026-04-12T15:07:35.138108
- Duration: 0.00s
- Output: [stderr] grep: /workspace/src/fts5.rs: No such file or directory


<a id="lib-exports"></a>
### Feature: lib_exports

**✓ 2/2 constraints passed**

<a id="lib-exports-cdylib-symbols"></a>
**✓ PASS** cdylib_symbols

- Executed: 2026-04-12T15:09:43.214635
- Duration: 0.03s

<a id="lib-exports-no-rust-variadic-exports"></a>
**✓ PASS** no_rust_variadic_exports

- Executed: 2026-04-12T15:09:43.216388
- Duration: 0.00s

<a id="toolchain-version"></a>
### Feature: toolchain_version

**✓ 2/2 constraints passed**

<a id="toolchain-version-c2rust-nightly"></a>
**✓ PASS** c2rust_nightly

- Executed: 2026-04-12T15:07:35.135796
- Duration: 0.00s

<a id="toolchain-version-stable-toolchain-or-missing"></a>
**✓ PASS** stable_toolchain_or_missing

- Executed: 2026-04-12T15:07:35.136642
- Duration: 0.00s