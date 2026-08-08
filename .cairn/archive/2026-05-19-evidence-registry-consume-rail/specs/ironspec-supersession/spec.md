## ADDED Requirements

### Requirement: Evidence Registry Selection

Cairn MUST provide a deterministic registry selection command that verifies an existing evidence registry, applies stable metadata selectors, and emits exactly one selected evidence index artifact plus a portable selection receipt.

r[cairn.supersede.evidence_registry_consume.select]

#### Scenario: Selects one registry entry

- GIVEN a valid evidence registry with one entry matching the requested stable selectors
- WHEN the operator runs registry selection
- THEN Cairn emits the selected index artifact and a deterministic receipt binding the registry hash, registry receipt hash, selected entry metadata, selector set, and receipt hash

#### Scenario: Rejects ambiguous or missing selection

- GIVEN a valid evidence registry with zero or multiple entries matching the requested stable selectors
- WHEN the operator runs registry selection
- THEN Cairn fails with deterministic issue codes and does not silently choose an entry

### Requirement: Evidence Registry Fetch Plan

Cairn MUST provide a deterministic registry fetch-plan command that verifies an existing evidence registry and emits a metadata-only plan describing which safe index refs, artifact roles, artifact hashes, and receipt-chain hashes downstream agents should fetch and verify.

r[cairn.supersede.evidence_registry_consume.fetch_plan]

#### Scenario: Emits portable fetch plan

- GIVEN a valid evidence registry
- WHEN the operator requests a fetch plan
- THEN Cairn emits sorted plan entries using stable registry metadata and portable path placeholders without performing network fetches

#### Scenario: Rejects invalid registry input

- GIVEN a tampered registry, invalid registry receipt, unsafe path reference, or nondeterministic registry metadata
- WHEN the operator requests a fetch plan
- THEN Cairn fails with deterministic issue codes and no raw host paths, environment values, credentials, or secrets in the receipt

### Requirement: Evidence Registry Consume Policy

Cairn MUST publish registry selection and fetch-plan receipt schemas through Nickel-authored policy and include both commands in the deterministic surface inventory.

r[cairn.supersede.evidence_registry_consume.policy]

#### Scenario: Policy exposes consume receipts

- GIVEN the Nickel policy source
- WHEN policy export freshness is checked
- THEN generated policy includes registry selection and fetch-plan receipt schemas and deterministic surface entries

### Requirement: Evidence Registry Consume Documentation and Verification

Cairn MUST document and test registry selection and fetch-plan behavior with positive and negative evidence, including deterministic receipt hashes and portable path normalization.

r[cairn.supersede.evidence_registry_consume.docs]

r[cairn.supersede.evidence_registry_consume.verify]

#### Scenario: Registry consume rail is verifiable

- GIVEN the source tree after implementation
- WHEN Cairn maintenance and test gates run
- THEN tests, Tracey coverage, strict validation, and policy freshness verify the registry consume rail without checked-in Python or shell helper scripts
