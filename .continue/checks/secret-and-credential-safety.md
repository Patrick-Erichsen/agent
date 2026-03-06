---
description: Ensure diffs don't leak secrets, log credentials, or weaken security boundaries
---

# Secret and credential safety

Stakpak is a security-hardened DevOps agent. It uses secret substitution so the LLM never sees plaintext credentials, Warden guardrails for network-level policies, and mTLS for MCP communication. Any PR that weakens these boundaries is a critical issue.

## What to check

1. **Hardcoded secrets**: API keys, tokens, passwords, or connection strings appearing as string literals in the diff (not in test fixtures with obviously fake values).
2. **Logging sensitive data**: New `tracing::info!`, `tracing::debug!`, `println!`, `eprintln!`, or `log::` calls that output variables likely to contain secrets (look for variable names like `token`, `secret`, `password`, `key`, `credential`, `api_key`, `auth`).
3. **Bypassing secret substitution**: Changes to the secret substitution system in `config/warden.rs` or related files that reduce coverage or skip redaction.
4. **Weakening TLS**: Disabling certificate verification, allowing insecure connections, downgrading from mTLS to plain TLS, or accepting invalid certificates.
5. **Overly permissive permissions**: New file I/O, network access, or process execution that bypasses Warden guardrails or sandbox boundaries.
6. **Privacy mode regressions**: Changes that could leak IP addresses or cloud account IDs when privacy mode is enabled.

## What is acceptable

- Test fixtures using clearly fake values like `"test-token-123"`, `"sk-fake"`, or `"password"`.
- Logging redacted or placeholder values (e.g., `"[REDACTED]"`, `"***"`).
- Intentional security configuration changes with clear justification in the PR description.

## How to verify

Read the full context of flagged lines. Check if the variable being logged/printed actually contains sensitive data by tracing its origin. Check if Warden policies or secret substitution logic is being modified.
