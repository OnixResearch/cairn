## MODIFIED Requirements

### Requirement: Determinism replay oracle

r[cairn.supersede.determinism_replay] The system MUST provide a maintained policy-driven replay oracle for commands whose receipts claim deterministic evidence.

#### Scenario: Replay plan is policy-owned
r[cairn.supersede.determinism_replay.policy_plan]
- GIVEN Cairn ships a generated policy bundle
- WHEN the workspace replay oracle builds its default plan
- THEN it MUST load maintained replay case metadata from policy rather than relying only on hidden CLI literals
- AND every policy case MUST declare id, command kind, arguments, fixture root, expected mutability, comparison modes, display-only path fields, hash fields, and normalization rules

#### Scenario: Replay policy schema is stable
r[cairn.supersede.determinism_replay.policy_schema]
- GIVEN agents and maintainers review deterministic replay coverage
- WHEN the policy is exported to generated JSON
- THEN the replay plan MUST be present in generated policy with stable keys and deterministic case ordering
- AND tests MUST fail if the generated policy and CLI-supported replay plan drift

#### Scenario: Replay expands across public deterministic surfaces
r[cairn.supersede.determinism_replay.expanded_surfaces]
- GIVEN the policy default replay plan is used
- WHEN an operator runs `cairn determinism replay --root <root>`
- THEN the plan MUST include maintained read-only or dry-run cases for status, validate, proposal/design/tasks gates, sync dry-run, archive dry-run, migration dry-run, agent-receipt validation, release-readiness, MCP describe, and selected MCP read-only calls where fixtures support those surfaces

#### Scenario: Policy replay diagnostics are deterministic
r[cairn.supersede.determinism_replay.policy_diagnostics]
- GIVEN a policy case is unsupported, skipped, or attempts mutation
- WHEN the replay oracle emits its receipt
- THEN skipped and failed policy cases MUST be reported with deterministic issue codes, case ids, command kind, and bounded reason text
- AND mutation-capable cases MUST remain denied unless the case is explicitly dry-run/read-only
