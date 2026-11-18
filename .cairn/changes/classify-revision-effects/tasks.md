# Revision classification tasks

## Implementation

- [x] [serial] [task:policy] Author the standalone Nickel policy for comparison bounds, policy-fragment bounds and the disabled transfer state. r[cairn.revision_classification.bounds] r[cairn.revision_classification.transfer_disabled]
- [x] [serial] [task:core] [after:policy] Add exact revision bindings, common domain/observable correspondence, finite world search, both implication directions and replayable witnesses. r[cairn.revision_classification.requirement_relation] r[cairn.revision_classification.bounds]
- [x] [serial] [task:assumptions] [after:core] Report assumptions, activation and guarantee relations separately from the overall obligation. r[cairn.revision_classification.assumption_guarantee]
- [x] [serial] [task:policy-fragment] [after:core] Add the supported acceptance fragment, universe, containment, feasibility and replayable policy witnesses. r[cairn.revision_classification.policy_fragment]
- [x] [serial] [task:evidence] [after:assumptions] Add per-claim reusable/not-reusable decisions that keep old receipts stale and refuse strengthening or weakening reuse. r[cairn.revision_classification.evidence_reuse]
- [x] [serial] [task:transfer] [after:evidence] Define the future transfer record and explicit disabled transfer admission. r[cairn.revision_classification.transfer_disabled]
- [x] [serial] [task:cli] [after:transfer] Add read-only CLI classification with bounded reads, explicit rejections and no lifecycle writes. r[cairn.revision_classification.inspection]
- [x] [serial] [task:tests] [after:cli] Author positive and negative core and CLI consumer tests for every classification and stale-evidence refusal. r[cairn.revision_classification.conformance]

## Parent acceptance

- [x] [serial] [task:generate] [after:tests] Generate `cairn-policy/generated/revision-classification.json` from `cairn-policy/revision-classification.ncl` and check policy freshness. r[cairn.revision_classification.bounds]
- [x] [serial] [task:verify] [after:generate] Run focused core and CLI tests, formatting, workspace tests, Clippy, validation, traceability and the maintained Nix checks. Retain current-source evidence. r[cairn.revision_classification.conformance]
- [x] [serial] [task:handoff] [after:verify] Review the specification delta, refresh docs/changelog, run native lifecycle gates, and sync/archive only after supported acceptance evidence is retained. r[cairn.revision_classification.inspection]

Focused tests, validation, traceability, policy freshness, formatting and Clippy
were executed for this change; workspace-wide and Nix checks remain for the
composed integration branch.
