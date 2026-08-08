# determinism-entropy-inventory

## Why

Cairn replay now gives agents and CI policy-owned cases, groups, machine-readable failures, and agent receipt binding. The remaining determinism gap is provenance: replay can say a surface was stable, but not which host/environment entropy classes were considered, normalized, rejected, or intentionally disclosed.

Agents need a reviewed, machine-readable inventory so replay evidence can answer: "which entropy sources were part of the deterministic contract?" without leaking host paths or secrets.

## What Changes

- Add a policy-owned determinism entropy inventory under the generated replay policy.
- Emit stable replay receipt fields describing the inventory, assumptions, and policy hash.
- Replace hard-coded hidden environment entropy checks with policy-owned `reject` entries.
- Add deterministic failure receipts for policy-rejected entropy sources.
- Document how clock/date, locale/timezone, cwd/root paths, env vars, external command versions, filesystem order/metadata, and Nix/build context are handled.

## Capabilities

### Modified Capabilities

- `cairn.supersede.determinism_replay`: replay receipts disclose policy-owned entropy handling and fail deterministically when rejected entropy is present.
- `cairn.supersede.receipt_schema`: stable public replay receipt schema includes entropy inventory fields.

## Impact

- **Files**: `crates/cairn-cli/src/main.rs`, `crates/cairn-cli/tests/cli.rs`, `cairn-policy/default.ncl`, `cairn-policy/contracts.ncl`, generated policy, README/docs, accepted specs.
- **APIs**: Adds stable public replay receipt fields; no new CLI flags.
- **Dependencies**: None.
- **Testing**: Policy export check, focused replay/schema tests, hidden entropy negative test, full CLI/core tests, Cairn validate/gates, Tracey, release-readiness, replay smokes.
