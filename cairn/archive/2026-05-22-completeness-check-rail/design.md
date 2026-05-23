## Context

The strongest lesson from the gate-analysis article is that agents usually forget work rather than contradict themselves. Cairn already links tasks to requirements, but it does not yet provide a dedicated completeness receipt that maps requirements, acceptance criteria, tasks, implementation refs, and verification refs into explicit omission findings.

## Decisions

### 1. Completeness is a rail, not an oracle

**Choice:** The rail checks declared mappings and refs; it does not infer hidden requirements.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 2. Omission-first issue model

**Choice:** Missing mappings and missing evidence are reported as `omission` category findings.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 3. No mutation in first slice

**Choice:** The command is read-only and suitable for preflight/release evidence composition.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

## Risks / Trade-offs

- False confidence if teams omit acceptance criteria entirely.
- Overly strict defaults may block early planning packages; provide policy-owned modes.
