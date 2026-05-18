# Cairn Supersession Delta

## MODIFIED Requirements

### Requirement: Cairn read-side parity
r[cairn.supersede.read_parity] The system MUST provide deterministic read commands for active changes and accepted specs without requiring the upstream OpenSpec CLI.

#### Scenario: List active changes
r[cairn.supersede.read_parity.change_list]
- GIVEN an OpenSpec-compatible repository with active changes
- WHEN the operator runs `cairn change list --root <path>`
- THEN the CLI emits a deterministic JSON receipt containing sorted active change summaries and task counts.

#### Scenario: Show active change
r[cairn.supersede.read_parity.change_show]
- GIVEN an OpenSpec-compatible repository with an active change
- WHEN the operator runs `cairn change show <name> --root <path>`
- THEN the CLI emits a deterministic JSON receipt for that change including artifact presence, delta specs, and task counts.

#### Scenario: List accepted specs
r[cairn.supersede.read_parity.spec_list]
- GIVEN an OpenSpec-compatible repository with accepted specs
- WHEN the operator runs `cairn spec list --root <path>`
- THEN the CLI emits a deterministic JSON receipt containing sorted accepted spec summaries.

#### Scenario: Show accepted spec
r[cairn.supersede.read_parity.spec_show]
- GIVEN an OpenSpec-compatible repository with an accepted spec
- WHEN the operator runs `cairn spec show <id> --root <path>`
- THEN the CLI emits a deterministic JSON receipt containing the requested spec id, path, and markdown content.

### Requirement: Supersession boundary
r[cairn.supersede.boundary] The system MUST keep lifecycle templates, read-model decisions, and validation decisions in the pure core while filesystem access and JSON presentation remain in the CLI shell.

#### Scenario: Pure read models
r[cairn.supersede.boundary.read_models]
- GIVEN change/spec read commands
- WHEN `cairn-core` builds the change/spec read models
- THEN it consumes only caller-provided strings/status values and performs no filesystem, environment, process, time, or stdout/stderr effects.
