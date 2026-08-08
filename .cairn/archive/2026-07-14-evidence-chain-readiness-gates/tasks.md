## Tasks

- [x] [serial] Define typed generic evidence-chain readiness inputs and gate outputs. r[cairn.stack_provenance_gates.evidence_chain_readiness.contract]
- [x] [serial] Add pure validation for canonical envelope hash, Valence validation hash, Mantle binding hash, source/binary linkage, policy hashes, claim scope, projections, and non-claims. r[cairn.stack_provenance_gates.evidence_chain_readiness.core]
- [x] [serial] Add explicit profile registry with function-address as the first supported profile. r[cairn.stack_provenance_gates.evidence_chain_readiness.profile_registry]
- [x] [serial] Preserve optional and required policy modes for generic evidence-chain profiles. r[cairn.stack_provenance_gates.evidence_chain_readiness.policy_modes]
- [x] [serial] Emit deterministic evidence-chain readiness hashes that commit to typed upstream links and policy identity. r[cairn.stack_provenance_gates.evidence_chain_readiness.chain_hash]
- [x] [serial] Add positive fixtures for optional absent, optional present, required function-address, and non-function-address stub evidence. r[cairn.stack_provenance_gates.evidence_chain_readiness.positive]
- [x] [serial] Add negative fixtures for unknown profile, missing canonical hash, stale Valence/Mantle hash, malformed digest, source/binary mismatch, unsupported scope, stale policy hash, projection drift, and weakened non-claims. r[cairn.stack_provenance_gates.evidence_chain_readiness.negative]
- [x] [serial] Run cargo tests, policy export check, lifecycle validation, and stack smoke through the generic gate. r[cairn.stack_provenance_gates.evidence_chain_readiness.validation]
  - Core tests pass 291/291, focused generic tests pass 7/7, the complete workspace rail passes, strict core Clippy and policy/lifecycle gates pass, and Tracey reports 1292/1292 referenced requirements after sync.
