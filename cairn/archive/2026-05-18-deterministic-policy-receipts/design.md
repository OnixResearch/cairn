## Context

Cairn already has a native layout, Nickel policy sources, JSON runtime receipts, native gate commands, Tracey coverage rails, and Slotcar-enforced FCIS boundaries. The next step is to make policy shape, evidence identity, and lifecycle gate decisions explicit enough that operators can audit why a change was allowed to sync/archive and can reproduce the same decision from the same inputs.

## Goals / Non-Goals

**Goals:**

- Expand Nickel contracts so policy/config/schema decisions are type-checked before generated runtime JSON is accepted.
- Add canonical BLAKE3 evidence identifiers to receipts that claim validation, gate, sync/archive, migration, policy, or traceability facts.
- Make mutation-adjacent lifecycle decisions depend on deterministic gates with stable receipts.
- Preserve FCIS: Rust core evaluates canonical data; CLI performs reads/writes and rendering.

**Non-Goals:**

- No runtime Nickel interpreter dependency.
- No NLP, network, or ambient-time gate decisions.
- No hashing of incidental process logs or noncanonical output.
- No implicit fallback to legacy `openspec/` content.

## Decisions

### 1. Nickel owns policy shape, not evaluation

**Choice:** Nickel contracts define accepted schemas for lifecycle stages, gates, task markers, findings, exemptions, sync/archive policy, migration policy, and receipt policy. Exported JSON remains the runtime artifact consumed by Rust.

**Rationale:** Nickel is strongest at declarative configuration contracts and mergeable policy. Rust is stronger for deterministic parsing, planning, and testable evaluation.

**Implementation:** Extend `cairn-policy/` contracts/defaults and keep `tools/export-policy.py --check` as a freshness gate. Runtime code reads checked-in generated JSON only.

### 2. BLAKE3 hashes identify canonical evidence

**Choice:** Receipts include hashes for canonical policy, canonical input snapshots, planned actions/manifests, and the emitted receipt where applicable.

**Rationale:** Hash-linked receipts make gate/sync/archive/migration claims auditable without storing every raw artifact in every receipt.

**Implementation:** Add pure canonicalization and BLAKE3 helpers in the core for data models that are already shell-supplied. CLI attaches hashes to JSON receipts after deterministic ordering is applied.

### 3. Gates become mandatory before mutation

**Choice:** Proposal/design/tasks gates stay explicit commands, while pre-sync and pre-archive checks become required blockers for `--execute` mutations.

**Rationale:** Mutation commands should not rely on operator memory to run the right checks first.

**Implementation:** Sync/archive execution plans include gate verdict inputs and refuse mutation when required deterministic gates fail. Dry-run receipts expose the same blockers.

## Risks / Trade-offs

- **Hash churn:** Canonical schemas must avoid incidental ordering/path/timestamp changes or receipts become noisy.
- **Policy surface growth:** Nickel contracts should cover stable policy boundaries, not every internal Rust detail.
- **Migration complexity:** Legacy OpenSpec manifests need explicit canonical source/destination rules to avoid accidental fallback semantics.
- **Receipt compatibility:** CLI goldens will need intentional updates when receipt fields gain hashes.
