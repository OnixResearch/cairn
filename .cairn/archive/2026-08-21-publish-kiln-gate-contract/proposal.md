# Publish a versioned Cairn gate interchange contract

## Why

Cairn publishes stable native gate receipts and a machine envelope, but it does not publish a narrow consumer contract that binds those receipts to an external CI candidate, policy, evidence set, effect, provider profile, and exact tool cohort.

Kiln needs that boundary before it can use Cairn as a gate provider without importing Cairn lifecycle authority or accepting a receipt for the wrong candidate.

## What Changes

- Add a standalone `no_std + alloc` `cairn-gate-contract` crate.
- Add typed request, result, tool-cohort, native-observation, finding, and diagnostic values.
- Add deterministic BLAKE3 identities and pure fail-closed admission.
- Add a typed Nickel contract with positive and negative fixtures.
- Add producer fixtures for passing and blocked native gate observations.
- Document the process-shell, authority, compatibility, and evidence boundaries.
- Add focused Nix checks for the Rust contract, Nickel fixtures, and host-independent build.

The existing `cairn gate` implementation and `cairn --machine` transport remain unchanged. Consumers execute them through their own bounded shells and map captured output into this contract.

## Impact

- **Files**: adds `crates/cairn-gate-contract`, `contracts/cairn-gate-contract.ncl`, fixtures, documentation, lifecycle specifications, and Nix checks.
- **Compatibility**: publishes version-one request and result schemas without changing existing gate receipt bytes.
- **Testing**: requires positive and negative Rust tests, producer fixture freshness, Nickel positive and negative checks, formatting, Clippy, Slotcar, traceability, Cairn validation, and full Nix checks.
