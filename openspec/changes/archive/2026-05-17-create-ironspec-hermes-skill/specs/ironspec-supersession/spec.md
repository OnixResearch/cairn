## ADDED Requirements

### Requirement: Hermes skill artifact

`cairn` MUST provide a repository-maintained Hermes skill source that captures the current development workflow, FCIS boundary, OpenSpec lifecycle, and verification rails.

#### Scenario: Agent discovers the skill source

- GIVEN a future Hermes session needs to work on `cairn`
- WHEN the agent inspects repository documentation for Hermes workflow guidance
- THEN a copyable Hermes skill source is available in the repository

#### Scenario: Skill reflects FCIS and verification contracts

- GIVEN the repository contains Slotcar-enforced FCIS boundaries and Nix verification gates
- WHEN the skill source describes how to work on `cairn`
- THEN it includes core purity rules, CLI shell responsibilities, OpenSpec-first behavior guidance, and maintained verification commands

#### Scenario: Skill remains audit-friendly

- GIVEN skill content may be copied into a local Hermes installation
- WHEN the repository stores the canonical skill source
- THEN the artifact excludes secrets, transient task progress, and machine-specific credentials
