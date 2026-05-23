# Change: Split agent receipt core modules

## Summary

Split `crates/cairn-core/src/verified/agent_receipt.rs` into a focused pure-core module tree while preserving the public `verified::agent_receipt` facade and all exported API behavior.

## Rationale

`agent_receipt.rs` has become one of the largest remaining core files. It mixes receipt DTOs, canonical hashing, required-field checks, no-mutation plan linkage, verification evidence checks, nondeterminism checks, replay-evidence checks, fixture generation, and unit tests. A focused module tree reduces review load without changing receipt semantics.

## Compatibility

This is behavior-preserving. Public constants, DTOs, fixture helpers, hash helpers, issue evaluation, and completion-gate APIs remain re-exported from `verified::agent_receipt` and from `cairn_core`.
