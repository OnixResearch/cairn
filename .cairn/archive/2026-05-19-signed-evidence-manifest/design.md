# Design: Signed Evidence Manifest

## Context

The existing verifier chain proves individual evidence artifacts and the bundle that links agent receipt evidence to replay and release readiness. A remaining operational gap is the publication boundary: CI stores multiple files, and downstream tools need a deterministic way to know which artifacts belong together and whether their hashes still match.

## Goals

- Provide one stable manifest verification command for agents and CI.
- Keep verification read-only and deterministic.
- Compose existing validators instead of duplicating artifact semantics.
- Keep signer/anchor metadata optional, public, and secret-free.
- Produce portable hashes and stable issue codes suitable for archived evidence.

## Non-Goals

- Implement signing key management or signature cryptography.
- Contact remote transparency logs, timestamp authorities, or forges.
- Treat human-readable artifact paths as stable evidence.

## Decisions

### 1. Manifest verification is a new CLI surface

**Choice:** Add `cairn evidence-manifest verify <manifest.json> --root .`.

**Rationale:** A distinct command avoids overloading `agent-receipt verify-bundle` and makes the publication/index layer explicit.

**Alternative:** Extend bundle verification to accept manifests. Rejected because bundle verification has a narrower persisted-evidence-chain contract.

### 2. Manifest entries use roles and declared hashes

**Choice:** The manifest contains `artifacts` entries with stable `role`, `path`, optional `schema`, and expected `hash` fields.

**Rationale:** Roles make verification policy explicit while paths remain read-only locators. Expected hashes detect artifact substitution before downstream consumers trust contents.

### 3. Existing validators remain authoritative

**Choice:** The manifest verifier invokes/composes current validation helpers for roles such as `agent_receipt`, `replay_receipt`, `replay_verification`, `agent_evidence_bundle`, and `release_readiness`.

**Rationale:** The manifest should index evidence, not redefine receipt validity.

### 4. Optional signing/anchor metadata is recorded but not cryptographically verified

**Choice:** Accept optional `signer` and `anchor` objects with stable public fields. The verifier checks they are JSON objects and includes stable metadata hashes, but does not validate secrets, signatures, or remote anchors.

**Rationale:** This creates a deterministic seam for future signing without creating false security claims.

### 5. Failure receipts are path-safe and secret-safe

**Choice:** Issues use deterministic roles/classes/codes and do not echo raw paths or environment values.

**Rationale:** Manifests are intended for CI publication and must be safe to archive.

## Validation Plan

- Focused CLI integration test for a valid manifest built from dynamic current replay/bundle evidence.
- Negative tests for mismatched declared hashes, missing required bundle/replay evidence, invalid referenced verification receipt, malformed metadata, and no raw path/secret leakage.
- Public receipt schema docs test and policy export check.
- Full CLI/core tests, Cairn validate, Tracey coverage, diff check, sync/archive, commit/push.
