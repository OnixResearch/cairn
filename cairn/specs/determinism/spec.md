# Determinism Specification

## Purpose

Defines the `determinism` capability.

## Requirements

### Requirement: Preserves receipt canonicalization seam

r[cairn.determinism.preserves_receipt_canonicalization.additive_internal_seam]
Cairn MUST provide an additive internal helper for encoding existing receipt-shaped values into Preserves canonical machine bytes without replacing existing JSON receipt surfaces.

#### Scenario: Encode receipt material through the additive seam

r[cairn.determinism.preserves_receipt_canonicalization.additive_internal_seam.encode]
- GIVEN a JSON-compatible receipt value
- WHEN Cairn computes Preserves canonical receipt bytes
- THEN the helper returns deterministic packed bytes
- AND existing JSON receipt schemas and public CLI outputs remain unchanged.

### Requirement: Stable Preserves receipt hashing

r[cairn.determinism.preserves_receipt_canonicalization.stable_hashing]
Cairn MUST compute a BLAKE3 hash from Preserves canonical receipt bytes that is stable for semantically equivalent receipt objects and changes when receipt content changes.

#### Scenario: Equivalent object order has the same hash

r[cairn.determinism.preserves_receipt_canonicalization.stable_hashing.object_order]
- GIVEN two receipt objects with the same keys and values in different source orders
- WHEN Cairn computes their Preserves canonical receipt hashes
- THEN both hashes are identical.

#### Scenario: Changed receipt content has a different hash

r[cairn.determinism.preserves_receipt_canonicalization.stable_hashing.content_change]
- GIVEN two receipt objects with different receipt content
- WHEN Cairn computes their Preserves canonical receipt hashes
- THEN the hashes differ.

### Requirement: Focused validation evidence

r[cairn.determinism.preserves_receipt_canonicalization.validation]
Cairn MUST verify the Preserves receipt canonicalization seam with focused unit tests and Cairn lifecycle gates before landing the spike.

#### Scenario: Validate the spike before landing

r[cairn.determinism.preserves_receipt_canonicalization.validation.focused]
- GIVEN the Preserves receipt canonicalization change package
- WHEN focused Rust tests and Cairn gates run
- THEN they pass with deterministic evidence.

### Requirement: Preserves receipt canonicalization

Cairn SHALL keep Preserves receipt canonicalization additive and internal until a specific public/stored surface is explicitly promoted.

#### Scenario: Agent-run fixture has an internal Preserves hash consumer

r[cairn.determinism.preserves_receipt_consumer.agent_run_internal_hash]

- GIVEN an agent-run receipt represented as JSON-compatible receipt material
- WHEN Cairn computes its internal Preserves receipt hash
- THEN the hash material SHALL exclude self-referential receipt hash fields
- AND the existing public JSON `receipt_hash` contract SHALL remain unchanged

#### Scenario: Positive agent-run fixture has a golden Preserves hash

r[cairn.determinism.preserves_receipt_consumer.golden_fixture]

- GIVEN the checked-in positive agent-run receipt fixture
- WHEN Cairn computes its internal Preserves receipt hash
- THEN the result SHALL match a stable golden BLAKE3 hex digest

#### Scenario: Consumer Preserves hash is stable and content-sensitive

r[cairn.determinism.preserves_receipt_consumer.stable_hashing]

- GIVEN equivalent agent-run receipt material with different JSON object key ordering
- WHEN Cairn computes internal Preserves receipt hashes
- THEN the hashes SHALL match
- AND a semantic content change SHALL produce a different internal Preserves hash

#### Scenario: Preserves receipt consumer verification is focused

r[cairn.determinism.preserves_receipt_consumer.validation]

- GIVEN the Preserves receipt consumer implementation
- WHEN focused tests and Cairn lifecycle validation run
- THEN the verification SHALL prove the golden fixture, stable hashing, content drift, lifecycle gates, and Tracey references

#### Scenario: Agent receipt Preserves hash CLI emits deterministic inspection receipts

r[cairn.determinism.preserves_hash_cli.command]

- GIVEN a path to a JSON agent-run receipt file
- WHEN an operator runs `cairn agent-receipt preserves-hash <path>`
- THEN Cairn SHALL emit a deterministic JSON inspection receipt containing the internal Preserves hash
- AND the command SHALL NOT mutate the input file

#### Scenario: Preserves hash CLI uses bounded receipt loading

r[cairn.determinism.preserves_hash_cli.bounded_read]

- GIVEN an agent-run receipt file path
- WHEN Cairn loads the file for Preserves hash inspection
- THEN it SHALL use the same bounded read limit as agent receipt validation
- AND the receipt SHALL disclose the bounded byte limit

#### Scenario: Preserves hash CLI does not change public receipt validation

r[cairn.determinism.preserves_hash_cli.no_schema_churn]

- GIVEN existing agent-run receipt validation and public JSON `receipt_hash` fields
- WHEN the Preserves hash inspection CLI is added
- THEN existing validation semantics and public/stored receipt fields SHALL remain unchanged

#### Scenario: Preserves hash CLI has positive and negative verification

r[cairn.determinism.preserves_hash_cli.validation]

- GIVEN the Preserves hash inspection CLI
- WHEN focused tests run
- THEN they SHALL prove the positive fixture golden hash, reproducible receipt output, and structured failure for non-object JSON input

#### Scenario: Agent receipt Preserves hash CLI is policy-owned replay evidence

r[cairn.determinism.preserves_hash_cli.replay_policy]
r[cairn.determinism.preserves_hash_cli.surface_inventory]
r[cairn.determinism.preserves_hash_cli.replay_execution]
r[cairn.determinism.preserves_hash_cli.replay_validation]

- GIVEN the `agent-receipt preserves-hash` operator command exists
- WHEN Cairn exports and validates its deterministic replay policy
- THEN `determinism_replay_policy.cases` SHALL include an `agent_receipt_preserves_hash` case for the checked-in positive agent receipt fixture
- AND the case SHALL be included in the `full` replay group without expanding existing `agent` or `release` required preflight groups
- AND `determinism_surface_inventory` SHALL list the command as a covered CLI surface
- AND `cairn determinism replay --case agent_receipt_preserves_hash` SHALL produce a passing deterministic replay receipt
- AND `cairn determinism coverage-audit` SHALL report the surface as covered

#### Scenario: Preserves canonicalization has a checked-in fixture corpus

r[cairn.determinism.preserves_canonicalization_corpus.fixture_corpus]

- GIVEN Cairn depends on Preserves packed encoding for internal deterministic receipt hashing
- WHEN maintainers review the Preserves canonicalization corpus
- THEN the corpus SHALL contain stable case identifiers, JSON values, and expected BLAKE3 hashes of canonical Preserves bytes
- AND the corpus SHALL include primitive, Unicode, array, nested-object, and receipt-shaped cases
- AND the corpus SHALL be checked into source control rather than generated only at test time

#### Scenario: Preserves corpus golden hashes are verified

r[cairn.determinism.preserves_canonicalization_corpus.golden_hashes]

- GIVEN the checked-in Preserves canonicalization corpus
- WHEN focused core tests recompute canonical Preserves hashes
- THEN every case SHALL match its expected hash
- AND object-key ordering equivalent material SHALL produce the same canonical bytes and hash

#### Scenario: Preserves corpus validation is content-sensitive and internal

r[cairn.determinism.preserves_canonicalization_corpus.validation]

- GIVEN Preserves canonicalization remains an internal deterministic evidence seam
- WHEN corpus validation runs
- THEN changed fixture content SHALL produce a different hash from the original case
- AND public/stored JSON receipt schemas and operator CLI output SHALL remain unchanged

#### Scenario: Preserves corpus has an explicit update rail

r[cairn.determinism.preserves_corpus_update_rail.command]

- GIVEN the checked-in Preserves canonicalization corpus
- WHEN an operator runs the corpus update rail without an execute flag
- THEN Cairn SHALL recompute expected hashes and report stale cases without mutating the corpus file.

#### Scenario: Preserves corpus update rail recomputes hashes through the core seam

r[cairn.determinism.preserves_corpus_update_rail.hash_recompute]

- GIVEN a corpus case with JSON `value` material
- WHEN the update rail evaluates the case
- THEN Cairn SHALL compute the replacement `expected_hash` through the existing internal Preserves canonical hash helper.

#### Scenario: Preserves corpus update rail emits deterministic receipts

r[cairn.determinism.preserves_corpus_update_rail.receipt]

- GIVEN valid or malformed corpus input
- WHEN the update rail completes
- THEN Cairn SHALL emit deterministic receipt fields for case count, stale count, mutation status, case summaries, and issue codes.

#### Scenario: Preserves corpus update rail validates stale and malformed cases

r[cairn.determinism.preserves_corpus_update_rail.validation]

- GIVEN focused CLI tests for stale, clean, update, and malformed corpus files
- WHEN Cairn runs the Preserves corpus update validation rail
- THEN stale dry-runs SHALL not mutate, explicit updates SHALL repair expected hashes, clean corpora SHALL remain no-op, and malformed corpora SHALL fail closed.

### Requirement: Preserves corpus has CI drift guards
r[cairn.determinism.preserves_corpus_ci_guard] Cairn MUST expose CI-visible checks that make Preserves canonicalization corpus drift fail closed.

#### Scenario: Clean corpus passes CI guard
r[cairn.determinism.preserves_corpus_ci_guard.clean_check]
- GIVEN the checked-in Preserves canonicalization corpus
- WHEN CI runs the Preserves corpus freshness guard
- THEN the guard runs the dry-run corpus update command
- AND the guard passes only when no stale hashes are detected

#### Scenario: Stale corpus fails CI guard
r[cairn.determinism.preserves_corpus_ci_guard.stale_detection]
- GIVEN a Preserves corpus with a deliberately stale expected hash
- WHEN CI runs the stale-detection guard
- THEN the dry-run corpus update command exits nonzero
- AND the guard passes only if stale drift is detected without mutation

#### Scenario: Preserves corpus guard validation is repeatable
r[cairn.determinism.preserves_corpus_ci_guard.validation]
- GIVEN the Preserves corpus CI guard checks
- WHEN Cairn validates the repository and runs Tracey coverage
- THEN the guard requirements have durable implementation and verification references
- AND the checks remain deterministic and secret-free

### Requirement: Preserves receipt workflow has operator documentation
r[cairn.determinism.preserves_operator_docs] Cairn MUST document how operators use internal Preserves receipt hashing without replacing public JSON receipt surfaces.

#### Scenario: Operator docs describe the Preserves workflow boundary
r[cairn.determinism.preserves_operator_docs.workflow]
- GIVEN Cairn exposes internal Preserves hashes, corpus fixtures, update rails, and CI drift guards
- WHEN an operator reads the Preserves receipt workflow documentation
- THEN the documentation distinguishes JSON `receipt_hash` compatibility from internal Preserves hash inspection
- AND it names the read-only agent receipt hash command, corpus update command, explicit execute rail, and CI checks

#### Scenario: Preserves workflow docs are discoverable
r[cairn.determinism.preserves_operator_docs.discoverability]
- GIVEN operators already consult README and agent-run receipt documentation
- WHEN the Preserves workflow docs are added
- THEN those existing docs link to the Preserves operator workflow

#### Scenario: Preserves operator docs are validated
r[cairn.determinism.preserves_operator_docs.validation]
- GIVEN the Preserves operator documentation
- WHEN Cairn validates lifecycle specs, Tracey coverage, and focused Preserves checks
- THEN the documentation contract has durable implementation and verification references

### Requirement: Preserves dependency bumps have a maintainer runbook
r[cairn.determinism.preserves_dependency_bump_runbook] Cairn MUST document the maintainer workflow for reviewing Preserves dependency bumps that may affect canonical receipt hashes.

#### Scenario: Runbook starts with fail-closed drift detection
r[cairn.determinism.preserves_dependency_bump_runbook.workflow]
- GIVEN a maintainer changes the `preserves` dependency or related serde conversion behavior
- WHEN they follow the Preserves dependency-bump runbook
- THEN the workflow SHALL run the dry-run corpus update rail and CI drift guards before any corpus mutation
- AND the no-drift path SHALL preserve checked-in golden hashes unchanged

#### Scenario: Intentional drift is explicit and reviewable
r[cairn.determinism.preserves_dependency_bump_runbook.intentional_drift]
- GIVEN Preserves canonical bytes intentionally change after review
- WHEN the maintainer refreshes corpus golden hashes
- THEN the runbook SHALL require `--execute`, review of the corpus diff, focused Preserves checks, workspace verification, and no public/stored JSON receipt schema change

#### Scenario: Dependency bump runbook is discoverable
r[cairn.determinism.preserves_dependency_bump_runbook.discoverability]
- GIVEN maintainers already consult README and Preserves receipt workflow docs
- WHEN the dependency-bump runbook is added
- THEN those surfaces SHALL link to the runbook

#### Scenario: Dependency bump runbook is validated
r[cairn.determinism.preserves_dependency_bump_runbook.validation]
- GIVEN the Preserves dependency-bump runbook
- WHEN Cairn validates lifecycle specs, Tracey coverage, and focused Preserves checks
- THEN the runbook contract SHALL have durable implementation and verification references

### Requirement: Preserves corpus freshness is policy-inventoried
r[cairn.determinism.preserves_corpus_policy_inventory.surface] Cairn MUST list the Preserves corpus freshness rail in policy-owned deterministic surface inventory.

#### Scenario: Corpus freshness has a non-mutating replay case
r[cairn.determinism.preserves_corpus_policy_inventory.replay_case]
- GIVEN the checked-in Preserves canonicalization corpus
- WHEN Cairn exports deterministic replay policy
- THEN policy SHALL include a non-mutating corpus freshness case
- AND replay SHALL evaluate the dry-run freshness path without `--execute`.

#### Scenario: Corpus policy inventory is verified
r[cairn.determinism.preserves_corpus_policy_inventory.tests]
r[cairn.determinism.preserves_corpus_policy_inventory.validation]
- GIVEN the Preserves corpus policy inventory case
- WHEN policy export, coverage audit, replay selection, Cairn validation, and Tracey coverage run
- THEN the surface and replay case SHALL be covered by durable verification.

### Requirement: Preserves corpus has a read-only explain command
r[cairn.determinism.preserves_corpus_explain_cli.command] Cairn MUST provide a read-only command that explains Preserves corpus cases as deterministic operator evidence.

#### Scenario: Explain command is read-only and deterministic
r[cairn.determinism.preserves_corpus_explain_cli.read_only]
- GIVEN a Preserves corpus file
- WHEN an operator runs `cairn preserves-corpus explain --path <corpus.json>`
- THEN Cairn SHALL emit deterministic JSON with case ids, descriptions, expected hash metadata, issue codes, and receipt hash
- AND it SHALL NOT mutate the corpus file.

#### Scenario: Explain command has positive and negative tests
r[cairn.determinism.preserves_corpus_explain_cli.tests]
r[cairn.determinism.preserves_corpus_explain_cli.validation]
- GIVEN valid and malformed corpus fixtures
- WHEN focused CLI tests and Cairn lifecycle validation run
- THEN read-only behavior, malformed failures, duplicate-case issues, Tracey coverage, and diff checks SHALL pass.

### Requirement: Preserves workflow docs tests are isolated from corpus behavior tests
r[cairn.determinism.preserves_workflow_test_split.docs_target] Cairn MUST keep Preserves workflow documentation coverage in a focused test surface separate from corpus mutation behavior checks.

#### Scenario: Corpus tests remain behavior-focused
r[cairn.determinism.preserves_workflow_test_split.corpus_focus]
- GIVEN Preserves corpus update integration tests
- WHEN documentation workflow assertions are split out
- THEN corpus tests SHALL continue to cover clean, stale, execute, and malformed corpus behavior without docs-only coupling.

#### Scenario: Split tests preserve coverage
r[cairn.determinism.preserves_workflow_test_split.tests]
r[cairn.determinism.preserves_workflow_test_split.validation]
- GIVEN the split Preserves workflow tests
- WHEN focused tests, Cairn validation, Tracey coverage, and diff checks run
- THEN all previous docs/runbook assertions and live Tracey references SHALL remain covered.
