# agent-receipt-replay-evidence

## Summary

Bind agent-run receipts to Cairn's deterministic replay oracle so CI and operators can prove an agent action used replay-checked Cairn surfaces instead of treating replay as a separate, unverifiable preflight.

## Motivation

Cairn now has portable receipts, policy-owned replay cases, broad replay coverage, and CI-consumable replay output. Agent receipts still validate their own completion evidence but do not cite the replay evidence that established Cairn's tool surfaces were deterministic. This leaves a provenance gap for agent/CI runs.

## Scope

- Extend the agent-run receipt contract with stable replay evidence fields.
- Validate replay evidence during `agent-receipt validate` and the MCP equivalent.
- Require referenced replay evidence to cite a passing replay verdict, current policy hash, and required replay case coverage.
- Emit deterministic validation issues and stable receipt fields for missing/stale/insufficient replay evidence.
- Update fixtures, docs, policy schemas, tests, and replay coverage.

## Non-goals

- Fetch remote replay artifacts.
- Trust arbitrary path artifacts without bounded JSON validation.
- Add new mutation-capable replay behavior.
