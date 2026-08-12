# Trace Index Specification Delta

## ADDED Requirements

### Requirement: Evidence bundle trace export

r[cairn.trace_index.evidence_bundle.export] Cairn MUST export the content-bound requirement registry, trace index, and passing current-root trace-index verification receipt as required artifacts in its normal evidence bundle.

#### Scenario: Evidence export creates trace artifacts

- GIVEN a valid repository with accepted requirements and declared source links
- WHEN the operator runs `cairn evidence export`
- THEN the bundle MUST contain the canonical requirement registry and trace index
- AND the bundle MUST contain a passing trace-index verification receipt

### Requirement: Trace evidence portability

r[cairn.trace_index.evidence_bundle.portability] Cairn MUST preserve declared trace artifacts through evidence import, republish, indexing, registry selection, mirror verification, and local fetch workflows that operate on generic bundle artifacts.

#### Scenario: Generic evidence transport retains trace artifacts

- GIVEN a valid evidence bundle with declared trace artifacts
- WHEN Cairn imports, republishes, indexes, or locally materializes that bundle
- THEN each trace artifact MUST retain its role, safe relative path, and exact BLAKE3

### Requirement: Typed trace artifact verification

r[cairn.trace_index.evidence_bundle.verification] Cairn MUST validate trace-index and trace-index-verification manifest roles with closed schemas, self-bound identities, and exact requirement-registry binding.

#### Scenario: Matching trace artifacts pass

- GIVEN a manifest with a valid registry, trace index, and passing verification receipt
- WHEN Cairn verifies the manifest
- THEN each trace artifact role MUST pass
- AND the index's requirement-registry BLAKE3 MUST match the declared registry artifact

#### Scenario: Tampered trace artifact fails

- GIVEN a manifest whose trace index, verification receipt, or registry binding changed
- WHEN Cairn verifies the manifest
- THEN verification MUST fail with a deterministic artifact issue

### Requirement: Bundle trace freshness

r[cairn.trace_index.evidence_bundle.freshness] Cairn MUST verify the bundled trace index against the current repository root during evidence bundle verification.

#### Scenario: Current source passes

- GIVEN an unchanged repository and its exported evidence bundle
- WHEN the operator runs `cairn evidence verify`
- THEN current-root trace-index verification MUST pass

#### Scenario: Changed source fails

- GIVEN a repository source changed after evidence export
- WHEN the operator runs `cairn evidence verify`
- THEN verification MUST fail with a stale trace-index issue

### Requirement: Bundle integration validation

r[cairn.trace_index.evidence_bundle.validation] Cairn MUST test successful trace artifact export and verification plus rejected hash tampering, registry mismatch, malformed schemas, and stale source content.

#### Scenario: Positive and negative bundle tests pass

- GIVEN current, tampered, malformed, mismatched, and stale bundle fixtures
- WHEN the focused evidence tests run
- THEN current artifacts MUST pass
- AND invalid artifacts MUST fail with deterministic diagnostics

### Requirement: Trace evidence documentation

r[cairn.trace_index.evidence_bundle.docs] Cairn MUST document the trace artifacts in normal evidence bundles and their declared-linkage claim boundary.

#### Scenario: Operator understands the bundle proof

- GIVEN the evidence and trace-index documentation
- WHEN an operator reads the bundle workflow
- THEN the documentation MUST name the trace artifacts and verification commands
- AND it MUST state that the artifacts do not prove requirement satisfaction or implementation correctness
