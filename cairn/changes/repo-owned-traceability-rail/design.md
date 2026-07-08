# Design: Repo-owned traceability rail

## Context

Cairn already exposes `cairn tracey coverage --root .` as a deterministic bootstrap rail. That command avoids a runtime dependency on Tracey but still carries Tracey naming, hard-coded scan roots, and limited receipt metadata. Trellis still uses external Tracey directly and has documented transient daemon-start cancellation. The stack should keep the `r[...]` marker invariant while removing external Tracey from authoritative gates.

## Decisions

### 1. Authoritative command uses traceability language

**Choice:** Add `cairn traceability coverage --root PATH` as the maintained command and keep `cairn tracey coverage --root PATH` as a compatibility alias.

**Rationale:** The evidence model is requirement traceability, not the external Tracey implementation. An alias preserves scripts while new docs and gates stop teaching the external tool name as authoritative.

### 2. Functional core owns coverage classification

**Choice:** Extract marker parsing and coverage classification into pure deterministic core functions that consume an in-memory profile and file snapshot. The CLI owns filesystem traversal, bounded reads, profile loading, and JSON presentation.

**Rationale:** Coverage decisions should be testable without standing up a repository, process, daemon, clock, or filesystem. This preserves the FCIS boundary and makes fixture-based positive and negative tests cheap.

### 3. Nickel owns scan profiles

**Choice:** Define traceability scan profiles in reviewed Nickel/policy data and export deterministic runtime JSON. The default Cairn profile preserves current accepted-spec and Rust/Nickel source behavior; downstream profiles can model Trellis-style `docs/requirements.md`, `src/**/*.rs`, and `tests/**/*.rs` roots.

**Rationale:** Tracey Styx config is external to Cairn's policy model. Nickel contracts can validate profile identifiers, include roots, source kinds, allowed marker verbs, and file-extension filters before runtime.

### 4. External Tracey remains optional

**Choice:** Keep docs for `tracey query ...` only as optional developer diagnostics. Maintained gates, release readiness, and Nix checks use the Cairn-owned command. Forking external Tracey is deferred unless a reviewed gap cannot be closed in the owned checker.

**Rationale:** Forking adds a separate release stream. The narrower and more durable path is to own the exact subset of traceability semantics the stack depends on.

### 5. Receipts stay bounded and reproducible

**Choice:** Emit stable JSON with profile identity, counts, missing requirement IDs, dangling reference IDs, bounded source summaries, input hash, policy hash, verdict, and receipt hash.

**Rationale:** Operators and downstream repos need reproducible evidence that can be replayed and compared without raw source content or daemon state.

## Risks / Trade-offs

- **Command churn:** Adding `traceability coverage` while keeping `tracey coverage` avoids immediate breakage but creates two names during migration.
- **Profile scope creep:** The first implementation should cover Cairn and Trellis-style profiles only; broad glob languages or semantic parsing are out of scope.
- **Overclaiming:** The checker proves marker coverage only. Docs must continue to state that traceability does not prove behavioral correctness.
- **Fork temptation:** Forking Tracey may look faster for gaps, but should remain a fallback after the owned checker subset is evaluated.
