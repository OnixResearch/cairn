# Targeted lifecycle context tasks

## Contract and implementation

- [x] [serial] [task:baseline] Capture current full-specification, context, and workflow-instruction positive and negative baselines. r[cairn.targeted_context.conformance]
- [x] [serial] [task:contracts] [after:baseline] Define typed selector unions, exact source spans, validity fields, prerequisite references, and named bounds. r[cairn.targeted_context.selectors] r[cairn.targeted_context.spans] r[cairn.targeted_context.authority]
- [x] [serial] [task:selectors] [after:contracts] Reuse existing parsers to resolve unique requirement, declared task, and heading-path selections without synthetic identities. r[cairn.targeted_context.selectors] r[cairn.targeted_context.spans]
- [x] [serial] [task:context] [after:selectors] Preserve whole-document diagnostics, authority labels, required constraints, and bounded follow-up references in selected results. r[cairn.targeted_context.authority]
- [x] [serial] [task:transport] [after:context] Add strict CLI and MCP narrow reads through existing bounded no-follow readers with unchanged whole-artifact routes. r[cairn.targeted_context.transport]

## Acceptance and handoff

- [x] [serial] [task:positive] [after:transport] Test exact Unicode and CRLF spans, nested sections, fenced examples, requirement scenarios, task links, and transport parity. r[cairn.targeted_context.spans] r[cairn.targeted_context.transport]
- [x] [serial] [task:negative] [after:positive] Test duplicate IDs, unknown selectors, absent task IDs, ambiguous headings, invalid documents, unsafe paths, and oversized blocks. r[cairn.targeted_context.selectors] r[cairn.targeted_context.authority] r[cairn.targeted_context.transport]
- [x] [serial] [task:benchmark] [after:negative] Record the exact selected-block representation size, keeping token cost and any paired saving explicitly unavailable. r[cairn.targeted_context.conformance]
- [x] [serial] [task:docs] [after:benchmark] Document selector grammar, complete-block limits, prerequisite retrieval, document validity, and whole-artifact fallback. r[cairn.targeted_context.selectors] r[cairn.targeted_context.authority]
- [x] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.targeted_context.conformance]
- [x] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.targeted_context.conformance]

Executed evidence is retained under `evidence/implementation/`.
`representation-measurement.json` records a 2319-byte targeted task read. The
whole-change baseline route exited non-zero with no stdout on this fixture, so it
is recorded as **unavailable**, not as a zero-size baseline; no paired byte
comparison or saving is asserted, and `tokens.state = "unavailable"`.

Rails exercised on this branch: 4 core targeted-context tests, 6 CLI
`targeted_context` tests, `cairn policy export --check`, `cairn validate --root .`
and `cairn traceability coverage --root .` (2377/2377).

Scoped limitations, stated rather than waived: no live agent campaign was run, so
token cost and model-visible exposure remain unavailable. The repository-wide
strict Octet catalog is a separately tracked decision and is not claimed here.
