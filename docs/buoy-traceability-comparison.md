# Buoy traceability comparison qualification

This page records the bounded comparison experiment from
[issue #3](https://github.com/OnixResearch/cairn/issues/3).
The comparison is advisory and report-only.

## Scope and runtime

Cairn executes its ordinary read-only `requirement-registry export`,
`traceability export`, and `traceability verify` commands against saved
fixture copies in a private temporary directory.
The retained fixture originals are hashed before and after execution.
Each executed case retains its final verification stdout, stderr, exit code,
and identity; intermediate export receipts are not promoted to evidence.

Upstream Buoy was not executed. The environment has no realized pinned Buoy
binary or Nix derivation. The reviewed consumer package sets
`doCheck = false`; full Devenv acceptance needs private Atoll access; and a
first source Cargo build downloads public registry dependencies. The declared
upstream observations are therefore pinned expectations, not results.

Pinned identity:

- repository: `https://github.com/Chelis-Lang/buoy`
- revision: `01096fbdd0b9bf39b2fec468f90e7b1db958f615`
- recursive source content hash:
  `sha256-1+/x0hY3+YMYgLVXs0uoUmA1VPPlg5i/7qt19NrBruc=`
- Rust toolchain: `nightly-2026-04-16`
- digest feature: `blake3-256`
- read-only commands: `static`, `trace`, `package-identity`
- report schemas: `buoy/trace-report/v1`, `buoy/package-identity/v1`, `chelis-static-report/v3`

The complete identity and evidence links are in
`fixtures/buoy-comparison/v1/manifest.json`.

## Run the qualification

```bash
cairn traceability compare \
  --root . \
  --fixtures fixtures/buoy-comparison/v1 \
  --out target/buoy-comparison/receipt.json \
  --observations target/buoy-comparison/observations
```

The dedicated Nix check is:

```bash
nix build .#checks.x86_64-linux.buoy-traceability-comparison --print-out-paths
```

It executes the fixture inventory, checks actual stdout, stderr, and exit
dispositions, and verifies that the retained fixture bytes are unchanged.

## Case dispositions

| Case | Executed native result | Declared upstream | Comparison |
| --- | --- | --- | --- |
| `clean` | current | current | agreement |
| `missing_marker` | stale item | stale item | agreement |
| `duplicate_marker` | stale item | stale item | agreement |
| `moved_span` | stale item | stale item | agreement |
| `changed_requirement_body` | stale requirement | stale requirement | agreement |
| `schema_version_drift` | unsupported | unsupported | comparison unsupported |
| `unicode_crlf` | current | unsupported | expected divergence |
| `cfg_not_any` | current | unsupported | expected divergence |
| `historical_only` | not executed | unsupported | comparison unsupported |
| `ambiguous_mapping` | not executed | unsupported | comparison unsupported |
| `unavailable_or_truncated_output` | not executed | unsupported | comparison unsupported |

Known disagreements are named, not silently waived:
`exact_byte_span_vs_normalized_semantic_identity` for Unicode/CRLF and
`rust_cfg_scope_not_in_exact_marker_v1` for cfg. Structural cfg interpretation
belongs to the separately versioned Rust-item profile.

Remaining unsupported upstream dimensions include historical-only records,
ambiguous mapping refusal, and truncated or unavailable upstream output. No
upstream result is inferred for them.

## Identities and freshness

Native Cairn receipts and declared Buoy observations use distinct BLAKE3
domains and retain their producer schemas. Equal digest bytes across schemas
are not reported as agreement.

A comparison receipt binds the pinned upstream identity, mapping ID and
projection version, policy bytes, manifest bytes, and every executed fixture
tree. Changing a source revision, toolchain, feature, command surface, mapping,
policy, manifest, or fixture changes the comparison identity and invalidates a
prior receipt.

A receipt remains valid when every selected case has an executed native
result or an explicit unsupported disposition, but `complete` is false while
unsupported cases remain. Unsupported cases never count as passing cases.

Mutation qualification requires each mutated case to change both its declared
upstream semantic class and its native expectation from the base case. The
evaluator also re-validates restated-buoy and restated-native clones; accepting
either is a comparison failure.

## Measurements

The dedicated Nix check retains wall-clock elapsed seconds for the
comparison command and the canonical receipt byte size in its output.
The check executes eight native fixtures with the ordinary registry, export, and verify commands as each case requires.
Setup cost is one Cairn build plus the check, with no upstream build, no
private acceptance lane, and no network after pinned inputs are realized.
No independent upstream defect is claimed: the retained disagreements are
declared projection differences, and the executed evidence covers Cairn's own
mutation detection only.

## Decision

Retain the bounded declared comparator and its mutation corpus.
Do not adopt it as a runtime traceability authority.
Re-execute upstream observations only after the pinned source closure is
realized offline and can be inspected without network access.

## Claim boundary

A valid comparison receipt proves only that executed Cairn trace-index
observations match independently declared expectations for saved fixture
copies and that declared upstream observations satisfy the pinned projection
contract. It does not prove Buoy execution, cross-implementation agreement,
requirement satisfaction, implementation correctness, test adequacy, proof
soundness, release eligibility, lifecycle readiness, or behavioral
correctness. Existing traceability gate meanings are unchanged.
