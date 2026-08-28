## Goals

- Measure whether patience line matching improves Cairn lifecycle review.
- Preserve exact source changes and deterministic output.
- Keep the candidate test-only and bounded.
- Produce an explicit owner decision with evidence.

## Pilot boundary

The pilot receives two already-loaded text artifacts, their identities, a selected profile, and explicit limits.

The pure comparator returns candidate hunks, exact changed spans, structural metrics, or a typed rejection.

The shell owns file reads, terminal rendering, clocks, review collection, and artifact writes.

Default Cairn diff, sync, archive, view, and receipt behavior remains unchanged.

## Decisions

### Decision: Keep the current diff as a baseline

**Choice:** Every corpus case stores current output and compares both candidates with the same exact source pair.

**Rationale:** The pilot needs a stable control.

### Decision: Preserve exact changes

**Choice:** Candidate output can reorder presentation hunks. It cannot omit changed bytes, collapse unequal numbers, or normalize meaningful whitespace.

**Rationale:** Review clarity must not hide lifecycle meaning.

### Decision: Reject numeric tolerance

**Choice:** Numeric-tolerance comparison is outside the pilot and has a negative fixture.

**Rationale:** A small numeric change can alter a bound, version, hash, date, threshold, or port.

### Decision: Use a lifecycle-specific corpus

**Choice:** The corpus covers requirements, scenarios, task lists, reordered sections, long lines, tables, identifiers, code blocks, numbers, whitespace, and line endings.

**Rationale:** General source-code examples do not prove lifecycle review value.

### Decision: Separate structural metrics from owner review

**Choice:** Deterministic metrics cover changed-span completeness, hunk stability, output size, runtime bounds, and locality.

A bounded owner review records readability preferences without becoming an automated correctness claim.

**Rationale:** Readability has a human component.

### Decision: Policy owns acceptance

**Choice:** Typed Nickel policy owns required fixtures, limits, structural thresholds, review quorum, and decision rules.

**Rationale:** Acceptance values must remain reviewable and named.

### Decision: Adoption needs another change

**Choice:** The pilot returns `AdoptCandidate`, `RejectCandidate`, or `InsufficientEvidence`.

A later Cairn must define CLI selection, compatibility, output versioning, migration, documentation, and rollback.

**Rationale:** Pilot evidence does not authorize default behavior.

## Verification

Positive fixtures cover stable unique anchors, moved Markdown sections, local word changes, exact numbers, code blocks, and deterministic reruns.

Negative fixtures cover binary input, invalid UTF-8, oversized input, excessive output, numeric tolerance, omitted changes, control characters, and candidate divergence.

## Evidence and non-claims

Evidence binds source revisions, corpus, baseline, candidates, profile, outputs, metrics, and owner decision.

Agreement proves only exact changed-span coverage for the supplied corpus. It does not prove review quality or implementation correctness.
