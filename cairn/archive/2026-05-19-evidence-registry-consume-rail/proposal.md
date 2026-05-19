# Proposal: Evidence Registry Consume Rail

## Why

Cairn can now export, verify, import, republish, index, and aggregate deterministic evidence into registries. Downstream agents still need a deterministic consumer-facing rail for choosing a registry entry and planning which portable artifacts to fetch. Without that rail, consumers must hand-pick indexes from a registry, which reintroduces ambiguous selection, host-specific paths, and non-reviewable automation.

## What Changes

- Add registry selection behavior that consumes an existing registry, verifies it, applies stable selectors, and emits a selected evidence index artifact plus deterministic receipt.
- Add registry fetch-plan behavior that consumes an existing registry, verifies it, and emits a deterministic plan describing the index refs, artifact roles, hashes, and receipt-chain hashes a downstream agent should fetch/verify.
- Add policy receipt schemas, docs, tests, and Tracey coverage for consumer-side registry use.

## Capabilities

### New Capabilities

- `evidence-registry-select`: Deterministically select one registry entry by stable metadata.
- `evidence-registry-fetch-plan`: Deterministically materialize a registry-derived fetch plan.

### Modified Capabilities

- `determinism-surface-inventory`: Include registry selection and fetch-plan receipt surfaces.
- `evidence-registry`: Extend registry lifecycle from publication to consumer-side consumption.

## Scope

In scope:

- Rust CLI implementation and tests.
- Nickel policy schema/surface updates.
- Portable receipts with stable `receipt_hash` values.
- Deterministic issue codes for invalid registry, no match, ambiguous match, unsafe refs, and tampering.

Out of scope:

- Network downloading.
- Credential handling.
- Cryptographic signature verification beyond existing hash/receipt verification.
- Checked-in Python or shell helper scripts.

## Verification

Run Cairn validation gates, targeted registry-consume tests, workspace tests, Rust/Nickel maintenance gates, Tracey coverage, strict validation, and flake checks before archive/commit/push.
