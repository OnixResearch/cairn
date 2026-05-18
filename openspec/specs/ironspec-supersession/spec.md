# Cairn Supersession Specification

## Purpose

Defines the OpenSpec-compatible authoring and read-side capabilities that let `cairn` supersede the upstream OpenSpec CLI while keeping stronger deterministic receipts and FCIS enforcement.
## Requirements
### Requirement: Cairn validation parity
r[cairn.supersede.validate_ears] The system MUST validate OpenSpec-compatible requirement markdown for EARS-style testability without requiring the upstream OpenSpec CLI.

#### Scenario: Accept structured requirements
r[cairn.supersede.validate_ears.accept]
- GIVEN an OpenSpec-compatible repository whose accepted specs and active delta specs contain requirements with normative strength words
- AND each scenario contains GIVEN, WHEN, and THEN clauses
- WHEN the operator runs `cairn validate --root <path>`
- THEN the CLI emits a deterministic JSON receipt with `valid=true`
- AND the spec issue collection is empty

#### Scenario: Reject weak requirement prose
r[cairn.supersede.validate_ears.weak_requirement]
- GIVEN an OpenSpec-compatible spec requirement without a MUST, SHALL, SHOULD, or MAY strength word
- WHEN the operator runs `cairn validate --root <path>`
- THEN the CLI emits a deterministic JSON receipt with `valid=false`
- AND the issue collection identifies the weak requirement location

#### Scenario: Reject incomplete EARS scenario
r[cairn.supersede.validate_ears.incomplete_scenario]
- GIVEN an OpenSpec-compatible spec scenario missing a GIVEN, WHEN, or THEN clause
- WHEN the operator runs `cairn validate --root <path>`
- THEN the CLI emits a deterministic JSON receipt with `valid=false`
- AND the issue collection identifies the incomplete scenario location

#### Scenario: Preserve FCIS validation boundary
r[cairn.supersede.validate_ears.boundary]
- GIVEN EARS-style validation support
- WHEN `cairn-core` evaluates requirement and scenario quality
- THEN it consumes only caller-provided markdown strings and performs no filesystem, environment, process, time, stdout/stderr, or JSON presentation effects

### Requirement: Cairn repository initialization
r[cairn.supersede.init] The system MUST initialize the native `cairn/` repository layout without requiring the upstream OpenSpec CLI.

#### Scenario: Initialize empty repository
r[cairn.supersede.init.empty]
- GIVEN a writable project root without an `cairn/` directory
- WHEN the operator runs `cairn init --root <path>`
- THEN the system creates `cairn/specs`, `cairn/changes`, and `cairn/archive`
- AND the JSON receipt identifies the `cairn` layout

#### Scenario: Initialize existing repository idempotently
r[cairn.supersede.init.idempotent]
- GIVEN a project root with the compatible native directories already present
- WHEN the operator runs `cairn init --root <path>`
- THEN no existing artifacts are overwritten
- AND the JSON receipt reports existing paths, the selected `cairn` layout, and `mutated=false`

### Requirement: Cairn change creation
r[cairn.supersede.change_create] The system MUST create a named active change skeleton in the selected native `cairn/` layout by default.

#### Scenario: Create change skeleton
r[cairn.supersede.change_create.skeleton]
- GIVEN an initialized native repository
- WHEN the operator runs `cairn change create <name> --root <path>`
- THEN the system creates `cairn/changes/<name>/proposal.md`, `design.md`, `tasks.md`, and `specs/`
- AND the JSON receipt reports `mutated=true`, the selected `cairn` layout, and the created artifacts

#### Scenario: Reject invalid change names
r[cairn.supersede.change_create.invalid_name]
- GIVEN a change name containing path separators, traversal, or unsupported characters
- WHEN the operator runs `cairn change create <name>`
- THEN the system refuses to create files
- AND reports the validation error

### Requirement: Hermes skill source
r[cairn.supersede.hermes_skill] The system MUST maintain a copyable Hermes skill source for future `cairn` development workflow.

#### Scenario: Discover repo-maintained skill
r[cairn.supersede.hermes_skill.discover]
- GIVEN a future Hermes session needs to work on `cairn`
- WHEN the agent inspects repository documentation for workflow guidance
- THEN the repository exposes a copyable Hermes skill source

#### Scenario: Skill captures development contracts
r[cairn.supersede.hermes_skill.contracts]
- GIVEN `cairn` uses FCIS boundaries, Slotcar, Nix checks, and OpenSpec-first changes
- WHEN the skill source describes the workflow
- THEN it includes core purity rules, CLI shell responsibilities, command contracts, and maintained verification gates

#### Scenario: Skill stays audit-friendly
r[cairn.supersede.hermes_skill.audit]
- GIVEN the skill source may be copied into local Hermes installations
- WHEN the artifact is committed to the repository
- THEN it excludes secrets, transient task progress, and machine-specific credentials

### Requirement: Supersession boundary
r[cairn.supersede.boundary] The system MUST keep lifecycle templates, read-model decisions, and validation decisions in the pure core while filesystem access and JSON presentation remain in the CLI shell.

#### Scenario: Pure lifecycle core
r[cairn.supersede.boundary.pure_core]
- GIVEN initialization and change creation support
- WHEN Slotcar verifies the pure core boundary
- THEN the core remains free of filesystem, environment, process, and presentation effects

#### Scenario: Pure read models
r[cairn.supersede.boundary.read_models]
- GIVEN change/spec read commands
- WHEN `cairn-core` builds the change/spec read models
- THEN it consumes only caller-provided strings/status values and performs no filesystem, environment, process, time, or stdout/stderr effects

### Requirement: Cairn read-side parity
r[cairn.supersede.read_parity] The system MUST provide deterministic read commands for active changes and accepted specs without requiring the upstream OpenSpec CLI.

#### Scenario: List active changes
r[cairn.supersede.read_parity.change_list]
- GIVEN an OpenSpec-compatible repository with active changes
- WHEN the operator runs `cairn change list --root <path>`
- THEN the CLI emits a deterministic JSON receipt containing sorted active change summaries and task counts

#### Scenario: Show active change
r[cairn.supersede.read_parity.change_show]
- GIVEN an OpenSpec-compatible repository with an active change
- WHEN the operator runs `cairn change show <name> --root <path>`
- THEN the CLI emits a deterministic JSON receipt for that change including artifact presence, delta specs, and task counts

#### Scenario: List accepted specs
r[cairn.supersede.read_parity.spec_list]
- GIVEN an OpenSpec-compatible repository with accepted specs
- WHEN the operator runs `cairn spec list --root <path>`
- THEN the CLI emits a deterministic JSON receipt containing sorted accepted spec summaries

#### Scenario: Show accepted spec
r[cairn.supersede.read_parity.spec_show]
- GIVEN an OpenSpec-compatible repository with an accepted spec
- WHEN the operator runs `cairn spec show <id> --root <path>`
- THEN the CLI emits a deterministic JSON receipt containing the requested spec id, path, and markdown content

### Requirement: Cairn native lifecycle layout
r[cairn.supersede.native_layout] The system MUST use the native `cairn/` lifecycle layout for normal runtime commands and MUST NOT silently fall back to legacy `openspec/` lifecycle content.

#### Scenario: Initialize native layout
r[cairn.supersede.native_layout.init]
- GIVEN a writable project root without an `cairn/` directory
- WHEN the operator runs `cairn init --root <path>`
- THEN the system creates `cairn/specs`, `cairn/changes`, and `cairn/archive`
- AND the JSON receipt identifies the `cairn` layout

#### Scenario: Use native layout only
r[cairn.supersede.native_layout.native_only]
- GIVEN a repository containing only legacy `openspec/` lifecycle content
- WHEN the operator runs read, status, doctor, validate, gate, sync, or archive commands
- THEN the command evaluates the native `cairn/` layout and does not read legacy lifecycle content

#### Scenario: Native layout is selected when both roots exist
r[cairn.supersede.native_layout.prefer]
- GIVEN a repository containing both `cairn/` and legacy `openspec/` lifecycle content
- WHEN the operator runs read, status, doctor, validate, sync, or archive commands
- THEN the system uses the native `cairn/` layout
- AND the JSON receipt identifies the selected layout

### Requirement: OpenSpec migration bridge
r[cairn.supersede.migrate_openspec] The system MUST provide a deterministic migration bridge from legacy `openspec/` layout to native `cairn/` layout without using legacy content as an implicit runtime fallback.

#### Scenario: Dry-run migration plan
r[cairn.supersede.migrate_openspec.dry_run]
- GIVEN a repository with legacy `openspec/` lifecycle content
- WHEN the operator runs `cairn migrate openspec --root <path>`
- THEN the CLI emits a deterministic JSON migration plan from legacy `openspec/` paths into native `cairn/` paths
- AND no filesystem content is mutated

#### Scenario: Execute migration copy
r[cairn.supersede.migrate_openspec.execute]
- GIVEN a repository with legacy `openspec/` lifecycle content and no conflicting native destination files
- WHEN the operator runs `cairn migrate openspec --root <path> --execute`
- THEN the system copies planned files into matching native `cairn/` paths
- AND the JSON receipt reports `mutated=true`

#### Scenario: Block conflicting migration
r[cairn.supersede.migrate_openspec.conflict]
- GIVEN a repository with legacy content and an existing native destination file
- WHEN the operator runs `cairn migrate openspec --root <path> --execute`
- THEN the system reports the conflict and does not overwrite the existing native file

#### Scenario: Migration remains explicit
r[cairn.supersede.migrate_openspec.boundary]
- GIVEN a repository containing only legacy `openspec/` lifecycle content
- WHEN the operator runs normal read, status, doctor, validate, gate, sync, or archive commands
- THEN those commands do not read legacy lifecycle content before explicit migration executes

### Requirement: Native Gate Policy Commands

r[cairn.supersede.gate_policy] The system MUST provide native `cairn gate proposal <change>`, `cairn gate design <change>`, and `cairn gate tasks <change>` commands that evaluate AgentKit-compatible stage policy from Cairn-owned Nickel configuration without depending on AgentKit at runtime.

#### Scenario: Gate receipt for a ready change

- GIVEN a repository with an active change whose required artifacts satisfy the requested gate stage
- WHEN an operator runs `cairn gate tasks <change>`
- THEN the command MUST emit a deterministic JSON receipt with the change name, stage, verdict, issues, policy, and layout.

#### Scenario: Gate failure for a missing artifact

- GIVEN a repository with an active change that is missing an artifact required by a requested gate stage
- WHEN an operator runs `cairn gate design <change>`
- THEN the command MUST emit a deterministic JSON receipt whose verdict is `FAIL` and whose issues identify the missing artifact.

#### Scenario: AgentKit policy alignment without vendoring

- GIVEN AgentKit gate policy defines stage vocabulary, marker policy, finding defaults, archive-ignore behavior, and bounded exemptions
- WHEN Cairn exports its default policy
- THEN the exported policy MUST include equivalent native fields owned under `cairn-policy/` rather than importing AgentKit policy at runtime.

### Requirement: Tracey Coverage Rail

r[cairn.supersede.tracey_coverage] The system MUST expose a first-class Tracey coverage rail that links accepted Cairn requirements to implementation or verification annotations without making Tracey a runtime dependency.

#### Scenario: Tracey configuration is discoverable

r[cairn.supersede.tracey_coverage.config]
- GIVEN a repository checkout with accepted Cairn specs
- WHEN an operator runs Tracey from the repository root
- THEN Tracey MUST discover `.config/tracey/config.styx` and scan accepted specs plus Rust source and test files

#### Scenario: Coverage check is deterministic

r[cairn.supersede.tracey_coverage.check]
- GIVEN accepted specs with `r[...]` requirement IDs and Rust comments with `r[impl ...]` or `r[verify ...]` references
- WHEN the maintained Nix verification runs
- THEN the repository MUST fail the traceability check for missing references without invoking runtime Cairn code

#### Scenario: Runtime boundary is preserved

r[cairn.supersede.tracey_coverage.boundary]
- GIVEN Tracey coverage support
- WHEN `cairn-core` is compiled and checked
- THEN the core MUST remain independent of Tracey configuration, Tracey processes, and filesystem coverage scanning

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
