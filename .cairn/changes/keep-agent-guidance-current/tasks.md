# Current entry guidance tasks

## Contract and implementation

- [ ] [serial] [task:baseline] Capture generator, installation, freshness, adapter, and actual host-loaded guidance baselines with positive and negative fixtures. r[cairn.agent_entry_guidance.conformance]
- [ ] [serial] [task:contracts] [after:baseline] Define typed entry descriptors, command-catalog identity, stage routes, mandatory boundaries, and named text bounds. r[cairn.agent_entry_guidance.generation] r[cairn.agent_entry_guidance.scope]
- [ ] [serial] [task:render] [after:contracts] Extend the existing generator with compact current entry guides and separate selected-stage instruction routes. r[cairn.agent_entry_guidance.generation] r[cairn.agent_entry_guidance.scope]
- [ ] [serial] [task:install] [after:render] Extend existing project-local manifests, preview plans, guarded installation, and freshness checks for entry-guide selection. r[cairn.agent_entry_guidance.install]
- [ ] [serial] [task:examples] [after:install] Add maintained command-example fixtures and explicit report-only inspection of identified copied guidance. r[cairn.agent_entry_guidance.examples]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:examples] Test current layouts, supported profiles, exact stage routes, reviewed installation, and semantic parity across supported adapters. r[cairn.agent_entry_guidance.generation] r[cairn.agent_entry_guidance.install] r[cairn.agent_entry_guidance.examples]
- [ ] [serial] [task:negative] [after:positive] Test stale catalogs, unknown profiles, small budgets, modified files, unsafe destinations, races, and interrupted-installation recovery. r[cairn.agent_entry_guidance.scope] r[cairn.agent_entry_guidance.install]
- [ ] [serial] [task:benchmark] [after:negative] Record paired actual entry and stage exposure, duplicate old-guide costs, and complete-task oracle outcomes. r[cairn.agent_entry_guidance.conformance]
- [ ] [serial] [task:docs] [after:benchmark] Update repository workflow source, example links, host support, explicit copied-guide inspection, and no-global-write guidance. r[cairn.agent_entry_guidance.examples] r[cairn.agent_entry_guidance.install]
- [ ] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.agent_entry_guidance.conformance]
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.agent_entry_guidance.conformance]

All tasks remain open. Save implementation evidence under `evidence/implementation/`.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
The shared verification and measurement rules are in `docs/agent-efficiency-roadmap.md`.
