# Scoped claim derivation tasks

## Implementation

- [x] Author the optional Nickel claim policy with admitted kinds, propositions, rule identities/versions, assurance scopes and bounds. (cairn.scoped_claims.bounds)
- [x] Add versioned claim identity over native requirement revision, interpretation, contract, subject, environment, bounds, kind and proposition. (cairn.scoped_claims.identity)
- [x] Add the closed acyclic rule vocabulary and bounded deterministic replay with unresolved/supported/refuted/disputed states. (cairn.scoped_claims.rules, cairn.scoped_claims.states)
- [x] Add native producer adapters that re-run the authoritative completion evaluator and ast-grep receipt validator and admit only their conclusions. (cairn.scoped_claims.native_producers)
- [x] Enforce exact scope separation and explicit rejection of unsupported rules, missing premises, cycles and exhausted bounds. (cairn.scoped_claims.scope, cairn.scoped_claims.bounds)
- [x] Add the read-only `claim-derivation explain` CLI with explicit advisory boundary and deterministic explanation. (cairn.scoped_claims.inspection, cairn.scoped_claims.authority)
- [x] Author positive and negative core/CLI consumer tests using actual native completion inputs without executing them.

## Parent acceptance

- [ ] Generate the entire standalone scoped-claim Nickel export to `cairn-policy/generated/scoped-claim-derivations.json` and check freshness.
- [ ] Rebase onto the verified typed-requirement-contracts change, then run focused core/CLI cases, maintained formatting, dependency, traceability, purity and repository rails required by current policy.
- [ ] Run the actual CLI explanation against a current repository completion case and retain the output as evidence.
- [ ] Refresh final docs/changelog, run native lifecycle gates, sync/archive and integrate only after supported acceptance evidence is retained.

All other tasks remain open until their evidence is retained.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
