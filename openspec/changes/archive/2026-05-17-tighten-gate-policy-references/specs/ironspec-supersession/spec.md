## ADDED Requirements

### Requirement: Native gate reference checks
r[cairn.supersede.gate_policy.references] The system MUST reject native gate receipts when proposal, design, or task artifacts contain unknown `r[...]` references relative to the change delta requirements.

#### Scenario: Unknown reference fails gate
- GIVEN an active change artifact references an unknown `r[...]` identifier
- WHEN native `cairn gate` evaluates the change
- THEN the JSON receipt reports a failing issue naming the unknown reference

### Requirement: Native gate task requirement links
r[cairn.supersede.gate_policy.task_links] The system MUST require each checkbox task at the tasks gate to include a known `r[...]` requirement reference or an explicit dependency marker.

#### Scenario: Unlinked task fails tasks gate
- GIVEN an active change task checkbox lacks a requirement reference and lacks a dependency marker
- WHEN native `cairn gate tasks` evaluates the change
- THEN the JSON receipt reports a failing issue naming the unlinked task line

### Requirement: Native gate bounded exemptions
r[cairn.supersede.gate_policy.exemptions] The system MUST require gate exemption markers to include reason, owner, and until fields before an exemption can satisfy policy.

#### Scenario: Malformed exemption fails gate
- GIVEN an active change artifact contains a gate exemption marker without reason, owner, or until fields
- WHEN native `cairn gate` evaluates the change
- THEN the JSON receipt reports a failing issue naming the malformed exemption
