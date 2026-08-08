# Design: Strengthen Nickel Contract Ownership

## Context

Current Cairn policy sources define useful Nickel contracts, but several high-value invariants remain outside Nickel:

- public receipt fields are documented in README and asserted by Rust tests, but not described as Nickel contracts;
- `Policy` parsing keeps many policy values as strings after generated JSON is loaded;
- task marker recognition is duplicated in Rust string checks;
- lifecycle artifact requirements are partly represented in policy but gate/plan logic still hardcodes proposal/design/tasks/spec semantics;
- gate exemption markers are checked with substring presence rather than a parsed, policy-owned shape.

The goal is not to execute Nickel at runtime. Nickel remains a development/export step that produces deterministic JSON, while Rust remains the pure evaluator of repository snapshots.

## Contract additions

### Receipt schema contracts

Add contracts under `cairn-policy/contracts.ncl` for:

- `Blake3Hex`: exact lowercase/uppercase-safe 64-character hexadecimal string contract;
- `GateReceipt`: `change`, `stage`, `verdict`, `valid`, `issues`, `policy`, `layout`, `policy_hash`, `input_hash`, `receipt_hash`;
- `GateIssue`: `message`, `scope`, `promoted`, and any future policy-owned finding fields that are promoted into the public receipt;
- `LifecycleReceipt`: shared sync/archive top-level fields, with sync additionally requiring `delta_specs`;
- `LifecycleAction`: stable `kind`, `path`, `description` keys;
- `MigrationReceipt`: migration top-level fields and manifest hashes;
- `MigrationAction`: stable `source`, `destination`, `source_hash`, `destination_hash` keys.

The exported default policy should include a `receipt_schemas` section that lists required top-level and action keys for each public command family. Rust tests can then compare emitted receipt keys to generated policy data rather than duplicating README-only lists.

### Semantic policy invariants

Strengthen contracts so malformed policy fails during export:

- `version` MUST match `SUPPORTED_POLICY_VERSION` for this policy generation line, initially `1`;
- `receipt_hash_policy.algorithm` MUST be `BLAKE3`;
- `receipt_hash_policy.canonicalization` MUST be `serde_json_compact_sorted_keys`;
- `schemas`, `lifecycle_stages`, `gate_policy.stages`, `report_contract.verdicts`, and marker arrays MUST be non-empty;
- stage, marker id, marker token, artifact id, and receipt key arrays SHOULD be duplicate-free;
- lifecycle artifact `requires` references MUST name known artifact IDs within the same schema.

Where Nickel cannot express an invariant cleanly or readably, keep a small deterministic export helper check beside `tools/export-policy.py`, but keep the invariant described by the Nickel policy contract surface.

### Generated vocabularies

Extend generated policy data so Rust no longer hardcodes policy vocabulary that already belongs to Nickel:

- task marker tokens/kinds drive task marker recognition;
- lifecycle artifact schemas drive required artifact/stage checks;
- finding defaults drive gate issue metadata;
- receipt schema declarations drive public receipt regression checks.

`cairn-core` should parse these into typed or enum-backed structures where practical while keeping pure evaluation logic in Rust.

### Exemption marker structure

Represent gate exemption markers as parsed data before validation. The policy contract should define required marker keys:

- `reason` non-empty;
- `owner` non-empty;
- `until` or exported canonical `expires` as an ISO date string;
- optional `scope` constrained to policy-owned scope values.

The gate should reject malformed or expired/invalid-date exemption markers deterministically. Expiry evaluation should only use explicit inputs if wall-clock behavior is added later; this change should avoid ambient time.

## FCIS boundary

- Nickel owns policy shape, vocabulary, defaults, and schema declarations.
- The CLI shell owns file reads, Nickel export/check invocation in development rails, and JSON rendering.
- `cairn-core` owns pure parsing/evaluation of generated policy data and repository snapshots.
- No runtime command invokes Nickel.

## Verification

Focused checks:

```bash
python3 tools/export-policy.py --check
cargo test --workspace
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- gate proposal strengthen-nickel-contracts --root .
nix run path:$PWD#cairn -- gate design strengthen-nickel-contracts --root .
nix run path:$PWD#cairn -- gate tasks strengthen-nickel-contracts --root .
```

Full gate after implementation:

```bash
nix develop -c bash -c 'python3 tools/export-policy.py --check && cargo fmt --check && cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo tigerstyle check --output-format json --artifact-dir artifacts/slotcar-run'
nix flake check path:$PWD -L
```
