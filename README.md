# RSLITE is a port of Sqlite3 on rust.

Our goal is seamless integration of SQLite into Rust applications, keeping it as close to the original SQLite architecture as possible and ensuring it passes existing sqlite tests.

We’re iteratively porting SQLite to Rust by first generating an unsafe Rust equivalent. Then we link the existing SQLite tests and make changes in small steps, verifying the tests at each iteration. This way, each modification is checked and validated through a continuous development loop.

At this point, we haven’t actually changed the interface, so it remains fully compatible with the original SQLite. But since we plan to change it in the future as the project evolves, we’ve already ported some of these tests to Rust. This way, when we modify the library, we’ll also be adjusting the tests suite accordingly.

Because the SQLite API is still unchanged, we continue to rely on the original test suite, which we run using `make c-tests`.
The tests in the c2rust directory are currently intended for future changes - specifically for when we start breaking the existing C-based sqlite3 API. At that point, we will begin migrating both the library and the tests to ensure we can continue using our testing approach.

# Good Code

When contributing to RSLITE, follow these principles:

## Safety First
- Unsafe code should be justified and minimized. All unsafe operations must be properly documented with `// SAFETY:` comments explaining why it's safe.
- Unsafe code should be confined to internal implementation details; the public API must remain completely safe.
- Use `#[deny(unsafe_code)]` on public-facing modules to enforce this boundary.

## Follow Rust Idioms
- Write idiomatic Rust—embrace the borrow checker and type system rather than fighting them.
- Use strong typing to prevent bugs at compile time. Avoid generic `Result<T, String>` in favor of proper error types.
- Use `Option` and `Result` instead of panicking or returning sentinel values.

## Test Your Changes
- All new functionality must include tests that verify correctness.
- Tests must pass both the Rust test suite and the original C SQLite test suite (`make c-tests`).
- If modifying internal behavior, ensure existing SQLite tests continue to pass.

## Maintain SQLite Compatibility
- Keep the external API compatible with SQLite/rusqlite unless there's a documented reason to break it.
- When changing behavior, verify compatibility with the original SQLite test suite.
- Document any intentional deviations from standard SQLite behavior.

## Document Your Code
- Write clear doc comments for all public types and functions using `///` or `//!`.
- Include examples in doc comments for public APIs when helpful.
- Leave explanatory comments for complex logic or non-obvious decisions.

## Performance Matters
- Avoid unnecessary allocations and copies in hot paths.
- Profile before optimizing, but keep performance in mind during code review.
- Document any performance-critical sections.

## Code Style
- Format code with `rustfmt` before submitting.
- Use `cargo clippy` to catch common mistakes and idiomatic issues.
- Run `cargo test` to verify all tests pass.

# Build and test

run `./build_all.sh` to build & test

## Run just a separate test, after tests built
cd /sqlite && ./rustfixture test/capi3.test
