## ADDED Requirements

### Requirement: Doctor diagnostics command
r[cairn.supersede.doctor] The system MUST provide a read-only `cairn doctor` command that emits deterministic JSON diagnostics for the selected repository root.

#### Scenario: Healthy repository reports valid
- GIVEN a repository with a selected lifecycle layout, readable policy, accepted specs, and no validation issues
- WHEN the operator runs `cairn doctor --root <path>`
- THEN the CLI emits a JSON receipt with `valid=true`, the selected layout, repository counts, policy status, and diagnostic checks

#### Scenario: Missing policy reports failure without panic
- GIVEN a repository root and a `--policy` path that does not exist
- WHEN the operator runs `cairn doctor --root <path> --policy <missing>`
- THEN the CLI emits a JSON receipt with `valid=false`
- AND the policy diagnostic check reports the missing policy as a failure

### Requirement: Doctor FCIS boundary
r[cairn.supersede.doctor.boundary] The system MUST keep doctor diagnostic classification in the pure core while filesystem and policy-file reads remain in the CLI shell.

#### Scenario: Core classifies caller-provided snapshot
- GIVEN a doctor diagnostic snapshot supplied by the CLI shell
- WHEN `cairn-core` builds the doctor report
- THEN it performs no filesystem, environment, process, time, stdout/stderr, or JSON presentation effects
