# Split semantic review core

## Why

`crates/cairn-core/src/verified/semantic_review.rs` now owns review receipts, revision handoffs, ledger analytics, completeness receipts, policy profiles, shared validators, and tests in one large pure-core file. That broad review surface makes future semantic-review changes harder to audit.

## What changes

Split the semantic-review functional core into focused Rust modules behind the existing `cairn_core::verified::semantic_review` facade while preserving public behavior, receipt schemas, hashes, issue codes, and Tracey references.

## Impact

Maintainers get smaller review surfaces for future semantic-review work without changing CLI behavior or deterministic evidence contracts.
