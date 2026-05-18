# Proposal: Align Gate Policy with AgentKit

## Why

Cairn already carries a Nickel policy, but the native CLI only exposes a raw `gate-context` helper and does not encode AgentKit/OpenGate stage semantics as first-class gate behavior.

## What Changes

- Add AgentKit-compatible gate policy fields to Cairn's Nickel contracts/defaults.
- Add deterministic native `cairn gate <proposal|design|tasks> <change>` receipts.
- Keep AgentKit as a reference shape only; Cairn remains self-owned.

## Capabilities

### Modified Capabilities
- `cairn-supersession`: Native lifecycle commands include first-class gate semantics.

## Impact

- **Files**: `cairn-policy/*`, CLI/core policy/gate code, tests, README/spec docs.
- **APIs**: Adds `cairn gate <stage> <change> [--root PATH] [--policy PATH]`.
- **Dependencies**: No runtime AgentKit dependency.
- **Testing**: Policy freshness, Rust unit/CLI tests, OpenSpec strict validation, flake gate.
