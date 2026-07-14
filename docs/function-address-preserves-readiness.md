# Function-address Preserves readiness

Cairn treats the canonical Kamacite Preserves receipt identity as the authority-bearing function-address carrier. The pure readiness core receives typed identities and link records; it does not read files, parse Rust functions, execute producers, or infer evidence truth.

## Typed chain

A present input binds:

- the exact `kamacite.function-address-receipt.v1` schema and `kamacite-function-address-receipt` role;
- the canonical Preserves receipt BLAKE3 identity;
- Valence receipt identity and its canonical-receipt link;
- a present Mantle binding identity and its Valence, source, and release-binary links;
- the accepted Valence policy identity;
- matching canonical, Valence, and Mantle claim scopes;
- required non-claims; and
- an optional JSON compatibility projection.

The evaluator delegates generic digest, linkage, policy, scope, projection, and non-claim validation to the evidence-chain readiness core. Function-address-specific validation adds exact schema, role, cross-producer claim-scope, and JSON-authority checks.

## Compatibility projection

The release-readiness CLI continues to accept documented JSON inputs. The Kamacite JSON document must identify its own wrapper hash, bind `canonical_preserves_receipt_hash`, and declare `projection_authority = "non-authoritative"`. A JSON projection cannot replace or drift from the canonical Preserves identity. Any shell read or normalization failure clears the evidence-chain hash before the receipt is emitted.

Optional mode skips only wholly absent function-address evidence. Once any Valence evidence is present, missing or malformed Preserves identity fails. Required mode fails on absence or any invalid link.

## Claim boundary

Passing proves typed canonical-receipt linkage and lifecycle-policy conformance only. It does not prove that caller-supplied digest text was measured from bytes, producer authorization, evidence truth, Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, runtime authority, or release eligibility. Measured-byte and signature authentication belongs to the separate authenticated-input change.

<!-- r[verify cairn.stack_provenance_gates.function_address_preserves_readiness.validation] -->
