## Context

Cairn already has stack provenance gates and release-readiness evidence surfaces. Function-address evidence should compose with those instead of creating a parallel lifecycle model. The gate consumes receipt metadata and graph summaries produced upstream; it never performs source analysis, build analysis, or evidence production.

The first slice should support optional context for repositories that are not ready to require the rail and required mode for Onix stack releases that want every checked function tied to content-address evidence.

## Decisions

### 1. Make policy mode explicit

**Choice:** Add disabled, optional, and required modes for function-address evidence. Disabled mode does not evaluate the evidence. Optional mode records valid evidence or absence as context. Required mode fails release-readiness on absence, staleness, malformed metadata, or invalid non-claims.

**Rationale:** Cairn should be usable across repos at different adoption stages without weakening required stack policy.

### 2. Consume Valence/Mantle receipts, not raw function inventories

**Choice:** The readiness gate validates Valence function-address profile receipts and Mantle bundle-local binding metadata. It may record Kamacite receipt identity as context, but it does not parse individual function rows.

**Rationale:** Valence owns evidence linkage; Mantle owns release artifact binding; Kamacite owns portable receipt identity. Cairn owns policy over their receipts.

### 3. Preserve verification-role boundaries

**Choice:** Function-address evidence contributes identity/linkage context by default. It counts as required evidence when policy requires presence, but it does not count as property proof unless a separate Valence-linked proof receipt is eligible under existing verification-role rules.

**Rationale:** A content address is not a behavioral proof. Cairn should prevent accidental promotion.

### 4. Keep readiness validation pure

**Choice:** Core readiness validation operates on loaded records and policy values. CLI shell code owns file reads, digest measurement, command arguments, receipt rendering, and stdout/stderr.

**Rationale:** This follows Cairn's functional-core / imperative-shell boundary and keeps fixtures deterministic.

### 5. Make non-claims visible in readiness output

**Choice:** Release-readiness receipts and docs must state that passing function-address evidence proves lifecycle policy conformance over supplied receipts only, not source correctness, build correctness, verifier soundness, or release eligibility outside the configured policy.

**Rationale:** Readiness gates are policy evidence, not a substitute for upstream analyzers or proofs.

## Risks / Trade-offs

- Required mode can fail releases until upstream evidence producers exist. Optional mode gives a migration path.
- Graph summaries can look semantically rich. The gate must keep summary fields as context bound to receipt hashes.
- If the gate duplicates stack-provenance logic, drift risk increases. Reuse existing artifact-ref, verification-role, and non-claim validation primitives where possible.
