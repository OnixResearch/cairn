## Why

Octet now has a bounded MIR selected-fixture runtime contract for `checked-arithmetic`, while default and non-fixture MIR remain `unsupported-planned`. The next Valence step should add a second selected MIR fixture shape without turning MIR into a broad semantic-proof surface.

Cairn should capture the lifecycle, receipt, replay, and downstream validation contract before Octet implements the next selector. Octet remains the Rust extraction and receipt-emission owner; Cairn defines what evidence downstream consumers may rely on and what claims remain forbidden.

## What Changes

- Define a native Cairn planning package for a second Octet MIR selected-fixture receipt contract.
- Require an explicit closed selector, fixture metadata, object-hash linkage, toolchain/extractor identity, replayable positive and negative evidence, and fail-closed unknown-shape handling.
- Preserve the existing boundary that default/non-fixture MIR remains `unsupported-planned` and that selected MIR fixtures do not claim semantic equivalence, correctness, cross-toolchain stability, or exhaustive MIR coverage.

## Non-Goals

- Implement Octet MIR extraction or fixture support inside Cairn.
- Choose a broad MIR object identity scheme or replace Function Object v1 source-token identity.
- Claim compiler-version-independent MIR identity, program correctness, semantic equivalence, panic freedom, unsafe freedom, or exhaustive Rust coverage.

## Impact

- **Files**: Adds Cairn lifecycle artifacts under `cairn/changes/add-octet-second-mir-fixture-receipt/` and a delta spec for content-addressed Rust evidence receipts.
- **Testing**: `cairn validate --root .`, proposal/design/tasks gates, `git diff --check`, and later Octet-focused fixture/parser/runtime/aggregate evidence when the implementation drains.
