## Why

Replay now proves deterministic outputs and discloses policy-owned entropy assumptions, but agents and CI still cannot cheaply prove which reviewed execution environment facts the replay ran under. A replay receipt can say it passed while leaving binary identity, generated policy identity, selected suite identity, and relevant toolchain/build context scattered across hashes and prose.

This matters for agent use because wrappers need one stable, secret-safe object they can bind into agent receipts and compare across machines without preserving host-specific paths or environment values.

## What Changes

- Add policy-owned replay environment attestation metadata under `determinism_replay_policy`.
- Add stable `environment_attestation` to `cairn determinism replay` receipts.
- Include deterministic facts for Cairn package/version, generated policy hash, entropy policy hash, selected cases/groups, replay scope, and reviewed build/toolchain context disclosures.
- Keep host-specific paths, raw environment values, and secrets out of the attestation.
- Allow agent-run receipts to carry optional replay environment attestation hash evidence and validate it deterministically when present.

## Capabilities

### Modified Capabilities
- `determinism_replay`: Replay receipts disclose the reviewed environment facts that shape replay reproducibility.
- `agent_receipt_replay_evidence`: Agent receipt validation can bind to an attestation hash in addition to replay receipt/policy/group/case evidence.

## Impact

- **Files**: `cairn-policy/{contracts,default,generated}/`, `crates/cairn-cli/src/main.rs`, `crates/cairn-cli/tests/cli.rs`, `crates/cairn-core/src/verified/agent_receipt.rs`, fixtures/goldens, README/docs, accepted specs.
- **APIs**: Adds stable receipt fields; no command-line flag changes.
- **Dependencies**: No new external dependencies.
- **Testing**: Focused replay/agent receipt tests, policy export check, schema/docs tests, Tracey coverage, full CLI/core tests, release-readiness, and replay smokes.
