# Kani receipt gates

r[verify cairn.kani_receipt_gates.operator_docs]
r[verify cairn.kani_receipt_gates.operator_docs.visible]

Cairn treats Kani as an opt-in probe/CI producer. Default gates verify saved
receipt metadata; they do not run `cargo kani` inline.

## Receipt profile

Use schema `cairn.kani_receipt.v1`. A receipt must bind:

- `requirement_id`
- `source_scope`
- `content_hash`
- `policy_hash`
- `verifier_identity`
- `toolchain_identity`
- `property_status`
- `contract_state`
- `unwind_state`
- `stub_classification`
- `claim_scope`
- `evidence_role`
- `non_claims`

The validator preserves these profile fields in its deterministic report and
emits named issue codes such as `missing_field`, `stale_content_hash`,
`failed_verdict`, `unsupported_unbounded_claim`, `unverified_stub_promotion`,
and `weakened_non_claims`.

## Eligibility

A Kani receipt is eligible only when it is current for the source content,
targets the evaluated requirement, passed Kani, carries required non-claims, and
has the right evidence for its claim scope:

- `bounded`: requires bounded or complete unwind evidence.
- `contract`: requires passing contract evidence.
- `unbounded`: requires complete unwind or passing contract evidence.

Unverified or environmental stubs cannot be promoted to `property` evidence.

## Gate workflow

1. Run Kani before the Cairn gate in CI or an explicit probe step.
2. Save the Kani receipt with the source/content hashes and non-claims.
3. Run Cairn gates against the saved lifecycle artifacts and receipt metadata.

Cairn receipt validation is metadata and eligibility checking only. It does not
claim Kani, CBMC, solver, Rust-model, behavioral, whole-program, release, or
verifier soundness correctness.
