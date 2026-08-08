# Change Dependency Graph Specification

## Purpose

Defines the `change-dependency-graph` capability.

## Requirements

### Requirement: Change dependency metadata
r[cairn.change_dependency_graph.metadata] Cairn MUST support optional change-package dependency declarations in change metadata without making dependencies mandatory for ordinary changes.

#### Scenario: Known dependencies are accepted
r[cairn.change_dependency_graph.metadata.known]
- GIVEN an active change metadata file declares `depends_on` entries for known active or archived change identifiers
- WHEN Cairn validates or reports the repository lifecycle state
- THEN the dependency declarations MUST be preserved in deterministic receipts.

#### Scenario: Malformed dependency metadata fails closed
r[cairn.change_dependency_graph.metadata.invalid]
- GIVEN a change metadata file declares a non-array `depends_on`, an empty dependency id, an invalid change id, an unknown change id, a duplicate dependency, or the change itself
- WHEN Cairn validates or reports the repository lifecycle state
- THEN Cairn MUST emit deterministic dependency issue codes and MUST NOT silently ignore the bad edge.

### Requirement: Pure dependency graph validation
r[cairn.change_dependency_graph.graph_validation] Cairn MUST build and validate the change dependency graph in pure deterministic core logic over in-memory lifecycle data.

#### Scenario: Acyclic dependency graph has stable order
r[cairn.change_dependency_graph.graph_validation.acyclic]
- GIVEN active changes with acyclic dependency declarations
- WHEN Cairn evaluates the dependency graph
- THEN the graph receipt MUST include stable nodes, dependency edges, reverse `blocks` edges, and dependency-first topological order.

#### Scenario: Cycles are rejected
r[cairn.change_dependency_graph.graph_validation.cycle]
- GIVEN active changes whose dependency declarations form a cycle
- WHEN Cairn evaluates the dependency graph
- THEN Cairn MUST report the cycle as invalid and MUST NOT emit the cyclic changes as drain-ready.

### Requirement: Trellis graph primitives guide implementation
r[cairn.change_dependency_graph.trellis_primitives] When Cairn introduces dependency graph algorithms, it SHOULD use the corresponding Trellis graph and scheduling primitives as the model source for topological order, cycle detection, and dependency readiness.

#### Scenario: Trellis primitive boundary remains portable
r[cairn.change_dependency_graph.trellis_primitives.portable]
- GIVEN Cairn implementation uses ideas or code derived from `../trellis/src/topo_sort.rs`, `../trellis/src/cycle_detect.rs`, or `../trellis/src/job_dag.rs`
- WHEN Cairn builds, validates, and releases from a standalone checkout
- THEN normal Cairn runtime builds MUST remain self-contained and MUST NOT require a sibling `../trellis` checkout.

### Requirement: Dependency-aware drain planning
r[cairn.change_dependency_graph.drain_order] Cairn MUST include change dependency graph readiness in read-only drain planning receipts.

#### Scenario: Drain plan orders dependencies first
r[cairn.change_dependency_graph.drain_order.topological]
- GIVEN active changes where one change depends on another active change
- WHEN the operator requests a repository drain plan
- THEN the receipt MUST place the dependency before the dependent in `drain_order` and MUST show the dependent as blocked by the dependency.

#### Scenario: Archived dependencies satisfy blockers
r[cairn.change_dependency_graph.drain_order.archived]
- GIVEN a change depends only on archived changes
- WHEN the operator requests a repository drain plan
- THEN those dependencies MUST be reported as satisfied and MUST NOT block the dependent change's drain readiness.

### Requirement: Dependency blockers protect mutation
r[cairn.change_dependency_graph.mutation_block] Cairn MUST prevent explicit lifecycle mutation for a change while declared dependencies remain unsatisfied.

#### Scenario: Sync and archive execution block on unsatisfied dependencies
r[cairn.change_dependency_graph.mutation_block.unsatisfied]
- GIVEN a change depends on another change that is not archived
- WHEN the operator runs sync or archive with `--execute` for the dependent change
- THEN Cairn MUST refuse mutation, report the unsatisfied dependency blocker, and preserve the existing files unchanged.

#### Scenario: Dry-run plans show dependency blockers
r[cairn.change_dependency_graph.mutation_block.dry_run]
- GIVEN a change has unsatisfied dependencies
- WHEN the operator runs sync or archive without `--execute`
- THEN Cairn MUST report the dependency blockers in the dry-run plan without mutating files.

### Requirement: Positive and negative graph coverage
r[cairn.change_dependency_graph.positive_negative_tests] Cairn MUST verify dependency graph behavior with positive and negative tests across core and CLI surfaces.

#### Scenario: Positive graph cases pass
r[cairn.change_dependency_graph.positive_negative_tests.positive]
- GIVEN changes with no dependencies, archived dependencies, or active acyclic dependencies
- WHEN focused graph and drain tests run
- THEN the tests MUST prove deterministic receipts and expected readiness/order behavior.

#### Scenario: Negative graph cases fail correctly
r[cairn.change_dependency_graph.positive_negative_tests.negative]
- GIVEN changes with unknown dependencies, self dependencies, duplicate dependencies, invalid metadata, or cycles
- WHEN focused graph and drain tests run
- THEN the tests MUST prove Cairn reports the expected issue codes and does not panic or mutate repository state.

### Requirement: Dependency graph verification rail
r[cairn.change_dependency_graph.validation] Cairn MUST include dependency graph checks in the verification rail for this feature.

#### Scenario: Repository checks cover graph behavior
r[cairn.change_dependency_graph.validation.repository_checks]
- GIVEN the dependency graph feature is implemented
- WHEN focused tests, Cairn validation, gates, and Tracey coverage run
- THEN the verification evidence MUST cover metadata parsing, graph validation, drain planning, mutation blockers, and the Trellis primitive boundary.

### Requirement: Dependency authoring CLI
r[cairn.change_dependency_graph.authoring_cli] Cairn MUST expose local change dependency authoring commands that list, add, and remove `depends_on` metadata without requiring operators to hand-edit `metadata.json`.

#### Scenario: Dependency list is deterministic
r[cairn.change_dependency_graph.authoring_cli.list]
- GIVEN a change declares local dependencies in metadata
- WHEN the operator runs the dependency list command
- THEN Cairn MUST emit the dependencies in deterministic change-id order and MUST NOT mutate lifecycle files.

#### Scenario: Dependency add previews by default
r[cairn.change_dependency_graph.authoring_cli.preview]
- GIVEN an operator requests adding a valid dependency between known changes
- WHEN the command runs without `--execute`
- THEN Cairn MUST emit a dry-run receipt showing the proposed metadata change and MUST leave files unchanged.

#### Scenario: Dependency add executes explicitly
r[cairn.change_dependency_graph.authoring_cli.execute]
- GIVEN an operator requests adding a valid dependency between known changes
- WHEN the command runs with `--execute`
- THEN Cairn MUST write deterministic metadata preserving unrelated supported fields and sorted unique `depends_on` entries.

#### Scenario: Dependency remove clears empty field
r[cairn.change_dependency_graph.authoring_cli.remove]
- GIVEN a change has one declared dependency
- WHEN the operator removes that dependency with `--execute`
- THEN Cairn MUST remove the `depends_on` field rather than writing an empty dependency array.

#### Scenario: Invalid dependency edit fails closed
r[cairn.change_dependency_graph.authoring_cli.invalid]
- GIVEN a requested dependency edit references an invalid change id, an unknown change, the same change, a duplicate dependency input, or a cycle-forming edge
- WHEN the operator previews or executes the edit
- THEN Cairn MUST reject the edit with deterministic diagnostics and MUST NOT mutate lifecycle files.

### Requirement: Human graph render formats
r[cairn.change_dependency_graph.render_formats] Cairn MUST keep the dependency graph JSON receipt as the automation default and MAY expose stable human-oriented graph render formats for review and documentation.

#### Scenario: JSON graph remains default
r[cairn.change_dependency_graph.render_formats.json_default]
- GIVEN an operator runs the existing change graph command without a format override
- WHEN Cairn renders the graph
- THEN the output MUST remain the deterministic JSON graph receipt.

#### Scenario: Mermaid and DOT outputs are stable
r[cairn.change_dependency_graph.render_formats.human]
- GIVEN an operator requests Mermaid or DOT graph output
- WHEN Cairn renders the graph
- THEN the output MUST sort nodes and edges deterministically and label active blockers separately from archived satisfied dependencies.

#### Scenario: Unknown format fails closed
r[cairn.change_dependency_graph.render_formats.invalid]
- GIVEN an operator requests an unsupported graph render format
- WHEN Cairn parses the command
- THEN Cairn MUST reject the request with a deterministic error and MUST NOT fall back to another format silently.

### Requirement: Next-ready change discovery
r[cairn.change_dependency_graph.next_ready] Cairn MUST expose a read-only command that identifies the next dependency-ready active change from the dependency-first drain order.

#### Scenario: Ready change is reported first
r[cairn.change_dependency_graph.next_ready.ready]
- GIVEN at least one active change has no unsatisfied dependency blockers or dependency issues
- WHEN the operator asks for the next change
- THEN Cairn MUST report the first dependency-ready change in deterministic drain order.

#### Scenario: Blocked repository explains blockers
r[cairn.change_dependency_graph.next_ready.blocked]
- GIVEN no active change is dependency-ready
- WHEN the operator asks for the next change
- THEN Cairn MUST report that no change is ready and MUST include deterministic blockers for blocked changes.

### Requirement: Dependency discoverability docs
r[cairn.change_dependency_graph.docs.discoverability] Cairn documentation MUST explain how to author local dependencies, inspect graph order, and drain active changes safely.

#### Scenario: Documentation covers local dependency semantics
r[cairn.change_dependency_graph.docs.discoverability.local]
- GIVEN an operator reads the local lifecycle documentation
- WHEN dependency graph behavior is described
- THEN the documentation MUST show `depends_on` metadata, authoring commands, graph rendering commands, active dependency blockers, archived dependency satisfaction, and cycle rejection.

#### Scenario: Documentation distinguishes evidence dependencies
r[cairn.change_dependency_graph.docs.discoverability.boundary]
- GIVEN cross-repo evidence dependencies are also documented
- WHEN local dependency graph behavior is described
- THEN the documentation MUST distinguish local ordering dependencies from cross-repo receipt/evidence dependencies.

### Requirement: Dependency authoring verification
r[cairn.change_dependency_graph.authoring_cli.validation] Cairn MUST verify dependency authoring behavior with positive and negative tests across pure core and CLI surfaces.

#### Scenario: Positive authoring tests pass
r[cairn.change_dependency_graph.authoring_cli.tests.positive]
- GIVEN dependency add, list, remove, render, archived-satisfaction, and next-ready happy paths
- WHEN focused tests run
- THEN the tests MUST prove deterministic receipts, sorted metadata, stable render output, and expected readiness behavior.

#### Scenario: Negative authoring tests fail correctly
r[cairn.change_dependency_graph.authoring_cli.tests.negative]
- GIVEN malformed metadata, unknown dependencies, self-dependencies, duplicate dependency input, cycle-forming edits, unsupported render formats, and blocked mutation attempts
- WHEN focused tests run
- THEN the tests MUST prove Cairn reports expected diagnostics, does not panic, and preserves existing lifecycle files.

### Requirement: Cross-repo dependency metadata contract
r[cairn.cross_repo_change_dependencies.contract] Cairn MUST support optional cross-repo dependency rows in change metadata without treating upstream repository changes as local graph nodes.

#### Scenario: Dependency row records upstream identity
r[cairn.cross_repo_change_dependencies.contract.identity]
- GIVEN a change declares a cross-repo dependency row
- WHEN Cairn parses the row
- THEN the row MUST preserve a stable dependency id, upstream repo identity, upstream change id, required receipt kind, expected status, optional policy hash constraint, optional receipt hash constraint, and explicit non-claim labels.

#### Scenario: Local and cross-repo dependencies remain separate
r[cairn.cross_repo_change_dependencies.contract.local_boundary]
- GIVEN a change declares both local `depends_on` metadata and `cross_repo_dependencies` metadata
- WHEN Cairn evaluates the change dependency graph
- THEN local dependencies MUST control same-repo drain ordering and cross-repo dependencies MUST be validated as evidence blockers without adding remote nodes to the local graph.

#### Scenario: Malformed metadata fails closed
r[cairn.cross_repo_change_dependencies.contract.invalid]
- GIVEN a cross-repo dependency row is missing required identity fields, uses an unsupported receipt kind, declares an invalid expected status, or contains malformed hash constraints
- WHEN Cairn validates the change metadata
- THEN Cairn MUST emit deterministic issue codes and MUST NOT silently ignore the bad row.

### Requirement: Cross-repo dependency fixtures
r[cairn.cross_repo_change_dependencies.fixtures] Cairn MUST include positive and negative fixtures that cover supported and rejected cross-repo dependency states.

#### Scenario: Satisfied upstream receipts pass
r[cairn.cross_repo_change_dependencies.fixtures.positive]
- GIVEN fixtures for matching upstream proposal gate, design gate, tasks gate, validation, sync, and archive receipts
- WHEN cross-repo dependency validation runs
- THEN each fixture MUST satisfy the matching dependency row and preserve upstream repo, change, receipt kind, status, policy hash, and receipt hash identity.

#### Scenario: Bad upstream receipts fail closed
r[cairn.cross_repo_change_dependencies.fixtures.negative]
- GIVEN fixtures for every supported non-pass status, missing receipts, stale-policy or stale-subject receipts, wrong-repo receipts, wrong-change receipts, wrong-kind receipts, unsupported receipt kinds, unsafe diagnostics, and receipt-hash mismatches
- WHEN cross-repo dependency validation runs
- THEN each fixture MUST fail with deterministic diagnostics naming the dependency row and failure class.

### Requirement: Cross-repo dependency validation core
r[cairn.cross_repo_change_dependencies.validation] Cairn MUST validate cross-repo dependencies in pure deterministic core logic over normalized dependency rows and normalized upstream receipt summaries.

#### Scenario: Matching evidence satisfies dependency
r[cairn.cross_repo_change_dependencies.validation.match]
- GIVEN a dependency row and a supplied upstream receipt summary with matching repo identity, change id, required receipt kind, expected status, policy hash when constrained, and receipt hash when constrained
- WHEN the pure validator evaluates the row
- THEN the row status MUST be `satisfied` and MUST include no blocker issue.

#### Scenario: Mismatched evidence is blocked
r[cairn.cross_repo_change_dependencies.validation.mismatch]
- GIVEN a dependency row and supplied evidence that is missing, failed, stale, from a different repo, from a different change, of the wrong kind, unsupported, or hash-mismatched
- WHEN the pure validator evaluates the row
- THEN the row status MUST be a deterministic blocked status and MUST include a stable issue code.

#### Scenario: Validation receipt binds inputs
r[cairn.cross_repo_change_dependencies.validation.receipt]
- GIVEN dependency rows, supplied upstream receipt summaries, and policy identity
- WHEN cross-repo dependency validation runs
- THEN Cairn MUST emit a deterministic receipt containing dependency count, satisfied count, blocked count, row statuses, issue codes, input hash, policy hash, and receipt hash.

### Requirement: Explicit evidence loading shell
r[cairn.cross_repo_change_dependencies.shell] Cairn CLI and MCP surfaces MUST resolve cross-repo dependency evidence only from explicit operator-supplied receipt files or imported evidence bundles.

#### Scenario: Explicit evidence input is summarized
r[cairn.cross_repo_change_dependencies.shell.explicit]
- GIVEN an operator supplies upstream Cairn receipt files or an imported evidence bundle
- WHEN the shell prepares cross-repo dependency validation
- THEN it MUST load bounded receipt summaries and pass normalized in-memory summaries to the core validator.

#### Scenario: No implicit remote fetch occurs
r[cairn.cross_repo_change_dependencies.shell.no_fetch]
- GIVEN a dependency row names a sibling repo or upstream project
- WHEN the operator does not supply evidence inputs
- THEN Cairn MUST NOT fetch remotes, inspect sibling checkouts, or run upstream commands by default and MUST report the dependency as missing evidence.

### Requirement: Cross-repo dependency blockers
r[cairn.cross_repo_change_dependencies.blockers] Cairn MUST surface unsatisfied cross-repo dependencies as blockers in lifecycle validation and planning receipts without mutating lifecycle files.

#### Scenario: Validation reports blockers
r[cairn.cross_repo_change_dependencies.blockers.validation]
- GIVEN a change has an unsatisfied cross-repo dependency
- WHEN Cairn validates the repository
- THEN validation MUST report the dependency blocker with the dependency id, upstream repo, upstream change, required receipt kind, and issue code.

#### Scenario: Planning reports blockers
r[cairn.cross_repo_change_dependencies.blockers.planning]
- GIVEN a change has an unsatisfied cross-repo dependency
- WHEN Cairn builds drain or release-readiness summaries
- THEN the summaries MUST include the dependency blocker and MUST NOT mark the change as ready solely because local same-repo dependencies are satisfied.

### Requirement: Cross-repo evidence is pass-only and actionable
r[cairn.cross_repo_change_dependencies.status_feedback] Cairn MUST satisfy a cross-repo dependency only from explicit current passing evidence that matches every declared constraint, and MUST return bounded actionable diagnostics for every non-passing row.

#### Scenario: Current passing evidence satisfies the row
r[cairn.cross_repo_change_dependencies.status_feedback.pass]
- GIVEN a supplied supported receipt explicitly passes and matches the dependency's repo, change, receipt kind, stage, policy hash, subject hash when constrained, and receipt hash when constrained
- WHEN Cairn validates the dependency
- THEN the row MAY be `satisfied`
- AND evidence loading or transport success alone MUST NOT determine the status.

#### Scenario: Every non-pass state remains blocked
r[cairn.cross_repo_change_dependencies.status_feedback.non_pass]
- GIVEN supplied evidence is failed, blocked, skipped, deferred, timed-out, dry-run, cancelled, infrastructure-failed, malformed, unsupported, unknown, stale, or mismatched
- WHEN Cairn validates the dependency
- THEN the row MUST retain a deterministic blocked status and issue code
- AND no drain, readiness, or completion summary may promote it to satisfied.

#### Scenario: Blocker diagnostic supports repair
r[cairn.cross_repo_change_dependencies.status_feedback.diagnostic]
- GIVEN a dependency row is blocked
- WHEN Cairn renders its diagnostic
- THEN it MUST include stable dependency and diagnostic IDs, exact status, expected/actual safe identities and hashes, severity, bounded remediation, explicit evidence-input or reproduction guidance, safe artifact refs when present, and non-claims
- AND it MUST NOT include credentials, raw receipt payloads, environment values, absolute private paths, or unbounded logs.

### Requirement: Dependency evidence documentation
r[cairn.cross_repo_change_dependencies.docs] Cairn documentation MUST explain cross-repo dependency metadata, evidence inputs, supported receipt kinds, pass-only status semantics, actionable diagnostics, and non-claim boundaries.

#### Scenario: Metadata example is visible
r[cairn.cross_repo_change_dependencies.docs.metadata]
- GIVEN an operator reads the dependency documentation
- WHEN cross-repo dependencies are introduced
- THEN the documentation MUST show a complete metadata example with repo identity, change id, required receipt kind, expected status, optional receipt hash, and non-claims.

#### Scenario: Non-claim is visible
r[cairn.cross_repo_change_dependencies.docs.non_claims]
- GIVEN a dependency is satisfied
- WHEN the supported claim is rendered or documented
- THEN Cairn MUST state that dependency satisfaction proves presented receipt linkage only and does not prove upstream correctness, remote trust, downstream integration correctness, or release eligibility by itself.

#### Scenario: Local dependency boundary is visible
r[cairn.cross_repo_change_dependencies.docs.local_boundary]
- GIVEN local `depends_on` dependencies and cross-repo dependencies both exist
- WHEN Cairn documents dependency behavior
- THEN the documentation MUST distinguish same-repo drain-order dependencies from cross-repo receipt/evidence dependencies.

### Requirement: Final validation rail
r[cairn.cross_repo_change_dependencies.final_validation] The change MUST include positive and negative verification evidence before archive.

#### Scenario: Positive validation passes
r[cairn.cross_repo_change_dependencies.final_validation.positive]
- GIVEN matching upstream receipt fixtures for each supported receipt kind
- WHEN focused core and CLI tests run
- THEN valid fixtures MUST pass and produce deterministic satisfied-row receipts.

#### Scenario: Negative validation fails correctly
r[cairn.cross_repo_change_dependencies.final_validation.negative]
- GIVEN every non-pass status, missing, stale-policy, stale-subject, wrong-repo, wrong-change, wrong-kind, unsupported-kind, unsafe-diagnostic, and hash-mismatch fixtures
- WHEN focused core and CLI tests run
- THEN invalid fixtures MUST fail closed with expected issue codes and MUST NOT mutate lifecycle files.

#### Scenario: Repository gates cover the feature
r[cairn.cross_repo_change_dependencies.final_validation.gates]
- GIVEN implementation and documentation are complete
- WHEN Cairn validation, proposal gate, design gate, tasks gate, focused tests, and release-readiness checks run
- THEN the evidence MUST show this change is ready to sync and archive.

### Requirement: Radicle issue receipt kind is supported

r[cairn.radicle_issue_cross_repo.receipt_kind] Cairn MUST accept the `radicle-issue.v1` cross-repo dependency receipt kind in the same allowlist as `proposal_gate`, `design_gate`, `tasks_gate`, `validation_receipt`, `sync_receipt`, and `archive_receipt`.

#### Scenario: Issue linkage with a matching receipt passes

r[cairn.radicle_issue_cross_repo.receipt_kind.valid]
- GIVEN a change declares a cross-repo dependency row with receipt kind `radicle-issue.v1` and an explicit matching issue receipt
- WHEN Cairn validates the dependency
- THEN the row MUST be `satisfied` and MUST preserve the repo identity, issue id, receipt kind, status, and hash constraints.

#### Scenario: Unsupported issue evidence kind fails closed

r[cairn.radicle_issue_cross_repo.receipt_kind.invalid]
- GIVEN a dependency row uses an unsupported issue kind or a malformed issue receipt
- WHEN Cairn validates the dependency
- THEN Cairn MUST emit the stable unsupported-kind issue code and MUST NOT promote the row to `satisfied`.

### Requirement: Issue receipt contract is typed and bounded

r[cairn.radicle_issue_cross_repo.contract] Cairn MUST define a typed `cairn.radicle-issue-receipt.v1` contract that records the upstream repo identity, issue id, COB object id, observed status, and subject hash constraints.

#### Scenario: Receipt binds issue identity and observed state

r[cairn.radicle_issue_cross_repo.contract.identity]
- GIVEN a bounded typed issue observation for one Radicle issue
- WHEN Cairn builds the issue receipt in pure core
- THEN the receipt MUST bind the repo identity, issue id, COB object id, observed status, subject hash, and receipt hash deterministically.

#### Scenario: Issued receipt omits raw private transport data

r[cairn.radicle_issue_cross_repo.contract.privacy]
- GIVEN a bounded issue receipt is built
- WHEN Cairn renders or stores it
- THEN the receipt MUST NOT include credentials, raw action payloads, environment values, or unbounded logs.

### Requirement: Issue reading is a bounded read-only adapter

r[cairn.radicle_issue_cross_repo.adapter_bounded] Cairn MUST build a `radicle-issue.v1` receipt only from a bounded typed issue observation. Cairn MUST NOT reduce the issue into lifecycle state or author an update to it.

#### Scenario: Adapter emits only a bounded typed observation

r[cairn.radicle_issue_cross_repo.adapter_bounded.observe]
- GIVEN a bounded typed issue observation
- WHEN Cairn builds the issue receipt
- THEN Cairn MUST emit only the bounded typed receipt value and MUST NOT mutate any COB or lifecycle file.

#### Scenario: Issue receipt carries explicit non-claims

r[cairn.radicle_issue_cross_repo.adapter_bounded.non_claims]
- GIVEN an issue receipt is produced
- WHEN Cairn documents or renders the supported claim
- THEN the receipt MUST state that it proves bounded local observation of one issue only and does not prove closure authority, peer convergence, remote trust, semantic truth, or release eligibility.

### Requirement: Issue evidence validation is pure core

r[cairn.radicle_issue_cross_repo.validation] Cairn MUST validate `radicle-issue.v1` dependency rows and normalized issue receipt summaries in pure deterministic core logic.

#### Scenario: Matching issue evidence is satisfied

r[cairn.radicle_issue_cross_repo.validation.match]
- GIVEN an issue dependency row and a supplied issue receipt with matching repo, issue id, kind, expected status, and hash constraints
- WHEN the pure cross-repo evaluator runs
- THEN the row status MUST be `satisfied` with no blocker issue.

#### Scenario: Mismatched issue evidence is blocked

r[cairn.radicle_issue_cross_repo.validation.mismatch]
- GIVEN an issue dependency row and missing, stale, wrong-repo, wrong-issue, wrong-kind, unsupported, or hash-mismatched evidence
- WHEN the pure cross-repo evaluator runs
- THEN the row MUST retain a deterministic blocked status with a stable issue code.

### Requirement: Issue linkage final verification

r[cairn.radicle_issue_cross_repo.final_validation] The change MUST include positive and negative verification evidence before archive.

#### Scenario: Positive issue fixtures pass

r[cairn.radicle_issue_cross_repo.final_validation.positive]
- GIVEN a matching issue receipt fixture for the supported kind
- WHEN focused core and CLI tests run
- THEN valid fixtures MUST produce deterministic satisfied-row receipts.

#### Scenario: Negative issue fixtures fail closed

r[cairn.radicle_issue_cross_repo.final_validation.negative]
- GIVEN missing, stale, wrong-repo, wrong-issue, wrong-kind, unsupported-kind, malformed, and hash-mismatch issue fixtures
- WHEN focused core and CLI tests run
- THEN invalid fixtures MUST fail closed with expected issue codes and MUST NOT mutate lifecycle files.

#### Scenario: Repository gates cover the feature

r[cairn.radicle_issue_cross_repo.final_validation.gates]
- GIVEN implementation and documentation are complete
- WHEN Cairn validation, proposal gate, design gate, tasks gate, focused tests, and release-readiness checks run
- THEN the evidence MUST show this change is ready to sync and archive.
