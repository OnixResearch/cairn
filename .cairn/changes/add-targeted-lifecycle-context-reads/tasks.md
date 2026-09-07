# Targeted lifecycle context tasks

## Contract and implementation

- [ ] [serial] [task:baseline] Capture current full-specification, context, and workflow-instruction positive and negative baselines before parser changes. r[cairn.targeted_context.conformance]
- [ ] [serial] [task:contracts] [after:baseline] Define typed selector unions, exact source spans, validity fields, prerequisite references, and named bounds. r[cairn.targeted_context.selectors] r[cairn.targeted_context.spans] r[cairn.targeted_context.authority]
- [ ] [serial] [task:selectors] [after:contracts] Reuse existing parsers to resolve unique requirement, declared task, and heading-path selections without synthetic identities. r[cairn.targeted_context.selectors] r[cairn.targeted_context.spans]
- [ ] [serial] [task:context] [after:selectors] Preserve whole-document diagnostics, authority labels, required constraints, and bounded follow-up references in selected results. r[cairn.targeted_context.authority]
- [ ] [serial] [task:transport] [after:context] Add strict CLI and MCP narrow reads through existing bounded no-follow readers with unchanged whole-artifact routes. r[cairn.targeted_context.transport]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:transport] Test exact Unicode and CRLF spans, nested sections, fenced examples, requirement scenarios, task links, and transport parity. r[cairn.targeted_context.spans] r[cairn.targeted_context.transport]
- [ ] [serial] [task:negative] [after:positive] Test duplicate IDs, unknown selectors, absent task IDs, ambiguous headings, invalid documents, unsafe paths, and oversized blocks. r[cairn.targeted_context.selectors] r[cairn.targeted_context.authority] r[cairn.targeted_context.transport]
- [ ] [serial] [task:benchmark] [after:negative] Record paired task and repair-loop tokens with follow-up reads and negative oracles for missing constraints. r[cairn.targeted_context.conformance]
- [ ] [serial] [task:docs] [after:benchmark] Document selector grammar, complete-block limits, prerequisite retrieval, document validity, and whole-artifact fallback. r[cairn.targeted_context.selectors] r[cairn.targeted_context.authority]
- [ ] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.targeted_context.conformance]
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.targeted_context.conformance]

All tasks remain open. Save implementation evidence under `evidence/implementation/`.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
The shared verification and measurement rules are in `docs/agent-efficiency-roadmap.md`.
