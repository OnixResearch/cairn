## Tasks

- [ ] [serial] Define typed generic evidence-chain readiness inputs and gate outputs. r[cairn.stack_provenance_gates.evidence_chain_readiness.contract]
- [ ] [serial] Add pure validation for canonical envelope hash, Valence validation hash, Mantle binding hash, source/binary linkage, policy hashes, claim scope, projections, and non-claims. r[cairn.stack_provenance_gates.evidence_chain_readiness.core]
- [ ] [serial] Add explicit profile registry with function-address as the first supported profile. r[cairn.stack_provenance_gates.evidence_chain_readiness.profile_registry]
- [ ] [serial] Preserve optional and required policy modes for generic evidence-chain profiles. r[cairn.stack_provenance_gates.evidence_chain_readiness.policy_modes]
- [ ] [serial] Emit deterministic evidence-chain readiness hashes that commit to typed upstream links and policy identity. r[cairn.stack_provenance_gates.evidence_chain_readiness.chain_hash]
- [ ] [serial] Add positive fixtures for optional absent, optional present, required function-address, and non-function-address stub evidence. r[cairn.stack_provenance_gates.evidence_chain_readiness.positive]
- [ ] [serial] Add negative fixtures for unknown profile, missing canonical hash, stale Valence/Mantle hash, malformed digest, source/binary mismatch, unsupported scope, stale policy hash, projection drift, and weakened non-claims. r[cairn.stack_provenance_gates.evidence_chain_readiness.negative]
- [ ] [serial] Run cargo tests, policy export check, lifecycle validation, and stack smoke through the generic gate. r[cairn.stack_provenance_gates.evidence_chain_readiness.validation]
