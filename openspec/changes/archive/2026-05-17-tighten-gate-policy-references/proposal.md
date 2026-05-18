## Why

Native gate commands currently check lifecycle artifact presence and task marker policy, but they do not inspect proposal/design/tasks text for requirement references or exemption hygiene. That leaves review gates weaker than the Tracey and sync rails now available.

## What Changes

- Strengthen `cairn gate` with pure reference and exemption checks.
- Keep CLI filesystem reads in the shell and deterministic gate decisions in `cairn-core`.
- Emit precise JSON receipt issues for missing requirement references, unknown references, and malformed exemptions.

## Scope

In scope: native gate checks for active change artifacts, tests, docs, Tracey markers, and policy-driven receipts.

Out of scope: hard dependency on external Tracey output or cross-repo AgentKit runtime coupling.

## Verification

Run focused gate tests, OpenSpec strict validation, Tracey bootstrap coverage, and the full Nix/Rust/Slotcar gate.
