## Tasks

- [ ] [serial] Register Trellis proof evidence as a generic evidence-chain readiness profile. r[cairn.stack_provenance_gates.trellis_proof_readiness.profile]
- [ ] [serial] Validate canonical envelope hash, Valence validation hash, Mantle binding hash, source/binary linkage, policy hashes, proof role, claim scope, and non-claims. r[cairn.stack_provenance_gates.trellis_proof_readiness.links]
- [ ] [serial] Support optional, required-recorded, and required-formal-proof policy modes. r[cairn.stack_provenance_gates.trellis_proof_readiness.policy_modes]
- [ ] [serial] Validate requirement/property scope and assumption policy for required proof evidence. r[cairn.stack_provenance_gates.trellis_proof_readiness.assumptions]
- [ ] [serial] Add positive fixtures for optional absent, optional recorded, required recorded, and required accepted formal-proof evidence. r[cairn.stack_provenance_gates.trellis_proof_readiness.positive]
- [ ] [serial] Add negative fixtures for missing canonical hash, stale Valence/Mantle hash, wrong role, missing scope, disallowed assumptions, malformed digest, unsupported scope, projection drift, source/binary mismatch, and overclaims. r[cairn.stack_provenance_gates.trellis_proof_readiness.negative]
- [ ] [serial] Run cargo tests, policy export check, lifecycle validation, and Trellis proof evidence readiness smoke. r[cairn.stack_provenance_gates.trellis_proof_readiness.validation]
