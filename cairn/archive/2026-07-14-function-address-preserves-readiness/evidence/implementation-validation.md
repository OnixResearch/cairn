# Function-address Preserves readiness validation

## Completion contract

Completion requires canonical Preserves identity to be first-class, exact schema/role/link/scope/non-claim checks in a pure core, JSON compatibility that cannot become authoritative, positive and negative core/CLI fixtures, accepted-spec sync, and passing focused, policy, lifecycle, traceability, and flake rails. A lexical digest check or an unbound JSON receipt is not completion.

## Bounded approach search

Three correlated serial lenses were evaluated because isolated workers were unavailable:

1. **Tighten only the legacy JSON evaluator** — rejected because it would keep compatibility serialization as the semantic core.
2. **Use only the generic evidence-chain evaluator** — rejected because generic linkage does not encode the exact Kamacite schema, function-address role, producer claim-scope agreement, or JSON authority posture.
3. **Compose a typed function-address Preserves adapter over the generic core** — selected. The adapter validates exact function-address metadata and delegates generic hash/link/policy/projection/non-claim invariants without parsing function records.

Adversarial review added explicit rejection of authoritative JSON projections, non-present Mantle dispositions, stale Valence or Mantle claim scopes, malformed mixed-type non-claim arrays, missing canonical identity, schema/role drift, projection drift, and overclaims. Shell-owned read failures clear any previously computed chain hash.

## Baseline

Before implementation:

```text
nix develop -c cargo test -p cairn-core function_address -- --nocapture
```

Result: 18 passed, 0 failed.

## Implemented scope

- `function_address_preserves_readiness.rs` defines typed canonical identity, Valence/Mantle links, policy identity, claim-scope links, non-claims, and JSON projection metadata.
- The pure evaluator composes the generic evidence-chain readiness core and emits its deterministic chain hash only after every generic and function-address-specific check passes.
- The established release-readiness shell remains the JSON compatibility adapter. Kamacite JSON must bind wrapper and canonical identities and declare `non-authoritative` projection status.
- Existing Valence and Mantle checks remain in force; their receipt identities and scopes must agree with the canonical Preserves receipt.
- Measured-byte and producer-signature authentication remain outside this change and are not claimed.

## Focused validation

```text
nix develop -c cargo test -p cairn-core function_address -- --nocapture
nix develop -c cargo test -p cairn-cli --test release_receipts function_address -- --nocapture
```

The core function-address family passes 22 tests after implementation, the focused release-receipt family passes 5/5, the complete core passes 295/295, and the complete workspace test rail passes. Policy export is fresh, lifecycle validation and gates pass, Tracey reports 1298/1298 referenced requirements, and `nix flake check` passes after archive. Positive coverage includes optional absent, optional present, required complete typed evidence, direct release-readiness input, and accepted digest spellings. Negative coverage includes missing canonical hash, stale Valence/Mantle identity or scope, non-present Mantle disposition, unsupported schema, malformed digest, role mismatch, projection drift, authoritative JSON, malformed non-claims, shell read failure, unsupported scope, weakened boundaries, and overclaims.

## Non-claims

Passing does not establish byte measurement, producer authorization, evidence truth, Rust semantics, compiler correctness, whole-program safety, verifier soundness, runtime authority, or release eligibility.
