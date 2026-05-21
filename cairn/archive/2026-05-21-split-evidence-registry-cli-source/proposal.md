# Proposal: Split evidence registry CLI source

## Why

`crates/cairn-cli/src/evidence_registry.rs` has become a dense registry command-family module. It now owns label validation, registry build/verify, mirror verification, consume/select matching, fetch-plan generation, and fetch-execute materialization in one file. That makes follow-up registry policy and materialization work harder to review than necessary.

## What Changes

- Split cohesive registry CLI implementation subfamilies into focused Rust modules under `crates/cairn-cli/src/evidence_registry/`.
- Keep `run_evidence_registry` and the public CLI behavior stable.
- Preserve existing JSON receipt shapes, issue codes, and command-line options.
- Run focused registry/evidence integration tests and full repository validation.

## Non-Goals

- No new registry product behavior.
- No changes to receipt schemas, policy inventory, or generated policy JSON.
- No broad shared-helper refactor outside the registry command family unless needed for compilation.
