## Context

`cairn determinism replay --root <root>` currently uses hard-coded Rust case definitions. The receipt is stable, but the plan itself is not a policy artifact and coverage is still narrower than the accepted deterministic public surface.

## Decisions

### 1. Policy owns replay case metadata

**Choice:** Add `determinism_replay_policy.cases` to the Nickel policy and generated JSON.

**Rationale:** The case list, compared hash fields, display-only paths, and expected mutability are operator contracts. Keeping them in policy makes drift visible and gives tests a single generated source of truth.

**Implementation:** Add a contract for replay cases, generate JSON, and have the CLI parse supported case IDs from the policy. Unknown or unsupported cases are skipped with deterministic diagnostics rather than silently ignored.

### 2. Expand with fixture-supported read-only/dry-run cases

**Choice:** The default policy plan covers `status`, `validate`, `gate proposal/design/tasks`, `sync` dry-run, `archive` dry-run, `migrate` dry-run, `agent-receipt validate`, `release-readiness`, `mcp --describe`, and selected MCP read-only calls.

**Rationale:** These are the public deterministic surfaces agents and CI are most likely to consume.

**Implementation:** Each case maps to existing pure receipt builders or dry-run plan builders. Cases requiring special fixtures carry fixture selectors in policy so the root under replay is still copied and compared deterministically.

### 3. Preserve non-mutating replay semantics

**Choice:** Replay continues to deny `--execute`/mutation flags and fails if any case reports mutation.

**Rationale:** Replay must be safe for agents/CI and should prove deterministic evidence without altering source fixtures.

## Risks / Mitigations

- **Policy/CLI drift:** Tests assert generated policy freshness and replay case IDs.
- **Fixture mismatch:** Policy case fixtures are explicit; skipped cases are listed deterministically if unsupported.
- **Golden churn:** Prefer schema and semantic tests for replay; existing plan goldens remain for lifecycle receipts.

## Validation

Run focused replay tests, full `cargo test -p cairn-cli`, policy export check, validation/gates, Tracey coverage, and release-readiness before archive.
