# Proposal: Agent Evidence Bundle Verification

## Why

Cairn now produces deterministic agent receipts, replay receipts, replay receipt verification receipts, environment attestation hashes, and release-readiness receipts. Agent/CI wrappers still have to stitch those artifacts together correctly. That leaves room for integration mistakes: stale replay receipts can be paired with fresh agent receipts, verify-receipt outputs can be omitted or ignored, and optional release-readiness evidence can be copied without checking its hash.

## What Changes

- Add a first-class `cairn agent-receipt verify-bundle <bundle.json> --root .` command.
- Define a stable bundle input contract that references the agent receipt and saved replay/verification/release evidence artifacts.
- Emit a deterministic pass/fail bundle verification receipt with stable issue codes.
- Validate agent receipt evidence, saved replay receipt verification, replay group coverage, environment attestation linkage, current policy hash, and optional release-readiness evidence.
- Keep all failure receipts secret-safe and path-safe.

## Capabilities

### Modified Capabilities

- `cairn.supersede.agent_run_receipts`: Agent receipts gain an end-to-end bundle verification workflow for CI and wrappers.
- `cairn.supersede.replay_receipt_verification`: Saved replay receipt verification becomes consumable as part of a larger evidence bundle.

## Impact

- **CLI**: Add `agent-receipt verify-bundle`.
- **Policy**: Add a public stable receipt schema for bundle verification.
- **Docs**: Document the bundle input and wrapper flow.
- **Tests**: Cover valid bundles, stale/tampered artifacts, missing/failed verification receipts, group coverage failures, release-readiness mismatch, and secret/path safety.
