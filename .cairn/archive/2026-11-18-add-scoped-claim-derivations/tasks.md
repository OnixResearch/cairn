# Scoped claim derivation tasks

## Implementation

- [x] [serial] [task:policy] Author the optional Nickel claim policy with admitted kinds, propositions, rule identities/versions, assurance scopes and bounds. r[cairn.scoped_claims.bounds]
- [x] [serial] [task:identity] [after:policy] Add versioned claim identity over native requirement revision, interpretation, contract, subject, environment, bounds, kind and proposition. r[cairn.scoped_claims.identity]
- [x] [serial] [task:rules] [after:identity] Add the closed acyclic rule vocabulary and bounded deterministic replay with unresolved, supported, refuted and disputed states. r[cairn.scoped_claims.rules] r[cairn.scoped_claims.states]
- [x] [serial] [task:producers] [after:rules] Add native producer adapters that re-run the authoritative completion evaluator and ast-grep receipt validator and admit only their conclusions. r[cairn.scoped_claims.native_producers]
- [x] [serial] [task:scope] [after:producers] Enforce exact scope separation and explicit rejection of unsupported rules, missing premises, cycles and exhausted bounds. r[cairn.scoped_claims.scope] r[cairn.scoped_claims.bounds]
- [x] [serial] [task:cli] [after:scope] Add the read-only `claim-derivation explain` CLI with explicit advisory boundary and deterministic explanation. r[cairn.scoped_claims.inspection] r[cairn.scoped_claims.authority]
- [x] [serial] [task:tests] [after:cli] Run the authored positive and negative core and CLI consumer tests against actual native completion inputs. r[cairn.scoped_claims.states] r[cairn.scoped_claims.inspection]

## Parent acceptance

- [x] [serial] [task:export] [after:tests] Generate the entire standalone scoped-claim Nickel export to `cairn-policy/generated/scoped-claim-derivations.json` and check freshness. r[cairn.scoped_claims.bounds]
- [x] [serial] [task:rebased] [after:export] Merge the archived typed-requirement-contracts change, then run focused core/CLI cases, formatting, traceability and lifecycle validation. r[cairn.scoped_claims.identity]
- [x] [serial] [task:evidence] [after:rebased] Exercise the actual CLI explanation against current repository completion inputs and retain the executed result. r[cairn.scoped_claims.inspection]
- [x] [serial] [task:handoff] [after:evidence] Refresh final docs/changelog, run native lifecycle gates, sync and archive only after supported acceptance evidence is retained. r[cairn.scoped_claims.authority]

Executed evidence: 11 core `scoped_claim` tests and 4 CLI
`scoped_claim_derivations` tests pass. The CLI suite invokes the real binary with
a native requirement registry row derived from the packaged
`cairn.specs/agent-completion` specification and real completion inputs, covering
advisory explanation, timeout, scope/rule rejection, malformed case and unknown
arguments. The case is constructed in-test rather than retained as a standalone
artifact; no signature or proof evidence is produced by this change.

Scoped limitations: the single admitted rule is `reviewed_completion_v1`; the
defensive cycle check is unreachable by rule shape and is covered by a direct
unit test rather than claimed as exercised end to end. Native gates, readiness,
applicability and verification roles retain their existing meanings, and this
change adds no lifecycle authority.
