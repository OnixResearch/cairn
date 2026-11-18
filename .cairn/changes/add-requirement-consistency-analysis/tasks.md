# Requirement consistency analysis tasks

## Implementation

- [x] [serial] [task:policy] Author the standalone Nickel analysis policy with the enabled query vocabulary and every bound. r[cairn.requirement_consistency.policy]
- [x] [serial] [task:core] [after:policy] Add the versioned case/scope model over admitted typed contracts, including exact revisions, contract-local variables and explicit cross-contract observable correspondence. r[cairn.requirement_consistency.scope]
- [x] [serial] [task:queries] [after:core] Add the bounded four-query enumeration with the five explicit statuses, retained exploration basis, conflict extraction and pairwise minimality. r[cairn.requirement_consistency.queries] r[cairn.requirement_consistency.statuses]
- [x] [serial] [task:nonvacuity] [after:queries] Add vacuous-implication, unreachable-activation, redundant-within-model, single-instance quantifier and indistinguishable-state findings that name exact revisions, domains and fragments. r[cairn.requirement_consistency.nonvacuity]
- [x] [serial] [task:replay] [after:nonvacuity] Add total-table witnesses and the pure replay validator that re-admits the case and cross-checks the typed-contract evaluator. r[cairn.requirement_consistency.replay]
- [x] [serial] [task:cli] [after:replay] Add read-only CLI inspection with native registry verification, optional retained-witness replay and explicit proof/lifecycle non-authority. r[cairn.requirement_consistency.inspection]
- [x] [serial] [task:tests] [after:cli] Author positive and negative core and CLI consumer tests, including a genuine vacuity case that a predicate-name check cannot distinguish. r[cairn.requirement_consistency.queries] r[cairn.requirement_consistency.nonvacuity] r[cairn.requirement_consistency.replay] r[cairn.requirement_consistency.inspection]

## Parent acceptance

- [ ] [serial] [task:export] [after:tests] Export the standalone analysis policy to `cairn-policy/generated/consistency-analysis.json` and confirm the checked artifact matches `cairn-policy/consistency-analysis.ncl`. r[cairn.requirement_consistency.policy]
- [ ] [serial] [task:cases] [after:export] Run the authored core and CLI consumer tests and retain the actual executed outputs. r[cairn.requirement_consistency.queries] r[cairn.requirement_consistency.replay] r[cairn.requirement_consistency.inspection]
- [ ] [serial] [task:pilot] [after:cases] Exercise the read-only inspection against a current native registry for the five pilot requirements and retain the report, per-query statuses and one replayed witness under `evidence/implementation/`. r[cairn.requirement_consistency.inspection] r[cairn.requirement_consistency.replay]
- [ ] [serial] [task:rails] [after:pilot] Run the maintained formatting, dependency, traceability, purity and repository rails required by current policy. r[cairn.requirement_consistency.statuses]
- [ ] [serial] [task:handoff] [after:rails] Refresh final docs/changelog, run native lifecycle gates, sync and archive only after supported acceptance evidence is retained. r[cairn.requirement_consistency.inspection]

## Pending parent commands

```sh
nix develop -c nickel export --format json cairn-policy/consistency-analysis.ncl \
  > cairn-policy/generated/consistency-analysis.json

cairn requirement-analysis inspect --root . \
  --case evidence/implementation/consistency-case.json \
  --registry target/requirement-registry.json \
  --witness evidence/implementation/consistency-witness.json
```

No test or check was executed while authoring this change. Scoped limitations: the
analysis is advisory; a found model is not unbounded validity, reactive
realizability, prose correspondence, implementation correctness or proof evidence.
No external solver is executed, no obligation is resolved and no gate verdict
changes. Without an explicitly admitted correspondence, equal observable names in
different contracts stay independent, and correspondence is reported as
`not_reviewed` unless a retained independent review declaration is supplied.
