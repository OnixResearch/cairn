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

### Requirement: Agent-run receipt schema
r[cairn.supersede.agent_run_receipts.schema] The system MUST define a canonical JSON agent-run receipt envelope for deterministic evidence about an agent task.

#### Scenario: Canonical receipt includes required evidence fields
r[cairn.supersede.agent_run_receipts.schema.required_fields]
- GIVEN an agent wrapper records a completed task attempt
- WHEN it emits a Cairn agent-run receipt
- THEN the canonical receipt includes `schema_version`, `task_id`, `agent_identity`, `input_hash`, `plan_hash`, `tool_manifest`, `mutation_manifest`, `verification_manifest`, `nondeterminism_manifest`, `verdict`, and `receipt_hash`
- AND `receipt_hash` is derived from canonical receipt content with the hash field excluded

#### Scenario: Receipt avoids raw transcript dependence
r[cairn.supersede.agent_run_receipts.schema.bounded_content]
- GIVEN an agent run includes prompts, tool outputs, logs, or generated artifacts
- WHEN the receipt records that evidence
- THEN the receipt stores bounded summaries, manifests, and content hashes instead of requiring an unbounded raw transcript
- AND secret-bearing values are represented by redacted metadata or hashes rather than plaintext

### Requirement: Agent-run plan linkage
r[cairn.supersede.agent_run_receipts.plan_link] The system MUST link side-effecting agent receipts to a canonical pre-mutation plan hash.

#### Scenario: Side-effecting run cites plan hash
r[cairn.supersede.agent_run_receipts.plan_link.side_effects]
- GIVEN an agent run performs filesystem mutation, lifecycle mutation, network write, secret access, deploy, or other side-effecting action
- WHEN the agent-run receipt is validated
- THEN the receipt includes a non-empty `plan_hash` derived from the canonical pre-mutation plan
- AND the mutation manifest is checked against the approved plan scope

#### Scenario: Read-only run may use explicit no-mutation plan
r[cairn.supersede.agent_run_receipts.plan_link.read_only]
- GIVEN an agent run performs only read-only inspection and produces no external side effects
- WHEN the agent-run receipt is validated
- THEN the receipt may cite a canonical no-mutation plan
- AND the mutation manifest reports no mutated files, lifecycle state, network writes, deploys, or secret-value exposures

### Requirement: Agent-run completion gate
r[cairn.supersede.agent_run_receipts.completion_gate] The system MUST provide a completion gate that validates agent-run receipts before a task is claimed complete.

#### Scenario: Verified completion passes gate
r[cairn.supersede.agent_run_receipts.completion_gate.verified]
- GIVEN an agent-run receipt reports a successful verdict
- WHEN the completion gate evaluates the receipt
- THEN the gate verifies required checks, artifact hashes, dirty-state disclosure, plan linkage, mutation scope, and receipt hash consistency
- AND it returns a deterministic pass receipt that can be cited by the agent final response

#### Scenario: Missing verification fails gate
r[cairn.supersede.agent_run_receipts.completion_gate.missing_verification]
- GIVEN an agent-run receipt reports a successful verdict without required verification evidence
- WHEN the completion gate evaluates the receipt
- THEN the gate fails with a deterministic issue for missing verification evidence
- AND the agent wrapper MUST NOT claim the task is complete based on that receipt

### Requirement: Agent-run replay fixtures
r[cairn.supersede.agent_run_receipts.replay] The system MUST maintain replay fixtures for canonical receipt stability and failure behavior.

#### Scenario: Positive replay fixture is byte-stable
r[cairn.supersede.agent_run_receipts.replay.positive]
- GIVEN a maintained replay fixture with captured inputs, tool manifest, plan, mutations, verification evidence, and nondeterminism manifest
- WHEN the canonical agent-run receipt is generated twice from the same fixture
- THEN both generated receipts are byte-identical
- AND all BLAKE3 hash fields are stable 64-character lowercase hexadecimal strings

#### Scenario: Negative replay fixtures fail deterministically
r[cairn.supersede.agent_run_receipts.replay.negative]
- GIVEN maintained replay fixtures with missing plan linkage, missing verification evidence, uncaptured nondeterminism, or mutation outside approved scope
- WHEN each fixture is gated
- THEN each failure returns a deterministic issue code and message
- AND no fixture is accepted as successful completion evidence

# Cairn Supersession MCP Delta

## Purpose

Specify a native MCP surface for Cairn lifecycle inspection, dry-run planning, and explicitly guarded lifecycle mutation while preserving deterministic receipts and FCIS boundaries.

## Requirements

### Requirement: Cairn MCP read-only tools
r[cairn.supersede.mcp_server.read_tools] The system MUST expose bounded read-only MCP tools for Cairn lifecycle inspection without enabling filesystem mutation.

#### Scenario: List lifecycle state through MCP
r[cairn.supersede.mcp_server.read_tools.status]
- GIVEN a repository with native `cairn/` lifecycle content
- WHEN an MCP client calls the Cairn status or content-listing tool
- THEN the tool returns structured content with accepted specs, active changes, archived changes, selected layout, and validation summary fields
- AND the tool is annotated as read-only and non-destructive

#### Scenario: Read selected lifecycle artifacts through MCP
r[cairn.supersede.mcp_server.read_tools.artifacts]
- GIVEN a repository with an accepted spec or active change artifact
- WHEN an MCP client requests that spec or change by lifecycle identifier
- THEN the tool returns bounded structured content for the selected lifecycle artifact
- AND it does not expose arbitrary repository paths outside the lifecycle contract

#### Scenario: Run diagnostics and gates through MCP
r[cairn.supersede.mcp_server.read_tools.gates]
- GIVEN a repository with a selected Cairn lifecycle root
- WHEN an MCP client calls doctor, validate, gate, or gate-context tools
- THEN each tool returns the corresponding deterministic Cairn receipt fields as structured content
- AND no repository files are mutated

### Requirement: Cairn MCP dry-run planning tools
r[cairn.supersede.mcp_server.plan_tools] The system MUST expose MCP tools for sync, archive, and migration planning that return dry-run receipts by default.

#### Scenario: Sync and archive plans remain non-mutating
r[cairn.supersede.mcp_server.plan_tools.lifecycle]
- GIVEN an active Cairn change
- WHEN an MCP client calls sync-plan or archive-plan
- THEN the tool returns a deterministic dry-run lifecycle receipt with planned actions, blockers, mutation state, and BLAKE3 evidence hashes
- AND no filesystem content is modified

#### Scenario: Migration plans remain non-mutating
r[cairn.supersede.mcp_server.plan_tools.migration]
- GIVEN a repository with legacy `openspec/` or `ironspec/` lifecycle content
- WHEN an MCP client calls the migration planning tool
- THEN the tool returns a deterministic dry-run migration receipt with source and destination manifest hashes
- AND no native lifecycle destination file is created or overwritten

### Requirement: Cairn MCP mutation guard
r[cairn.supersede.mcp_server.mutation_guard] The system MUST disable mutating MCP tools by default and require an explicit server capability before lifecycle mutation can be requested.

#### Scenario: Default server denies mutation tools
r[cairn.supersede.mcp_server.mutation_guard.default_deny]
- GIVEN the Cairn MCP server is started without the mutation capability flag
- WHEN an MCP client lists or calls init, change-create, sync-execute, archive-execute, or migrate-execute tools
- THEN the server either omits those tools or returns a structured capability error
- AND no lifecycle files are mutated

#### Scenario: Explicit capability allows guarded mutation
r[cairn.supersede.mcp_server.mutation_guard.enabled]
- GIVEN the Cairn MCP server is started with an explicit mutation capability flag
- WHEN an MCP client calls a mutating lifecycle tool with valid inputs
- THEN the tool applies the same validation, gate, blocker, and receipt semantics as the equivalent Cairn CLI command
- AND the response reports whether mutation occurred

### Requirement: Cairn MCP FCIS boundary
r[cairn.supersede.mcp_server.boundary] The system MUST keep MCP transport, filesystem access, path bounding, and JSON-RPC effects in the imperative shell while preserving pure deterministic logic in `cairn-core`.

#### Scenario: Core remains transport independent
r[cairn.supersede.mcp_server.boundary.core]
- GIVEN MCP server support is implemented
- WHEN `cairn-core` builds lifecycle models, diagnostics, gates, or plans
- THEN it consumes caller-supplied snapshots and policy data only
- AND it performs no MCP transport, filesystem, environment, process, time, stdout/stderr, or JSON-RPC effects

#### Scenario: Shell bounds lifecycle access
r[cairn.supersede.mcp_server.boundary.shell]
- GIVEN an MCP client supplies a root, lifecycle identifier, or relative artifact selector
- WHEN the server resolves the request
- THEN the shell rejects traversal, absolute artifact escapes, dotfile reads, build output reads, and unrelated repository file reads
- AND accepted reads are size-bounded before content is returned

### Requirement: Cairn MCP verification and documentation
r[cairn.supersede.mcp_server.verification] The system MUST verify and document the MCP tool contract, including schemas, annotations, safety flags, and structured receipt behavior.

#### Scenario: Tool registration is tested
r[cairn.supersede.mcp_server.verification.registration]
- GIVEN the maintained MCP test suite starts the Cairn MCP server
- WHEN it initializes the MCP session and lists tools
- THEN the expected read-only and planning tools are registered with stable names, input schemas, and read/destructive annotations

#### Scenario: Safety failures are tested
r[cairn.supersede.mcp_server.verification.negative]
- GIVEN the maintained MCP test suite exercises unsafe inputs
- WHEN it sends traversal selectors, oversized read requests, or mutation calls without the mutation capability
- THEN each call returns a structured error
- AND the fixture repository remains unchanged

#### Scenario: Documentation covers agent workflow
r[cairn.supersede.mcp_server.verification.docs]
- GIVEN an operator wants to configure Cairn for MCP-capable agents
- WHEN they inspect repository documentation
- THEN the documentation lists available tools, mutation gating, response receipt fields, and example read-only and mutation-enabled workflows

### Requirement: Agent receipt validation surface
r[cairn.supersede.agent_receipt_validation_surface] The system MUST expose a deterministic public validation surface for agent-run receipts so wrappers can gate completion claims without linking against Cairn internals.

#### Scenario: CLI validates receipt file
r[cairn.supersede.agent_receipt_validation_surface.cli]
- GIVEN an operator has a JSON agent-run receipt file under an allowed path
- WHEN they run the Cairn agent receipt validation command for that file
- THEN the system MUST evaluate the receipt with the pure agent-run completion gate
- AND it MUST return structured JSON rather than prose-only output
- AND it MUST NOT mutate lifecycle content.

#### Scenario: CLI validation receipt has stable schema
r[cairn.supersede.agent_receipt_validation_surface.cli.schema]
- GIVEN the CLI validation command emits a validation receipt
- WHEN the same receipt input is validated repeatedly in the same repository state
- THEN the output MUST expose stable fields for command, path or input identity, validity, verdict, issue list, agent receipt hash, input hash, and top-level receipt hash
- AND the repeated outputs MUST be byte-stable except for explicitly captured nondeterministic inputs, if any.

#### Scenario: MCP validates receipt through read-only tool
r[cairn.supersede.agent_receipt_validation_surface.mcp]
- GIVEN an agent wrapper is connected to the Cairn MCP server
- WHEN it calls the agent receipt validation tool with a bounded receipt path or inline receipt object
- THEN the server MUST evaluate the receipt through the same validation behavior as the CLI surface
- AND it MUST return a structured validation receipt suitable for the wrapper to cite before claiming completion.

#### Scenario: MCP validation remains read-only by default
r[cairn.supersede.agent_receipt_validation_surface.mcp.read_only]
- GIVEN the Cairn MCP server starts without mutation capability
- WHEN the client lists tools or calls the agent receipt validation tool
- THEN the validation tool MUST appear as read-only and non-destructive
- AND path input MUST reject traversal or root escape
- AND inline input MUST be bounded before validation.

#### Scenario: Dogfood fixtures replay pass and fail deterministically
r[cairn.supersede.agent_receipt_validation_surface.fixtures]
- GIVEN source-controlled positive and negative agent-run receipt fixtures
- WHEN validation tests replay those fixtures through the public surface
- THEN positive fixtures MUST produce byte-stable passing validation receipts
- AND negative fixtures MUST fail deterministically with expected issue codes.

#### Scenario: Wrapper flow is documented
r[cairn.supersede.agent_receipt_validation_surface.docs]
- GIVEN an agent wrapper author wants deterministic completion evidence
- WHEN they read the Cairn documentation
- THEN it MUST describe the flow for creating a plan, producing an agent-run receipt, validating it through CLI or MCP, and citing the validation receipt hash.

#### Scenario: Validation surface is verified by repository gates
r[cairn.supersede.agent_receipt_validation_surface.verification]
- GIVEN the validation surface implementation is complete
- WHEN repository verification runs
- THEN focused CLI/MCP tests, formatting, workspace tests, Clippy, Slotcar, Tracey coverage, Cairn validation, and Nix flake checks MUST pass.

### Requirement: Strict MCP tool input schemas
r[cairn.supersede.mcp_server.strict_schemas] The system MUST advertise deterministic per-tool MCP input schemas and validate tool arguments against those schemas before lifecycle reads, plans, or mutations are dispatched.

#### Scenario: Tool catalog advertises bounded schemas
r[cairn.supersede.mcp_server.strict_schemas.catalog]
- GIVEN the Cairn MCP server exposes its tool catalog
- WHEN an MCP client lists tools or an operator runs `cairn mcp --describe`
- THEN each tool MUST advertise an object input schema with explicit properties for its accepted arguments
- AND the schema MUST reject unknown properties unless the tool intentionally accepts an inline receipt object
- AND mutation tool schemas MUST remain unavailable unless the server starts with the explicit mutation capability.

#### Scenario: Argument validation precedes lifecycle effects
r[cairn.supersede.mcp_server.strict_schemas.validation]
- GIVEN an MCP client calls a Cairn tool with missing required arguments, unknown arguments, invalid enum values, or non-string lifecycle selectors
- WHEN the server receives the call
- THEN it MUST reject the request before reading lifecycle artifacts, planning lifecycle actions, or attempting filesystem mutation.

#### Scenario: Invalid arguments produce deterministic errors
r[cairn.supersede.mcp_server.strict_schemas.errors]
- GIVEN an MCP client calls a tool with malformed arguments
- WHEN the call is evaluated through direct `--call` or JSON-RPC `tools/call`
- THEN the server MUST return a deterministic invalid-argument error identifying the tool and argument class
- AND mutation-denied responses MUST continue to report `mutated=false` without depending on schema validation success.

#### Scenario: Schema contract is documented and verified
r[cairn.supersede.mcp_server.strict_schemas.verification]
- GIVEN the strict schema behavior is implemented
- WHEN repository verification runs
- THEN focused MCP tests MUST assert catalog schemas for representative read, planning, validation, and mutation tools
- AND negative tests MUST cover missing required keys, unknown keys, invalid stage/source enum values, traversal selectors, and mutation calls without the capability flag.

#### Scenario: Operators can use schema guidance
r[cairn.supersede.mcp_server.strict_schemas.docs]
- GIVEN an operator or agent wrapper author reads the Cairn MCP documentation
- WHEN they inspect tool usage guidance
- THEN the documentation MUST describe each tool's required and optional arguments, selector bounds, invalid-argument behavior, and the continued default-deny mutation policy.

### Requirement: MCP agent smoke receipts
r[cairn.supersede.mcp_agent_smoke_receipts] The system MUST provide maintained smoke evidence that proves the default Cairn MCP surface can describe tools, validate an agent-run receipt, and reject malformed tool arguments without mutation.

#### Scenario: Smoke flow exercises catalog and agent receipt validation
r[cairn.supersede.mcp_agent_smoke_receipts.flow]
- GIVEN a repository with the default Cairn MCP tool surface and a maintained positive agent-run receipt fixture
- WHEN the operator runs the maintained MCP agent smoke flow without mutation capability
- THEN the flow MUST inspect the MCP tool catalog
- AND it MUST invoke `cairn_agent_receipt_validate` through the MCP call path against the fixture
- AND it MUST complete without requiring an external MCP client or network service

#### Scenario: Smoke evidence contains stable fields
r[cairn.supersede.mcp_agent_smoke_receipts.evidence]
- GIVEN the maintained MCP agent smoke flow completes
- WHEN its evidence is reviewed by an operator or release gate
- THEN the evidence MUST assert the presence of the `cairn_agent_receipt_validate` tool schema
- AND it MUST assert a structured MCP response containing `structuredContent`
- AND it MUST assert the agent receipt validation verdict and stable hash fields where the receipt contract provides them
- AND it MUST avoid depending on human prose summaries as the only proof

#### Scenario: Smoke flow remains read-only and rejects malformed calls
r[cairn.supersede.mcp_agent_smoke_receipts.read_only]
- GIVEN the maintained MCP agent smoke flow runs without `--allow-mutations`
- WHEN the flow submits a malformed or schema-invalid MCP tool call
- THEN the response MUST be a deterministic invalid-argument error
- AND no lifecycle read, lifecycle plan, or lifecycle mutation dispatch MUST occur for that invalid call
- AND any default mutation-denial branch observed by the smoke evidence MUST report `mutated=false`

#### Scenario: Smoke workflow is documented for operators
r[cairn.supersede.mcp_agent_smoke_receipts.docs]
- GIVEN an operator wants to verify the Cairn MCP and agent receipt surfaces before release
- WHEN they consult the maintained Cairn MCP or receipt documentation
- THEN the documentation MUST name the local smoke command or fixture
- AND it MUST describe which stable receipt fields are intended as release evidence

#### Scenario: Smoke workflow is verified before archive
r[cairn.supersede.mcp_agent_smoke_receipts.verification]
- GIVEN the MCP agent smoke receipt change is ready to archive
- WHEN maintainers run final verification
- THEN focused smoke tests, Cairn validation, stage gates, Tracey coverage, and maintained repository checks MUST pass before the change is synced and archived

### Requirement: Release-readiness receipt bundle
r[cairn.supersede.release_readiness_bundle] The system MUST provide a local release-readiness receipt bundle that summarizes required Cairn operator evidence for a checkout without mutating lifecycle content.

#### Scenario: Bundle receipt has stable schema
r[cairn.supersede.release_readiness_bundle.schema]
- GIVEN an operator requests release-readiness evidence for a Cairn checkout
- WHEN the release-readiness bundle command emits its JSON receipt
- THEN the receipt MUST include stable top-level fields for command, root, validity, verdict, checks, cited evidence hashes, and receipt hash
- AND each check entry MUST identify its stable check name, pass/fail status, source command or artifact, and any cited receipt hash

#### Scenario: Bundle runs locally without mutation
r[cairn.supersede.release_readiness_bundle.local]
- GIVEN a checkout with native `cairn/` lifecycle content
- WHEN the operator runs the release-readiness bundle command without mutation flags
- THEN the command MUST NOT create, edit, sync, archive, migrate, or delete lifecycle artifacts
- AND it MUST complete using local repository evidence rather than requiring a network service or long-running MCP client

#### Scenario: Bundle cites core readiness evidence
r[cairn.supersede.release_readiness_bundle.evidence]
- GIVEN the checkout has maintained Cairn validation, Tracey coverage, MCP agent smoke, and repository verification evidence
- WHEN the bundle evaluates readiness
- THEN it MUST include or cite Cairn validation evidence
- AND it MUST include or cite Tracey coverage evidence
- AND it MUST include or cite the MCP agent smoke receipt hash
- AND it MUST include or cite repository gate evidence such as formatting, tests, Clippy, Slotcar, or flake checks according to the maintained local policy

#### Scenario: Bundle fails transparently on missing or failing evidence
r[cairn.supersede.release_readiness_bundle.failure]
- GIVEN a required readiness component fails or required evidence is missing
- WHEN the release-readiness bundle command emits its receipt
- THEN the receipt MUST report `valid=false` or a failing verdict
- AND it MUST include a deterministic check or issue naming the failed or missing evidence component
- AND it MUST preserve any available component receipt hashes so operators can diagnose the failure without scraping prose output

#### Scenario: Bundle workflow is documented
r[cairn.supersede.release_readiness_bundle.docs]
- GIVEN an operator wants to prepare a Cairn checkout for release or agent handoff
- WHEN they read the maintained documentation
- THEN it MUST name the release-readiness bundle command
- AND it MUST describe which stable receipt fields and cited hashes are intended as release evidence
- AND it MUST explain that the bundle complements, rather than replaces, the maintained repository gate commands

#### Scenario: Bundle is verified before archive
r[cairn.supersede.release_readiness_bundle.verification]
- GIVEN the release-readiness bundle implementation is complete
- WHEN maintainers run final verification
- THEN focused bundle tests MUST assert stable receipt fields and hash-shaped evidence
- AND negative tests MUST cover missing or failing required evidence where practical
- AND Cairn validation, Tracey coverage, repository formatting, tests, Clippy, Slotcar, and flake checks MUST pass before the change is synced and archived

### Requirement: Canonical Cairn Hermes workflow skill
r[cairn.supersede.hermes_skill.canonical_cairn] The system MUST maintain the repository workflow skill under the canonical Cairn identity rather than an OpenSpec or Ironspec primary name.

#### Scenario: Canonical repo skill path
r[cairn.supersede.hermes_skill.canonical_cairn.path]
- GIVEN the repository carries a copyable Hermes workflow skill source
- WHEN a future agent inspects `docs/hermes/`
- THEN the canonical artifact is named `cairn-workflow-skill.md`

#### Scenario: Canonical live skill
r[cairn.supersede.hermes_skill.canonical_cairn.live]
- GIVEN a Hermes session is working in `/home/brittonr/git/cairn`
- WHEN it loads a project workflow skill
- THEN the skill name is `cairn-workflow`
- AND the skill describes Cairn-native lifecycle, MCP, receipt, release-readiness, Tracey, Slotcar, and Nix verification contracts

### Requirement: Legacy workflow skill redirect
r[cairn.supersede.hermes_skill.redirect] The system MUST preserve a safe compatibility path from older Ironspec/OpenSpec workflow skill names to the canonical Cairn workflow skill.

#### Scenario: Older skill points to Cairn
r[cairn.supersede.hermes_skill.redirect.local]
- GIVEN a future session loads the older `ironspec-workflow` skill while working on Cairn
- WHEN the skill body is read
- THEN it directs the agent to use `cairn-workflow` for Cairn repository work
- AND it does not present OpenSpec or Ironspec as the primary Cairn identity

### Requirement: Portable receipt hash inputs
r[cairn.supersede.portable_receipts] The system MUST distinguish operator-facing display fields from canonical hash inputs so semantically identical repository evidence can remain portable across checkout roots.

#### Scenario: Cross-root receipts preserve semantic hashes
r[cairn.supersede.portable_receipts.cross_root]
- GIVEN two filesystem roots containing byte-identical Cairn lifecycle content at different absolute paths
- WHEN the operator runs a deterministic read, gate, sync dry-run, archive dry-run, migration dry-run, release-readiness, or agent-receipt validation command against each root
- THEN canonical policy, input, plan, manifest, and receipt hashes that describe repository semantics MUST match across the two roots
- AND any operator-facing absolute path fields MUST be excluded from portable hash material unless the command declares them as explicit inputs

#### Scenario: Display paths remain actionable
r[cairn.supersede.portable_receipts.display_paths]
- GIVEN a receipt includes a path for an operator to inspect or mutate
- WHEN the CLI renders JSON for a local command
- THEN the receipt MAY include display path fields
- AND the receipt MUST identify which canonical root-relative path or hash input was used for deterministic evidence

#### Scenario: Absolute external inputs are explicit
r[cairn.supersede.portable_receipts.explicit_absolute_inputs]
- GIVEN a command depends on an absolute path outside the selected repository root
- WHEN the command includes that path in deterministic evidence
- THEN the receipt MUST declare the absolute path as an explicit external input or represent it by bounded metadata and content hash
- AND hidden ambient absolute paths MUST NOT affect evidence hashes

### Requirement: MCP reproducibility surface
r[cairn.supersede.mcp_reproducibility] The system MUST treat MCP descriptions and tool-call receipts as deterministic agent-facing Cairn use surfaces.

#### Scenario: MCP describe output is stable
r[cairn.supersede.mcp_reproducibility.describe]
- GIVEN the Cairn MCP server is invoked with `--describe`
- WHEN the maintained regression suite runs the description command repeatedly
- THEN the emitted tool names, schemas, mutability flags, and descriptions MUST be byte-stable or normalized-stable

#### Scenario: MCP tool calls match CLI semantics
r[cairn.supersede.mcp_reproducibility.call]
- GIVEN an MCP tool call wraps a Cairn read, validate, gate, sync dry-run, archive dry-run, migration dry-run, release-readiness, or agent-receipt validation operation
- WHEN the maintained regression suite compares the MCP receipt with the corresponding CLI receipt
- THEN both surfaces MUST agree on verdict, validity, issue codes, policy hash, input hash, plan/hash fields, and mutation state

### Requirement: BLAKE3 evidence identifiers
r[cairn.supersede.blake3_receipts] The system MUST use BLAKE3 hashes to identify canonical policy, input snapshots, plans, manifests, and receipts for Cairn decisions that claim validation, gate, sync, archive, migration, release-readiness, MCP, traceability, or agent-run evidence.

#### Scenario: Gate hashes identify policy and input evidence
r[cairn.supersede.blake3_receipts.gate]
- GIVEN Cairn emits a gate receipt
- WHEN the receipt reports policy, input, and receipt hashes
- THEN each hash MUST be a BLAKE3 identifier of canonical deterministic evidence

#### Scenario: Lifecycle plan hashes identify sync and archive evidence
r[cairn.supersede.blake3_receipts.lifecycle_plans]
- GIVEN Cairn emits a sync or archive plan receipt
- WHEN the plan reports policy, input, plan, and receipt hashes
- THEN each hash MUST be a BLAKE3 identifier of canonical deterministic evidence

#### Scenario: Migration hashes identify source and destination manifests
r[cairn.supersede.blake3_receipts.migration]
- GIVEN Cairn emits a migration receipt
- WHEN the receipt reports source, destination, manifest, policy, and receipt hashes
- THEN each hash MUST be a BLAKE3 identifier of canonical deterministic evidence

#### Scenario: Hashing remains canonical and bounded
r[cairn.supersede.blake3_receipts.canonical]
- GIVEN Cairn computes evidence hashes
- WHEN hashes are generated
- THEN the hashed material MUST exclude nondeterministic process noise, display-only absolute paths, filesystem traversal order, and wall-clock timestamps unless supplied as explicit bounded inputs
- AND portable repository artifact paths MUST be normalized relative to the selected lifecycle root before entering canonical hash material

### Requirement: Agent-run nondeterminism manifest
r[cairn.supersede.agent_run_receipts.nondeterminism] The system MUST record and classify nondeterministic inputs that can affect agent-run or Cairn-command replay.

#### Scenario: Captured nondeterminism is explicit
r[cairn.supersede.agent_run_receipts.nondeterminism.captured]
- GIVEN an agent-run receipt includes a nondeterminism manifest
- WHEN replay-relevant nondeterministic inputs are known and permitted
- THEN the manifest MUST name each input class with bounded deterministic metadata

#### Scenario: Hidden nondeterminism fails validation
r[cairn.supersede.agent_run_receipts.nondeterminism.hidden]
- GIVEN an agent-run receipt omits replay-relevant nondeterministic input evidence
- WHEN the completion gate evaluates the receipt
- THEN validation MUST fail with a deterministic nondeterminism issue code

#### Scenario: Environment and path entropy are captured
r[cairn.supersede.agent_run_receipts.nondeterminism.environment]
- GIVEN a receipt-producing run reads environment variables, archive date inputs, external command availability, absolute paths outside the repository root, or machine-specific metadata
- WHEN the run emits or validates a Cairn receipt
- THEN each replay-relevant source MUST be represented in the nondeterminism manifest with a class, bounded metadata, and content hash where content is available
- AND policy-forbidden uncaptured sources MUST fail validation deterministically

### Requirement: Release-readiness evidence bundle
r[cairn.supersede.release_readiness] The system MUST provide a release-readiness evidence bundle that an operator can run before claiming Cairn is ready for agent/CI use.

#### Scenario: Release-readiness references replay evidence
r[cairn.supersede.release_readiness.determinism_replay]
- GIVEN the workspace replay oracle exists
- WHEN `release-readiness` emits its evidence bundle
- THEN it MUST either include the replay oracle receipt hash or explicitly list the replay oracle as a required external operator preflight
- AND the release-readiness receipt MUST make the distinction machine-readable

### Requirement: Agent-run receipts

Cairn SHALL validate deterministic agent-run receipts that bind agent completion evidence to replay-checked Cairn surfaces.

#### Scenario: Replay evidence schema

- **GIVEN** an agent wrapper records completion evidence for a Cairn task
- **WHEN** an agent-run receipt is produced
- **THEN** it SHALL include a `replay_evidence` object containing `replay_receipt_hash`, `replay_policy_hash`, `replay_selected_cases`, and `replay_verdict`
- **AND** the receipt MAY include a display-only `replay_artifact_path`
- **AND** the agent receipt hash SHALL remain the canonical BLAKE3 hash of receipt content excluding only `receipt_hash`
r[cairn.supersede.agent_receipt_replay_evidence.schema]

#### Scenario: Replay evidence validation

- **GIVEN** an agent-run receipt cites deterministic replay evidence
- **WHEN** `agent-receipt validate` or the MCP validation tool validates an agent-run receipt
- **THEN** Cairn SHALL verify that replay evidence cites a passing replay verdict, a BLAKE3-shaped replay receipt hash, the current generated policy hash, and the required replay case coverage
- **AND** the validation receipt SHALL expose the current replay policy hash and required replay cases used for that decision
r[cairn.supersede.agent_receipt_replay_evidence.validation]

#### Scenario: Deterministic replay evidence failures

- **GIVEN** an agent-run receipt has absent, stale, failed, or incomplete replay evidence
- **WHEN** Cairn validates the receipt
- **THEN** agent receipt validation SHALL fail deterministically with stable issue codes and messages
r[cairn.supersede.agent_receipt_replay_evidence.failures]

#### Scenario: Replay oracle covers receipt validation

- **GIVEN** the policy-owned replay plan includes agent receipt validation
- **WHEN** `cairn determinism replay` runs the policy-owned replay plan
- **THEN** the replay oracle SHALL continue to cover `agent-receipt validate` using a replay-evidence-positive receipt fixture
r[cairn.supersede.agent_receipt_replay_evidence.replay]

#### Scenario: Operator documentation and schema

- **GIVEN** operators inspect Cairn public docs or generated policy schemas
- **WHEN** they review agent receipt validation requirements
- **THEN** the stable replay-evidence fields and validation failure semantics SHALL be documented and tested
r[cairn.supersede.agent_receipt_replay_evidence.docs]

### Requirement: Determinism replay oracle

r[cairn.supersede.determinism_replay] The system MUST provide a maintained policy-driven replay oracle for commands whose receipts claim deterministic evidence, and that oracle MUST be invocable by agents/CI as an operator-facing workspace preflight.


#### Scenario: Workspace oracle covers public deterministic surfaces
r[cairn.supersede.determinism_replay.workspace_oracle]
- GIVEN an operator invokes the workspace replay oracle against a Cairn repository or fixture root
- WHEN the oracle builds its default plan
- THEN the plan MUST include maintained cases for status, validate, gate, sync dry-run, archive dry-run, migration dry-run, release-readiness, MCP describe, and selected MCP read-only calls where fixtures support those surfaces
- AND every case MUST declare command kind, arguments, expected mutability, comparison mode, display-only path allowances, and hash fields under comparison

#### Scenario: Repeated command receipts are stable
r[cairn.supersede.determinism_replay.repeated]
- GIVEN an unchanged repository fixture and unchanged explicit command inputs
- WHEN the replay oracle runs the same Cairn command multiple times in the same root
- THEN normalized JSON output and declared evidence hash fields MUST match for every run
- AND mismatch reporting MUST identify the command case and JSON field path that changed

#### Scenario: Cross-root command receipts are stable
r[cairn.supersede.determinism_replay.cross_root]
- GIVEN the replay oracle copies the same fixture into two distinct root paths
- WHEN it runs the same deterministic Cairn command against both roots
- THEN normalized JSON output and declared portable hash fields MUST match
- AND differences limited to documented display-only path fields MUST NOT fail the replay check
- AND undocumented path-dependent differences MUST fail the replay check

#### Scenario: Normalization is explicit and bounded
r[cairn.supersede.determinism_replay.normalization]
- GIVEN the replay oracle compares command outputs
- WHEN a field differs between roots or repeated runs
- THEN the field MAY be ignored only if the case plan declares it as display-only or nondeterminism evidence excluded from portable hashes
- AND the oracle receipt MUST include the normalization rule identifier used for the field

#### Scenario: Oracle receipt summarizes deterministic evidence
r[cairn.supersede.determinism_replay.receipt]
- GIVEN the replay oracle finishes its plan
- WHEN it emits JSON
- THEN the receipt MUST include sorted cases, per-run output hashes, normalized-output hashes, portable evidence hashes, skipped cases, mismatch diagnostics, nondeterminism failures, mutability observations, verdict, and receipt hash
- AND the oracle receipt hash MUST exclude display-only temp-root values from portable hash material

#### Scenario: Oracle receipt schema is stable
r[cairn.supersede.determinism_replay.receipt.schema]
- GIVEN agents parse the oracle receipt
- WHEN a compatible Cairn release emits the replay receipt
- THEN the top-level keys and documented nested diagnostic keys MUST be stable, sorted, and covered by golden or schema tests

#### Scenario: MCP cases are replayed through the same oracle
r[cairn.supersede.determinism_replay.mcp_cases]
- GIVEN the replay plan includes MCP describe or read-only tool calls
- WHEN the oracle runs those cases repeatedly and across roots
- THEN MCP outputs MUST be compared with the same normalized JSON and portable hash rules as CLI cases
- AND read-only MCP calls that correspond to CLI commands SHOULD compare shared semantic fields with the CLI receipt

#### Scenario: Replay is non-mutating by default
r[cairn.supersede.determinism_replay.no_mutation]
- GIVEN a replay case attempts to run an execute command or a command reports mutation
- WHEN the oracle is running in default mode
- THEN the oracle MUST fail that case with a deterministic mutation-denied issue code
- AND the fixture root MUST remain unchanged relative to its pre-case manifest

#### Scenario: Hidden nondeterminism is rejected
r[cairn.supersede.determinism_replay.hidden_entropy]
- GIVEN a command fixture whose output or evidence hash would depend on an uncaptured clock, environment variable, random value, filesystem ordering, external command probe, or ambient absolute path
- WHEN the replay oracle or receipt gate evaluates the command
- THEN it MUST fail with a deterministic issue code naming the nondeterminism class
- AND the issue MUST include bounded metadata sufficient for an operator to reproduce or suppress the class by policy

#### Scenario: Mismatches are actionable
r[cairn.supersede.determinism_replay.mismatch_diagnostics]
- GIVEN the oracle detects any repeated-run or cross-root mismatch
- WHEN it records the failure
- THEN each mismatch MUST name the command case, comparison mode, JSON field path, mismatch class, expected hash or value summary, and actual hash or value summary
- AND mismatch entries MUST be sorted deterministically

#### Scenario: Replay plan is policy-owned
r[cairn.supersede.determinism_replay.policy_plan]
- GIVEN Cairn ships a generated policy bundle
- WHEN the workspace replay oracle builds its default plan
- THEN it MUST load maintained replay case metadata from policy rather than relying only on hidden CLI literals
- AND every policy case MUST declare id, command kind, arguments, fixture root, expected mutability, comparison modes, display-only path fields, hash fields, and normalization rules

#### Scenario: Replay policy schema is stable
r[cairn.supersede.determinism_replay.policy_schema]
- GIVEN agents and maintainers review deterministic replay coverage
- WHEN the policy is exported to generated JSON
- THEN the replay plan MUST be present in generated policy with stable keys and deterministic case ordering
- AND tests MUST fail if the generated policy and CLI-supported replay plan drift

#### Scenario: Replay expands across public deterministic surfaces
r[cairn.supersede.determinism_replay.expanded_surfaces]
- GIVEN the policy default replay plan is used
- WHEN an operator runs `cairn determinism replay --root <root>`
- THEN the plan MUST include maintained read-only or dry-run cases for status, validate, proposal/design/tasks gates, sync dry-run, archive dry-run, migration dry-run, agent-receipt validation, release-readiness, MCP describe, and selected MCP read-only calls where fixtures support those surfaces

#### Scenario: Policy replay diagnostics are deterministic
r[cairn.supersede.determinism_replay.policy_diagnostics]
- GIVEN a policy case is unsupported, skipped, or attempts mutation
- WHEN the replay oracle emits its receipt
- THEN skipped and failed policy cases MUST be reported with deterministic issue codes, case ids, command kind, and bounded reason text
- AND mutation-capable cases MUST remain denied unless the case is explicitly dry-run/read-only

#### Scenario: CI replay case selection
r[cairn.supersede.determinism_replay.ci_case_selection]
- GIVEN a policy-owned determinism replay plan exists for the root
- WHEN an operator runs `cairn determinism replay --root <root> --case <id>`
- THEN Cairn MUST run only the selected policy-owned replay case
- AND the replay receipt MUST list the sorted selected and available case IDs
- AND an unknown case ID MUST produce a deterministic failure receipt rather than an ad hoc diagnostic

#### Scenario: CI replay output modes and artifacts
r[cairn.supersede.determinism_replay.ci_output_modes]
- GIVEN a determinism replay receipt can be produced for the selected case set
- WHEN an operator selects `--format json`, `--format jsonl`, or `--format summary`
- THEN Cairn MUST render deterministic stdout suitable for the selected mode
- AND `--out <path>` MUST write the full JSON replay receipt as an artifact while preserving deterministic stdout rendering

#### Scenario: CI replay failure ergonomics
r[cairn.supersede.determinism_replay.ci_failure_ergonomics]
- GIVEN a replay run produces invalid evidence
- WHEN replay detects an unknown case, mismatch, hidden entropy, or other invalid replay evidence
- THEN Cairn MUST return a nonzero exit status
- AND mismatch evidence MUST include stable failing case IDs, mismatch classes, field paths, expected/actual hash summaries, and a reproduction command

#### Scenario: CI replay tests and public documentation
r[cairn.supersede.determinism_replay.ci_tests_docs]
- GIVEN replay selection and rendering are public agent/CI surfaces
- WHEN Cairn changes replay selection, output modes, artifact behavior, or failure receipts
- THEN tests, public schema checks, and operator documentation MUST cover those stable public replay surfaces

#### Scenario: Policy-owned replay groups
r[cairn.supersede.determinism_replay.groups.policy]
- GIVEN the generated replay policy defines named replay groups
- WHEN operators inspect the policy or run replay group selection
- THEN group membership SHALL be policy-owned and SHALL refer only to known replay case IDs

#### Scenario: Replay group CLI selection
r[cairn.supersede.determinism_replay.groups.cli]
- GIVEN an operator wants a named replay suite
- WHEN they run `cairn determinism replay --group <id>`
- THEN Cairn SHALL replay the policy-defined cases for that group while preserving the existing default full-suite behavior
- AND `--case` and `--group` used together SHALL be rejected before replay

#### Scenario: Replay group failure receipts
r[cairn.supersede.determinism_replay.groups.failures]
- GIVEN an operator selects an unknown replay group or combines incompatible replay selectors
- WHEN replay handles the selection failure
- THEN Cairn SHALL emit a deterministic machine-readable failure receipt with stable mismatch class, expected values, actual selector, and reproduction command
- AND replay SHALL exit nonzero after rendering the receipt

#### Scenario: Replay group receipt fields
r[cairn.supersede.determinism_replay.groups.receipt]
- GIVEN replay is run with default, case, or group selection
- WHEN the replay receipt is emitted
- THEN it SHALL expose sorted `available_groups` and `selected_groups` fields while preserving existing `available_cases` and `selected_cases` semantics

#### Scenario: Replay group docs and tests
r[cairn.supersede.determinism_replay.groups.docs]
- GIVEN replay groups become a public CI/agent interface
- WHEN Cairn changes group policy, selection, receipts, or failure semantics
- THEN tests, public schema checks, and operator docs SHALL cover the stable group behavior

### Requirement: Agent-run receipts

Cairn SHALL validate deterministic agent-run receipts that bind agent completion evidence to replay-checked Cairn surfaces.

#### Scenario: Replay evidence schema

- **GIVEN** an agent wrapper records completion evidence for a Cairn task
- **WHEN** an agent-run receipt is produced
- **THEN** it SHALL include a `replay_evidence` object containing `replay_receipt_hash`, `replay_policy_hash`, `replay_selected_cases`, and `replay_verdict`
- **AND** the receipt MAY include a display-only `replay_artifact_path`
- **AND** the agent receipt hash SHALL remain the canonical BLAKE3 hash of receipt content excluding only `receipt_hash`
r[cairn.supersede.agent_receipt_replay_evidence.schema]

#### Scenario: Replay evidence validation

- **GIVEN** an agent-run receipt cites deterministic replay evidence
- **WHEN** `agent-receipt validate` or the MCP validation tool validates an agent-run receipt
- **THEN** Cairn SHALL verify that replay evidence cites a passing replay verdict, a BLAKE3-shaped replay receipt hash, the current generated policy hash, and the required replay case coverage
- **AND** the validation receipt SHALL expose the current replay policy hash and required replay cases used for that decision
r[cairn.supersede.agent_receipt_replay_evidence.validation]

#### Scenario: Deterministic replay evidence failures

- **GIVEN** an agent-run receipt has absent, stale, failed, or incomplete replay evidence
- **WHEN** Cairn validates the receipt
- **THEN** agent receipt validation SHALL fail deterministically with stable issue codes and messages
r[cairn.supersede.agent_receipt_replay_evidence.failures]

#### Scenario: Replay oracle covers receipt validation

- **GIVEN** the policy-owned replay plan includes agent receipt validation
- **WHEN** `cairn determinism replay` runs the policy-owned replay plan
- **THEN** the replay oracle SHALL continue to cover `agent-receipt validate` using a replay-evidence-positive receipt fixture
r[cairn.supersede.agent_receipt_replay_evidence.replay]

#### Scenario: Operator documentation and schema

- **GIVEN** operators inspect Cairn public docs or generated policy schemas
- **WHEN** they review agent receipt validation requirements
- **THEN** the stable replay-evidence fields and validation failure semantics SHALL be documented and tested
r[cairn.supersede.agent_receipt_replay_evidence.docs]

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

### Requirement: Agent-run receipt replay evidence

Agent-run receipt validation MUST derive required replay evidence coverage from the generated policy `determinism_replay_policy.groups["agent"]` instead of a hidden Rust case list, while preserving deterministic machine-readable validation receipts.

#### Scenario: Policy-derived agent group coverage

r[cairn.supersede.agent_receipt_replay_group_coverage.validation]
- GIVEN generated replay policy defines an `agent` group with replay cases
- WHEN `cairn agent-receipt validate <receipt>` validates a receipt with passing replay evidence
- THEN the validation receipt includes `required_replay_group = "agent"`
- AND `required_replay_cases` equals the sorted cases from the policy `agent` group
- AND validation succeeds only when `replay_selected_cases` covers those policy-derived cases

#### Scenario: Replay selected groups evidence is stable

r[cairn.supersede.agent_receipt_replay_group_coverage.schema]
- GIVEN an agent-run receipt includes `replay_evidence.replay_selected_groups`
- WHEN Cairn validates and summarizes the receipt
- THEN the validation receipt preserves the selected group list under `replay_evidence`
- AND public schema policy lists `replay_selected_groups`, `required_replay_group`, and `required_replay_cases` as stable receipt evidence fields

#### Scenario: Missing or stale group coverage fails deterministically

r[cairn.supersede.agent_receipt_replay_group_coverage.failures]
- GIVEN replay evidence omits the `agent` selected group or omits a policy-required expanded case
- WHEN Cairn validates the receipt repeatedly
- THEN the validation receipts are byte-stable for hash fields
- AND the receipt includes deterministic issue codes for missing group coverage or insufficient case coverage

#### Scenario: Documentation names policy-owned agent replay preflight

r[cairn.supersede.agent_receipt_replay_group_coverage.docs]
- GIVEN an operator is wiring agent receipt wrappers
- WHEN they read Cairn's replay evidence documentation
- THEN the docs identify `cairn determinism replay --group agent` as the policy-owned preflight
- AND the docs explain that validation expands the policy group to required cases

#### Scenario: Replay oracle covers group-backed validation

r[cairn.supersede.agent_receipt_replay_group_coverage.replay]
- GIVEN Cairn's deterministic replay oracle includes agent receipt validation surfaces
- WHEN `cairn determinism replay --group agent` runs
- THEN it exercises the group-backed agent receipt validation path
- AND the replay receipt remains deterministic across repeated runs

### Requirement: Determinism replay entropy inventory

The system MUST disclose, normalize, or reject replay-relevant entropy through a policy-owned inventory that agents and CI can verify without seeing secret values.

#### Scenario: Policy-owned entropy inventory
r[cairn.supersede.determinism_entropy_inventory.policy]
- GIVEN generated policy with replay cases and groups
- WHEN replay policy is exported
- THEN the policy MUST include a stable inventory of replay-relevant entropy sources including clock/date, locale/timezone, cwd/root paths, environment variables, filesystem metadata/order, external commands, and Nix/build context

#### Scenario: Entropy inventory receipt disclosure
r[cairn.supersede.determinism_entropy_inventory.receipt]
- GIVEN `cairn determinism replay` emits a receipt
- WHEN agents or CI inspect replay evidence
- THEN the receipt MUST expose stable `entropy_inventory`, `entropy_assumptions`, and `entropy_policy_hash` fields derived from policy
- AND the receipt hash MUST bind those fields without including secret environment values

#### Scenario: Entropy inventory schema
r[cairn.supersede.determinism_entropy_inventory.schema]
- GIVEN public replay receipt schemas
- WHEN policy is exported and schema tests run
- THEN replay receipt stable fields MUST include `entropy_inventory`, `entropy_assumptions`, and `entropy_policy_hash`

#### Scenario: Rejected entropy failures
r[cairn.supersede.determinism_entropy_inventory.failures]
- GIVEN a replay-relevant rejected entropy source is present in the environment
- WHEN `cairn determinism replay` runs
- THEN replay MUST emit a deterministic nonzero failure receipt naming the source id, class, and handling
- AND the receipt MUST NOT expose the source value

#### Scenario: Entropy inventory documentation
r[cairn.supersede.determinism_entropy_inventory.docs]
- GIVEN agent and CI operators inspect Cairn determinism docs
- WHEN they read replay guidance
- THEN entropy handling and secret-safe disclosure rules MUST be documented

#### Scenario: Entropy inventory tests
r[cairn.supersede.determinism_entropy_inventory.tests]
- GIVEN replay emits entropy inventory evidence and rejects configured environment entropy
- WHEN focused replay tests run
- THEN positive receipt disclosure and negative rejected-entropy behavior MUST be covered

#### Scenario: Entropy inventory verification
r[cairn.supersede.determinism_entropy_inventory.verification]
- GIVEN the entropy inventory behavior changes public replay evidence
- WHEN Cairn is prepared for archive
- THEN policy export, formatting, focused/full tests, validation, Tracey coverage, release-readiness, and replay smokes MUST pass

#### Scenario: Entropy inventory archive
r[cairn.supersede.determinism_entropy_inventory.archive]
- GIVEN verification passes
- WHEN the change is archived
- THEN the accepted spec and archive artifacts MUST preserve the entropy inventory requirements and evidence

## MODIFIED Requirements

### Requirement: Determinism replay oracle

The system MUST emit a stable, policy-owned environment attestation in replay receipts so agents and CI can prove which reviewed execution facts shaped replay without leaking host-specific paths, raw environment values, or secrets.

#### Scenario: Replay receipt includes environment attestation

r[cairn.supersede.replay_environment_attestation.policy]
r[cairn.supersede.replay_environment_attestation.receipt]

- GIVEN a repository with generated replay policy
- WHEN `cairn determinism replay --root <root>` runs successfully
- THEN the receipt includes `environment_attestation`
- AND the attestation includes stable Cairn version, generated policy hash, entropy policy hash, selected cases, selected groups, replay scope, policy fact disclosures, and an attestation hash
- AND repeated and cross-root replay produce the same attestation hash for equivalent selected cases/groups
- r[task cairn.supersede.replay_environment_attestation.policy]
- r[task cairn.supersede.replay_environment_attestation.receipt]
- r[verify cairn.supersede.replay_environment_attestation.policy]
- r[verify cairn.supersede.replay_environment_attestation.receipt]

#### Scenario: Environment attestation is secret safe

r[cairn.supersede.replay_environment_attestation.secret_safe]

- GIVEN host environment variables and checkout paths may contain sensitive values
- WHEN replay renders environment attestation or deterministic rejection receipts
- THEN the attestation exposes reviewed source names, classes, handling modes, package/version identity, and hashes only
- AND it does not expose raw environment values, usernames, absolute checkout roots, or machine-specific temporary paths
- r[task cairn.supersede.replay_environment_attestation.receipt]
- r[verify cairn.supersede.replay_environment_attestation.secret_safe]

### Requirement: Agent receipt replay evidence

Agent receipt validation SHOULD accept optional replay environment attestation hash evidence and report or reject malformed attestation evidence deterministically without making the field mandatory for older wrappers.

#### Scenario: Agent receipt binds optional replay attestation hash

r[cairn.supersede.replay_environment_attestation.agent]

- GIVEN an agent-run receipt with replay evidence from a passing `agent` replay group
- WHEN the replay evidence includes `replay_environment_attestation_hash`
- THEN validation requires the value to be a BLAKE3 hex hash
- AND validation reports the optional attestation hash in the replay evidence summary
- AND malformed optional attestation hash evidence emits a deterministic issue code
- r[task cairn.supersede.replay_environment_attestation.agent]
- r[verify cairn.supersede.replay_environment_attestation.agent]

#### Scenario: Replay attestation change is verified and archived

r[cairn.supersede.replay_environment_attestation.docs]
r[cairn.supersede.replay_environment_attestation.verification]
r[cairn.supersede.replay_environment_attestation.archive]

- GIVEN implementation, docs, schema, tests, and fixtures are updated
- WHEN the change is completed
- THEN Cairn validation, Tracey coverage, policy export checks, focused/full Rust tests, release-readiness, and replay smokes pass before archive
- r[task cairn.supersede.replay_environment_attestation.docs]
- r[task cairn.supersede.replay_environment_attestation.verification]
- r[task cairn.supersede.replay_environment_attestation.archive]
- r[verify cairn.supersede.replay_environment_attestation.docs]
- r[verify cairn.supersede.replay_environment_attestation.verification]
- r[verify cairn.supersede.replay_environment_attestation.archive]

## MODIFIED Requirements

### Requirement: Determinism replay oracle

The system MUST provide a read-only replay receipt verification command so saved replay evidence can be validated later by agents, CI, and release gates without rerunning replay cases.

#### Scenario: Saved replay receipt verifies successfully

r[cairn.supersede.replay_receipt_verification.contract]
r[cairn.supersede.replay_receipt_verification.cli]
r[cairn.supersede.replay_receipt_verification.validation]
r[cairn.supersede.replay_receipt_verification.receipt]
r[cairn.supersede.replay_receipt_verification.schema]

- GIVEN a saved replay receipt produced by `cairn determinism replay --root <root>`
- WHEN `cairn determinism verify-receipt <receipt.json> --root <root>` runs
- THEN the command emits a deterministic verification receipt
- AND it validates the saved receipt hash, current replay policy hash, entropy policy hash, selected cases, selected groups, and environment attestation hash
- AND it reports `valid: true` and `verdict: pass` without mutating the workspace or rerunning replay cases
- r[task cairn.supersede.replay_receipt_verification.contract]
- r[task cairn.supersede.replay_receipt_verification.cli]
- r[task cairn.supersede.replay_receipt_verification.validation]
- r[task cairn.supersede.replay_receipt_verification.receipt]
- r[verify cairn.supersede.replay_receipt_verification.cli]
- r[verify cairn.supersede.replay_receipt_verification.validation]
- r[verify cairn.supersede.replay_receipt_verification.receipt]

#### Scenario: Saved replay receipt failures are deterministic

r[cairn.supersede.replay_receipt_verification.failures]
r[cairn.supersede.replay_receipt_verification.secret_safe]

- GIVEN a saved replay receipt with malformed JSON, missing modern fields, stale policy hash, tampered receipt hash, invalid selected case/group, invalid entropy policy hash, or malformed environment attestation hash
- WHEN `cairn determinism verify-receipt <receipt.json> --root <root>` runs
- THEN the command emits a deterministic failure receipt with stable issue codes
- AND the failure receipt does not expose raw environment values, credentials, absolute checkout roots, or machine-specific temporary paths in hash material
- r[task cairn.supersede.replay_receipt_verification.validation]
- r[task cairn.supersede.replay_receipt_verification.receipt]
- r[verify cairn.supersede.replay_receipt_verification.failures]

#### Scenario: Replay receipt verification is documented and archived

r[cairn.supersede.replay_receipt_verification.docs]
r[cairn.supersede.replay_receipt_verification.verification]
r[cairn.supersede.replay_receipt_verification.archive]

- GIVEN implementation, docs, tests, and specs are updated
- WHEN the change is completed
- THEN Cairn validation, Tracey coverage, policy export checks, focused/full Rust tests, release-readiness, and replay smokes pass before archive
- r[task cairn.supersede.replay_receipt_verification.docs]
- r[task cairn.supersede.replay_receipt_verification.verification]
- r[task cairn.supersede.replay_receipt_verification.archive]
- r[verify cairn.supersede.replay_receipt_verification.docs]
- r[verify cairn.supersede.replay_receipt_verification.verification]
- r[verify cairn.supersede.replay_receipt_verification.archive]

### Requirement: Agent Evidence Bundle Verification Command r[cairn.supersede.agent_evidence_bundle_verification.cli]

Cairn SHALL provide a read-only `cairn agent-receipt verify-bundle <bundle.json> --root .` command for validating an end-to-end agent evidence bundle.

#### Scenario: Bundle command accepts wrapper evidence

r[cairn.supersede.agent_evidence_bundle_verification.cli]

- GIVEN a JSON bundle that references an agent receipt, replay receipt, replay verification receipt, and optional release-readiness receipt
- WHEN an operator runs `cairn agent-receipt verify-bundle <bundle.json> --root .`
- THEN Cairn SHALL load the referenced artifacts without mutating the workspace
- AND Cairn SHALL emit a deterministic bundle verification receipt.

### Requirement: Agent Evidence Bundle Validation r[cairn.supersede.agent_evidence_bundle_verification.validation]

Cairn SHALL validate that bundle artifacts agree about the current replay policy, agent replay evidence, saved replay receipt hash, replay verification receipt, required replay group coverage, and environment attestation hash.

#### Scenario: Valid bundle passes

r[cairn.supersede.agent_evidence_bundle_verification.validation]

- GIVEN an agent receipt with passing replay evidence for the policy-owned `agent` group
- AND a saved replay receipt whose receipt hash and environment attestation hash verify
- AND a replay verification receipt that reports `valid=true` for the saved replay receipt
- WHEN Cairn verifies the bundle
- THEN the bundle receipt SHALL report `valid=true` and `verdict=pass`.

#### Scenario: Stale or mismatched evidence fails deterministically

r[cairn.supersede.agent_evidence_bundle_verification.validation]
r[cairn.supersede.agent_evidence_bundle_verification.secret_safe]

- GIVEN a bundle with stale policy hashes, mismatched replay hashes, missing group coverage, failed replay verification evidence, or mismatched environment attestation evidence
- WHEN Cairn verifies the bundle
- THEN the bundle receipt SHALL report `valid=false`
- AND SHALL include stable issue codes for each mismatch class.

### Requirement: Agent Evidence Bundle Receipt Schema r[cairn.supersede.agent_evidence_bundle_verification.receipt]

Cairn SHALL publish a stable schema for bundle verification receipts so CI and wrappers can consume one deterministic gate artifact.

#### Scenario: Bundle receipt exposes stable fields

r[cairn.supersede.agent_evidence_bundle_verification.receipt]

- GIVEN any bundle verification result
- WHEN Cairn emits the bundle verification receipt
- THEN the receipt SHALL include stable fields for command, validity, verdict, issues, current replay policy hash, required replay group, agent receipt hash, replay receipt hash, replay verification receipt hash, environment attestation hash, optional release-readiness receipt hash, and receipt hash.

### Requirement: Agent Evidence Bundle Secret-Safe Failures r[cairn.supersede.agent_evidence_bundle_verification.secret_safe]

Cairn SHALL keep bundle verification failure receipts secret-safe and path-safe.

#### Scenario: Bundle failure omits raw paths and secrets

r[cairn.supersede.agent_evidence_bundle_verification.secret_safe]

- GIVEN a bundle that references invalid or tampered evidence artifacts
- WHEN Cairn emits a failure receipt
- THEN the receipt SHALL identify artifact classes and stable issue codes
- AND SHALL NOT include raw environment values, credentials, absolute checkout roots, or host-specific paths.

### Requirement: Agent Evidence Bundle Documentation r[cairn.supersede.agent_evidence_bundle_verification.docs]

Cairn SHALL document how wrappers create and verify end-to-end evidence bundles.

#### Scenario: Wrapper flow is documented

r[cairn.supersede.agent_evidence_bundle_verification.docs]

- GIVEN a wrapper that runs agent receipt validation, replay, replay receipt verification, and optional release-readiness
- WHEN the wrapper consults Cairn documentation
- THEN the documentation SHALL show how to assemble a bundle and run `agent-receipt verify-bundle` before publishing evidence.

### Requirement: Agent Evidence Bundle Public Schema r[cairn.supersede.agent_evidence_bundle_verification.schema]

Cairn SHALL include bundle verification receipt fields in the policy-owned public receipt schema catalog.

#### Scenario: Schema catalog lists bundle verification receipt

r[cairn.supersede.agent_evidence_bundle_verification.schema]

- GIVEN the generated Cairn policy bundle
- WHEN a consumer inspects public receipt schemas
- THEN the catalog SHALL include the stable fields for `agent-receipt verify-bundle` receipts.

### Requirement: Agent Evidence Bundle Test Coverage r[cairn.supersede.agent_evidence_bundle_verification.tests]

Cairn SHALL test valid and invalid bundle verification flows.

#### Scenario: Tests cover bundle verifier behavior

r[cairn.supersede.agent_evidence_bundle_verification.tests]

- GIVEN Cairn's CLI test suite
- WHEN the tests run
- THEN they SHALL cover passing bundles, tampered replay receipts, failed replay verification receipts, group coverage mismatch, optional release-readiness mismatch, deterministic issue codes, and path/secret safety.

### Requirement: Agent Evidence Bundle Archive Evidence r[cairn.supersede.agent_evidence_bundle_verification.archive]

Cairn SHALL archive the completed bundle verification change only after validation and coverage gates pass.

#### Scenario: Archive captures completed change

r[cairn.supersede.agent_evidence_bundle_verification.archive]

- GIVEN the bundle verifier implementation is complete
- WHEN the change is synced and archived
- THEN the accepted specs SHALL retain the bundle verifier requirements
- AND Tracey coverage SHALL pass for the new requirement IDs.

### Requirement: Signed Evidence Manifest Verification

Cairn SHALL verify a persisted evidence manifest that indexes agent and CI evidence artifacts with deterministic pass/fail behavior. r[cairn.supersede.signed_evidence_manifest.verify]

#### Scenario: Valid manifest verifies artifact index

r[cairn.supersede.signed_evidence_manifest.verify]

- GIVEN a manifest that lists a valid agent evidence bundle and referenced evidence artifacts with matching stable hashes
- WHEN the operator runs `cairn evidence-manifest verify <manifest.json> --root .`
- THEN Cairn SHALL emit a deterministic receipt with `valid=true`, `verdict="pass"`, stable artifact roles, stable artifact hashes, and its own `receipt_hash`.

#### Scenario: Invalid artifact hash is rejected

r[cairn.supersede.signed_evidence_manifest.verify]

- GIVEN a manifest whose declared artifact hash does not match the referenced artifact content
- WHEN the operator verifies the manifest
- THEN Cairn SHALL emit a deterministic failure receipt with a stable issue code and SHALL exit nonzero.

#### Scenario: Referenced evidence chain is invalid

r[cairn.supersede.signed_evidence_manifest.chain]

- GIVEN a manifest whose referenced bundle, replay receipt, replay verification receipt, agent receipt, or release-readiness receipt fails its authoritative verifier
- WHEN the operator verifies the manifest
- THEN Cairn SHALL emit a deterministic failure receipt that names only stable roles/classes and SHALL exit nonzero.

### Requirement: Signed Evidence Manifest Public Schema

Cairn SHALL publish a stable public receipt schema for evidence manifest verification. r[cairn.supersede.signed_evidence_manifest.schema]

#### Scenario: Stable manifest verification fields are policy-owned

r[cairn.supersede.signed_evidence_manifest.schema]

- GIVEN generated Cairn policy
- WHEN public receipt schemas are inspected
- THEN the `evidence-manifest verify` receipt schema SHALL include stable top-level fields for command, root, valid, verdict, issues, artifacts, manifest hash, optional signer metadata hash, optional anchor metadata hash, and receipt hash.

### Requirement: Evidence Manifest Validator Composition

Cairn SHALL compose existing deterministic validators when verifying manifest artifact roles. r[cairn.supersede.signed_evidence_manifest.chain]

#### Scenario: Bundle and replay validators remain authoritative

r[cairn.supersede.signed_evidence_manifest.chain]

- GIVEN a manifest that references an agent evidence bundle and replay verification artifacts
- WHEN Cairn verifies the manifest
- THEN Cairn SHALL use existing bundle and replay receipt validation semantics rather than accepting wrapper-provided claims.

### Requirement: Public Metadata Only

Evidence manifest signer and anchor metadata SHALL be optional, deterministic, JSON-object-shaped, and secret-free. r[cairn.supersede.signed_evidence_manifest.metadata]

#### Scenario: Metadata is hashed but not cryptographically trusted

r[cairn.supersede.signed_evidence_manifest.metadata]

- GIVEN signer or anchor metadata in a manifest
- WHEN Cairn verifies the manifest
- THEN Cairn SHALL include stable metadata hashes in the verification receipt and SHALL NOT claim remote anchoring or signature cryptographic validity.

### Requirement: Manifest Documentation

Cairn SHALL document the evidence manifest format, CLI usage, stable receipt fields, and non-goals. r[cairn.supersede.signed_evidence_manifest.docs]

#### Scenario: Operators can build manifests without hidden wrapper conventions

r[cairn.supersede.signed_evidence_manifest.docs]

- GIVEN the README and agent receipt documentation
- WHEN an operator follows the documented evidence flow
- THEN the operator can save replay, replay-verification, agent, bundle, and optional release-readiness artifacts and verify them through the manifest command.

### Requirement: Manifest Verification Tests

Cairn SHALL test valid and invalid evidence manifest verification behavior. r[cairn.supersede.signed_evidence_manifest.tests]

#### Scenario: Failure receipts do not leak host paths or secrets

r[cairn.supersede.signed_evidence_manifest.tests]

- GIVEN malformed or tampered manifest evidence with path-like and secret-like input values
- WHEN Cairn emits a failure receipt
- THEN the receipt SHALL avoid raw absolute paths, checkout roots, raw environment values, and credentials.

### Requirement: Manifest Change Lifecycle

Cairn SHALL sync and archive the signed evidence manifest change after verification. r[cairn.supersede.signed_evidence_manifest.archive]

#### Scenario: Archived manifest change retains review evidence

r[cairn.supersede.signed_evidence_manifest.archive]

- GIVEN implementation and verification are complete
- WHEN the change is synced and archived
- THEN the accepted spec and archive SHALL retain traceable requirements and verification references.

### Requirement: Manifest Change Gates

Cairn SHALL pass proposal, design, and tasks gates for the signed evidence manifest change. r[cairn.supersede.signed_evidence_manifest.gates]

#### Scenario: Change artifacts are gate-ready

r[cairn.supersede.signed_evidence_manifest.gates]

- GIVEN proposal, design, tasks, and spec delta artifacts
- WHEN Cairn gates each stage
- THEN each gate receipt SHALL be valid.

### Requirement: Evidence Manifest Generation Command r[cairn.supersede.evidence_manifest_generate.cli]

Cairn SHALL provide a deterministic `cairn evidence-manifest generate` command for producing manifest JSON from role-specific evidence artifacts.

#### Scenario: Command writes canonical manifest JSON

r[cairn.supersede.evidence_manifest_generate.cli]

- GIVEN valid paths for an agent evidence bundle, agent receipt, replay receipt, and replay verification receipt
- WHEN the operator runs `cairn evidence-manifest generate --bundle bundle.json --agent-receipt agent.json --replay replay.json --replay-verification replay-verify.json --out manifest.json --root .`
- THEN Cairn SHALL write a manifest JSON artifact that `cairn evidence-manifest verify manifest.json --root .` can consume.

### Requirement: Canonical Manifest Artifact Ordering r[cairn.supersede.evidence_manifest_generate.canonical]

Generated evidence manifests SHALL use stable artifact roles, stable artifact hashes, and deterministic artifact ordering independent of CLI argument order.

#### Scenario: Artifact roles are ordered and hashed

r[cairn.supersede.evidence_manifest_generate.canonical]

- GIVEN the same artifact inputs in any CLI argument order
- WHEN Cairn generates a manifest
- THEN artifact entries SHALL be emitted in stable role order
- AND each entry SHALL include the stable role, wrapper-provided path, and BLAKE3 artifact hash.

### Requirement: Public Metadata Generation r[cairn.supersede.evidence_manifest_generate.metadata]

Cairn SHALL support optional signer and anchor metadata JSON objects when generating evidence manifests.

#### Scenario: Metadata is public and object-shaped

r[cairn.supersede.evidence_manifest_generate.metadata]

- GIVEN optional signer or anchor metadata file arguments
- WHEN Cairn generates a manifest
- THEN Cairn SHALL require each metadata file to parse as a JSON object
- AND SHALL include the metadata object in the generated manifest without claiming cryptographic signature or remote-anchor validity.

### Requirement: Generation Failure Receipts r[cairn.supersede.evidence_manifest_generate.failures]

Cairn SHALL emit deterministic failure receipts for invalid manifest generation inputs.

#### Scenario: Missing or invalid inputs fail safely

r[cairn.supersede.evidence_manifest_generate.failures]

- GIVEN missing required artifact flags, unreadable artifacts, invalid metadata JSON, or non-object metadata JSON
- WHEN Cairn attempts to generate a manifest
- THEN Cairn SHALL emit a deterministic failure receipt and exit nonzero
- AND the receipt SHALL avoid raw absolute paths, checkout roots, environment values, credentials, and host-specific values.

### Requirement: Manifest Generation Documentation r[cairn.supersede.evidence_manifest_generate.docs]

Cairn SHALL document the manifest generation command, stable roles, and generate-to-verify flow.

#### Scenario: Operator flow includes generation

r[cairn.supersede.evidence_manifest_generate.docs]

- GIVEN the README and agent receipt documentation
- WHEN an operator follows the documented evidence flow
- THEN the operator can generate a manifest from saved artifacts and verify the generated manifest before publication.

### Requirement: Manifest Generation Test Coverage r[cairn.supersede.evidence_manifest_generate.tests]

Cairn SHALL test valid and invalid evidence manifest generation behavior.

#### Scenario: Tests cover round-trip and failures

r[cairn.supersede.evidence_manifest_generate.tests]

- GIVEN Cairn's CLI test suite
- WHEN manifest generation tests run
- THEN they SHALL cover generate-to-verify success, canonical role order, metadata inclusion, missing input failures, invalid metadata failures, and path/secret-safe receipts.

### Requirement: Manifest Generation Change Gates r[cairn.supersede.evidence_manifest_generate.gates]

Cairn SHALL pass proposal, design, and tasks gates for the evidence manifest generation change.

#### Scenario: Change artifacts are gate-ready

r[cairn.supersede.evidence_manifest_generate.gates]

- GIVEN proposal, design, tasks, and spec delta artifacts
- WHEN Cairn gates each stage
- THEN each gate receipt SHALL be valid.

### Requirement: Manifest Generation Change Lifecycle r[cairn.supersede.evidence_manifest_generate.archive]

Cairn SHALL sync and archive the evidence manifest generation change after verification.

#### Scenario: Archived change retains review evidence

r[cairn.supersede.evidence_manifest_generate.archive]

- GIVEN implementation and verification are complete
- WHEN the change is synced and archived
- THEN the accepted spec and archive SHALL retain traceable requirements and verification references.

### Requirement: Deterministic Surface Coverage Audit Command r[cairn.supersede.determinism_surface_coverage_audit.cli]

Cairn SHALL provide a deterministic `cairn determinism coverage-audit --root .` command that audits policy-owned replay coverage for public deterministic surfaces.

#### Scenario: Audit emits pass/fail receipt

r[cairn.supersede.determinism_surface_coverage_audit.cli]

- GIVEN Cairn's generated policy
- WHEN the operator runs `cairn determinism coverage-audit --root .`
- THEN Cairn SHALL emit a deterministic JSON receipt with verdict, issues, current replay policy hash, surface inventory hash, covered surfaces, missing surfaces, and receipt hash.

### Requirement: Policy-Owned Deterministic Surface Inventory r[cairn.supersede.determinism_surface_coverage_audit.inventory]

Cairn SHALL declare deterministic public receipt surfaces and their required replay coverage in generated policy.

#### Scenario: Inventory declares coverage expectations

r[cairn.supersede.determinism_surface_coverage_audit.inventory]

- GIVEN a public deterministic CLI or MCP receipt surface
- WHEN that surface is expected to support agent or CI determinism claims
- THEN the policy inventory SHALL name the surface id, command label, surface kind, required replay cases, required replay groups, and determinism claim category.

### Requirement: Missing Coverage Failure Semantics r[cairn.supersede.determinism_surface_coverage_audit.failures]

Cairn SHALL fail deterministically when an inventoried surface lacks policy replay coverage.

#### Scenario: Missing case or group is reported safely

r[cairn.supersede.determinism_surface_coverage_audit.failures]

- GIVEN a surface inventory entry with a missing replay case, missing replay group, or group that omits a required case
- WHEN Cairn audits coverage
- THEN the receipt SHALL set `valid=false`, exit nonzero, and report stable issue codes without raw roots, paths, environment values, credentials, or host-specific values.

### Requirement: Deterministic Surface Coverage Documentation r[cairn.supersede.determinism_surface_coverage_audit.docs]

Cairn SHALL document coverage audit usage and scope.

#### Scenario: Operator can distinguish audit scope

r[cairn.supersede.determinism_surface_coverage_audit.docs]

- GIVEN Cairn's README and agent receipt documentation
- WHEN an operator reads the deterministic evidence flow
- THEN the docs SHALL explain that coverage-audit checks policy replay catalog completeness and does not by itself run replay or prove all workspace behavior.

### Requirement: Deterministic Surface Coverage Tests r[cairn.supersede.determinism_surface_coverage_audit.tests]

Cairn SHALL test passing and failing deterministic surface coverage audit behavior.

#### Scenario: Tests cover real and broken inventories

r[cairn.supersede.determinism_surface_coverage_audit.tests]

- GIVEN Cairn's CLI test suite
- WHEN coverage audit tests run
- THEN tests SHALL prove the real policy passes and synthetic missing coverage produces deterministic, path-safe failure receipts.

### Requirement: Deterministic Surface Coverage Gates r[cairn.supersede.determinism_surface_coverage_audit.gates]

Cairn SHALL pass proposal, design, and tasks gates for the deterministic surface coverage audit change.

#### Scenario: Change artifacts are gate-ready

r[cairn.supersede.determinism_surface_coverage_audit.gates]

- GIVEN proposal, design, tasks, and spec delta artifacts
- WHEN Cairn gates each stage
- THEN each gate receipt SHALL be valid.

### Requirement: Deterministic Surface Coverage Lifecycle r[cairn.supersede.determinism_surface_coverage_audit.archive]

Cairn SHALL sync and archive the deterministic surface coverage audit change after verification.

#### Scenario: Archived change retains review evidence

r[cairn.supersede.determinism_surface_coverage_audit.archive]

- GIVEN implementation and verification are complete
- WHEN the change is synced and archived
- THEN the accepted spec and archive SHALL retain traceable requirements and verification references.

### Requirement: Determinism Coverage Release Evidence

r[cairn.supersede.determinism_coverage_release_evidence.release]

Cairn MUST include deterministic surface coverage-audit evidence in release-readiness so agents and CI can prove that public deterministic surfaces had reviewed replay coverage at release time.

#### Scenario: Release-readiness includes coverage audit evidence

- GIVEN a workspace with generated policy and deterministic surface inventory
- WHEN an operator runs `cairn release-readiness --root .`
- THEN the release-readiness receipt includes stable coverage-audit validity and receipt hash fields
- AND the receipt remains deterministic, path-safe, and secret-safe

#### Scenario: Coverage audit failure blocks release-readiness

- GIVEN a policy state where deterministic surface coverage audit would fail
- WHEN release-readiness evaluates deterministic evidence
- THEN release-readiness reports a deterministic coverage-audit issue
- AND the failure does not expose raw checkout paths, environment values, or secrets

### Requirement: Determinism Coverage Bundle And Manifest Evidence

r[cairn.supersede.determinism_coverage_release_evidence.bundle_manifest]

Cairn SHOULD allow agent evidence bundles and evidence manifests to carry a coverage-audit artifact so durable CI evidence chains can prove the audit artifact that was published.

#### Scenario: Bundle verifies coverage audit artifact linkage

- GIVEN an agent evidence bundle with a coverage-audit artifact path and expected receipt hash
- WHEN an operator runs `cairn agent-receipt verify-bundle <bundle.json> --root .`
- THEN the bundle verification receipt validates the coverage audit artifact, records its receipt hash, and fails deterministically when the artifact is missing, failing, malformed, or hash-mismatched

#### Scenario: Evidence manifest verifies coverage audit artifact role

- GIVEN an evidence manifest that includes a coverage-audit artifact role
- WHEN an operator runs `cairn evidence-manifest verify <manifest.json> --root .`
- THEN the manifest verification receipt validates the coverage audit artifact hash and records the coverage-audit receipt hash
- AND generated manifests can include the coverage-audit role using canonical role ordering

### Requirement: Deterministic Coverage Evidence Failures

r[cairn.supersede.determinism_coverage_release_evidence.failures]

Coverage-audit evidence failures MUST use stable issue codes/classes and MUST NOT include raw artifact paths, absolute checkout roots, raw environment values, credentials, or secrets.

r[cairn.supersede.determinism_coverage_release_evidence.docs]

Cairn MUST document the coverage-audit evidence order for agent/operator workflows so wrappers run coverage audit before replay, bundle, manifest, and release-readiness evidence is trusted.

#### Scenario: Secret-safe coverage evidence errors

- GIVEN coverage-audit evidence paths or metadata containing secret-looking strings
- WHEN release-readiness, bundle verification, or manifest verification reports invalid coverage evidence
- THEN the emitted receipt uses deterministic issue codes and classes only
- AND the emitted receipt does not include the secret-looking input text

#### Scenario: Deterministic coverage evidence gates

- GIVEN the coverage evidence implementation is complete
- WHEN project gates, Tracey coverage, tests, coverage audit, and release-readiness run
r[cairn.supersede.determinism_coverage_release_evidence.gates]

- THEN all new requirements are referenced by implementation or verification evidence.

#### Scenario: Coverage evidence archive

- GIVEN implementation and verification tasks are complete
- WHEN the change is synced and archived
r[cairn.supersede.determinism_coverage_release_evidence.archive]

- THEN the archive preserves the accepted requirement evidence for later review.

### Requirement: Evidence Export Bundle Command

Cairn MUST provide a first-class evidence export command that materializes the canonical local evidence chain into a caller-provided output directory. r[cairn.supersede.evidence_export_bundle.command]

#### Scenario: Export creates complete evidence set

Given a repository with valid Cairn policy and agent receipt fixtures
When an operator runs `cairn evidence export --root . --out <dir>`
Then Cairn MUST write coverage audit, replay, replay verification, release-readiness, agent receipt, agent evidence bundle, bundle verification, evidence manifest, manifest verification, and export receipt artifacts under `<dir>`.

#### Scenario: Export composes existing validators

Given generated export artifacts
When Cairn builds the bundle, manifest, and export receipt
Then the export MUST use the same validation semantics as `determinism verify-receipt`, `agent-receipt verify-bundle`, and `evidence-manifest verify` rather than accepting wrapper-only assertions.

### Requirement: Evidence Export Artifact Manifest

Cairn MUST emit a deterministic export receipt that lists generated artifacts by stable role, relative path, and content hash. r[cairn.supersede.evidence_export_bundle.artifacts]

#### Scenario: Stable artifact roles and hashes

Given an evidence export completed successfully
When the export receipt is inspected
Then it MUST include stable artifact role entries sorted by role with relative paths and BLAKE3 content hashes, plus the evidence manifest hash, manifest verification receipt hash, and export receipt hash.

#### Scenario: Deterministic repeated exports

Given two evidence exports for equivalent repository content into different output directories
When host-specific root and output prefixes are normalized
Then the logical export receipt hashes and artifact role/hash mappings MUST match.

### Requirement: Evidence Export Failure Semantics and Documentation

Evidence export failures MUST be deterministic, path-safe, and documented as the preferred publication flow. r[cairn.supersede.evidence_export_bundle.failures]

#### Scenario: Safe failure receipt

Given export cannot read a required artifact, generate a receipt, or validate the generated evidence chain
When the command reports failure
Then the failure receipt MUST use stable issue codes/classes and MUST NOT include raw absolute artifact paths, raw environment values, credentials, or secrets.

#### Scenario: Documentation describes canonical publication flow

Given agent/operator documentation describes local evidence collection
When the export command is available
Then the docs MUST prefer `cairn evidence export --root . --out <dir>` for publication while preserving the lower-level receipt commands for debugging.

r[cairn.supersede.evidence_export_bundle.command]
r[cairn.supersede.evidence_export_bundle.artifacts]
r[cairn.supersede.evidence_export_bundle.failures]
r[cairn.supersede.evidence_export_bundle.docs]
r[cairn.supersede.evidence_export_bundle.gates]
r[cairn.supersede.evidence_export_bundle.archive]

### Requirement: Evidence Bundle Verify Command

Cairn MUST provide a first-class evidence bundle verification command that validates a directory produced by `cairn evidence export`. r[cairn.supersede.evidence_bundle_verify_command.command]

#### Scenario: Verify exported bundle

Given a directory produced by `cairn evidence export --root . --out <dir>`
When an operator runs `cairn evidence verify --root . --bundle <dir>`
Then Cairn MUST emit a deterministic verification receipt with `valid=true`, an empty `issues` array, the exported artifact roles, the export receipt hash, manifest hash, manifest verification receipt hash, and a receipt hash.

### Requirement: Evidence Bundle Artifact Verification

Cairn MUST rehash exported artifacts, reject unsafe artifact paths, require canonical exported roles, and compose existing evidence validators. r[cairn.supersede.evidence_bundle_verify_command.artifacts]

#### Scenario: Artifact hashes and nested verifiers match

Given an exported evidence bundle
When Cairn verifies the bundle
Then every declared artifact hash MUST match the file content
And required artifact roles MUST be present exactly once
And replay verification, bundle verification, manifest verification, coverage audit, and release-readiness receipts MUST be valid.

### Requirement: Evidence Bundle Verify Failure Semantics and Documentation

Evidence bundle verification failures MUST be deterministic, path-safe, secret-safe, and documented as the preferred consumer-side workflow. r[cairn.supersede.evidence_bundle_verify_command.failures]

#### Scenario: Safe failure receipt

Given an evidence bundle with a missing or modified artifact
When Cairn verifies the bundle
Then Cairn MUST emit `valid=false`
And the receipt MUST include stable issue codes without leaking absolute bundle paths or secret values.

#### Scenario: Consumer documentation

Given the export/verify workflow is available
When an agent or CI needs to publish and consume evidence
Then the docs MUST describe `cairn evidence export --root . --out <dir>` for producers and `cairn evidence verify --root . --bundle <dir>` for reviewers or CI consumers.

r[cairn.supersede.evidence_bundle_verify_command.command]
r[cairn.supersede.evidence_bundle_verify_command.artifacts]
r[cairn.supersede.evidence_bundle_verify_command.failures]
r[cairn.supersede.evidence_bundle_verify_command.docs]
r[cairn.supersede.evidence_bundle_verify_command.gates]
r[cairn.supersede.evidence_bundle_verify_command.archive]

### Requirement: Evidence Import Consume Rail

Cairn MUST provide a first-class evidence import command that verifies and materializes a deterministic local copy of an exported evidence bundle for downstream CI or reviewer consumption.

r[cairn.supersede.evidence_import_consume_rail.command]

#### Scenario: Import command verifies before copying

- GIVEN an evidence directory produced by `cairn evidence export`
- WHEN an operator runs `cairn evidence import --root . --bundle <dir> --into <dir>`
- THEN Cairn MUST first verify the bundle using the same deterministic checks as `cairn evidence verify`
- AND Cairn MUST emit a deterministic import receipt with the nested verification receipt hash.

r[cairn.supersede.evidence_import_consume_rail.artifacts]

#### Scenario: Import materializes declared artifacts only

- GIVEN a valid exported evidence bundle with an export receipt artifact index
- WHEN the bundle is imported into a local evidence directory
- THEN Cairn MUST copy only files declared by the export receipt artifact index
- AND Cairn MUST preserve declared relative artifact paths
- AND Cairn MUST record each imported artifact role, relative path, and BLAKE3 content hash in sorted order.

r[cairn.supersede.evidence_import_consume_rail.failures]

#### Scenario: Import rejects unsafe or undeclared bundle content

- GIVEN an exported evidence directory with a failed verifier, unsafe declared path, missing artifact, hash mismatch, or undeclared extra file
- WHEN an operator imports the bundle
- THEN Cairn MUST emit a deterministic failure receipt
- AND Cairn MUST use stable issue codes without leaking absolute bundle paths, raw host environment values, credentials, or secrets.

r[cairn.supersede.evidence_import_consume_rail.gates]

#### Scenario: Import receipt is portable across directories

- GIVEN the same valid evidence bundle imported into two distinct local directories
- WHEN Cairn emits import receipts for both imports
- THEN the portable receipt hashes MUST match
- AND the receipt MUST normalize bundle and destination output locations.

r[cairn.supersede.evidence_import_consume_rail.archive]

#### Scenario: Import rail is archived with deterministic evidence

- GIVEN the evidence import change is complete
- WHEN the change is synced and archived
- THEN the accepted spec, archive, tests, and docs MUST retain requirement traceability for the import command, artifact rules, deterministic failures, verification gates, and completion evidence.

### Requirement: Evidence Import Republish Rail

Cairn MUST provide a first-class evidence republish command that verifies an imported evidence directory and materializes a deterministic re-published evidence bundle.

r[cairn.supersede.evidence_import_republish_rail.command]

#### Scenario: Republish command verifies imported evidence

- GIVEN an evidence directory produced by `cairn evidence import`
- WHEN an operator runs `cairn evidence republish --root . --imported <dir> --out <dir>`
- THEN Cairn MUST verify the imported evidence before copying artifacts
- AND Cairn MUST write a deterministic `evidence-republish-receipt.json` into the output directory
- AND the receipt MUST use stable placeholders for input and output paths.

r[cairn.supersede.evidence_import_republish_rail.artifacts]

#### Scenario: Republish materializes declared imported artifacts only

- GIVEN an imported evidence directory with `evidence-import-receipt.json`
- WHEN Cairn republishes the evidence
- THEN Cairn MUST copy only artifact paths declared by the import receipt
- AND Cairn MUST preserve each declared artifact role, relative path, and hash
- AND Cairn MUST bind the republish receipt to the import receipt hash.

r[cairn.supersede.evidence_import_republish_rail.failures]

#### Scenario: Republish rejects invalid imported evidence

- GIVEN missing, tampered, duplicate, extra, unsafe, or path-traversing imported artifacts
- WHEN republish is attempted
- THEN Cairn MUST refuse to materialize a successful republish
- AND Cairn MUST emit deterministic issue codes without leaking host-specific paths, environment values, credentials, or secrets.

r[cairn.supersede.evidence_import_republish_rail.gates]

#### Scenario: Republish rail gates pass

- GIVEN the republish rail is implemented
- WHEN final project gates run
- THEN policy export checks, Rust tests, clippy, Cairn validation, Tracey coverage, coverage audit, release-readiness, evidence export, evidence verify, evidence import, evidence republish, whitespace checks, and Nix flake checks MUST pass.

r[cairn.supersede.evidence_import_republish_rail.archive]

#### Scenario: Republish rail is archived with deterministic evidence

- GIVEN the republish rail has passed final gates
- WHEN the change is completed
- THEN the change MUST be synced into accepted specs, archived with the current date, committed, and pushed.

### Requirement: Evidence Publish Index Rail

Cairn MUST provide first-class deterministic evidence index generation and verification commands for published evidence bundles.

r[cairn.supersede.evidence_publish_index_rail.command]

#### Scenario: Index command catalogs a bundle

- GIVEN an evidence bundle produced by export, import, or republish
- WHEN an operator runs `cairn evidence index --root . --bundle <dir> --out <file>`
- THEN Cairn MUST detect the bundle kind deterministically
- AND Cairn MUST write a portable index receipt to the requested output file
- AND the index MUST include artifact roles, relative paths, BLAKE3 hashes, and bound receipt-chain hashes.

r[cairn.supersede.evidence_publish_index_rail.verify]

#### Scenario: Index verify rechecks indexed evidence

- GIVEN an evidence index file
- WHEN an operator runs `cairn evidence index verify --root . --index <file>`
- THEN Cairn MUST validate the index self hash
- AND Cairn MUST re-detect and re-hash the referenced bundle
- AND Cairn MUST compare the current bundle kind, artifacts, and receipt-chain hashes with the index.

r[cairn.supersede.evidence_publish_index_rail.failures]

#### Scenario: Index commands reject ambiguous or tampered evidence

- GIVEN missing, ambiguous, unsafe, tampered, or path-traversing bundle/index inputs
- WHEN index generation or verification is attempted
- THEN Cairn MUST emit deterministic failure receipts
- AND Cairn MUST use stable issue codes without leaking host paths, raw environment values, credentials, or secrets.

r[cairn.supersede.evidence_publish_index_rail.gates]

#### Scenario: Index rail gates pass

- GIVEN the index rail is implemented
- WHEN final project gates run
- THEN policy export checks, Rust tests, clippy, Cairn validation, Tracey coverage, coverage audit, release-readiness, evidence export, evidence verify, evidence import, evidence republish, evidence index, evidence index verify, whitespace checks, and Nix flake checks MUST pass.

r[cairn.supersede.evidence_publish_index_rail.archive]

#### Scenario: Index rail is archived with deterministic evidence

- GIVEN the index rail has passed final gates
- WHEN the change is completed
- THEN the change MUST be synced into accepted specs, archived with the current date, committed, and pushed.

### Requirement: Evidence Registry Rail

Cairn MUST provide first-class deterministic evidence registry build and verification commands for aggregating published evidence indexes.

r[cairn.supersede.evidence_registry_rail.build]

#### Scenario: Registry build aggregates indexes

- GIVEN one or more deterministic evidence index receipt files
- WHEN an operator runs `cairn evidence registry build --root . --index <file> --out <file>`
- THEN Cairn emits a registry receipt with stable command, root, out, validity, verdict, issues, sorted entries, entry count, index hashes, bundle kinds, bundle receipt hashes, receipt chains, artifact hashes, and receipt hash fields

r[cairn.supersede.evidence_registry_rail.verify]

#### Scenario: Registry verification recomputes indexed evidence

- GIVEN a saved evidence registry receipt
- WHEN an operator runs `cairn evidence registry verify --root . --registry <file>`
- THEN Cairn recomputes every referenced index, validates the registry hash, compares each entry with current index evidence, and emits a deterministic verification receipt

r[cairn.supersede.evidence_registry_rail.failures]

#### Scenario: Registry commands reject unsafe or stale evidence

- GIVEN missing indexes, duplicate index hashes, malformed registry content, stale index entries, unsafe paths, or tampered artifacts behind an index
- WHEN registry build or verify runs
- THEN Cairn fails with deterministic issue codes and does not leak raw host paths, environment values, credentials, or secrets

r[cairn.supersede.evidence_registry_rail.gates]

#### Scenario: Registry rail gates pass

- GIVEN the registry implementation, policy schemas, docs, tests, and fixtures
- WHEN the required verification gates run
- THEN policy export, formatting, tests, clippy, validation, Tracey, coverage audit, release-readiness, evidence smoke commands, whitespace checks, and Nix flake checks pass

r[cairn.supersede.evidence_registry_rail.archive]

#### Scenario: Registry rail is archived with deterministic evidence

- GIVEN the registry rail tasks are complete
- WHEN the change is synced and archived
- THEN the accepted specification and dated archive retain the registry requirements, scenarios, and verification references

### Requirement: Rust/Nickel policy export gate
r[cairn.supersede.rust_nickel_only_gates.policy_export]
Cairn SHALL expose policy export and freshness validation through a Rust CLI command that uses Nickel only for Nickel policy evaluation.

#### Scenario: Policy export freshness is checked without Python helpers
GIVEN a Cairn repository with checked-in generated policy
WHEN an operator runs `cairn policy export --check`
THEN the command SHALL render Nickel policy, validate semantic invariants in Rust, compare the generated JSON, and exit nonzero when the generated policy is stale.

### Requirement: Rust/Nickel traceability coverage gate
r[cairn.supersede.rust_nickel_only_gates.tracey_coverage]
Cairn SHALL expose accepted requirement coverage checking through a Rust CLI command.

#### Scenario: Tracey bootstrap coverage is enforced without Python helpers
GIVEN accepted specs with `r[...]` requirement markers
WHEN an operator runs `cairn tracey coverage --root .`
THEN the command SHALL require each accepted requirement to have an implementation or verification reference in Rust or Nickel source and SHALL reject missing or dangling references deterministically.

### Requirement: Rust rename classification gate
r[cairn.supersede.rust_nickel_only_gates.rename_classification]
Cairn SHALL expose legacy rename classification through a Rust CLI command.

#### Scenario: Rename classification rejects unresolved legacy usage
GIVEN source-controlled Cairn text files
WHEN an operator runs `cairn rename-classification`
THEN the command SHALL classify legacy Ironspec occurrences and exit nonzero if any occurrence remains in the `needs-fix` bucket.

### Requirement: No checked-in Python or shell helper scripts
r[cairn.supersede.rust_nickel_only_gates.no_python_scripts]
Cairn SHALL NOT rely on checked-in Python or shell helper scripts for maintained gates.

#### Scenario: Repository gate helpers are Rust or Nickel
GIVEN the source-controlled Cairn repository
WHEN an operator lists maintained helper scripts and Nix check dependencies
THEN checked-in Python and shell helper scripts SHALL be absent and maintained checks SHALL use Rust CLI surfaces and Nickel policy instead.

### Requirement: Strict validation exit mode
r[cairn.supersede.rust_nickel_only_gates.gates]
Cairn SHOULD provide a strict validation mode for CI orchestration.

#### Scenario: Validation can fail by exit code
GIVEN a Cairn repository with validation issues
WHEN an operator runs `cairn validate --strict --root PATH`
THEN the command SHALL emit the validation receipt and exit nonzero when `valid` is false.
