# Split release receipts integration-test target into submodules

## Summary

Split `crates/cairn-cli/tests/release_receipts.rs` into a small facade plus focused submodules for portable receipts, agent bundle verification, signed evidence manifests, public schemas, agent replay validation, and release-readiness receipts.

## Motivation

The previous split removed receipt/release tests from the residual `cli.rs` target, but the new target is still a 1,331-line review surface. A focused submodule layout preserves behavior while making future receipt, release, and evidence-manifest changes easier to review and verify independently.

## Scope

- Behavior-preserving integration-test organization only.
- Shared helpers stay local to the test target under a narrow support module.
- No public CLI, receipt schema, policy, fixture, or golden behavior changes.
