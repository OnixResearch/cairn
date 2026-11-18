# Support maintenance design

## Ownership and API

`cairn-policy/support-maintenance.ncl` owns the admitted change vocabulary and evaluation bounds.
Its entire standalone root exports to `cairn-policy/generated/support-maintenance.json`; the CLI embeds that checked artifact.
The optional policy is separate from the legacy default policy and from the scoped-claim policy, so no existing policy or receipt hash material changes.

Rust owns the versioned wire model and pure recomputation in `cairn_core::support_maintenance`, exported at the crate root.
Public entry points are `support_context_identity`, `admit_support_policy` and `evaluate_support_maintenance`.
Admission and evaluation return the existing `DerivationError` outcomes (`unsupported`, `scope_mismatch`, `invalid_input`, `bound_exhaustion`, `cycle`) rather than inventing a parallel error vocabulary.
Deserialization denies unknown fields, so an unborn schema or event kind is rejected instead of ignored.

## Immutable records and alternative branches

A `SupportCase` names exact `ScopedClaim` records, immutable `EvidenceRecord` observations, alternative `SupportBranch` values, ordered `ChangeEvent`s and optional lifecycle obligations.
Every branch is one conjunction of exact premise claim identities with its own polarity, rule identity, assurance scope, applicability and residual assumptions.
A claim therefore supports `(A AND B) OR C` as two branches; the conjunction is not flattened into one evidence set.
A refuting branch has `polarity = refute` and contributes counterevidence only when its own premises are cleanly supported.
Evidence records are supporting or counterevidence for one exact claim identity, so counterevidence cannot drift to another proposition or context.

Branches and claims bind the exact `ClaimContext` from the scoped-claim capability.
All premises of a branch must share its applicability, and evidence targets the claim whose context it observes.
A branch may cross native requirement revisions only when it declares a non-empty BLAKE3 `cross_revision_relation`; the branch is then checked for current applicability like any other branch.
Revision classification events never create, copy or widen support.

## Ordered change semantics

The change log is sorted by explicit `(sequence, id)` and every sequence is unique.
Events apply in that order and remain in the report:
- evidence stale/withdrawn removes current applicability of an existing evidence record, retaining its historical polarity;
- assumption withdrawal removes every branch and evidence record that names the assumption directly or through a supported premise;
- evidence admitted adds a new immutable record after its sequence, including counterevidence;
- claim revision marks the prior claim superseded and the new claim current without transferring support;
- policy/context changes make claims, evidence and branches in the previous selected scope inapplicable.

An event that targets unknown or already-inapplicable material is rejected rather than silently applied.
No event reverses a prior withdrawal; a new observation is a new record, not a rewritten old one.
The explicit sequence is the ordering semantics; reversing the input arrays does not change the result.

## Full recomputation and propagation

The baseline and current states are both computed by full deterministic recursion over the bounded branch graph.
Replay evaluates premise claims before a branch, detects cycles, and propagates a cause from the nearest invalidated evidence/assumption or unresolved premise to every dependent branch and claim.
A branch is active only when its applicability is current, none of its assumptions is withdrawn, and every premise is cleanly supported.
A branch whose premise is refuted or disputed is withdrawn without refuting the conclusion; this does not invent a negative composite proposition.
A disabled alternative never hides a declared counterevidence branch: claim state remains `disputed` whenever applicable support and counterevidence both remain.

Each report retains surviving branches, withdrawn branches with their historical cause, discharge paths from premises to their support sources, transitively shared assumptions, baseline/current states, newly unresolved claims, affected claims, revision explanations and advisory obligation impacts.
`analysis.complete` is true only for a completed full recomputation; bound exhaustion and cycles are explicit errors, never a partial set presented as complete.

## Bounds and authority boundary

Nickel owns claim, branch, evidence, premise, assumption, event, obligation and depth bounds within fixed implementation ceilings.
Depth and premise counts bound recursion and conjunctions; the report exposes limits, counts and maximum observed depth.
No truth-maintenance authority is added beyond the existing gate/readiness meanings.
Support maintenance writes no lifecycle record, creates no registry, executes no check and transfers no evidence.
Its report is advisory: surviving support is not an accepted exception, an action authorization, readiness or behavioral requirement satisfaction.

## Tests and evidence

Permanent core tests cover:
- `(A AND B) OR C` with B withdrawn and C surviving, including the withdrawn branch's historical reason and discharge path;
- one shared assumption withdrawing every dependent branch;
- a stale receipt becoming inapplicable without becoming refutation;
- admitted counterevidence remaining visible and forcing `disputed` while another support branch succeeds;
- an unlinked revision staying unresolved, an explicit relation branch supporting it, and a missing relation being rejected;
- old policy/context scopes becoming inapplicable while new scopes remain supported;
- identical input permutations replaying identically, explicit event ordering, bound failure and cycle reporting;
- advisory boundary, invalid change targets and unknown case schemas.

CLI tests exercise the actual binary with bounded reads, positive surviving/withdrawn alternatives, scope rejection, malformed input, unknown arguments and no lifecycle writes.
No test claims native execution honesty, oracle adequacy, automatic revision transfer or whole-product correctness.
