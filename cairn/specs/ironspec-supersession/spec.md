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

### Requirement: Nickel policy contract coverage
r[cairn.supersede.nickel_contracts] The system MUST keep lifecycle policy, gate schema, task marker vocabulary, finding severity, exemption shape, and sync/archive policy in Nickel-authored contracts that export deterministic runtime policy data.

#### Scenario: Export typed policy data
r[cairn.supersede.nickel_contracts.export]
- GIVEN Nickel policy contracts for lifecycle stages, gates, task markers, findings, exemptions, and sync/archive policy
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

### Requirement: BLAKE3 evidence identifiers
r[cairn.supersede.blake3_receipts] The system MUST use BLAKE3 hashes to identify canonical policy, input snapshots, plans, and receipts for Cairn decisions that claim validation, gate, sync, archive, migration, or traceability evidence.

#### Scenario: Gate receipt includes stable hashes
r[cairn.supersede.blake3_receipts.gate]
- GIVEN an Cairn gate evaluation with canonical policy data and a canonical change snapshot
- WHEN the operator runs `cairn gate <stage> <change> --root <path>`
- THEN the JSON receipt MUST include BLAKE3 hashes for the policy, gate input snapshot, and emitted receipt content
- AND repeated evaluation of unchanged inputs MUST produce the same hashes

#### Scenario: Sync and archive plans include stable hashes
r[cairn.supersede.blake3_receipts.lifecycle_plans]
- GIVEN a sync or archive dry-run plan
- WHEN the operator runs `cairn sync <change>` or `cairn archive <change>`
- THEN the JSON receipt MUST include BLAKE3 hashes for the canonical plan input and plan output
- AND the plan hash MUST change when planned lifecycle actions change

#### Scenario: Migration manifest includes source and destination hashes
r[cairn.supersede.blake3_receipts.migration]
- GIVEN an explicit `migrate openspec` plan or execution receipt
- WHEN legacy OpenSpec files are mapped into native Cairn paths
- THEN the migration receipt MUST include BLAKE3 hashes for canonical source manifests, destination manifests, and emitted receipt content

#### Scenario: Hashing remains canonical and bounded
r[cairn.supersede.blake3_receipts.canonical]
- GIVEN Cairn computes evidence hashes
- WHEN hashes are generated
- THEN the hashed material MUST exclude nondeterministic process noise, ambient absolute paths unless explicitly part of the contract, and wall-clock timestamps unless supplied as explicit inputs

### Requirement: Deterministic mandatory gates
r[cairn.supersede.deterministic_gates] The system MUST provide deterministic mandatory gates for proposal, design, tasks, pre-sync, pre-archive, migration, policy freshness, and accepted-spec traceability decisions.

#### Scenario: Stage gates are reproducible
r[cairn.supersede.deterministic_gates.stage]
- GIVEN an active change and unchanged policy data
- WHEN the operator runs proposal, design, or tasks gates repeatedly
- THEN each gate MUST return the same verdict, issue list, policy hash, input hash, and receipt hash

#### Scenario: Pre-sync gate blocks unsafe mutation
r[cairn.supersede.deterministic_gates.pre_sync]
- GIVEN a change whose delta specs fail validation or whose task gate fails
- WHEN the operator requests sync execution
- THEN Cairn MUST block filesystem mutation
- AND the receipt MUST identify the deterministic gate failure that prevented sync

#### Scenario: Pre-archive gate blocks incomplete changes
r[cairn.supersede.deterministic_gates.pre_archive]
- GIVEN a change with incomplete tasks, unsynced delta specs, or failing traceability requirements
- WHEN the operator requests archive execution
- THEN Cairn MUST block filesystem mutation
- AND the receipt MUST identify the deterministic gate failure that prevented archive

#### Scenario: Policy freshness gate rejects stale generated policy
r[cairn.supersede.deterministic_gates.policy_freshness]
- GIVEN Nickel policy sources and generated runtime policy JSON
- WHEN the maintained verification gate runs
- THEN it MUST fail if the generated policy JSON is stale relative to Nickel sources

#### Scenario: Traceability gate rejects uncovered accepted requirements
r[cairn.supersede.deterministic_gates.traceability]
- GIVEN accepted specs containing `r[...]` requirements
- WHEN the traceability gate runs
- THEN it MUST fail if an accepted requirement lacks at least one source-controlled implementation or verification reference

### Requirement: Receipt JSON schema stability
r[cairn.supersede.receipt_schema] The system MUST treat gate, sync, archive, and migration JSON receipt top-level fields as a stable public contract for operator automation and evidence tooling.

#### Scenario: Gate receipt fields remain stable
r[cairn.supersede.receipt_schema.gate]
- GIVEN an Cairn gate command emits a JSON receipt
- WHEN the maintained CLI regression suite runs
- THEN the suite MUST fail if the gate receipt omits or renames required top-level fields for the change, stage, verdict, issues, policy, layout, policy hash, input hash, receipt hash, or validity result

#### Scenario: Sync and archive receipt fields remain stable
r[cairn.supersede.receipt_schema.lifecycle]
- GIVEN Cairn sync or archive commands emit JSON plan receipts
- WHEN the maintained CLI regression suite runs
- THEN the suite MUST fail if the lifecycle receipt omits or renames required top-level fields for the change, actions, blockers, dry-run and mutation state, delta specs where applicable, policy, layout, policy hash, input hash, plan hash, or receipt hash
- AND the suite MUST fail if lifecycle action entries omit or rename their kind, path, or description fields

#### Scenario: Migration receipt fields remain stable
r[cairn.supersede.receipt_schema.migration]
- GIVEN `cairn migrate openspec` emits a JSON migration receipt
- WHEN the maintained CLI regression suite runs
- THEN the suite MUST fail if the migration receipt omits or renames required top-level fields for command, root, source layout, destination layout, actions, conflicts, dry-run and mutation state, policy, policy hash, source manifest hash, destination manifest hash, manifest hash, or receipt hash
- AND the suite MUST fail if migration action entries omit or rename source, destination, source hash, or destination hash fields

#### Scenario: Receipt schema is documented
r[cairn.supersede.receipt_schema.docs]
- GIVEN operators need to consume Cairn receipts outside the CLI process
- WHEN they inspect repository documentation
- THEN the documentation MUST list the stable public fields for gate, sync, archive, and migration receipts

### Requirement: Cairn native lifecycle layout
r[cairn.supersede.native_layout] The system MUST use the canonical `cairn/` lifecycle layout for normal runtime commands after the rename and MUST NOT silently fall back to legacy `ironspec/` or `openspec/` lifecycle content.

#### Scenario: Initialize canonical Cairn layout
r[cairn.supersede.native_layout.init_cairn]
- GIVEN a writable project root without a `cairn/` directory
- WHEN the operator runs `cairn init --root <path>`
- THEN the system creates `cairn/specs`, `cairn/changes`, and `cairn/archive`
- AND the JSON receipt identifies the `cairn` layout

#### Scenario: Prefer canonical layout over compatibility layout
r[cairn.supersede.native_layout.prefer_cairn]
- GIVEN a repository containing both canonical `cairn/` lifecycle content and legacy `ironspec/` lifecycle content
- WHEN the operator runs read, status, doctor, validate, gate, sync, or archive commands
- THEN the system uses the canonical `cairn/` layout
- AND the JSON receipt identifies the selected layout

#### Scenario: Reject implicit legacy fallback
r[cairn.supersede.native_layout.no_implicit_legacy_after_rename]
- GIVEN a repository containing only legacy `ironspec/` lifecycle content
- WHEN the operator runs `cairn` read, status, doctor, validate, gate, sync, or archive commands without an explicit compatibility flag or migration command
- THEN the command evaluates the canonical `cairn/` layout
- AND it does not silently read legacy lifecycle content

### Requirement: OpenSpec migration bridge
r[cairn.supersede.migrate_openspec] The system MUST provide deterministic migration bridges from legacy `openspec/` and `ironspec/` layouts to canonical `cairn/` layout without using legacy content as an implicit runtime fallback.

#### Scenario: Dry-run legacy Ironspec migration plan
r[cairn.supersede.migrate_openspec.ironspec_dry_run]
- GIVEN a repository with legacy `ironspec/` lifecycle content
- WHEN the operator runs `cairn migrate ironspec --root <path>`
- THEN the CLI emits a deterministic JSON migration plan from legacy `ironspec/` paths into canonical `cairn/` paths
- AND no filesystem content is mutated

#### Scenario: Execute legacy Ironspec migration copy
r[cairn.supersede.migrate_openspec.ironspec_execute]
- GIVEN a repository with legacy `ironspec/` lifecycle content and no conflicting canonical destination files
- WHEN the operator runs `cairn migrate ironspec --root <path> --execute`
- THEN the system copies planned files into matching canonical `cairn/` paths
- AND the JSON receipt reports `mutated=true`

### Requirement: Rename identity and compatibility
r[cairn.supersede.rename_cairn] The system MUST expose `Cairn` / `cairn` as the canonical project, CLI, package, docs, and runtime lifecycle identity while preserving legacy `ironspec` only as explicit compatibility where needed.

#### Scenario: Canonical CLI identity
r[cairn.supersede.rename_cairn.cli]
- GIVEN a built release after the rename
- WHEN an operator invokes the canonical CLI help, version, status, gate, validate, sync, archive, or migrate commands
- THEN the executable, help text, examples, and deterministic receipts identify the command as `cairn`
- AND they do not present `ironspec` as the canonical command

#### Scenario: Legacy command compatibility is visible
r[cairn.supersede.rename_cairn.legacy_alias]
- GIVEN the legacy `ironspec` command alias is installed during the migration window
- WHEN an operator invokes the alias
- THEN the command executes the matching `cairn` behavior
- AND the JSON receipt or human-facing warning identifies `ironspec` as a compatibility alias for `cairn`

#### Scenario: Canonical packaging metadata
r[cairn.supersede.rename_cairn.packaging]
- GIVEN the Rust workspace and Nix flake after the rename
- WHEN package metadata, crate names, binary names, Nix packages, Nix apps, and flake checks are inspected
- THEN new canonical surfaces use `cairn`
- AND any remaining `ironspec` surfaces are documented compatibility aliases or historical references

#### Scenario: Deterministic rename classification evidence
r[cairn.supersede.rename_cairn.classification]
- GIVEN the rename implementation is ready for review
- WHEN the deterministic rename evidence checker scans repository text surfaces
- THEN every remaining `ironspec` occurrence is classified as `needs-fix`, `compatibility-alias`, `historical-archive`, `requirement-id-history`, or `intentional-reference`
- AND the verification fails when any occurrence remains in `needs-fix`
