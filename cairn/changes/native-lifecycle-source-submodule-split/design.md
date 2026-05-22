# Design: Split native lifecycle CLI source into focused modules

## Module boundary

The implementation should replace the current large target `crates/cairn-cli/src/native.rs` with a facade or smaller entry module plus named implementation modules. The expected boundaries are:

- command/entry dispatch stays in the facade;
- cohesive responsibilities are split across modules for change creation, validation/doctor read models, gate execution, sync/archive mutation shell, and receipt presentation;
- shared fixture/helper code stays local to the module family unless an existing external consumer requires a narrow bridge.

## Compatibility

This is a mechanical maintainability slice. It must preserve:

- public CLI behavior and help output except for incidental source organization;
- deterministic receipt field names and hash inputs;
- policy schema semantics and generated-policy freshness;
- fixture/golden expectations unless a hash changes solely because source-path metadata legitimately changes;
- Tracey requirement references.

## Implementation notes

- Prefer `mod.rs` facade plus sibling modules when the current file becomes a directory.
- Prefer direct sibling imports over broad facade re-exports.
- Keep visibility at `pub(super)` or private unless a caller outside the family needs a small bridge.
- Run focused `cargo check`/test loops after extraction before full workspace gates.
- Stage new module files before flake/Nix checks that evaluate tracked sources.
