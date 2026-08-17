# Proposal: Adopt Nickel 1.17 for lifecycle policy

## Why

Cairn policy checks currently resolve to Nickel `1.16.0`. Stack lifecycle policy must use the reviewed Nickel `1.17.0` cohort.

## What Changes

- Pin Nickel `1.17.0` at upstream commit `1320a983e6c3d1e2fb53dd2464b084b4903b1426`.
- Use the pin for policy export, policy freshness, lifecycle contracts, fixtures, and self-dogfood.
- Add positive and negative compatibility fixtures for exports, contracts, merge behavior, and diagnostics.
- Record the exact evaluator identity in relevant evidence outputs.

## Impact

Cairn policy generation and validation will use the stack Nickel cohort. Cairn retains lifecycle, gate, receipt, and readiness semantics.

## Dependencies

This change uses the upstream Nickel `1.17.0` release.

## Non-goals

- Do not change lifecycle semantics only because diagnostics change.
- Do not update unrelated dependencies only to obtain Nickel.
- Do not claim source correctness from policy export success.
