# Change: repo-owned-traceability-rail

## Why

Cairn's maintained traceability gate should be a repo-owned deterministic rail, not a dependency on the external Tracey daemon or release binary. The current marker model is valuable: accepted `r[...]` requirements must stay connected to `r[impl ...]` and `r[verify ...]` evidence. The operational risk is the external tool boundary, transient daemon startup failures, and configuration that Cairn cannot validate through Nickel policy.

## What Changes

- Promote a Cairn-owned traceability coverage command as the authoritative gate.
- Keep `cairn tracey coverage` as a compatibility alias while the maintained command and docs move toward traceability wording.
- Move scan profiles into reviewed Nickel/policy-owned configuration so downstream repos can select source roots without depending on Tracey Styx config.
- Emit deterministic JSON/receipt fields for coverage counts, missing refs, dangling refs, profile identity, and input/policy hashes.
- Keep external Tracey optional as developer UX only; do not fork it unless a documented parity gap cannot be closed in the Cairn-owned checker.

## Impact

- **Files**: CLI traceability command, pure coverage core, Nickel policy/profile schema and generated policy JSON, Nix checks, release-readiness docs, README/workflow docs, and fixtures/tests.
- **Testing**: positive and negative core/CLI coverage fixtures, alias compatibility tests, policy export check, Cairn validation/gates, traceability coverage, workspace tests, Clippy, Slotcar, and flake check.
