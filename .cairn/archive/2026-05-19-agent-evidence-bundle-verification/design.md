# Design: Agent Evidence Bundle Verification

## Context

Individual Cairn evidence artifacts are deterministic, but current wrappers must manually compose them. The new verifier should be a thin CLI shell around existing deterministic validation logic, producing one receipt that operators can gate on.

## Goals

- Verify a wrapper-provided bundle without mutating the workspace.
- Reuse the same deterministic validation semantics as `agent-receipt validate` and `determinism verify-receipt`.
- Produce stable issue codes and a stable receipt hash.
- Avoid raw secret, environment, absolute-root, or host-specific values in receipts.

## Non-Goals

- No cryptographic signing or remote artifact fetching.
- No new replay execution mode; bundles verify already-saved artifacts.
- No requirement that release-readiness evidence be present for every agent run; when provided, it must be validated.

## Input Contract

`verify-bundle` reads a JSON object with stable string fields:

- `agent_receipt_path`
- `replay_receipt_path`
- `replay_verification_path`
- optional `release_readiness_path`
- optional expected hashes: `agent_receipt_hash`, `replay_receipt_hash`, `replay_verification_receipt_hash`, `release_readiness_receipt_hash`

Paths are resolved relative to the bundle file directory unless absolute paths are supplied. Paths are display-only inputs; failure receipts report field names and artifact classes, not raw paths.

## Validation Flow

1. Load current generated policy.
2. Parse the bundle object and validate stable field types.
3. Validate the agent receipt using the existing agent receipt validator.
4. Verify the saved replay receipt using the same verifier core as `determinism verify-receipt`.
5. Parse the saved replay verification receipt and require `valid=true` plus matching replay hash/policy/attestation linkage.
6. Require agent replay evidence to cover policy group `agent` and match the saved replay receipt hash/policy hash/attestation hash.
7. If release-readiness evidence is present, require it to be valid and match any declared hash.
8. Emit a bundle verification receipt with sorted issues and deterministic hash.

## Receipt Shape

The bundle verification receipt includes:

- `command`
- `valid`
- `verdict`
- `issues`
- `current_replay_policy_hash`
- `required_replay_group`
- `agent_receipt_hash`
- `replay_receipt_hash`
- `replay_verification_receipt_hash`
- `environment_attestation_hash`
- `release_readiness_receipt_hash`
- `receipt_hash`

## Risks / Trade-offs

- **Duplicate logic**: The implementation should factor helper functions rather than shelling out or parsing stdout.
- **Fixture churn**: Policy schema changes will update generated policy, goldens, and agent receipt fixture hashes.
- **Path leakage**: Receipts must identify failed artifact classes/fields without including absolute paths.
