## Context

`validate_spec_source` iterates requirement and scenario headings but emits no issue when a spec has no such headings. `required_artifact_issues` and `change_issues` distinguish only present from absent files. `count_tasks` returns zero for an empty task document, and `incomplete_tasks_blocker` treats zero todo, in-progress, and unmarked tasks as archive-ready. `has_dependency_marker` accepts any task line containing the configured dependency prefix, without requiring a closed or non-empty payload.

A reproduced repository containing empty proposal, design, tasks, accepted spec, and delta spec files returned `valid=true` from repository validation and `PASS` from all three stage gates.

## Decisions

### 1. Put substance policy in Nickel

**Choice:** Define reviewed minimum-count, required-content-class, and placeholder-token policy for proposal, design, tasks, accepted specs, and delta specs in Nickel and export it to runtime JSON.

**Rationale:** Required artifact semantics are configuration and schema policy, not hidden Rust literals. Named policy fields avoid unexplained thresholds.

### 2. Inventory structure in pure core

**Choice:** Extend in-memory artifact inputs with content and build pure inventories for non-whitespace text, recognized sections, requirement blocks, bare IDs, normative body statements, scenarios, checkbox tasks, and parsed task markers.

**Rationale:** The core can decide substance without reading files or relying on shell state.

### 3. Reject vacuity, not stylistic variation

**Choice:** Required artifacts must contain policy-recognized substantive content, but the validator will not prescribe one prose style beyond configured sections and existing EARS-style contracts. Placeholder-only generated skeleton text is not substantive.

**Rationale:** The gate should prevent empty success without becoming an opinionated prose formatter.

### 4. Keep normative strength in the requirement body

**Choice:** A requirement's normative strength must occur in its own body before the first scenario or peer heading. A MUST/SHALL/SHOULD/MAY token appearing only in a scenario, fenced example, comment, or later requirement does not strengthen the requirement.

**Rationale:** The current whole-block search can borrow strength from unrelated text and produce another form of vacuous pass.

### 5. Require real task work

**Choice:** A gate-ready task artifact contains at least the policy-owned minimum number of substantive checkbox tasks. Empty labels, generated placeholders, checkbox-only rows, and rows without a valid requirement or dependency linkage do not count. Archive readiness additionally requires every counted task to be complete and valid; zero substantive tasks is not completion.

**Rationale:** Absence of work, or a checked placeholder, is not evidence that work finished.

### 6. Parse dependency markers

**Choice:** Validate the full configured dependency-marker shape, including delimiters, non-empty payload, allowed characters, and policy-defined target class. A malformed marker neither satisfies concurrency marking nor exempts a task from a known requirement link.

**Rationale:** Prefix matching is not structural validation.

### 7. Share diagnostics across surfaces

**Choice:** Repository validation, proposal/design/tasks gates, sync/archive guards, and MCP lifecycle receipts consume the same pure substance findings and stable issue codes.

**Rationale:** A package must not be substantive on one path and vacuous on another.

### 8. Keep progress counters non-authoritative

**Choice:** Cairn may expose unchecked counts, checked counts, and task-state deltas for scheduling and progress reporting, but no counter or task-file mutation may independently derive readiness or completion. The current authoritative gate must still pass with every required probe, dependency, and receipt blocker resolved for the current input hash.

**Rationale:** A smaller todo count says work state changed; it does not prove the change is correct or verified.

## Risks / Trade-offs

- Fresh `change create` skeletons will intentionally validate as incomplete until placeholders are replaced with reviewed content.
- Existing historical archives remain historical inputs; active changes and accepted specs may need one-time repairs if they rely on empty or placeholder-only artifacts.
- Minimum substance proves only that required lifecycle structure is present. It does not prove prose quality, implementation correctness, or requirement satisfaction.
