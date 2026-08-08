# Proposal: Test Harness Real Rails

## Summary

Extend the Cairn test harness from synthetic self-test cases to deterministic receipt wrappers for existing verification rails.

## Motivation

The current `test-harness` command proves receipt, replay, classification, artifact, and flake-triage behavior using synthetic cases. The next useful step is to bind that receipt model to real project checks so failures in `validate`, Tracey, policy export, and release-readiness can be preserved with replay commands and classified diagnostics.

## Scope

- Add explicit real-rail selection to `cairn test-harness`.
- Emit deterministic JSON receipts for supported rail invocations.
- Preserve the existing synthetic case mode for harness self-tests.
- Classify nonzero rail exits as product failures and launch failures as environment failures.
- Reject unknown rail names deterministically as harness failures.

## Non-goals

- Replacing native `validate`, `tracey coverage`, `policy export`, or `release-readiness` commands.
- Adding networked CI orchestration.
- Introducing shell/Python gate wrappers.
