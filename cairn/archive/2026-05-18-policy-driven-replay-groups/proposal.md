# policy-driven-replay-groups

## Summary

Add policy-owned determinism replay groups so agents and CI can select stable named suites instead of tracking individual case IDs.

## Motivation

Cairn replay cases are now policy-owned, CI-consumable, and tied to agent receipts. The remaining usability gap is suite selection: operators still need to remember raw case IDs for smoke, agent, release, or full coverage. Case-level selection is precise but fragile as the policy grows.

## Scope

- Extend generated replay policy with named groups.
- Add `cairn determinism replay --group <name>` selection.
- Emit deterministic unknown-group failure receipts.
- Expose `available_groups` and `selected_groups` in replay receipts.
- Update docs, tests, stable schema policy, Tracey refs, and golden receipts.

## Non-goals

- Removing `--case`.
- Changing existing default replay behavior.
- Making agent receipt validation require groups in this slice.
