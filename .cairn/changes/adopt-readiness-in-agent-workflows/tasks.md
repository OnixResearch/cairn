# Readiness-backed workflow tasks

## Contract and implementation

- [ ] [serial] [task:baseline] Capture legacy drain, direct readiness, original validators, and generated guidance across supported positive and negative profiles. r[cairn.agent_workflow_readiness.conformance]
- [ ] [serial] [task:contracts] [after:baseline] Define the typed per-change evidence manifest, new drain view, aggregate budgets, and exact guidance routes. r[cairn.agent_workflow_readiness.compatibility] r[cairn.agent_workflow_readiness.guidance]
- [ ] [serial] [task:composition] [after:contracts] Implement pure shared-observation composition with existing dependency order, ready-task order, and unresolved-action behavior. r[cairn.agent_workflow_readiness.shared] r[cairn.agent_workflow_readiness.order]
- [ ] [serial] [task:adapter] [after:composition] Add explicit CLI and MCP readiness-backed drain adapters with shared bounded observation and unchanged legacy receipts. r[cairn.agent_workflow_readiness.shared] r[cairn.agent_workflow_readiness.compatibility]
- [ ] [serial] [task:guidance] [after:adapter] Update apply, verify, drain, and archive descriptors to request readiness and exact artifact instructions without new authority. r[cairn.agent_workflow_readiness.guidance]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:guidance] Compare stage decisions, exact next actions, profile behavior, and dependency order against direct validators and readiness. r[cairn.agent_workflow_readiness.shared] r[cairn.agent_workflow_readiness.order]
- [ ] [serial] [task:negative] [after:positive] Test missing results, stale subjects, failed probes, malformed manifests, cycles, aggregate bounds, and independent stage-failure preservation. r[cairn.agent_workflow_readiness.compatibility] r[cairn.agent_workflow_readiness.conformance]
- [ ] [serial] [task:benchmark] [after:negative] Record paired complete workflow-loop tokens, diagnostic calls, and oracle outcomes with explicit evidence selection. r[cairn.agent_workflow_readiness.conformance]
- [ ] [serial] [task:docs] [after:benchmark] Document new-view selection, legacy differences, per-change inputs, exact instruction routes, and unchanged mutation preflight. r[cairn.agent_workflow_readiness.compatibility] r[cairn.agent_workflow_readiness.guidance]
- [ ] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.agent_workflow_readiness.conformance]
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed sync previews for both changed spec domains and an archive preview after implementation evidence passes. r[cairn.agent_workflow_readiness.conformance]

All tasks remain open. Save implementation evidence under `evidence/implementation/`.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
The shared verification and measurement rules are in `docs/agent-efficiency-roadmap.md`.
