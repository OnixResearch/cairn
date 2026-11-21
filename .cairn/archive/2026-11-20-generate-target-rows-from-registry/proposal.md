## Why

The declarative codebase loop binds each target to exact registry bytes, but a consumer still copies six fields from `cairn requirement-registry export` into its targets file by hand, hashes included. That copy is the one remaining hand-maintained lock: a stale copy fails closed only after every other input is assembled, and nothing in the authoring contract can regenerate it.

## What Changes

- Add `requirement_from_registry registry requirement_id` and `specification_from_registry registry requirement_id` to `contracts/assurance-targets.ncl`. Each selects exactly one row from the parsed `cairn requirement-registry export` and fails closed when the export does not declare `cairn.content-bound-requirement-registry.v1` or does not contain exactly one row for the id.
- Add a registry-derived fixture that reproduces the existing octet example targets and must export byte-identical JSON, plus missing, duplicate and wrong-schema negatives, all wired into the `assurance-targets-contracts` flake check.
- Document the derivation in `docs/convergence.md` and record it in `CHANGELOG.md`.

## Impact

- **Current consumers**: Octet's `standards/requirements/cairn-pilot/targets.ncl`, which replaces its copied rows and contract specification with helper calls.
- **Immediate outcome**: the same exported targets JSON from derived rows, proven byte-identical by the flake check.
- **Durable contribution**: targets rows and the contract specification can be regenerated from the registry export rather than hand-copied.
- **Maintenance owner**: Cairn verified-core maintainers own the helpers; consumers own their registry export and targets.
- **Files**: `contracts/assurance-targets.ncl`, `fixtures/assurance-targets/registry-derived/`, `fixtures/assurance-targets/negative-registry-*.ncl`, `flake.nix`, `docs/convergence.md`, `CHANGELOG.md`.
- **Testing**: the flake check exports the derived fixture and compares it byte for byte with the hand-copied one, and requires all three new negatives to fail; `cargo test -p cairn-core --lib assurance_targets`, `policy export --check`, `validate`, the three lifecycle gates and the full flake check.

Rust admission is unchanged: it still re-verifies all six row fields and the contract specification against the registry, so a derived target is admitted by the same checks as a copied one. No runtime command starts Nickel and no gate changes.
