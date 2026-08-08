## ADDED Requirements

### Requirement: Receipt JSON schema stability
r[cairn.supersede.receipt_schema] The system MUST treat gate, sync, archive, and migration JSON receipt top-level fields as a stable public contract for operator automation and evidence tooling.

#### Scenario: Gate receipt fields remain stable
r[cairn.supersede.receipt_schema.gate]
- GIVEN an Cairn gate command emits a JSON receipt
- WHEN the maintained CLI regression suite runs
- THEN the suite MUST fail if the gate receipt omits or renames required top-level fields for the change, stage, verdict, issues, policy, layout, policy hash, input hash, receipt hash, or validity result

#### Scenario: Sync and archive receipt fields remain stable
r[cairn.supersede.receipt_schema.lifecycle]
- GIVEN Cairn sync or archive commands emit JSON plan receipts
- WHEN the maintained CLI regression suite runs
- THEN the suite MUST fail if the lifecycle receipt omits or renames required top-level fields for the change, actions, blockers, dry-run and mutation state, delta specs where applicable, policy, layout, policy hash, input hash, plan hash, or receipt hash
- AND the suite MUST fail if lifecycle action entries omit or rename their kind, path, or description fields

#### Scenario: Migration receipt fields remain stable
r[cairn.supersede.receipt_schema.migration]
- GIVEN `cairn migrate openspec` emits a JSON migration receipt
- WHEN the maintained CLI regression suite runs
- THEN the suite MUST fail if the migration receipt omits or renames required top-level fields for command, root, source layout, destination layout, actions, conflicts, dry-run and mutation state, policy, policy hash, source manifest hash, destination manifest hash, manifest hash, or receipt hash
- AND the suite MUST fail if migration action entries omit or rename source, destination, source hash, or destination hash fields

#### Scenario: Receipt schema is documented
r[cairn.supersede.receipt_schema.docs]
- GIVEN operators need to consume Cairn receipts outside the CLI process
- WHEN they inspect repository documentation
- THEN the documentation MUST list the stable public fields for gate, sync, archive, and migration receipts
