## Why

Cairn has accumulated many deterministic lifecycle primitives: validation, gates, sync/archive, semantic review, release evidence, registries, replay, and metrics rails. Operators and agents currently compose those primitives through prose procedures, repo-specific skills, and hardcoded CLI sequencing. That keeps the trusted core small, but it makes reviewed workflow customization hard and pushes too much orchestration into prompts or one-off shell glue.

Steel is a good fit for the programmable layer if Cairn keeps Rust as the authority boundary. A Steel script can describe a lifecycle plan, but Rust must validate policy, authorize host functions, execute effects, and emit deterministic receipts.

## What Changes

- Define Steel lifecycle orchestration as an optional Cairn subsystem for composing existing Cairn primitives into typed action plans.
- Require Steel scripts to run without ambient authority: they may request reviewed host functions and produce action envelopes, but Rust/Nickel/UCAN decide whether anything executes.
- Require deterministic, redacted receipts for Steel-driven runs, including script identity, requested capabilities, authorized actions, fallback behavior, and output hashes.
- Preserve the existing Rust/Nickel trust split: Rust owns validators/effects/receipts, Nickel owns static policy/config, and Steel owns reviewable orchestration glue.

## Impact

- **Files**: New Cairn change package under `cairn/changes/steel-lifecycle-orchestration/` and future implementation in `crates/cairn-core`, `crates/cairn-cli`, `cairn-policy`, and fixtures.
- **Testing**: Future implementation must include positive and negative fixtures for denied host functions, malformed action envelopes, deterministic receipt stability, and fallback/no-Steel operation.
