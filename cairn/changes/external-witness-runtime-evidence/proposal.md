## Why

Cairn already produces deterministic local receipts for agent runs, evidence bundles,
release-readiness, and publication indexes. Those receipts prove bounded local facts
about the artifacts an operator presents, but they intentionally do not prove that
every runtime action or evidence artifact that ever existed was preserved. The
halo-record model makes that boundary crisp: a self-held hash chain proves
integrity of the presented sequence, while completeness requires a checkpoint held
outside the chain producer's control.

This change brings that distinction into Cairn without importing halo-record's
Python implementation or its hash choices. Cairn should provide a stack-owned,
BLAKE3-based runtime evidence chain and witness-checkpoint verifier so agent
wrappers can publish bounded action evidence, disclose capture strength, and let
consumers distinguish local integrity from externally witnessed completeness.

## What Changes

- Add a Cairn runtime evidence model for subject-scoped action records. Records
  carry stable subject identity, action class, authorization decision, capture
  origin, redacted or hash-only input/output identities, and a BLAKE3 chain link.
- Add a witness checkpoint model with stable fields for subject, chain root,
  record count, head hash, witness identity metadata, and the checkpoint's own
  BLAKE3 identity.
- Add a pure verifier that checks presented runtime chains, validates bounded
  record content, classifies capture origin, and compares one or more external
  checkpoints against the presented chain.
- Add CLI/evidence integration so wrappers can verify runtime chains and attach
  optional runtime-chain and witness-checkpoint artifacts to agent evidence
  bundles and evidence manifests.
- Document the non-claim boundary: a local chain proves integrity only;
  independently controlled checkpoints provide bounded completeness evidence;
  Cairn does not decide whether a witness authority is socially or legally
  trusted.

## Impact

- **Files**: expected changes in `crates/cairn-core/src/verified/`,
  `crates/cairn-cli/src/`, `cairn-policy/default.ncl`, generated policy JSON,
  fixtures under `fixtures/`, CLI/release receipt tests, README and
  `docs/agent-run-receipts.md`.
- **Testing**: positive and negative tests for valid chains, tampering,
  truncation below checkpoint count, wrong subject, stale checkpoints, malformed
  records, secret-bearing summaries, unknown capture origins, and manifest/bundle
  linkage failures.
- **Non-claims**: this is not a network witness service, not a signature trust
  oracle, not enforcement before an action runs, and not a proof of agent
  correctness. Halo/AARM compatibility remains adapter-level; Cairn-owned hashes
  stay BLAKE3 unless an imported artifact explicitly declares another algorithm.
