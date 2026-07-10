## Context

The accepted `change-dependency-graph` capability already provides the pure graph core and validation rail. It parses optional `depends_on` arrays from change metadata, rejects bad graph edges, reports dependency readiness, and protects sync/archive execution while active dependencies remain unsatisfied.

The missing layer is operator ergonomics. Hand-editing `metadata.json` bypasses Cairn's normal preview/receipt style and makes dependency graph support harder for agents to discover. Drain tools can already consume the graph, but the authoring path should be just as explicit as `change create`, `change show`, and `change graph`.

## Decisions

### 1. Keep the graph model local and unchanged

**Choice:** This change adds UX around existing local `depends_on` semantics. It does not replace the graph core and does not add cross-repo receipt dependencies.

**Rationale:** Local dependency ordering and cross-repo evidence dependencies solve different problems. Keeping this change focused avoids mixing simple local DAG ordering with evidence-link validation.

### 2. Add pure metadata update helpers

**Choice:** Implement dependency add/remove normalization in `cairn-core` as pure functions over parsed metadata plus known lifecycle names. The CLI shell reads/writes `metadata.json`, prints receipts, and handles dry-run/execute behavior.

**Rationale:** This preserves the functional-core / imperative-shell boundary and lets tests exercise duplicate handling, sorting, self-dependency rejection, unknown dependencies, and cycle prevention without filesystem setup.

### 3. Use preview-by-default mutation commands

**Choice:** `cairn change depend add` and `cairn change depend remove` preview the metadata change by default and require `--execute` to write files. `cairn change depend list`, `cairn change graph`, and `cairn change next` remain read-only.

**Rationale:** Dependency edits affect lifecycle mutation readiness. Preview-by-default gives agents and operators a safe review step while still allowing explicit local mutation.

### 4. Preserve deterministic metadata output

**Choice:** Executed dependency edits preserve unrelated supported metadata fields and write a deterministic `depends_on` array sorted by change id with duplicates removed. Empty dependency lists remove the `depends_on` field rather than writing an empty array.

**Rationale:** Stable output minimizes review noise and keeps graph receipts deterministic.

### 5. Add stable render formats without weakening JSON receipts

**Choice:** `cairn change graph` keeps existing JSON behavior and adds explicit `--format json`, `--format mermaid`, and `--format dot`. Human graph formats use sorted nodes/edges and label blocked versus satisfied dependencies without becoming authoritative receipts.

**Rationale:** JSON stays the source of automation truth. Mermaid/DOT make docs, reviews, and agent summaries easier.

## Risks / Trade-offs

- Preview-by-default is safer but one extra step for quick local metadata edits.
- Human graph render formats add snapshot-test burden; keeping them small and sorted reduces churn.
- Cycle prevention in add/remove commands must reuse the graph validation core so CLI UX cannot diverge from `validate`.
- This change intentionally does not solve cross-repo evidence dependencies; that remains covered by the separate `cross-repo-change-dependencies` change.
