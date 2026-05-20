## Why

Octet's Valence-oriented object receipts are currently persisted as readable JSON and specified alongside Markdown change/evidence artifacts. That is a good operator default, but Cairn should make the boundary explicit so reviewers do not mistake file extensions for the long-term compatibility contract.

## What Changes

- Define Markdown as the human lifecycle/review artifact format for Cairn/Octet evidence narratives.
- Define deterministic JSON as the current machine receipt interchange and fixture format for Octet evidence receipts.
- Require emitted receipt schemas to remain Rust DTO-owned, with generated/registered Nickel contract metadata for compatibility and policy review.
- Require canonical hashing and validation to treat JSON bytes as deterministic serialized evidence, not as correctness or semantic-equivalence proof.
- Leave room for future canonical binary bundles without replacing the v1 reviewable JSON fixture surface.

## Scope

In scope: file-format policy for Octet content-addressed Rust evidence receipts, review artifacts, contract ownership, compatibility metadata, and future-format boundaries.

Out of scope: changing Octet's current JSON CLI output, implementing a new Cairn verifier, requiring binary bundles in v1, or claiming behavioral correctness from receipt hashes.

## Verification

Run Cairn validation and proposal/design/tasks gates for the change package.
