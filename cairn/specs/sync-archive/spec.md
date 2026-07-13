# Sync Archive Specification

## Purpose

Defines deterministic dry-run planning for `cairn sync` and `cairn archive`.

## Requirements

### Requirement: Sync dry-run plan
r[sync_archive.plan.sync] The system MUST compute a deterministic sync dry-run plan for a named change without mutating repository files.

#### Scenario: Sync plan reports delta specs
r[sync_archive.plan.sync.delta_specs]
- GIVEN a change with delta specs
- WHEN the operator runs a sync dry-run
- THEN the output reports the change name, dry-run status, mutation status, planned actions, and delta spec paths

### Requirement: Archive dry-run plan
r[sync_archive.plan.archive] The system MUST compute a deterministic archive dry-run plan for a named change without mutating repository files.

#### Scenario: Archive plan reports blockers
r[sync_archive.plan.archive.blockers]
- GIVEN a change that is not ready to archive
- WHEN the operator runs an archive dry-run
- THEN the output reports the change name, dry-run status, mutation status, and blocking reasons

### Requirement: Plan core purity
r[sync_archive.plan.purity] Sync and archive planning MUST remain in the pure core boundary and consume shell-supplied change/policy snapshots.

#### Scenario: Planning has no shell effects
r[sync_archive.plan.purity.no_effects]
- GIVEN sync/archive planning inputs
- WHEN the core plan builder runs
- THEN it does not read files, inspect environment variables, spawn processes, or render JSON

### Requirement: Sync plan execution

r[sync_archive.execute.sync] The system MUST execute a sync plan only when the caller explicitly requests non-dry-run execution, the computed plan is unblocked, and delta specs are merged into accepted specs without replacing accepted-spec wrapper content.

#### Scenario: Sync applies delta specs when explicitly requested

r[sync_archive.execute.sync.applies]
- GIVEN a change with one or more delta spec files and no sync blockers
- WHEN the operator runs sync with the explicit execution flag
- THEN the system merges each planned delta spec into the accepted specs tree
- AND the JSON receipt reports `dry_run=false`, `mutated=true`, and the completed actions

#### Scenario: Sync remains dry-run by default

r[sync_archive.execute.sync.default_dry_run]
- GIVEN a change with one or more delta spec files
- WHEN the operator runs sync without the explicit execution flag
- THEN the system emits a dry-run plan
- AND no repository files are modified

#### Scenario: Existing accepted spec wrapper is preserved

r[sync_archive.execute.sync.merge_preserves_wrapper]
- GIVEN an accepted spec with a title, purpose, and requirements wrapper
- AND a delta spec containing only delta requirement sections
- WHEN the operator runs sync with the explicit execution flag
- THEN the accepted spec still contains its title, purpose, and requirements wrapper
- AND the new or modified requirement content appears under the accepted requirements section

#### Scenario: New accepted spec is created from delta sections

r[sync_archive.execute.sync.merge_creates_spec]
- GIVEN a change with a delta spec for a domain that does not exist in the accepted specs tree
- WHEN the operator runs sync with the explicit execution flag
- THEN the system creates an accepted spec document with a stable title, purpose, and requirements wrapper
- AND the delta requirement content appears under the accepted requirements section

### Requirement: Archive plan execution
r[sync_archive.execute.archive] The system MUST execute an archive plan only when the caller explicitly requests non-dry-run execution and the computed plan is unblocked.

#### Scenario: Archive moves completed change when explicitly requested
r[sync_archive.execute.archive.moves]
- GIVEN a completed change with synced delta specs and no archive blockers
- WHEN the operator runs archive with the explicit execution flag
- THEN the system moves the active change directory under `openspec/changes/archive/<date>-<change>`
- AND the JSON receipt reports `dry_run=false`, `mutated=true`, and the archive destination

#### Scenario: Blocked archive does not mutate
r[sync_archive.execute.archive.blocked_noop]
- GIVEN a change with incomplete tasks or missing required artifacts
- WHEN the operator runs archive with the explicit execution flag
- THEN the system refuses to mutate repository files
- AND the JSON receipt reports the blockers and `mutated=false`

### Requirement: Execution boundary
r[sync_archive.execute.boundary] The system MUST keep execution effects in the CLI shell layer while pure core code computes plans from shell-supplied observations.

#### Scenario: Core remains pure after execution support
r[sync_archive.execute.boundary.pure_core]
- GIVEN sync/archive execution support is implemented
- WHEN Slotcar verifies the pure core boundary
- THEN no filesystem, environment, process, stdout/stderr, or JSON presentation effects are present under `crates/cairn-core/src/verified/`

### Requirement: Lifecycle mutation manifests
r[sync_archive.execute.mutation_manifest] The system MUST emit deterministic before/after mutation manifests for explicit lifecycle executions that mutate repository files.

#### Scenario: Sync execution reports touched file hashes
r[sync_archive.execute.mutation_manifest.sync]
- GIVEN a sync plan is executed with the explicit execution flag
- WHEN the system merges delta specs into accepted specs
- THEN the JSON receipt MUST include a before/after manifest for every planned touched file using root-relative paths and BLAKE3 content hashes
- AND the manifest hash MUST be stable for identical pre-state, delta content, and execution result across checkout roots

#### Scenario: Archive execution reports moved tree hashes
r[sync_archive.execute.mutation_manifest.archive]
- GIVEN an archive plan is executed with the explicit execution flag
- WHEN the system moves a completed change into the archive tree
- THEN the JSON receipt MUST include a before/after manifest for the active change tree and archive destination tree using root-relative paths and BLAKE3 content hashes
- AND the receipt MUST report `mutated=false` if no filesystem move occurred

#### Scenario: Migration execution reports copied file hashes
r[sync_archive.execute.mutation_manifest.migration]
- GIVEN a migration plan is executed with the explicit execution flag
- WHEN legacy lifecycle files are copied into the native Cairn layout
- THEN the JSON receipt MUST include source and destination before/after manifests using root-relative paths where possible and explicit external-input metadata otherwise
- AND any conflict or partial failure MUST be reflected in deterministic blocker or issue fields

### Requirement: Archive compaction plan
r[cairn.sync_archive.compaction.plan] Cairn MUST compute archive compaction plans as deterministic dry-run receipts before any archive pack, ledger, pointer, or pruning mutation is allowed.

#### Scenario: Compaction plan is read-only
r[cairn.sync_archive.compaction.plan.read_only]
- GIVEN a repository with completed changes under `cairn/archive/`
- WHEN an operator requests archive compaction without `--execute`
- THEN Cairn MUST emit a deterministic plan with candidate archives, planned pack artifacts, planned retained pointers, blockers, input hash, policy hash, and plan hash
- AND Cairn MUST NOT mutate repository files.

#### Scenario: Candidate order is stable
r[cairn.sync_archive.compaction.plan.ordering]
- GIVEN multiple eligible archived changes with different names and dates
- WHEN Cairn builds a compaction plan
- THEN Cairn MUST sort candidates by policy-declared stable archive identity rather than filesystem iteration order.

#### Scenario: Stale plan is rejected
r[cairn.sync_archive.compaction.plan.stale]
- GIVEN a saved compaction plan for an earlier archive state
- WHEN an operator attempts to execute the plan after archive content or policy changes
- THEN Cairn MUST reject the plan with deterministic diagnostics and MUST NOT mutate lifecycle files.

### Requirement: Archive and pack manifest roots
r[cairn.sync_archive.compaction.manifest_roots] Cairn MUST represent compacted archive content with canonical BLAKE3 manifest roots over normalized root-relative file entries.

#### Scenario: File entries bind content identity
r[cairn.sync_archive.compaction.manifest_roots.entries]
- GIVEN an archived change selected for compaction
- WHEN Cairn computes its archive manifest
- THEN each file entry MUST include the archive identity, normalized root-relative path, byte length, BLAKE3 content hash, and file role when known.

#### Scenario: Manifest roots are deterministic
r[cairn.sync_archive.compaction.manifest_roots.stable]
- GIVEN identical archived file contents reached from different checkout roots
- WHEN Cairn computes archive and pack manifest roots
- THEN the roots MUST be identical and MUST NOT include host-specific absolute paths.

#### Scenario: Duplicate content may be deduplicated
r[cairn.sync_archive.compaction.manifest_roots.deduplicate]
- GIVEN two archive files with identical bytes
- WHEN Cairn writes a lossless pack
- THEN Cairn MAY store one object for the shared BLAKE3 content hash while preserving separate manifest entries for each original path.

### Requirement: Compaction receipt chain
r[cairn.sync_archive.compaction.receipt_chain] Cairn MUST chain compaction receipts with domain-separated BLAKE3 material that binds the previous chain hash and current receipt hash.

#### Scenario: Receipt binds operation state
r[cairn.sync_archive.compaction.receipt_chain.fields]
- GIVEN a compaction operation has a verified plan and mutation manifest
- WHEN Cairn emits the compaction receipt
- THEN the receipt MUST include `prev_chain_hash`, `pre_state_root`, `post_state_root`, `packed_archives_root`, `retained_manifest_root`, `lossiness`, `policy_hash`, `plan_hash`, `mutation_manifest_hash`, `receipt_hash`, and `chain_hash`.

#### Scenario: Chain hash detects removed operation
r[cairn.sync_archive.compaction.receipt_chain.removed]
- GIVEN a compaction ledger with two or more receipts
- WHEN one earlier receipt is removed or modified
- THEN compaction verification MUST report a chain mismatch for the affected suffix.

#### Scenario: Genesis is explicit
r[cairn.sync_archive.compaction.receipt_chain.genesis]
- GIVEN the first compaction receipt for a repository
- WHEN Cairn computes its `prev_chain_hash`
- THEN Cairn MUST use a policy-owned genesis value rather than an implicit missing or empty predecessor.

### Requirement: Lossless archive pack execution
r[cairn.sync_archive.compaction.lossless_pack] Cairn MUST execute lossless archive compaction only after writing and verifying deterministic pack artifacts that can restore the original archived bytes.

#### Scenario: Execute writes pack before pruning source
r[cairn.sync_archive.compaction.lossless_pack.write_first]
- GIVEN an unblocked lossless compaction plan
- WHEN an operator executes the plan
- THEN Cairn MUST write pack objects, pack manifests, retained pointer metadata, and a mutation manifest before removing or replacing expanded archive files.

#### Scenario: Unpack restores exact bytes
r[cairn.sync_archive.compaction.lossless_pack.unpack]
- GIVEN a verified lossless pack and retained archive pointer
- WHEN an operator requests unpacking with `--execute`
- THEN Cairn MUST restore the archived change tree with exact original file bytes and MUST verify restored BLAKE3 hashes before reporting success.

#### Scenario: Blocked execution does not mutate
r[cairn.sync_archive.compaction.lossless_pack.blocked]
- GIVEN a plan with unsafe paths, missing archive files, hash drift, unsupported profile, or verification failure
- WHEN an operator executes compaction
- THEN Cairn MUST report deterministic blockers and MUST NOT prune expanded archive files.

### Requirement: Compaction verification
r[cairn.sync_archive.compaction.verify] Cairn MUST provide read-only verification for compaction ledgers, receipt chains, manifests, pack objects, retained pointers, and unpackability metadata.

#### Scenario: Valid compacted archive verifies
r[cairn.sync_archive.compaction.verify.valid]
- GIVEN a losslessly compacted archive with all declared objects and receipts present
- WHEN an operator runs compaction verification
- THEN Cairn MUST report a passing receipt that binds the ledger hash, latest chain hash, verified pack roots, policy hash, and verification receipt hash.

#### Scenario: Tampered object fails verification
r[cairn.sync_archive.compaction.verify.tampered_object]
- GIVEN a pack object whose bytes no longer match its declared BLAKE3 content hash
- WHEN an operator runs compaction verification
- THEN Cairn MUST report the object mismatch and MUST NOT claim the compacted archive is valid.

#### Scenario: Unsafe pointer fails verification
r[cairn.sync_archive.compaction.verify.unsafe_pointer]
- GIVEN retained pointer metadata with an absolute path, parent-directory traversal, duplicate path, or unknown archive identity
- WHEN an operator runs compaction verification
- THEN Cairn MUST reject the pointer with deterministic diagnostics.

### Requirement: Lossiness policy
r[cairn.sync_archive.compaction.lossiness_policy] Cairn MUST deny lossy archive summary compaction unless an explicit reviewed policy profile enables it and requires tombstone identities plus visible non-claims.

#### Scenario: Lossy request is rejected by default
r[cairn.sync_archive.compaction.lossiness_policy.default_reject]
- GIVEN the default Cairn policy
- WHEN an operator requests a lossy summary compaction profile
- THEN Cairn MUST reject the request and MUST NOT mutate archive files.

#### Scenario: Lossy receipt declares non-claims when enabled
r[cairn.sync_archive.compaction.lossiness_policy.non_claims]
- GIVEN a future policy enables a lossy compaction profile
- WHEN Cairn emits a lossy compaction receipt
- THEN the receipt MUST include original path hashes, byte lengths, summary hash, lossiness class, irreversible-action marker, and non-claims that the original bytes are not reconstructable from the summary alone.

### Requirement: Compaction boundary
r[cairn.sync_archive.compaction.boundary] Cairn MUST keep compaction planning, manifest hashing, receipt hash material, chain validation, and verification decisions in pure core code while filesystem effects remain in the CLI shell.

#### Scenario: Core compaction logic has no shell effects
r[cairn.sync_archive.compaction.boundary.pure_core]
- GIVEN compaction support is implemented
- WHEN Slotcar verifies the pure core boundary
- THEN planning, root calculation, chain validation, and diagnostic construction MUST NOT read files, inspect environment variables, spawn processes, print output, or write JSON.

#### Scenario: CLI owns mutation and presentation
r[cairn.sync_archive.compaction.boundary.cli_shell]
- GIVEN an operator runs compaction, verification, or unpacking commands
- WHEN Cairn interacts with repository files or renders command output
- THEN those effects MUST remain in the CLI shell and MUST call pure core functions with shell-supplied snapshots.

### Requirement: Compaction test coverage
r[cairn.sync_archive.compaction.tests] Cairn MUST verify archive compaction with paired positive and negative tests across pure core fixtures and CLI behavior.

#### Scenario: Positive compaction tests pass
r[cairn.sync_archive.compaction.tests.positive]
- GIVEN valid archive fixtures, deterministic plans, lossless pack artifacts, ledger append receipts, and unpack targets
- WHEN focused tests run
- THEN tests MUST prove stable roots, stable chain hashes, successful verification, and exact-byte unpack restoration.

#### Scenario: Negative compaction tests fail closed
r[cairn.sync_archive.compaction.tests.negative]
- GIVEN tampered objects, missing objects, wrong previous chain hashes, duplicate manifest entries, stale plans, unsafe paths, unsupported lossy profiles, and interrupted executions
- WHEN focused tests run
- THEN tests MUST prove Cairn reports expected diagnostics, does not panic, and avoids further mutation.

### Requirement: Compaction operator documentation
r[cairn.sync_archive.compaction.docs] Cairn documentation MUST explain archive compaction commands, receipt fields, chain/Merkle distinction, verification workflow, restore workflow, and non-claims.

#### Scenario: Documentation distinguishes chain and content roots
r[cairn.sync_archive.compaction.docs.chain_merkle]
- GIVEN an operator reads the compaction documentation
- WHEN Cairn describes compaction evidence
- THEN the documentation MUST state that chain hashes prove operation ordering while Merkle-style manifest roots prove represented archive content.

#### Scenario: Documentation states non-claims
r[cairn.sync_archive.compaction.docs.non_claims]
- GIVEN an operator reads the compaction documentation
- WHEN Cairn describes a passing compaction verification receipt
- THEN the documentation MUST state that verification proves local representation integrity only and does not prove archived change correctness, release readiness, remote trust, or semantic review completeness.

### Requirement: Compaction final validation
r[cairn.sync_archive.compaction.final_validation] The compaction change MUST pass focused tests, Cairn validation and gates, policy freshness checks, and release-readiness evidence before archive.

#### Scenario: Final validation evidence is present
r[cairn.sync_archive.compaction.final_validation.evidence]
- GIVEN archive compaction implementation is ready to archive
- WHEN the change is reviewed
- THEN it MUST include evidence for focused positive and negative compaction tests, `cairn validate`, proposal/design/tasks gates, policy freshness, and release-readiness checks.
