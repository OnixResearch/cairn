# Final validation evidence

Date: 2026-07-29

## Passing checks

The following checks passed after the migration:

```text
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --features redb-registry -- -D warnings
cairn policy export --check
cairn validate --root .
cairn gate proposal type-lifecycle-evidence-identities --root .
cairn gate design type-lifecycle-evidence-identities --root .
cairn gate tasks type-lifecycle-evidence-identities --root .
cairn traceability coverage --root . --json
cairn release-readiness --root .
```

The workspace run included 408 `cairn-core` tests and 12 core doc tests. All compile-fail identity examples passed.

Release readiness returned `valid: true` and `verdict: pass`. It kept `onix_release_ready: false` because external heavyweight gates remain separate.

## Compatibility results

The stable agent Preserves algorithm still ignores object-key order and self-hash fields. Its policy-bound fixture hash changed after the required generated-policy refresh.

The refreshed values are:

- Generated policy hash: `d49884fab800116dc3510182dd3f0a6595e6b53ce2a4711c9c4c43e7d58f12b7`.
- Agent replay receipt: `c5281b1e6a5bbee623a71dc4de183eb632cfcfe79f3c524b08e10f8ee18b6b93`.
- Replay environment attestation: `c64f1b17213a85a866f7fa88a456089bc6804c5d7c9aaae4bab331208980231d`.
- Agent receipt hash: `1864671182af9a79cd57bef3c3d218449bc68977805572f55f36b87a27988c14`.
- Agent Preserves content hash: `b2561613ac6bb44bda72b2dcb29700fe086b5b0d958468b369497af8b61671df`.

The replay run passed five selected cases with no mismatch or entropy failure. Existing archive, collaboration, registry, readiness, and lifecycle receipt tests stayed green.

## Sync evidence

Cairn synced the accepted specification with mutation manifest hash `56b32ae62592c71e17058e30e934aed2c93878689bae79a7e9e8037ec2111c12`.

The sync receipt hash is `e5e7853f83af78654ffd82147320df49821c995c3c6441cddcb9ad7a86c968e5`.

## Bounded Nix blockers

`nix flake check path:$PWD -L` reached the `slotcar` check. Evaluation then failed because this environment could not fetch `https://github.com/brittonr/subwayrat.git`.

The direct Slotcar build had the same external fetch blocker. The local Tiger Style fallback started, but it rejected pre-existing non-canonical nested `Cargo.lock` paths.

These blockers do not come from the nominal identity code. The passing workspace tests and Clippy run are the local substitute evidence.

## Claim boundary

This evidence proves selected local checks, role admission, stable wire tests, and policy linkage only. It does not prove evidence truth, verifier soundness, or release eligibility.

<!--
r[verify cairn.nominal_evidence_identities.readiness.validation]
r[verify cairn.nominal_evidence_identities.receipt_chains.validation]
r[verify cairn.nominal_evidence_identities.lifecycle.validation]
r[verify cairn.nominal_evidence_identities.compatibility]
r[verify cairn.nominal_evidence_identities.final_checks.boundary]
-->
