# Typed requirement contract design

## Ownership and API

`cairn-policy/typed-requirement-contracts.ncl` owns the finite profile, five contracts and proposed correspondence. Its entire standalone root exports to `cairn-policy/generated/typed-requirement-contracts.json`; the CLI embeds that checked artifact. This optional inventory is separate from the legacy default policy, so it changes no existing policy/receipt hash material. Rust owns the versioned wire model and pure interpretation in `cairn_core::typed_requirement_contracts`, also exported at the crate root.

Public entry points are `parse_typed_requirement_inventory_value`, `normalize_contract(profile, native_row, contract)`, `admit_correspondence_review(normalized, review, authorized_reviewers)` and `evaluate_example(profile, normalized, example)`. Errors are `IllTyped`, `UnboundVariable`, `UnknownProfile`, `UnsupportedConstruct`, `BoundExhaustion`, `RequirementRevisionMismatch`, `InvalidReview`, `InvalidExample` or `Serialization`, serialized as tagged explicit outcomes. No error becomes false, a passing proof, or a fallback profile.

`SemanticContract.requirement` is the exact existing `ContentBoundRequirementRow`, including specification digest, source occurrence, body digest and row digest. There is no second registry. The CLI verifies that the supplied native registry matches current accepted specifications and policy bytes before inspecting a row. Core callers are responsible for native-registry admission; core equality alone does not authenticate a repository revision.

## Finite interpretation

Domains are nonempty named nominal finite sets. Values carry their domain; equality and membership cannot cross domains. Boolean formulas support constants, equality, membership, negation, conjunction, disjunction, implication and explicitly domain-bound universal/existential quantification. Quantifier shadowing and free variables are rejected. Empty conjunction is true and empty disjunction false. Declared domains cannot be empty. There are no implicit free parameters.

Observable mappings are typed finite total functions with ordered nominal argument domains, a result domain, a source/projection description and one of `state`, `before`, `after` phases. A single-step relation is an ordinary formula comparing explicitly supplied before/after observations; no implicit temporal closure or execution is performed. Supplied tables must have the exact observable set, exact finite Cartesian cardinality, unique argument tuples and in-domain arguments/results, even if Boolean short-circuiting would not visit a row. Unsupported arithmetic, temporal syntax and program behavior have explicit unsupported/missing-observation outcomes.

The profile bounds syntax nodes, nesting, values per domain, and interpretation steps under hard implementation ceilings of 4096 nodes, 64 levels, 256 domain values and 1000000 steps. Contract/domain/observable counts and table cardinalities are bounded. Evaluation charges table validation, expression/term visits and observable-row search. The CLI bounds each input artifact to 4 MiB and current policy bytes to 16 MiB. Bound exhaustion is not a verdict about the proposition.

Evaluation returns `assumptions_hold`, `activated`, `guarantee_holds` and `implication_holds` separately, plus consumed steps and exact contract/example identities. Thus a vacuous implication is not reported as an activation witness. Interpretation only evaluates one supplied finite example; it does not enumerate all state assignments, prove a law or establish the accuracy of observable extraction.

## Normalization and identities

Normalization validates before transforming, sorts domain values, sorts/deduplicates membership literals and recursively sorts/deduplicates conjunctions/disjunctions and assumptions by deterministic serialized syntax. Map keys use ordered maps. Quantifier names and order, implication direction, phase and term argument order are preserved. No alpha-equivalence, arithmetic simplification, theorem rewriting or source-code equivalence is claimed.

Each BLAKE3 preimage is UTF-8 domain separator, zero byte, then existing canonical JSON. The interpretation identity hashes the complete profile under `cairn.requirement-interpretation.v1`; the mapping identity hashes exact observable declarations under `cairn.requirement-observables.v1`; the contract identity hashes interpretation identity, mapping identity and normalized complete contract under `cairn.requirement-contract.v1`. This binds the exact native row, formulas, declared finite universes, coverage/remainder and proposer. The canonical example identity sorts table rows by typed arguments and uses `cairn.requirement-example.v1`. Review declaration identity uses `cairn.requirement-correspondence-review.v1`.

Any changed requirement occurrence/revision, formula, interpretation/profile limit, or observable mapping invalidates the corresponding old review identity. Identical admitted input yields identical normalized output and identities; syntactic forms beyond the documented normalization need not share identities.

## Correspondence trust and proof boundary

`CorrespondenceProposal` is only author rationale. `CorrespondenceReview` names exact row, interpretation, contract and mapping digests, revision-bound proposer, independent reviewer, retained review-artifact digest and approved decision. Admission requires all identities current, a reviewer different from the proposer, explicit reviewer allowlisting and a syntactically valid retained artifact digest. Review declarations are not signatures: artifact existence, reviewer identity/authenticity and review quality remain explicit external trust. The CLI labels this `reviewed_declaration`, not proof or authenticated review. Empty default reviewer/review arrays prevent invented shipped approvals.

An independent reviewer must inspect the current prose, normalized meaning, all observable projections, supported portion and remainder. Retain their actual review artifact, hash it, authorize that reviewer in the Nickel inventory, and retain one matching record per accepted contract. Parent owns that admission and final acceptance; records cannot be inferred from passing examples or EARS keywords.

EARS classification remains an optional template authoring aid only. Neither it nor structural lifecycle validation approves correspondence or proves any formula. Existing Trellis/evidence-chain admission remains the only relevant external proof boundary; this feature accepts no proof payload and always reports `admitted_proof_evidence: not_supplied`. No example/review result changes a legacy completion gate.

## Pilot observable coverage

1. Verification-contract identity compares independently repository-supplied repository, registry, selection, policy, subject, required-check, command and prerequisite identities and detects missing independent contracts.
2. Exact pass-only coverage quantifies over check IDs and result slots, distinguishing missing/duplicate/unknown rows, pass status and supplied eligibility.
3. Freshness compares eligible result, contract and receipt subject identities to the final post-mutation subject.
4. Completion authority requires a current passing native receipt and finite exact selected-check coverage when a consumer asserts authoritative completion.
5. Claim revision compares before/after exact epoch tokens, requires open lifecycle after a changed epoch and preserves selected operation history. Existing WorkflowState does not expose exact per-fact evidence/finding/decision claim-root bindings; that stronger prose and authorization/conflict/full-history properties remain named informal.

All projection sources and detailed mappings are in Nickel, not duplicated runtime policy. Identity tokens are injective labels for exact values in one supplied finite universe, not validated digest contents. Finite cardinalities and summarized native eligibility checks never become universal claims about arbitrary Rust execution.

## CLI and tests

`cairn requirement-contract inspect --requirement ID --registry FILE [--root DIR] [--policy FILE] [--inventory FILE] [--review FILE] [--example FILE]` performs only bounded reads and prints normalized meaning, separate well-formedness/correspondence/example/proof statuses. Missing contracts are informal, not invalid legacy requirements. Counterexamples are successful example evaluations with false formula fields; malformed inputs, stale native linkage, review rejection and exhausted bounds fail explicitly.

Permanent tests exercise actual core consumers and CLI outputs, native row equality, determinism, mapping/profile/formula/revision invalidation, independent declared-review admission, finite operators, total tables, resource rejection and concrete positive/negative pilot states. They do not claim executed evidence until the parent runs them. Existing command/default compatibility remains intentionally unchanged.
