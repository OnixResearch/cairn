# Cairn Supersession Specification Delta

## Purpose

Strengthen Cairn's Nickel policy boundary so Nickel owns public schema declarations, policy vocabularies, and semantic policy invariants while Rust remains the runtime evaluator.

## Requirements

### Requirement: Nickel policy contract coverage
r[cairn.supersede.nickel_contracts] The system MUST keep lifecycle policy, gate schema, task marker vocabulary, finding severity, exemption shape, sync/archive policy, public receipt schemas, and generated policy vocabularies in Nickel-authored contracts that export deterministic runtime policy data.

#### Scenario: Export typed policy data
r[cairn.supersede.nickel_contracts.export]
- GIVEN Nickel policy contracts for lifecycle stages, gates, task markers, findings, exemptions, receipt schemas, generated vocabularies, and sync/archive policy
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

#### Scenario: Receipt schemas are Nickel-owned
r[cairn.supersede.nickel_contracts.receipt_schemas]
- GIVEN Cairn emits gate, sync, archive, and migration JSON receipts as stable public automation contracts
- WHEN the maintained policy export and CLI regression checks run
- THEN Nickel-authored contracts MUST declare required top-level receipt keys, action-entry keys, issue keys, and BLAKE3 hash field shape for each public receipt family
- AND emitted CLI receipts MUST be checked against the generated schema declarations

#### Scenario: Semantic policy invariants fail before generation
r[cairn.supersede.nickel_contracts.semantic_invariants]
- GIVEN a Cairn policy source with unsupported policy version, non-BLAKE3 receipt hashing, wrong canonicalization name, empty required arrays, duplicate vocabulary entries, or lifecycle artifact dependencies referencing unknown artifact IDs
- WHEN the maintained policy export check runs
- THEN the check MUST fail before generated runtime policy JSON is accepted
- AND the failure MUST identify the malformed policy surface closely enough for maintainers to repair it

#### Scenario: Runtime consumes generated policy vocabularies
r[cairn.supersede.nickel_contracts.generated_vocabularies]
- GIVEN Nickel policy exports task markers, marker kinds, lifecycle artifact schemas, finding defaults, and public receipt schema declarations
- WHEN Cairn evaluates task markers, gates, sync/archive readiness, or public receipt stability
- THEN Rust MUST consume the generated policy vocabulary where practical rather than relying on duplicated hardcoded strings
- AND any remaining Rust literals MUST be justified as command syntax or compatibility shell behavior rather than policy source of truth

#### Scenario: Gate exemption markers use structured contracts
r[cairn.supersede.nickel_contracts.exemption_markers]
- GIVEN an active change artifact contains a `gate-exempt[...]` marker
- WHEN native `cairn gate` evaluates the change
- THEN Cairn MUST parse the marker into structured fields and validate required reason, owner, and until/expires date-shape fields against Nickel-owned exemption marker policy
- AND malformed exemption markers MUST produce deterministic gate issues without adding ambient wall-clock time to the pure core

#### Scenario: Contract rails reject drift and malformed examples
r[cairn.supersede.nickel_contracts.contract_rails]
- GIVEN Nickel contracts, generated policy JSON, invalid fixtures, and representative receipt examples
- WHEN the maintained verification rail runs
- THEN it MUST reject stale generated policy, malformed policy semantics, malformed receipt schema examples, duplicate vocabulary entries, and unknown lifecycle artifact dependencies
- AND the rail MUST remain a development/build-time check rather than a runtime dependency of normal Cairn commands
