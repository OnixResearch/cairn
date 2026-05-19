## Phase 1: Policy and receipt contract

- [x] [serial] Add policy/schema support for replay environment attestation facts. r[task cairn.supersede.replay_environment_attestation.policy]
- [x] [serial] Emit stable replay `environment_attestation` and attestation hash without host path/env leakage. r[task cairn.supersede.replay_environment_attestation.receipt]
- [x] [serial] Add optional agent receipt replay attestation hash validation/reporting. r[task cairn.supersede.replay_environment_attestation.agent]

## Phase 2: Docs, tests, and verification

- [x] [depends:receipt] Update docs, public schema tests, fixtures, and goldens. r[task cairn.supersede.replay_environment_attestation.docs]
- [x] [depends:docs] Run focused/full verification, sync specs, and archive. r[task cairn.supersede.replay_environment_attestation.verification] r[task cairn.supersede.replay_environment_attestation.archive]
