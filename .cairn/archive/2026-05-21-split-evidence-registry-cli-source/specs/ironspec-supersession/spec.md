## ADDED Requirements

### Requirement: Focused Evidence Registry CLI Source

The system MUST keep evidence registry CLI implementation in focused source modules so registry command-family changes can be reviewed without editing one dense implementation file.

#### Scenario: Registry source is split by subfamily

- GIVEN the existing evidence registry command-family implementation r[cairn.supersede.split_evidence_registry_cli_source.focused_modules]
- WHEN the source split is complete
- THEN registry build/verify, mirror, consume/select/fetch-plan, and fetch-execute logic are organized into focused Rust modules under the CLI crate

#### Scenario: Registry CLI behavior is preserved

- GIVEN existing evidence registry CLI commands and receipts r[cairn.supersede.split_evidence_registry_cli_source.behavior_preserved]
- WHEN focused and full verification run after the source split
- THEN the existing registry/evidence integration tests continue to pass without receipt schema, policy, or command behavior changes
