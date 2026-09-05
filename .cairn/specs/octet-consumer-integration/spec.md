# Octet Consumer Integration Specification

## Purpose

Defines the `octet-consumer-integration` capability.

## Requirements

### Requirement: Canonical pinned consumer integration
r[cairn.octet_integration.canonical] Cairn MUST expose the Octet Nix input and check under `octet`, select `cargo-octet`, and use the canonical Octet configuration tables.
Cairn MUST use an immutable `OnixResearch/octet` source revision and preserve the existing consumer check scope during this rename.

#### Scenario: Canonical integration resolves
r[cairn.octet_integration.canonical.valid]
- GIVEN the reviewed Octet source and current Cairn config
- WHEN Nix evaluates the integration
- THEN the canonical input, check, executable, and config tables resolve
- AND the workspace and all-target check scope remains intact

#### Scenario: Legacy integration cannot become canonical
r[cairn.octet_integration.canonical.legacy]
- GIVEN a mutation that restores a legacy input, check, executable, or config table
- WHEN the integration regression controls run
- THEN the controls reject that mutation

### Requirement: Canonical operator guidance
r[cairn.octet_integration.guidance] Cairn MUST name Octet in maintained development guidance and emit the canonical Octet check command in release-readiness reports.
The command list MUST retain the other required repository checks and MUST NOT claim that listing a command executes it.

#### Scenario: Operator receives the current command
r[cairn.octet_integration.guidance.valid]
- GIVEN a valid release-readiness request
- WHEN Cairn emits its repository gate summary
- THEN the required command list includes the Octet check
- AND the other required repository checks remain present

#### Scenario: Legacy command does not appear in new guidance
r[cairn.octet_integration.guidance.legacy]
- GIVEN a newly generated release-readiness report
- WHEN the regression controls inspect its required command list
- THEN the list contains no legacy Slotcar or Tiger Style command

### Requirement: Preserve producer identity and historical evidence
r[cairn.octet_integration.evidence] This migration MUST preserve the selected producer revision, source NAR identity, and archived or recorded receipt bytes.
Only Nix MUST generate changes to `flake.lock`.
Cairn MUST distinguish integration checks from successful execution of the Octet consumer check.

#### Scenario: Source identity remains unchanged
r[cairn.octet_integration.evidence.valid]
- GIVEN the recorded producer revision and source NAR identity before the rename
- WHEN Nix generates the canonical input lock
- THEN the selected producer revision and source NAR identity remain equal
- AND historical receipt bytes remain unchanged

#### Scenario: Toolchain failure remains a blocker
r[cairn.octet_integration.evidence.blocked]
- GIVEN an Octet build that fails before consumer analysis
- WHEN maintainers record verification results
- THEN they record the build as blocked
- AND they do not claim a passing Octet consumer check
