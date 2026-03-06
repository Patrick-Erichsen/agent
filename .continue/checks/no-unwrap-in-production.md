---
description: Ensure production Rust code never uses unwrap(), expect(), panic!(), or todo!()
---

# No unwrap/expect/panic in production code

This project strictly forbids `unwrap()`, `expect()`, `panic!()`, and `todo!()` in non-test code (enforced by Clippy workspace lints). Review the diff for any of these patterns introduced **outside** of `#[cfg(test)]` modules or test helper files.

## What to check

1. **Direct calls**: `.unwrap()`, `.expect(...)`, `panic!(...)`, `todo!(...)`, `unimplemented!(...)` in production (non-test) code paths.
2. **Indirect patterns**: `unwrap_or_else(|| panic!(...))` or similar constructs that panic at runtime.
3. **Macro invocations**: `assert!()` and `assert_eq!()` outside of test code (these panic on failure).

## What is acceptable

- Any of the above inside `#[cfg(test)] mod tests { ... }` blocks.
- Any of the above in files under a `tests/` directory or in `*_test.rs` files.
- `unreachable!()` when the code path is provably unreachable (e.g., after an exhaustive match).

## How to verify

Read the full file context around each flagged line to confirm it is not inside a test module. Check the `#[cfg(test)]` attribute scope.

## What to suggest instead

- `.unwrap()` / `.expect()` → Use `?` with `.context("descriptive message")` (from `anyhow`).
- `panic!()` → Return `Err(anyhow!("..."))` or use `bail!("...")`.
- `todo!()` → Either implement the code or return a proper error.
