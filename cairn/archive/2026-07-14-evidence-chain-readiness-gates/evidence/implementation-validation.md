# Evidence-chain readiness implementation validation

## Completion contract

The change is complete only when Cairn has a pure typed evaluator with an explicit profile registry, exact optional/required semantics, deterministic chain identity, positive and negative fixtures, and passing core/lifecycle/traceability rails. Merely wrapping caller-controlled JSON, accepting unknown optional evidence, or hashing unvalidated digest strings is not completion.

## Bounded approach search

Three correlated serial lenses were evaluated because isolated review workers were unavailable:

1. **Typed dedicated core** — selected. A new pure module carries explicit identity/link DTOs, profile policy, deterministic issues, and chain hashing.
2. **Generic `serde_json::Value` validator** — rejected because it preserves caller-controlled shape ambiguity and weakens compile-time role/link separation.
3. **Immediate refactor of the established function-address evaluator** — deferred because it broadens regression risk before the generic contract exists. Function-address is instead the first exact registered profile; the Preserves readiness dependency owns its later adapter integration.

The adversarial audit requires unknown optional present evidence to fail rather than skip, required absence to fail, stale links to produce field-specific issues, set-like input order not to change chain identity, and overclaim text not to pass as a non-claim.

## Baseline

Before core edits:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test -p cairn-core function_address -- --nocapture
```

Result: 17 passed, 0 failed.

## Implemented scope

- `crates/cairn-core/src/verified/evidence_chain_readiness.rs` defines typed policy, profile, Valence link, Mantle link, projection, input, issue, and readiness records.
- Function-address is the first exact registered profile.
- The evaluator checks BLAKE3 spelling, canonical-envelope/Valence/Mantle/source/binary linkage, required policy identities, claim scope, projections, required non-claims, and overclaim fragments.
- Profile scope and non-claim registries are bounded and reject empty or duplicate entries; evidence rejects empty non-claims and projection roles.
- Optional absence skips; optional present and required evidence validate fully and fail closed.
- Passing present evidence receives an order-normalized BLAKE3 chain hash bound to the Cairn gate-policy identity.
- Payload bytes remain opaque; measurement, canonical receipt recomputation, and signature verification remain later shell-owned authentication work.

## Focused and core validation

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test -p cairn-core evidence_chain_readiness -- --nocapture
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo clippy -p cairn-core --all-targets -- -D warnings
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test -p cairn-core
```

Results:

- evidence-chain focused tests: 7 passed, 0 failed;
- complete `cairn-core`: 291 passed, 0 failed;
- complete workspace test rail: passed after accepted-spec sync;
- strict core Clippy: passed.

Positive coverage includes optional absence, required function-address, explicit non-function stub registration, and order-independent chain identity. Negative coverage includes unknown or malformed profiles, over-bound profile registries, empty or duplicate profile boundaries, missing/malformed identity, stale Valence/Mantle links, source/binary mismatch, stale policy, unsupported scope, empty or duplicate projection roles, projection drift, empty or weakened non-claims, overclaims, invalid optional present evidence, and required absence.

## Non-claims

These checks prove deterministic typed-link and lifecycle-policy behavior only. They do not prove that digest text was measured from bytes, that a producer is authorized, that evidence payloads are true, or that a release is eligible.
