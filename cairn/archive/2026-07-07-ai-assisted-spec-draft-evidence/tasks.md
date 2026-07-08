## Phase 1: Draft provenance model

- [x] [serial] r[cairn.ai_spec_drafts.provenance] Add optional AI-assisted draft provenance fields for artifact kind, assistant class, redaction class, target requirement, validation command, review decision, and draft status.
- [x] [serial] r[cairn.ai_spec_drafts.not_verification] Enforce that AI draft provenance is not verification evidence by itself.
- [x] [serial] r[cairn.ai_spec_drafts.acceptance_paths] Define accepted completion paths through machine-check receipts, human-review decisions, or explicit docs-only rationale.

## Phase 2: Gate behavior and fixtures

- [x] [parallel] r[cairn.ai_spec_drafts.positive_fixtures] Add positive fixtures for AI-drafted Kani contract plus passing receipt and AI-drafted Cairn spec plus review acceptance.
- [x] [parallel] r[cairn.ai_spec_drafts.negative_fixtures] Add negative fixtures for AI-only proof claims, missing validation status, missing target requirement, and rejected draft reuse.
- [x] [serial] r[cairn.ai_spec_drafts.gate_diagnostics] Emit deterministic diagnostics for unsupported AI-only evidence claims.

## Phase 3: Documentation and validation

- [x] [serial] r[cairn.ai_spec_drafts.operator_docs] Document how agents should record draft provenance without storing raw prompts.
- [x] [serial] r[cairn.ai_spec_drafts.not_verification] Run fixture tests, gate tests, policy export check, and Cairn validation/gates before archive.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| AI-drafted contract with passing receipt is accepted | Positive fixture test |
| AI-drafted spec with review acceptance is accepted | Positive fixture test |
| AI-only proof claim fails | Negative fixture test |
| Missing validation status fails | Negative fixture test |
| Missing target requirement fails | Negative fixture test |
| Rejected draft reuse fails | Negative fixture test |
| Operator docs avoid raw prompts | Docs check |
