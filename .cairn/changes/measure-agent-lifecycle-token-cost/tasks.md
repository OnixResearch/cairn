# Agent efficiency measurement tasks

## Contract and implementation

- [ ] [serial] [task:baseline] Capture current harness, readiness, and MCP positive and negative baselines before implementation. r[cairn.agent_efficiency.conformance]
- [ ] [serial] [task:contracts] [after:baseline] Define typed Nickel scenario, run, accounting, cohort, and comparison contracts with named bounds. r[cairn.agent_efficiency.cohort] r[cairn.agent_efficiency.accounting]
- [ ] [serial] [task:core] [after:contracts] Implement pure admission, checked aggregation, comparable-cohort rules, and unavailable measurement results. r[cairn.agent_efficiency.accounting] r[cairn.agent_efficiency.cohort]
- [ ] [serial] [task:oracles] [after:core] Add fresh-root scenario oracles for correct completion, correct blocking, false completion, failure, and interruption. r[cairn.agent_efficiency.outcomes]
- [ ] [serial] [task:shell] [after:oracles] Add bounded offline CLI import and comparison without model calls, implicit transcripts, or raw-content disclosure. r[cairn.agent_efficiency.import]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:shell] Test exact provider and tokenizer accounting, distinct cached input, complete cohorts, and deterministic reports. r[cairn.agent_efficiency.accounting] r[cairn.agent_efficiency.conformance]
- [ ] [serial] [task:negative] [after:positive] Test duplicate calls, overflow, mixed cohorts, missing usage, false success, unsafe paths, and truncated observations. r[cairn.agent_efficiency.cohort] r[cairn.agent_efficiency.outcomes] r[cairn.agent_efficiency.import]
- [ ] [serial] [task:baseline-corpus] [after:negative] Record source-bound baseline scenarios for all planned consumers and separate representation replays from authorized live-host observations. r[cairn.agent_efficiency.conformance]
- [ ] [serial] [task:docs] [after:baseline-corpus] Document accounting conventions, oracle scope, host authority, sampling budgets, raw-data privacy, and unavailable-measurement behavior. r[cairn.agent_efficiency.accounting] r[cairn.agent_efficiency.outcomes] r[cairn.agent_efficiency.import]
- [ ] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.agent_efficiency.conformance]
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.agent_efficiency.conformance]

All tasks remain open. Save implementation evidence under `evidence/implementation/`.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
The shared verification and measurement rules are in `docs/agent-efficiency-roadmap.md`.
