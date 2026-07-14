## Phase 1: Policy and pure inventories

- [x] [serial] Define Nickel contracts and generated runtime policy for artifact substance, minimum inventories, and placeholder classes using named fields. r[cairn.lifecycle_validation.artifact_substance]
- [x] [serial] Build pure proposal/design/task/spec substance inventories over in-memory content and stable source identities. r[cairn.lifecycle_validation.artifact_substance]
- [x] [serial] Require at least one well-formed requirement block per accepted or delta spec and confine normative-strength detection to each requirement body. r[cairn.lifecycle_validation.spec_substance]
- [x] [serial] Require the policy-owned minimum substantive checkbox task count, reject empty/placeholder/unlinked rows, and make zero-substantive-task changes non-archive-ready. r[cairn.lifecycle_validation.task_substance]
- [x] [serial] Keep checkbox counts and task-state deltas as hash-bound progress observations that cannot override current gate, probe, dependency, or receipt blockers. r[cairn.lifecycle_validation.task_progress_non_authority]
- [x] [serial] Parse dependency markers structurally and reject empty, unclosed, malformed, or unsupported payloads without granting link exemptions. r[cairn.lifecycle_validation.dependency_marker_shape]

## Phase 2: Surface integration

- [x] [serial] Emit shared deterministic substance issue codes from repository validation and proposal/design/tasks gates. r[cairn.lifecycle_validation.artifact_substance]
- [x] [serial] Apply the same findings to sync/archive and mutation-capable MCP preflight so vacuous packages cannot mutate lifecycle state. r[cairn.lifecycle_validation.mutation_block]
- [x] [serial] Update scaffold/templates so generated placeholders are clearly incomplete yet easy to replace with gate-ready content. r[cairn.lifecycle_validation.artifact_substance]
- [x] [serial] Update policy-owned receipt schemas and docs for substance counts, issue codes, repair guidance, and non-claims. r[cairn.lifecycle_validation.mutation_block]

## Phase 3: Verification

- [x] [parallel] Add positive fixtures for substantive proposal, design, requirement, scenario, task, and dependency-marker variants. r[cairn.lifecycle_validation.tests]
- [x] [parallel] Add negative fixtures for missing, empty, whitespace-only, placeholder-only, zero-requirement, borrowed-strength, zero-task, checked-placeholder-task, progress-with-evidence-blocker, empty-dependency, unclosed-dependency, and unsupported-dependency inputs. r[cairn.lifecycle_validation.tests]
- [x] [serial] Add CLI/MCP regressions proving all stage gates and mutation guards report the same vacuity classes and preserve files on failure. r[cairn.lifecycle_validation.mutation_block]
- [x] [serial] Run policy freshness, focused core/CLI tests, Cairn validation and stage gates, traceability coverage, Slotcar, and release-readiness checks. r[cairn.lifecycle_validation.tests]
