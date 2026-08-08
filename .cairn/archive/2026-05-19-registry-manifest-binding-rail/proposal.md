# registry-manifest-binding-rail

## Why

Cairn can now export evidence bundles, publish bundle indexes, aggregate registries, and select/fetch plans from policy-labelled registries. Operators still need a first-class way to bind a published registry into the existing evidence manifest chain so downstream agents can verify that a registry, its channel labels, and the registry consumption receipts are the intended published evidence surface.

## What Changes

- Extend evidence manifest generation to accept registry artifacts and registry verification/consumption receipts.
- Extend manifest verification so registry artifacts are re-verified with existing registry verification semantics before the manifest is accepted.
- Include stable registry metadata summaries in manifest artifacts without leaking absolute paths, URLs, environment values, credentials, or secrets.
- Preserve existing metadata-only signer/anchor semantics; this change binds hashes and receipts but does not claim real cryptographic signing.

## Non-Goals

- Fetching registries or bundles from networks.
- Implementing new cryptographic signing primitives.
- Replacing registry hash verification or label policy checks.
- Adding Python or shell helper scripts.

## Verification

- Rust CLI tests cover manifest generation/verification with registry artifacts and deterministic registry tamper failures.
- Nickel policy schemas and deterministic surface inventory remain fresh.
- Full Rust/Nickel gates pass: fmt, tests, clippy, policy export, strict validate, Tracey coverage, and flake check.
