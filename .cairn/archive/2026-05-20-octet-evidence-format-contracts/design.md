## Context

Octet now emits function objects, object bundles, corpus receipts, receipt checks, and schema-hardened invalid-input reports as deterministic JSON. Cairn already uses Markdown for lifecycle specs and JSON for deterministic receipts across replay, bundle, manifest, and release-readiness workflows. The remaining risk is ambiguous ownership: JSON is an interchange encoding, not the semantic source of truth for receipt compatibility.

## Goals

- Keep human review artifacts readable and diffable.
- Keep machine receipts deterministic, parseable, and CI-friendly.
- Make Rust DTOs the source of truth for emitted receipt shape.
- Require generated/registered Nickel contract metadata where compatibility policy matters.
- Preserve explicit non-claims: no correctness proof and no semantic-equivalence proof from hashes.

## Non-Goals

- Replacing Markdown lifecycle artifacts.
- Replacing JSON receipt output in the current Octet CLI.
- Mandating a binary canonical format before there is a concrete consumer need.
- Moving Octet extraction/object ownership into Cairn.

## Design

### Human artifacts remain Markdown

Cairn change packages, design notes, task/evidence narratives, and review rationale should remain Markdown. They are reviewed by humans, must preserve context and caveats, and should not be optimized for machine replay.

### Machine receipts remain deterministic JSON v1

Octet evidence receipts should continue to use deterministic JSON for current fixtures and CLI output. JSON is easy to inspect, diff, archive, validate in CI, and include in evidence bundles. Canonicalization must sort fields/arrays where required by the receipt contract and exclude self-referential hash fields from hash material.

### Compatibility is owned by Rust DTOs plus Nickel contracts

For emitted receipts, Rust DTOs should own the runtime schema and serde behavior. When a receipt family becomes compatibility-relevant, Cairn policy should track the family as Rust-derived and require generated or registered Nickel contract metadata rather than hand-maintained JSON schema as the normative source.

### Future formats are additive

A future canonical binary or bundle format may be added for size, speed, signing, or transport needs. It must be additive to the v1 JSON review surface unless a later change explicitly migrates consumers. Binary encodings must still expose deterministic hashes, schema identity, compatibility rules, and human-reviewable summaries.

### Boundary language is mandatory

File-format receipts must preserve the same Valence boundary language used by Octet: object hashes and corpus hashes identify reviewed evidence/provenance. They do not prove Rust behavioral correctness, semantic equivalence, cross-toolchain determinism, or distributed execution correctness.
