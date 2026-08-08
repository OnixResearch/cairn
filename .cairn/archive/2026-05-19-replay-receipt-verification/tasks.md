## Phase 1: Change contract

- [x] [serial] Define replay receipt verification requirements and deterministic issue surface. r[task cairn.supersede.replay_receipt_verification.contract]

## Phase 2: Implementation

- [x] [depends:contract] Add `cairn determinism verify-receipt` read-only CLI command. r[task cairn.supersede.replay_receipt_verification.cli]
- [x] [depends:cli] Validate receipt hash, policy hash, selected cases/groups, entropy policy hash, and attestation hash. r[task cairn.supersede.replay_receipt_verification.validation]
- [x] [depends:validation] Emit stable verification receipts and deterministic issue codes. r[task cairn.supersede.replay_receipt_verification.receipt]

## Phase 3: Evidence

- [x] [depends:receipt] Update docs, public schema tests, and focused CLI tests. r[task cairn.supersede.replay_receipt_verification.docs]
- [x] [depends:docs] Run validation, Tracey, Rust tests, release-readiness, replay smokes, then sync/archive. r[task cairn.supersede.replay_receipt_verification.verification] r[task cairn.supersede.replay_receipt_verification.archive]
