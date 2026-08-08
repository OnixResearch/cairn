## MODIFIED Requirements

### Requirement: Agent-run receipt replay evidence

Agent-run receipt validation MUST derive required replay evidence coverage from the generated policy `determinism_replay_policy.groups["agent"]` instead of a hidden Rust case list, while preserving deterministic machine-readable validation receipts.

#### Scenario: Policy-derived agent group coverage

r[cairn.supersede.agent_receipt_replay_group_coverage.validation]
- GIVEN generated replay policy defines an `agent` group with replay cases
- WHEN `cairn agent-receipt validate <receipt>` validates a receipt with passing replay evidence
- THEN the validation receipt includes `required_replay_group = "agent"`
- AND `required_replay_cases` equals the sorted cases from the policy `agent` group
- AND validation succeeds only when `replay_selected_cases` covers those policy-derived cases

#### Scenario: Replay selected groups evidence is stable

r[cairn.supersede.agent_receipt_replay_group_coverage.schema]
- GIVEN an agent-run receipt includes `replay_evidence.replay_selected_groups`
- WHEN Cairn validates and summarizes the receipt
- THEN the validation receipt preserves the selected group list under `replay_evidence`
- AND public schema policy lists `replay_selected_groups`, `required_replay_group`, and `required_replay_cases` as stable receipt evidence fields

#### Scenario: Missing or stale group coverage fails deterministically

r[cairn.supersede.agent_receipt_replay_group_coverage.failures]
- GIVEN replay evidence omits the `agent` selected group or omits a policy-required expanded case
- WHEN Cairn validates the receipt repeatedly
- THEN the validation receipts are byte-stable for hash fields
- AND the receipt includes deterministic issue codes for missing group coverage or insufficient case coverage

#### Scenario: Documentation names policy-owned agent replay preflight

r[cairn.supersede.agent_receipt_replay_group_coverage.docs]
- GIVEN an operator is wiring agent receipt wrappers
- WHEN they read Cairn's replay evidence documentation
- THEN the docs identify `cairn determinism replay --group agent` as the policy-owned preflight
- AND the docs explain that validation expands the policy group to required cases

#### Scenario: Replay oracle covers group-backed validation

r[cairn.supersede.agent_receipt_replay_group_coverage.replay]
- GIVEN Cairn's deterministic replay oracle includes agent receipt validation surfaces
- WHEN `cairn determinism replay --group agent` runs
- THEN it exercises the group-backed agent receipt validation path
- AND the replay receipt remains deterministic across repeated runs

