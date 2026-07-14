# Trellis proof readiness

Cairn registers Trellis proof evidence as a profile of the generic evidence-chain readiness core. The profile validates typed Kamacite canonical-envelope, Valence validation, Mantle binding, source/binary, policy, projection, role, requirement/property scope, assumption, claim-scope, and non-claim links without parsing proof payloads. r[impl cairn.stack_provenance_gates.trellis_proof_readiness.profile] r[impl cairn.stack_provenance_gates.trellis_proof_readiness.links]

Modes are `optional`, `required-recorded`, and `required-formal-proof`. Recorded-only evidence can satisfy recorded mode but cannot satisfy formal-proof mode. Required scope IDs must be present and every declared assumption must be explicitly allowed. r[impl cairn.stack_provenance_gates.trellis_proof_readiness.policy_modes] r[impl cairn.stack_provenance_gates.trellis_proof_readiness.assumptions]

Passing Trellis proof readiness proves lifecycle policy conformance for declared proof evidence only. It does not prove verifier soundness, compiler correctness, whole-program safety, Rust semantic correctness, downstream system correctness, or release eligibility. r[verify cairn.stack_provenance_gates.trellis_proof_readiness.validation]
