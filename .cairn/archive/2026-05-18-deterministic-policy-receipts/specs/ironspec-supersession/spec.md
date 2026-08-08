# Cairn Supersession Delta

## ADDED Requirements

### Requirement: Nickel policy contract coverage
r[cairn.supersede.nickel_contracts] The system MUST keep lifecycle policy, gate schema, task marker vocabulary, finding severity, exemption shape, and sync/archive policy in Nickel-authored contracts that export deterministic runtime policy data.

#### Scenario: Export typed policy data
r[cairn.supersede.nickel_contracts.export]
- GIVEN Nickel policy contracts for lifecycle stages, gates, task markers, findings, exemptions, and sync/archive policy
- WHEN the maintainer exports Cairn runtime policy JSON
- THEN the export MUST validate against the Nickel contracts before runtime policy data is updated
- AND the generated policy JSON MUST use deterministic field ordering and stable values

#### Scenario: Reject malformed bounded exemption policy
r[cairn.supersede.nickel_contracts.exemptions]
- GIVEN a policy definition for gate exemptions
- WHEN the exemption schema omits reason, owner, or until fields
- THEN the Nickel contract check MUST fail before generated policy JSON is accepted

#### Scenario: Keep runtime logic out of Nickel
r[cairn.supersede.nickel_contracts.boundary]
- GIVEN Nickel-authored policy contracts
- WHEN Cairn evaluates a repository gate, sync, archive, or migration decision
- THEN Nickel data MUST define the allowed policy shape
- AND Rust core logic MUST evaluate shell-supplied repository snapshots against that policy without invoking Nickel at runtime

### Requirement: BLAKE3 evidence identifiers
r[cairn.supersede.blake3_receipts] The system MUST use BLAKE3 hashes to identify canonical policy, input snapshots, plans, and receipts for Cairn decisions that claim validation, gate, sync, archive, migration, or traceability evidence.

#### Scenario: Gate receipt includes stable hashes
r[cairn.supersede.blake3_receipts.gate]
- GIVEN an Cairn gate evaluation with canonical policy data and a canonical change snapshot
- WHEN the operator runs `cairn gate <stage> <change> --root <path>`
- THEN the JSON receipt MUST include BLAKE3 hashes for the policy, gate input snapshot, and emitted receipt content
- AND repeated evaluation of unchanged inputs MUST produce the same hashes

#### Scenario: Sync and archive plans include stable hashes
r[cairn.supersede.blake3_receipts.lifecycle_plans]
- GIVEN a sync or archive dry-run plan
- WHEN the operator runs `cairn sync <change>` or `cairn archive <change>`
- THEN the JSON receipt MUST include BLAKE3 hashes for the canonical plan input and plan output
- AND the plan hash MUST change when planned lifecycle actions change

#### Scenario: Migration manifest includes source and destination hashes
r[cairn.supersede.blake3_receipts.migration]
- GIVEN an explicit `migrate openspec` plan or execution receipt
- WHEN legacy OpenSpec files are mapped into native Cairn paths
- THEN the migration receipt MUST include BLAKE3 hashes for canonical source manifests, destination manifests, and emitted receipt content

#### Scenario: Hashing remains canonical and bounded
r[cairn.supersede.blake3_receipts.canonical]
- GIVEN Cairn computes evidence hashes
- WHEN hashes are generated
- THEN the hashed material MUST exclude nondeterministic process noise, ambient absolute paths unless explicitly part of the contract, and wall-clock timestamps unless supplied as explicit inputs

### Requirement: Deterministic mandatory gates
r[cairn.supersede.deterministic_gates] The system MUST provide deterministic mandatory gates for proposal, design, tasks, pre-sync, pre-archive, migration, policy freshness, and accepted-spec traceability decisions.

#### Scenario: Stage gates are reproducible
r[cairn.supersede.deterministic_gates.stage]
- GIVEN an active change and unchanged policy data
- WHEN the operator runs proposal, design, or tasks gates repeatedly
- THEN each gate MUST return the same verdict, issue list, policy hash, input hash, and receipt hash

#### Scenario: Pre-sync gate blocks unsafe mutation
r[cairn.supersede.deterministic_gates.pre_sync]
- GIVEN a change whose delta specs fail validation or whose task gate fails
- WHEN the operator requests sync execution
- THEN Cairn MUST block filesystem mutation
- AND the receipt MUST identify the deterministic gate failure that prevented sync

#### Scenario: Pre-archive gate blocks incomplete changes
r[cairn.supersede.deterministic_gates.pre_archive]
- GIVEN a change with incomplete tasks, unsynced delta specs, or failing traceability requirements
- WHEN the operator requests archive execution
- THEN Cairn MUST block filesystem mutation
- AND the receipt MUST identify the deterministic gate failure that prevented archive

#### Scenario: Policy freshness gate rejects stale generated policy
r[cairn.supersede.deterministic_gates.policy_freshness]
- GIVEN Nickel policy sources and generated runtime policy JSON
- WHEN the maintained verification gate runs
- THEN it MUST fail if the generated policy JSON is stale relative to Nickel sources

#### Scenario: Traceability gate rejects uncovered accepted requirements
r[cairn.supersede.deterministic_gates.traceability]
- GIVEN accepted specs containing `r[...]` requirements
- WHEN the traceability gate runs
- THEN it MUST fail if an accepted requirement lacks at least one source-controlled implementation or verification reference
