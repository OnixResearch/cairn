# Design: Content-bound trace evidence in portable bundles

## Context

`cairn evidence export` creates Cairn's normal portable evidence chain. The chain already carries release, replay, coverage, and agent artifacts. The evidence manifest can carry a content-bound requirement registry, but export does not create it. No manifest role represents the trace index or its current-root verification receipt.

## Goals

- Generate trace artifacts from the same repository snapshot as the rest of the evidence bundle.
- Preserve exact registry-to-index identity binding.
- Recheck source freshness when `cairn evidence verify` runs.
- Keep the artifacts portable through import, republish, and evidence indexing.

## Non-Goals

- Do not replace full-corpus traceability coverage.
- Do not convert declared links into correctness or satisfaction evidence.
- Do not add filesystem access to `cairn-core`.

## Decision: Export three required trace artifacts

Evidence export will create `requirement-registry.json`, `trace-index.json`, and `trace-index-verification.json`. The registry uses the repository identity `onixresearch/cairn` and a deterministic opaque evidence-snapshot revision label. The trace index binds to the exact registry BLAKE3. The verification receipt binds the index to the current repository source.

The generated evidence manifest and export receipt will declare all three artifacts. Existing import, republish, index, registry, mirror, and local fetch paths will preserve them through their generic artifact handling.

## Decision: Validate artifact meaning at the manifest boundary

The manifest verifier will parse the trace index with the closed core schema. It will validate the index's self-bound identity. It will also require the manifest entry to bind the index to the declared requirement-registry artifact.

The trace-index verification role will require a passing `cairn.trace-index.verify-receipt.v1` receipt. Evidence verification will additionally run current-root trace-index verification with the bundled registry and index. A changed source, registry, index, or receipt will fail the evidence bundle.

## Architecture

The existing pure trace-index and registry functions remain authoritative in `cairn-core`. The evidence shell owns repository reads, artifact writes, and orchestration. Manifest roles validate bounded loaded values. Evidence verification performs current-root I/O and calls the pure validators.

## Claim Boundary

A passing evidence bundle proves artifact integrity, declared registry linkage, and freshness against the inspected root. It does not prove implementation correctness, test adequacy, requirement satisfaction, or release eligibility.

## Risks / Trade-offs

- Evidence export becomes larger because the complete registry and index are required artifacts.
- Evidence verification depends on the inspected root. This is intentional because source freshness is the feature's main value.
