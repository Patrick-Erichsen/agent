---
description: Ensure error propagation includes meaningful context for debugging
---

# Error context quality

This project uses `anyhow::Result` for application-level error handling. Bare `?` operators lose context about *where* and *why* an error occurred. New error paths in user-facing or I/O-heavy code should include `.context()` or `.with_context()` calls.

## What to check

1. **Bare `?` on I/O operations**: File reads/writes, network requests, process spawning, and database queries should have `.context("what was being attempted")` so failures produce actionable error messages.
2. **Bare `?` on deserialization**: `serde_json::from_str(...)`, `toml::from_str(...)`, and similar parsing calls should include context about what was being parsed and from where.
3. **Generic error messages**: `.context("error")`, `.context("failed")`, or similar vague messages that don't help with debugging.
4. **Lost error chains**: Using `match` to handle a `Result` and then returning a new error that discards the original error's context (e.g., `Err(_) => bail!("something failed")` instead of `Err(e) => bail!("something failed: {e}")`).

## What is acceptable

- Internal helper functions where the caller adds context.
- Bare `?` on infallible-in-practice operations (e.g., `Mutex::lock()` in single-threaded code).
- Code paths where the error type already contains sufficient context (e.g., custom `thiserror` types with descriptive messages).
- Test code — tests can use bare `?` freely.

## How to verify

For each new `?` in the diff, check whether the surrounding function is user-facing (CLI output, TUI display, API response) or I/O-heavy. If so, verify that either this line or a nearby caller adds `.context()`. Read the error type to see if it already carries enough information.
