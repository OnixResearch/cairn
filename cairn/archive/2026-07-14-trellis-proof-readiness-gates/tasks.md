## Tasks

- [x] [serial] Register Trellis proof evidence as a generic evidence-chain readiness profile. r[cairn.stack_provenance_gates.trellis_proof_readiness.profile]
- [x] [serial] Validate canonical envelope hash, Valence validation hash, Mantle binding hash, source/binary linkage, policy hashes, proof role, claim scope, and non-claims. r[cairn.stack_provenance_gates.trellis_proof_readiness.links]
- [x] [serial] Support optional, required-recorded, and required-formal-proof policy modes. r[cairn.stack_provenance_gates.trellis_proof_readiness.policy_modes]
- [x] [serial] Validate requirement/property scope and assumption policy for required proof evidence. r[cairn.stack_provenance_gates.trellis_proof_readiness.assumptions]
- [x] [serial] Add positive fixtures for optional absent, optional recorded, required recorded, and required accepted formal-proof evidence. r[cairn.stack_provenance_gates.trellis_proof_readiness.positive]
- [x] [serial] Add negative fixtures for missing canonical hash, stale Valence/Mantle hash, wrong role, missing scope, disallowed assumptions, malformed digest, unsupported scope, projection drift, source/binary mismatch, and overclaims. r[cairn.stack_provenance_gates.trellis_proof_readiness.negative]
- [x] [serial] Run cargo tests, policy export check, lifecycle validation, and Trellis proof evidence readiness smoke. r[cairn.stack_provenance_gates.trellis_proof_readiness.validation]
