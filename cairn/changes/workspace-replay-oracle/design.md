## Context

Portable hashes and mutation manifests make individual Cairn receipts more deterministic, but users still need a one-command way to prove those properties on the current workspace. Agents often run from ephemeral checkout paths and CI workers frequently copy or cache source trees. A regression in absolute-path handling, JSON key stability, MCP schema ordering, or hidden environment reads should be caught before an agent trusts Cairn receipts.

## Goals / Non-Goals

**Goals:**

- Provide a maintained `determinism replay` operator surface that agents/CI can run as a preflight check.
- Cover repeated-run and cross-root invariants for Cairn's public read/dry-run/MCP surfaces.
- Emit a deterministic oracle receipt that is itself portable and BLAKE3-addressed.
- Make mismatches actionable by reporting command, field path, mismatch class, and expected/actual hashes.
- Reject mutation attempts and uncaptured entropy before they can affect fixtures.

**Non-Goals:**

- No default execution of `sync --execute`, `archive --execute`, or migration mutation flows.
- No attempt to make external commands deterministic if they are outside Cairn's explicit evidence model.
- No distributed replay between different hosts in this change.

## Decisions

### 1. Oracle plan is explicit and policy-bounded

**Choice:** Represent the replay surface as an explicit plan of command cases, not ad hoc shell scripts.

**Rationale:** Cairn can sort cases, document command coverage, validate mutability, and produce a stable receipt. Agents should not need to infer which commands are safe.

**Alternative:** Keep replay as integration tests only. Rejected because CI/agent users need an operator-facing receipt.

**Implementation:** Add a plan builder for built-in cases and a bounded fixture root selector. Each case records command kind, args, expected mutability, normalization rules, and hash fields to compare.

### 2. Two comparison modes

**Choice:** Use strict normalized JSON equality for repeated runs in the same root and portable evidence equality for cross-root runs.

**Rationale:** Repeated runs should be byte/JSON stable after canonicalization. Cross-root runs may legitimately differ in display path fields, but portable hashes and normalized semantic fields must match.

**Alternative:** Strip all paths before comparison. Rejected because it can hide broken operator-facing evidence.

**Implementation:** Each case declares display-only JSON paths allowed to differ across roots. All other normalized fields must match.

### 3. Oracle receipt is deterministic evidence

**Choice:** The oracle emits a receipt with sorted cases, per-run output hashes, normalized-output hashes, compared field paths, mismatch list, skipped list, and a receipt hash.

**Rationale:** The oracle should be consumable by agents without parsing console logs.

**Alternative:** Exit status only. Rejected because it is not enough to diagnose nondeterminism.

**Implementation:** Reuse canonical JSON/BLAKE3 helpers and root-relative path normalization. The oracle's own receipt hash excludes display-only temp-root values.

### 4. Mutations are denied by default

**Choice:** Replay cases are read-only or dry-run by default, and any case that declares or observes mutation fails unless an explicit future unsafe mode is added.

**Rationale:** Agents/CI should be able to run replay preflights without changing the workspace.

**Alternative:** Run execute flows in copied temp roots. Rejected for this slice because lifecycle mutation evidence is already covered by focused tests and should not be the default operator preflight.

### 5. Hidden entropy becomes a named failure

**Choice:** If the oracle detects or is told about replay-relevant environment, clock, random, filesystem-order, external-command, or absolute-path entropy that is not declared in the case plan, it fails with a deterministic issue code.

**Rationale:** Determinism bugs should be rejected, not normalized away silently.

**Alternative:** Warn only. Rejected because agents may treat warnings as success.

## Risks / Trade-offs

**Fixture copying cost** → Keep default fixtures small and bounded; allow future policy limits for file count and bytes.

**Over-normalization can mask bugs** → Keep display-only difference allowlists explicit and golden-tested.

**Coverage drift** → Add public schema docs and tests that ensure every built-in replay case names its command kind, mutability, normalization, and hash comparisons.

**MCP/CLI mismatch noise** → Start with selected read-only MCP calls and expand only when corresponding CLI semantics are stable.
