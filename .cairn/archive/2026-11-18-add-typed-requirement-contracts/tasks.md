# Typed requirement contract tasks

## Implementation

- [x] [serial] [task:contracts] Author optional Nickel profile and all five exact native pilot contracts with program-observable mappings and explicit supported/informal portions. r[cairn.typed_requirements.finite_contract]
- [x] [serial] [task:core] [after:contracts] Add pure finite typing, normalization, evaluation, revision linkage and revision-bound independent review-declaration admission APIs. r[cairn.typed_requirements.finite_contract] r[cairn.typed_requirements.correspondence] r[cairn.typed_requirements.examples]
- [x] [serial] [task:cli] [after:core] Add read-only CLI inspection preserving informal absence and legacy command/default behavior. r[cairn.typed_requirements.inspection]
- [x] [serial] [task:tests] [after:cli] Run the authored positive and negative core and CLI consumer tests. r[cairn.typed_requirements.finite_contract] r[cairn.typed_requirements.correspondence] r[cairn.typed_requirements.examples] r[cairn.typed_requirements.inspection]

## Parent acceptance

- [x] [serial] [task:export] [after:tests] Generate the entire standalone typed-requirement Nickel export and check freshness. r[cairn.typed_requirements.finite_contract]
- [x] [serial] [task:review] [after:export] Obtain and retain an actual independent correspondence review for all five current mappings, with exact revision-bound review records under `evidence/implementation/reviews/`. Proposed mappings alone are not approvals. r[cairn.typed_requirements.correspondence]
- [x] [serial] [task:rails] [after:review] Run the remaining maintained formatting, dependency, traceability, purity and repository rails required by current policy. r[cairn.typed_requirements.inspection]
- [x] [serial] [task:handoff] [after:rails] Refresh final docs/changelog, run native lifecycle gates, sync/archive and integrate only after supported acceptance evidence is retained. r[cairn.typed_requirements.inspection]

Executed evidence retained under `evidence/implementation/`: registry export receipt
(881 rows, valid), per-pilot inspection outputs, review artifacts and their BLAKE3
identities, and `inspection-summary.json` showing all five pilots as
`reviewed_declaration` with `admitted_proof_evidence: not_supplied`.

Rails exercised on this branch: 8 core typed-contract tests, 4 CLI
`typed_requirement_contracts` tests, `cargo fmt`, `cairn policy export --check`,
`cairn validate --root .` (0 issues) and
`cairn traceability coverage --root .` (2371/2371).

Scoped limitations: the correspondence records are independent implementer
declarations, not signatures or human domain-expert reviews, and no proof
evidence is admitted. The repository-wide strict Octet catalog is a separately
tracked decision and is not claimed here.
