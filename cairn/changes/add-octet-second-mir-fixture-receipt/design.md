## Context

Octet's current MIR selected-fixture contract supports only `checked-arithmetic` through an explicit `rustc-mir-fixture` selector. That slice intentionally left default and non-fixture MIR as `unsupported-planned`. A second selector should be planned as a receipt/lifecycle contract first so downstream Valence consumers get stable replay expectations without overclaiming MIR semantics.

## Goals / Non-Goals

**Goals:**

- Define the Cairn-owned downstream contract for accepting a second Octet MIR selected-fixture receipt.
- Keep the selector closed and exact, with deterministic rejection for aliases and unsupported shapes.
- Require object-hash linkage, fixture path/function/shape metadata, rustc/toolchain/extractor identity, and replayable positive/negative evidence.
- Preserve non-claims around correctness, semantic equivalence, cross-toolchain identity, panic/drop/unsafe proof, and broad MIR coverage.

**Non-Goals:**

- Implement Octet CLI, parser, DTO, or fixture code in Cairn.
- Generalize MIR support beyond named selected fixtures.
- Make MIR hashes canonical object identity or proof material.

## Decisions

### 1. Cairn owns downstream receipt expectations, Octet owns extraction

**Choice:** Cairn specifies the lifecycle contract and validation expectations for downstream consumers; Octet will implement the actual MIR selector, emitted receipt, parser/runtime tests, and aggregate Valence evidence.

**Rationale:** This keeps cross-repo ownership clean. Cairn governs receipt lifecycle and replay contracts; Octet governs Rust extraction and object emission.

### 2. The second selector remains closed and exact

**Choice:** The implementation slice must name exactly one new MIR fixture shape and reject unknown shapes and aliases deterministically.

**Rationale:** Closed selectors avoid accidental broad MIR claims and make downstream replay expectations auditable.

### 3. Receipt evidence must bind fixture, object, and toolchain identity

**Choice:** The new selected-fixture receipt must bind `family=mir`, the selected fixture shape, fixture path/function metadata, the linked Function Object v1 hash, rustc/toolchain identity, extractor identity, support status, and explicit boundary flags.

**Rationale:** Downstream Cairn validation should be able to distinguish stable review evidence from unsupported or overclaimed MIR data.

### 4. Negative evidence is first-class

**Choice:** The draining implementation must save or expose deterministic negative evidence for unsupported shapes, selector alias rejection, tampered fixture/object linkage, and non-fixture/default MIR remaining `unsupported-planned`.

**Rationale:** The previous MIR slice derived most of its safety from fail-closed boundaries; the second selector should preserve that pattern rather than only adding a positive happy path.

## Risks / Trade-offs

- **Overclaiming MIR meaning**: mitigated by explicit non-claim requirements and default/non-fixture `unsupported-planned` boundaries.
- **Cross-repo drift**: mitigated by keeping Cairn's package about receipt/replay contract and Octet's implementation about CLI/DTO/runtime behavior.
- **Selector ambiguity**: mitigated by exact shape naming and deterministic parser rejection tests.

## Verification Plan

- Validate this active package with `nix run path:$PWD#cairn -- validate --root .`.
- Gate proposal, design, and tasks receipts and inspect `valid=true` / `verdict=PASS`.
- During implementation drain, require Octet-focused parser, runtime, object-corpus, and aggregate Valence evidence before archive.
