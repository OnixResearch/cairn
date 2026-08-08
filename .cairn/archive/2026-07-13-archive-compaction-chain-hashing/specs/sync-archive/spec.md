# Sync Archive Specification

## Purpose

Adds deterministic archive compaction for completed Cairn changes using Merkle-style content roots and chain-hashed compaction receipts.

## Requirements

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
