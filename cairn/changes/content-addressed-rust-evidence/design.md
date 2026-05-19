## Context

Valence needs a narrow Cairn-owned contract for content-addressed Rust evidence before Octet objects can become durable review, replay, and downstream interop artifacts. Octet should continue to own extraction and hashing; Cairn should own lifecycle contracts, receipt schemas, replay requirements, and non-overclaim gates.

## Goals / Non-Goals

**Goals:**

- Specify the first Cairn receipt contract for Octet function objects.
- Make canonical object hashing and replay boundaries explicit.
- Define name-pointer movement and drift semantics.
- Define machine-readable caveats for dependencies, macros, effects, and compiler IR evidence.
- Prepare bounded Crunch/Mantle and Aspen pilot receipts.

**Non-Goals:**

- Implement Octet code changes in Cairn.
- Require HIR/MIR extraction for the first object version.
- Claim semantic equivalence, correctness, or compiler-version-independent identity.
- Replace downstream stores, schedulers, CI systems, or Trellis proofs.

## Decisions

### 1. Cairn specifies receipts, Octet emits evidence

**Choice:** Cairn defines accepted requirements and future receipt validation surfaces; Octet remains the extraction and object-emission authority.

**Rationale:** This preserves the functional-core / imperative-shell split across projects. Octet can evolve source, HIR, and MIR extractors while Cairn keeps replay and lifecycle policy deterministic.

**Alternative:** Put object schema ownership entirely in Octet. Rejected because downstream replay, mutation manifests, policy hashes, and lifecycle gates are Cairn's domain.

### 2. Start with source-token function object v1

**Choice:** The first contract accepts source-token function objects with explicit caveats instead of requiring rustc HIR/MIR evidence immediately.

**Rationale:** Octet already has function fingerprints. Making that evidence canonical, caveated, and replayable gives useful review identity before taking on rustc internals.

**Alternative:** Start with HIR/MIR semantic objects. Rejected because it couples the first milestone to rustc-private extraction and risks overclaiming semantic stability.

### 3. Treat HIR/MIR as supplemental compiler-grounded evidence

**Choice:** HIR/MIR summaries are future evidence layers with exact toolchain and extractor identity fields.

**Rationale:** HIR can improve resolved dependency identity and MIR can improve panic/drop/unsafe/async summaries, but both are compiler-version-coupled and should not invalidate source-token objects.

**Alternative:** Make MIR hash the canonical object identity. Rejected because MIR is not stable enough across rustc versions for a broad content-addressed review object.

### 4. Fail closed on unknown dependencies/effects

**Choice:** Unknown dependency identity, macro-hidden code, and unresolved effects block purity/cache eligibility unless stronger evidence is present.

**Rationale:** Crunch/Mantle and Aspen can safely consume conservative evidence. False purity claims are worse than explicit caveats.

### 5. Pilot downstream interop with bounded receipts

**Choice:** Crunch/Mantle and Aspen pilots should reference immutable evidence objects and bounded claim classes, not claim their full systems are proven by Octet hashes.

**Rationale:** This gives immediate value in parity rows, checker identity, CI job cache/replay keys, and dogfood receipts while preserving downstream proof boundaries.

## Risks / Trade-offs

**Schema churn** → Mitigate with explicit `schema` and `normalization.version` fields plus versioned requirements.

**Overclaiming hash meaning** → Mitigate with required non-correctness and toolchain-coupling receipt language.

**Too much Cairn coupling to Octet internals** → Mitigate by validating portable receipts and bundles, not Octet source implementation details.

**HIR/MIR instability** → Mitigate by making compiler IR evidence supplemental and exact-toolchain scoped.

## Validation Plan

- `nix run path:$PWD#cairn -- validate --root .`
- `nix run path:$PWD#cairn -- gate proposal content-addressed-rust-evidence --root .`
- `nix run path:$PWD#cairn -- gate design content-addressed-rust-evidence --root .`
- `nix run path:$PWD#cairn -- gate tasks content-addressed-rust-evidence --root .`
- Later implementation slices add deterministic fixture receipts for canonical hashing, pointer movement, bundle replay, Crunch pilot, and Aspen pilot.
