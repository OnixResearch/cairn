# Scoped claim derivation design

## Ownership and API

`cairn-policy/scoped-claim-derivations.ncl` owns the admitted claim kinds, proposition names, rule identities/versions, assurance scopes and evaluation bounds.
Its entire standalone root exports to `cairn-policy/generated/scoped-claim-derivations.json`; the CLI embeds that checked artifact.
The optional policy is separate from the legacy default policy, so no existing policy or receipt hash material changes.

Rust owns the versioned wire model and the pure derivation replay in `cairn_core::scoped_claim_derivations`, exported at the crate root.
Public entry points are `claim_identity`, `admit_claim_policy` and `evaluate_derivations`. Errors are `Unsupported`, `ScopeMismatch`, `InvalidInput`, `BoundExhaustion` or `Cycle`, serialized as tagged explicit outcomes.
No error becomes an empty conclusion, a silent pass or a default policy. Deserialization denies unknown fields, so an unborn schema or rule tag is rejected rather than ignored.

## Claim identity and scope

A claim carries the schema version, declared kind, exact `ContentBoundRequirementRow`, interpretation and contract BLAKE3 identities, `ClaimContext` and `Proposition`.
`ClaimContext` names the final subject hash, native policy hash, repository target, environment and explicit bounds map. Every component is mandatory and non-empty.
The native policy hash is the one bound by the producer contract (for example the repository gate policy); it is not the scoped-claim policy export.
`claim_identity` hashes `cairn.scoped-claim.identity.v1`, a zero byte, then canonical JSON of the whole claim. A changed requirement occurrence, interpretation, contract, subject, environment or bound changes the identity.
The declared kind must equal the proposition's own kind, so a recorded-only proposition cannot be relabeled as behavior.

Claims must rebind the request's exact native requirement row and the normalized typed-contract interpretation/contract identities.
There is no second registry: the native requirement registry remains authoritative, and row freshness is a named caller-supplied trust assumption in this advisory surface.
Derivations are only admitted when their conclusion is a declared claim, their applicability equals the conclusion's exact context, and every declared premise has that same context.
A producer conclusion is attached only to claims with the exact same proposition and context. Different revisions, targets, environments or bounds therefore cannot be combined: v1 admits no cross-scope relation, and an attempted combination fails with `scope_mismatch`.

## Native producer adapters

`AuthoritativeCompletion` invokes the existing `evaluate_authoritative_agent_completion` with the declared subject and policy hashes.
Only its own conclusions become evidence:

- one passing, unambiguous selected check row supports the exact `CheckPassed { check_id }` observation;
- one unambiguous `failed` row is counterevidence for that same exact observation;
- a fully valid report supports `NativeCompletionEligible`.

Missing, duplicate, blocked, malformed, unsupported-status and timed-out rows establish neither support nor refutation. Any contract-level or result-set defect invalidates every observation instead of being reduced to per-check evidence.
The native report's diagnostics are retained under its receipt identity so a consumer can see why a row was not admitted.

`StructuralAstGrep` invokes the existing `validate_ast_grep_receipt` in inventory mode with an explicit repository scope and the mandated non-claim fragments.
It admits only `ReceiptStructurallyValid { receipt_blake3 }` with a `recorded_only:` assurance scope. Structural receipts never produce observations, completion eligibility or requirement satisfaction.

Correspondence support comes from `admit_correspondence_review`; a reviewed declaration supports only `ReviewedCorrespondence` claims and records reviewer/artifact fidelity as a trust assumption.

## Rule vocabulary and replay

The closed v1 vocabulary is:

- `reviewed_completion_v1`: exactly one `NativeCompletionEligible` and one `ReviewedCorrespondence` premise establish `ReviewedCompletionDecision`.

The rule requires the policy-declared assurance scope exactly. A derivation using an unadmitted rule, an unadmitted version or a different scope fails with `unsupported`.
An undeclared premise is not invented; it remains an explicit unresolved premise. Premises are claims with a different proposition from the conclusion, so admitted graphs are acyclic by construction.
Replay still detects cycles defensively and reports them as `cycle` rather than truncating; a malformed internal graph is never silently cut.

Replay is canonical: derivations are ordered by identity, premise evidence is computed before use, and links are sorted before explanation.
A supported derivation requires every premise to be cleanly established: support present and no applicable counterevidence.
A premise that is unresolved, refuted or disputed leaves the conclusion unsupported and names that premise. A composite judgment is not itself refuted here; this avoids inventing negative behavior claims.
Depth and premise counts are bounded by policy. Bound exhaustion is an explicit incomplete outcome, not a verdict about the proposition.

## States, assumptions and independence

Claim states are `unresolved`, `supported`, `refuted` and `disputed`, determined only by applicable admitted support and counterevidence.
Each explanation retains the exact links, the rule identity/version, premises, applicability, assurance scope and residual assumptions, plus unresolved premises (including transitive ones).
The report deduplicates residual assumptions into `trust_dependencies`, so a shared cargo-test oracle, verifier or provenance assumption remains visibly shared rather than being double-counted.
The report states `independence: distinct actors, artifact counts and separate derivation paths do not establish statistical, oracle or implementation independence`.
It never claims actor, reviewer or oracle independence from counts or identifiers.

## CLI and authority boundary

`cairn claim-derivation explain --case FILE [--policy FILE]` performs bounded capability-relative reads only.
It admits the policy, admits the case as an `EvaluationRequest`, evaluates, and prints one versioned advisory explanation. Unresolved claims remain successful evaluations; rejected admissions print an explicit error and exit non-zero.
The command does not write lifecycle state, create a registry, execute checks or alter any existing command default.

Support is not an accepted exception, an action authorization, readiness, verification-role assignment or behavioral requirement satisfaction.
Existing stage validators, `GateApplicability`, `ReadinessState`, verification roles and the `Pass | Block` wire meaning retain authority.
The explanation is a projection over admitted facts; it does not upgrade structural or recorded-only evidence into behavior.

## Tests and evidence

Permanent tests exercise actual native completion inputs, not recorded-only fixtures pretending to be behavior.
Positive cases cover a supported exact observation, completion eligibility, reviewed correspondence and the explained completion decision, including deterministic replay.
Negative or boundary cases cover timeout and unsupported statuses (neither polarity), a failed observation (refutes only itself), support plus refutation (disputed), structural-only evidence, cross-revision/target/environment/bounds combinations, unadmitted rule/version/scope, missing premises, cycles and exhausted bounds.
CLI tests exercise the actual binary, generated policy, bounded reads and rejection output without lifecycle mutation.
No test claims faithfulness of natural-language intent, execution honesty, verifier soundness or whole-product correctness.
