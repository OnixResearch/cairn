# Design: Portable Determinism Usage Rails

## Context

Cairn currently separates pure deterministic core decisions from CLI shell effects and already uses BLAKE3 hashes for policy, input snapshots, plans, and receipts. The remaining risk is not the hash function; it is what enters the hashed material when Cairn is used by agents, CI, temp fixtures, and MCP clients.

## Goals / Non-Goals

**Goals:**
- Make semantically identical repository states produce identical evidence hashes across checkout roots.
- Detect accidental nondeterminism from filesystem ordering, absolute paths, environment variables, archive dates, external command probes, and MCP presentation.
- Preserve FCIS: normalization and decision logic stay pure where possible; filesystem probing and command execution remain in CLI shell.
- Keep receipt evolution auditable with schema tests and explicit migration choices.

**Non-Goals:**
- Replaying arbitrary external network/model behavior inside Cairn.
- Replacing agent-run receipt validation with a full transcript recorder.
- Removing intentionally user-facing path fields from human-readable JSON when they are not part of stable hash material.

## Decisions

### 1. Separate display paths from portable hash paths

**Choice:** Receipts may continue to display operator-facing paths, but canonical hash inputs must use normalized root-relative paths for repository artifacts unless the command explicitly declares an absolute path as part of the input contract.

**Rationale:** Agents and CI often copy fixtures into fresh temp roots. Absolute checkout paths should not change a lifecycle decision hash.

**Alternative:** Strip all paths. Rejected because operators need actionable receipts and mutation manifests.

**Implementation:** Build a pure normalizer that accepts shell-supplied path observations plus a declared root and returns stable relative path strings for hash material.

### 2. Add a command replay oracle

**Choice:** Add either `cairn determinism-check --root <path> -- <command...>` or an equivalent maintained test helper that runs commands multiple times and compares normalized JSON plus selected hash fields.

**Rationale:** A replay oracle prevents regressions from `read_dir` ordering, map ordering, clock/env leaks, and command-output drift.

**Alternative:** Rely on existing golden tests only. Rejected because golden tests do not prove cross-root portability.

**Implementation:** Start with direct CLI tests over copied fixtures, then expose a CLI command if operators need it.

### 3. Capture environment and date entropy explicitly

**Choice:** Receipt inputs that depend on environment variables, archive date sources, external command availability, or policy paths must disclose those inputs in a bounded manifest or fail if they are hidden.

**Rationale:** Hidden entropy makes replay evidence misleading even when output looks stable in one checkout.

**Alternative:** Ban all environment use. Rejected because `CAIRN_ARCHIVE_DATE` is an intentional explicit input for real archives.

### 4. Record before/after mutation manifests

**Choice:** Side-effecting lifecycle executions emit BLAKE3 manifests of planned and touched files before and after mutation.

**Rationale:** Determinism after use requires evidence of what changed, not just that a plan was executed.

**Alternative:** Trust dry-run plan hashes. Rejected because execution can fail or partially mutate for shell reasons outside the pure plan.

### 5. Extend MCP/read-side fixtures

**Choice:** Add reproducibility tests for MCP describe/call surfaces and read commands, including cross-root normalization where relevant.

**Rationale:** Agents will likely use Cairn through MCP or read-side commands; these are first-class use surfaces.

## Risks / Trade-offs

- **Schema churn** → Mitigate by adding fields additively or bumping schema versions where necessary.
- **Operator confusion between display paths and hash paths** → Mitigate with documentation and field names such as `display_path` vs `canonical_path`/`hash_input`.
- **False confidence from replay oracle coverage** → Mitigate with both positive cross-root fixtures and negative hidden-entropy fixtures.
