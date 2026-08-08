# Change Dependency Graph Specification

## Purpose

Adds explicit cross-repo dependency declarations for Cairn change packages as receipt-linked evidence dependencies, separate from same-repo `depends_on` ordering.

## ADDED Requirements

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
