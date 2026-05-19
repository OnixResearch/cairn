## MODIFIED Requirements

### Requirement: Determinism replay oracle

The system SHALL expose a deterministic replay oracle for agent/CI use.

#### Scenario: CI replay case selection

- r[cairn.supersede.determinism_replay.ci_case_selection]
- **GIVEN** a policy-owned determinism replay plan exists for the root
- **WHEN** an operator runs `cairn determinism replay --root <root> --case <id>`
- **THEN** Cairn SHALL run only the selected policy-owned replay case
- **AND** the replay receipt SHALL list the sorted selected and available case IDs
- **AND** an unknown case ID SHALL produce a deterministic failure receipt rather than an ad hoc diagnostic.

#### Scenario: CI replay output modes and artifacts

- r[cairn.supersede.determinism_replay.ci_output_modes]
- **GIVEN** a determinism replay receipt can be produced for the selected case set
- **WHEN** an operator selects `--format json`, `--format jsonl`, or `--format summary`
- **THEN** Cairn SHALL render deterministic stdout suitable for the selected mode
- **AND** `--out <path>` SHALL write the full JSON replay receipt as an artifact while preserving deterministic stdout rendering.

#### Scenario: CI replay failure ergonomics

- r[cairn.supersede.determinism_replay.ci_failure_ergonomics]
- **GIVEN** a replay run produces invalid evidence
- **WHEN** replay detects an unknown case, mismatch, hidden entropy, or other invalid replay evidence
- **THEN** Cairn SHALL return a nonzero exit status
- **AND** mismatch evidence SHALL include stable failing case IDs, mismatch classes, field paths, expected/actual hash summaries, and a reproduction command.

#### Scenario: CI replay tests and public documentation

- r[cairn.supersede.determinism_replay.ci_tests_docs]
- **GIVEN** replay selection and rendering are public agent/CI surfaces
- **WHEN** Cairn changes replay selection, output modes, artifact behavior, or failure receipts
- **THEN** tests, public schema checks, and operator documentation SHALL cover those stable public replay surfaces.
