## Context

`merge_delta_spec` extracts blocks only from exact delta section headings and returns an empty operation set for other Markdown shapes. `execute_sync_plan` still writes the computed result and `run_sync` unconditionally sets `mutated=true` after the loop succeeds. This permits a successful no-op receipt for a full-spec-shaped delta.

Replacement currently collects requirement headings and removes accepted blocks whose heading string matches. Requirement headings are presentation text, not stable identity. A MODIFIED block that keeps the bare requirement identity `demo.existing` but changes its heading does not replace the original; it creates two accepted blocks with the same ID, and `validate_spec_sources` does not diagnose the collision.

The sync `input_hash` is derived from `ChangeStatus`, which contains artifact presence, task counts, metadata, and delta paths but not the delta bytes or accepted-spec pre-state.

## Decisions

### 1. Parse a typed delta model in pure core

**Choice:** Introduce an in-memory delta parser that emits ordered operations with operation kind, canonical requirement ID, heading, normalized block, and BLAKE3-bound block identity, plus deterministic diagnostics.

**Rationale:** Planning and merge decisions become explicit, testable, and independent of filesystem effects.

### 2. Require explicit non-empty operation sections

**Choice:** Sync accepts reviewed `## ADDED Requirements`, `## MODIFIED Requirements`, and `## REMOVED Requirements` sections. A full accepted-spec projection, empty file, section with no requirement operation, heading inside a fenced example, or unsupported/ambiguous shape is a blocker with repair guidance.

**Rationale:** Silent interpretation is unsafe. Cairn should reject an unrecognized delta instead of claiming success while dropping intent.

### 3. Use requirement IDs as merge keys

**Choice:** Every operation block carries exactly one valid bare `r[...]` requirement ID associated with the requirement heading. Accepted pre-state must also have unique requirement IDs. Heading text may change during MODIFIED operations without changing identity.

**Rationale:** IDs are the lifecycle identity contract; headings are mutable prose.

### 4. Define target cardinality and idempotency

**Choice:** ADDED targets an absent ID; an identical existing block is reported as already applied, while different existing content is a conflict. MODIFIED targets exactly one existing ID; identical desired content is already applied and an unknown ID is a blocker. REMOVED targets exactly one existing ID; an already-absent ID is reported as already applied. Duplicate operation IDs or duplicate accepted IDs block the whole semantic plan.

**Rationale:** Replays remain safe without letting contradictory content overwrite silently.

### 5. Precompute every semantic result before writing

**Choice:** The shell loads bounded delta and accepted bytes, passes them to the core, and receives all resulting documents and blockers before the first write. Any semantic blocker clears mutation actions. Filesystem write failure handling remains a shell concern and may be strengthened separately.

**Rationale:** One malformed delta must not permit earlier semantic actions to commit before the plan is known to be valid.

### 6. Bind source, pre-state, operations, and result

**Choice:** Sync hash material includes root-portable source paths, delta content hashes, accepted before hashes, normalized operation summaries, expected after hashes, policy hash, and blocker/status classes. Raw absolute prefixes remain display-only.

**Rationale:** A dry-run receipt must identify the exact merge it authorized.

### 7. Derive mutation from byte changes

**Choice:** `mutated=true` requires at least one successful before/after byte change. A fully already-applied plan reports a deterministic no-op status with `mutated=false`; a blocked plan performs no writes.

**Rationale:** Receipt language must describe observed effects rather than attempted calls.

## Risks / Trade-offs

- Historical full-spec-shaped change packages will become explicit blockers instead of silent no-ops; diagnostics and migration guidance must show how to add delta headings.
- Duplicate requirement IDs already present in accepted specs will block affected syncs until repaired.
- Receipt hashes and possibly additive receipt fields will change because sync finally binds source and pre-state content.
- Passing merge validation proves only deterministic delta application, not requirement quality or implementation correctness.
