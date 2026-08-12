## Context

The current traceability core classifies coverage from in-memory files. Its reference occurrences contain only a role, path, and display line.
The content-bound requirement registry already owns canonical requirement rows, source spans, specification hashes, policy hashes, and BLAKE3 identities.
Ambient provides the implementation precedent for separate symbol identity and source occurrences. Unison provides the conceptual precedent for immutable content identity and mutable names.

## Decisions

### Decision: Compose with the content-bound requirement registry

**Choice:** The trace index consumes a validated `ContentBoundRequirementRegistry`. Each link includes the matched requirement-row BLAKE3.

**Rationale:** This avoids a second requirement parser and preserves one authority for requirement content identity.

### Decision: Keep index meaning in the pure core

**Choice:** `cairn-core` parses marker occurrences, validates inputs, calculates domain-separated hashes, sorts records, verifies freshness, and answers requirement queries.

**Rationale:** `cairn-cli` only reads bounded files, selects policy, writes artifacts, and renders JSON.

### Decision: Bind declared markers to exact source content

**Choice:** Each locator includes a safe repository-relative path, exact source-file BLAKE3, exact marker BLAKE3, and a half-open byte span. Line numbers are display data and do not define the marker hash.

**Rationale:** Exact content and byte bounds detect stale, moved, and copied declarations without treating line numbers as identity.

### Decision: Use a declared assurance level

**Choice:** Nickel profiles declare `assurance_level = "declared"` and `anchor_kind = "exact_marker_bytes"` for this trial.

**Rationale:** The index does not claim an AST-resolved symbol, executed test, requirement satisfaction, or code correctness. Later adapters can add structural or executed evidence through new versioned contracts.

### Decision: Export, verify, then query

**Choice:** `traceability export` consumes a current requirement registry and writes a new canonical index. `traceability verify` compares an index with the current root. `traceability show` queries a saved index by requirement ID.

**Rationale:** The saved artifact is directly queryable. Verification remains an explicit freshness operation.

## Risks / Trade-offs

- A comment-only change updates the exact file identity and makes the saved index stale.
- A moved marker updates its locator and link identity, even when the marker text is unchanged.
- Marker-level anchors do not provide rename-stable code identity. AST-aware adapters remain future work.
- The trial requires an exported content-bound requirement registry, which keeps repository and revision identity explicit.
