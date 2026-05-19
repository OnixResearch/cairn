## MODIFIED Requirements

### Requirement: Determinism replay oracle

Cairn SHALL provide a deterministic replay oracle for policy-owned read-only and dry-run surfaces.

#### Scenario: Policy-owned replay groups

- **GIVEN** the generated replay policy defines named replay groups
- **WHEN** operators inspect the policy or run replay group selection
- **THEN** group membership SHALL be policy-owned and SHALL refer only to known replay case IDs
r[cairn.supersede.determinism_replay.groups.policy]

#### Scenario: Replay group CLI selection

- **GIVEN** an operator wants a named replay suite
- **WHEN** they run `cairn determinism replay --group <id>`
- **THEN** Cairn SHALL replay the policy-defined cases for that group while preserving the existing default full-suite behavior
- **AND** `--case` and `--group` used together SHALL be rejected before replay
r[cairn.supersede.determinism_replay.groups.cli]

#### Scenario: Replay group failure receipts

- **GIVEN** an operator selects an unknown replay group or combines incompatible replay selectors
- **WHEN** replay handles the selection failure
- **THEN** Cairn SHALL emit a deterministic machine-readable failure receipt with stable mismatch class, expected values, actual selector, and reproduction command
r[cairn.supersede.determinism_replay.groups.failures]

#### Scenario: Replay group receipt schema

- **GIVEN** replay receipts are consumed by agents and CI
- **WHEN** replay emits a receipt
- **THEN** the stable public receipt fields SHALL include `available_groups` and `selected_groups` alongside existing case selection fields
r[cairn.supersede.determinism_replay.groups.receipt]

#### Scenario: Replay group documentation and tests

- **GIVEN** replay groups are public CLI and policy behavior
- **WHEN** Cairn changes replay groups or selection behavior
- **THEN** documentation, tests, stable schema checks, and Tracey coverage SHALL describe and verify the behavior
r[cairn.supersede.determinism_replay.groups.docs]
