## Why

Autonomous agents often finish work with prose claims, partial transcripts, or tool-specific logs that are difficult to compare, replay, or gate. Cairn already models deterministic lifecycle receipts and BLAKE3 provenance for plans, gates, sync, archive, and migration. Extending that receipt discipline to agent runs would let operators distinguish verified work from ungrounded claims and would make model/tool changes regression-testable. r[cairn.supersede.agent_run_receipts.schema]

## What Changes

- Specify a canonical agent-run receipt envelope for deterministic evidence about prompts, plans, tool manifests, mutations, verification, nondeterministic inputs, and verdicts. r[cairn.supersede.agent_run_receipts.schema]
- Require plan-before-mutation linkage using a stable `plan_hash` so side effects can be compared against the approved intent. r[cairn.supersede.agent_run_receipts.plan_link]
- Specify explicit capture and classification of nondeterministic inputs such as wall clock, network reads, model identity, environment, and remote APIs. r[cairn.supersede.agent_run_receipts.nondeterminism]
- Require completion gates that reject "done" receipts lacking verification evidence, dirty-state disclosure, or required artifact hashes. r[cairn.supersede.agent_run_receipts.completion_gate]
- Add replay fixture expectations for byte-stable canonical receipts and negative cases that prove missing evidence or hidden entropy fails deterministically. r[cairn.supersede.agent_run_receipts.replay]

## Impact

- **Files**: Future implementation likely touches `crates/cairn-core` canonical receipt models and hashing helpers, `crates/cairn-cli` or future MCP/agent shells for capture/orchestration, fixtures/goldens, documentation, and Nix checks.
- **Testing**: Positive byte-stability tests for repeated receipt generation, negative tests for missing plan linkage, missing verification evidence, and uncaptured nondeterminism, plus existing formatting, workspace tests, Clippy, Slotcar, `cairn validate --root .`, and `nix flake check`.
