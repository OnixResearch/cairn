## MODIFIED Requirements

### Requirement: Determinism replay oracle

The system MUST provide a read-only replay receipt verification command so saved replay evidence can be validated later by agents, CI, and release gates without rerunning replay cases.

#### Scenario: Saved replay receipt verifies successfully

r[cairn.supersede.replay_receipt_verification.contract]
r[cairn.supersede.replay_receipt_verification.cli]
r[cairn.supersede.replay_receipt_verification.validation]
r[cairn.supersede.replay_receipt_verification.receipt]
r[cairn.supersede.replay_receipt_verification.schema]

- GIVEN a saved replay receipt produced by `cairn determinism replay --root <root>`
- WHEN `cairn determinism verify-receipt <receipt.json> --root <root>` runs
- THEN the command emits a deterministic verification receipt
- AND it validates the saved receipt hash, current replay policy hash, entropy policy hash, selected cases, selected groups, and environment attestation hash
- AND it reports `valid: true` and `verdict: pass` without mutating the workspace or rerunning replay cases
- r[task cairn.supersede.replay_receipt_verification.contract]
- r[task cairn.supersede.replay_receipt_verification.cli]
- r[task cairn.supersede.replay_receipt_verification.validation]
- r[task cairn.supersede.replay_receipt_verification.receipt]
- r[verify cairn.supersede.replay_receipt_verification.cli]
- r[verify cairn.supersede.replay_receipt_verification.validation]
- r[verify cairn.supersede.replay_receipt_verification.receipt]

#### Scenario: Saved replay receipt failures are deterministic

r[cairn.supersede.replay_receipt_verification.failures]
r[cairn.supersede.replay_receipt_verification.secret_safe]

- GIVEN a saved replay receipt with malformed JSON, missing modern fields, stale policy hash, tampered receipt hash, invalid selected case/group, invalid entropy policy hash, or malformed environment attestation hash
- WHEN `cairn determinism verify-receipt <receipt.json> --root <root>` runs
- THEN the command emits a deterministic failure receipt with stable issue codes
- AND the failure receipt does not expose raw environment values, credentials, absolute checkout roots, or machine-specific temporary paths in hash material
- r[task cairn.supersede.replay_receipt_verification.validation]
- r[task cairn.supersede.replay_receipt_verification.receipt]
- r[verify cairn.supersede.replay_receipt_verification.failures]

#### Scenario: Replay receipt verification is documented and archived

r[cairn.supersede.replay_receipt_verification.docs]
r[cairn.supersede.replay_receipt_verification.verification]
r[cairn.supersede.replay_receipt_verification.archive]

- GIVEN implementation, docs, tests, and specs are updated
- WHEN the change is completed
- THEN Cairn validation, Tracey coverage, policy export checks, focused/full Rust tests, release-readiness, and replay smokes pass before archive
- r[task cairn.supersede.replay_receipt_verification.docs]
- r[task cairn.supersede.replay_receipt_verification.verification]
- r[task cairn.supersede.replay_receipt_verification.archive]
- r[verify cairn.supersede.replay_receipt_verification.docs]
- r[verify cairn.supersede.replay_receipt_verification.verification]
- r[verify cairn.supersede.replay_receipt_verification.archive]
