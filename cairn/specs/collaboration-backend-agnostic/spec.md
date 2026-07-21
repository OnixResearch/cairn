# Collaboration Backend Agnostic Specification

## Purpose

Defines the `collaboration-backend-agnostic` capability.

## Requirements

### Requirement: Transport-neutral semantic core
r[cairn.collaboration_backend.semantic_core] The system MUST represent collaborative active-change state with transport-neutral workspace and typed base-revision identities while preserving deterministic conflicts, lifecycle, evidence, materialization, and explicit authority facts.

#### Scenario: Neutral state does not require Radicle context
r[cairn.collaboration_backend.semantic_core.neutral]
- GIVEN exact supported active-change artifacts, a workspace ID, typed base revision, policy hash, and actor
- WHEN Cairn initializes and reduces the collaborative state
- THEN no Radicle home, repository ID syntax, COB object ID, or Git-only base field is required by the semantic API

#### Scenario: Legacy Radicle identities remain stable
r[cairn.collaboration_backend.semantic_core.legacy]
- GIVEN a valid `cairn.radicle-change-cob.v1` state
- WHEN the generalized core decodes and hashes the state
- THEN its historical canonical content root and state hash remain unchanged

### Requirement: Portable collaboration state
r[cairn.collaboration_backend.portable_state] The system MUST construct and validate a deterministic BLAKE3-bound portable collaboration envelope over one neutral reduced change state.

#### Scenario: Portable round trip preserves semantic identity
r[cairn.collaboration_backend.portable_state.roundtrip]
- GIVEN a valid neutral collaborative state
- WHEN Cairn serializes and reloads a portable envelope
- THEN exact artifacts, lifecycle state, content root, and state hash are preserved

#### Scenario: Tampered portable state is rejected
r[cairn.collaboration_backend.portable_state.tamper]
- GIVEN a portable envelope whose state, content root, state hash, schema, or backend label was changed
- WHEN Cairn validates the envelope
- THEN validation fails without returning trusted collaboration state

### Requirement: Portable backend shell
r[cairn.collaboration_backend.portable_shell] The system MUST provide preview-first portable import, read-only status, and preview-first exact checkout through bounded local filesystem operations without requiring Radicle.

#### Scenario: Portable workflow succeeds without Radicle
r[cairn.collaboration_backend.portable_shell.positive]
- GIVEN an active Cairn change and an absent portable store and checkout destination
- WHEN the operator previews and executes portable import, status, and checkout
- THEN Cairn verifies the same neutral state and materializes exact bytes without Radicle context

#### Scenario: Portable mutation fails closed
r[cairn.collaboration_backend.portable_shell.negative]
- GIVEN an existing or symlinked store, tampered state, stale lock, changed preview input, conflicted state, or divergent checkout destination
- WHEN portable import or checkout execution is requested
- THEN Cairn reports blockers and does not overwrite trusted state or local work

### Requirement: Backend-neutral CLI
r[cairn.collaboration_backend.cli] The system MUST expose `cairn collaboration import|status|checkout` with an explicit backend selector and backend-specific arguments while retaining `cairn radicle` as a compatibility entry point.

#### Scenario: Portable CLI omits Radicle arguments
r[cairn.collaboration_backend.cli.portable]
- GIVEN the operator selects `portable-json.v1`
- WHEN Cairn parses a collaboration command
- THEN `--store` is accepted and `--rad-home` and `--rid` are not required

#### Scenario: Backend argument mismatch is rejected
r[cairn.collaboration_backend.cli.arguments]
- GIVEN options for a different or unsupported backend
- WHEN Cairn parses the collaboration command
- THEN it fails closed with a deterministic diagnostic before mutation

### Requirement: Backend-labeled receipts
r[cairn.collaboration_backend.receipts] The system MUST emit deterministic BLAKE3-bound collaboration receipts identifying backend, workspace, typed base revision, actor observation, state/content identities, policy, exact artifacts, local classification, blockers, execution mode, and mutation result.

#### Scenario: Receipt does not overstate portable authority
r[cairn.collaboration_backend.receipts.authority]
- GIVEN a passing portable import, status, or checkout
- WHEN Cairn emits its receipt
- THEN caller-declared actor and local filesystem capability are distinguished from cryptographic or remote authority proof

### Requirement: Bounded backend claims
r[cairn.collaboration_backend.non_claims] The system MUST state that backend selection and local collaboration success do not prove network publication, peer synchronization, remote trust, caller identity, universal authority, gate validity, evidence truth, dependency readiness, archive execution, source correctness, or release eligibility.

#### Scenario: One backend does not define collaboration semantics
r[cairn.collaboration_backend.non_claims.provider]
- GIVEN equivalent neutral state is observed through a supported provider
- WHEN Cairn reports status or checkout
- THEN provider-specific object, signature, or authority facts remain adapter observations rather than semantic lifecycle proof
