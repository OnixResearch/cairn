# Refinement and typed dependency design

## Ownership and API

`cairn-policy/refinement-typed-dependencies.ncl` owns the admitted dependency
relations, establishing assurance levels, feasibility methods, policy-accepted root
assumptions, proof checkers and evaluation bounds. Its standalone root exports to
`cairn-policy/generated/refinement-typed-dependencies.json`; the CLI embeds that
checked artifact. The optional policy is separate from the legacy default policy, so
no existing policy or receipt hash material changes.

Rust owns the versioned wire model and pure evaluation in
`cairn_core::refinement_typed_dependencies`, exported at the crate root.
Public entry points are `admit_refinement_policy`, `evaluate_refinement`,
`dependency_relation_contract`, `assumption_statement_identity` and
`guarantee_statement_identity`. Errors are `Unsupported`, `ScopeMismatch`,
`InvalidInput`, `BoundExhaustion` or `Cycle`, serialized as tagged explicit
outcomes. No error becomes an empty conclusion, silent pass or default policy.
Deserialization denies unknown fields, so an unborn schema, relation or assurance is
rejected rather than ignored.

The module reuses the archived #5 identity conventions (BLAKE3 over a domain tag,
zero byte and canonical JSON) and the archived #6 patterns: exact `deny_unknown_fields`
DTOs, policy-owned bounds and vocabulary, native re-admission, canonical ordering,
tagged explicit errors, and report/input identities. It does not copy or modify the
older modules or their public APIs.

## Decomposition records

A `Decomposition` binds a schema version, identity, exact parent `RequirementRevision`
(native `ContentBoundRequirementRow`, interpretation identity and contract identity),
one exact `ArgumentScope`, declared children, environment assumptions, integration
obligations, a finite `Composition`, an `ImplicationRelation` and a
`FeasibilityDisposition`.

`ArgumentScope` names the composed subject, native policy hash, repository target,
environment and explicit bounds. Every child guarantee must carry the identical
scope; v1 admits no widening or implicit transfer. Parent and child revisions are
therefore exact, and evidence from another target, environment or bound scope cannot
be combined.

The finite composition vocabulary is:

- `child`: one declared child obligation;
- `all_of`: every listed obligation, with at least one entry;
- `any_of`: named alternatives and one explicitly selected alternative. Only the
  selected alternative is evaluated; partial evidence from an unselected alternative
  is never combined;
- `conditional`: an observed applicability condition and two non-empty branches.
  Missing or conflicting observations keep the node pending.

`selected_alternatives` in the explanation records the selected alternative identity
and each conditional branch decision by deterministic composition path.

## Assurance and implication

Each consumer names the argument form `environment AND child guarantees AND
integration obligations IMPLIES parent`. The retained `ImplicationRelation` carries
one of `unassessed`, `reviewed`, `bounded_check` or `checked_proof`, a witness
identity and an explicit statement.

- `unassessed` never supports a parent, even when every child is established.
- `reviewed` and `bounded_check` must be admitted by policy and require a retained
  witness identity.
- `checked_proof` additionally requires a policy-admitted proof checker. The core
  records the checker identity and a `proof-checker:<id>:trusted-declaration` trust
  dependency; it does not claim to verify the proof.
- Parent assurance is the weakest applicable relation, composition, feasibility and
  integration assurance; an unsupported parent is reported as unassessed.

Child guarantees are not task completions or free-standing assertions. Each
`ChildGuarantee` names a saved scoped-claim case and claim identity. Evaluation
re-runs the #6 evaluator; only its own `Supported`, `Pending`, `Refuted` or `Disputed`
state is used, and its residual assumptions flow into the parent trust set. A
missing case, missing claim, revision mismatch or scope mismatch is an explicit
unsupported obligation.

## Feasibility and non-vacuity

Feasibility is a separate premise with its own disposition, method, assurance,
retained justification and covered premise set. Policy admits the method and
establishing assurance.

- `infeasible` makes the parent infeasible;
- `unresolved` leaves the parent pending;
- `feasible` requires non-unassessed, policy-admitted assurance, an admitted method,
  and coverage of every environment assumption and every selected child;
- contradictory assumption atoms (the same predicate both asserted and its negation)
  or a contradicted selected child make feasibility infeasible regardless of the
  declared disposition;
- pending or unsupported selected children make feasibility pending/unsupported.

A false antecedent therefore cannot be used to make an implication vacuously true.
An unresolved environment assumption that is neither discharged nor a
policy-accepted root keeps the parent pending and remains visible.

Integration obligations are named premises. A required obligation stays pending
until exact evidence binds its identity, statement and complete scope. Referencing or
completing child tasks never establishes it, and a non-required pending obligation is
reported without blocking the parent.

## Typed dependencies

`TypedDependency` sits alongside legacy `depends_on`; it does not modify it. Each
record binds producer and consumer `RequirementRevision` values, work and revision
identities, a cross-repository context, coverage and optional relation-specific
fields. `dependency_relation_contract` documents the meaning, authoritative evidence
and non-claims of every supported relation.

| relation | authoritative evidence |
| --- | --- |
| `ordering_archive` | archive receipt binding change, repository, exact revision and whole-change or selected-task coverage |
| `verified_specification` | specification gate receipt binding repository, exact revision and every required gate; no archive is required |
| `verified_candidate` | candidate verification receipt binding repository, candidate and exact revision |
| `candidate_input` | candidate input receipt binding repository, required input identity and exact revision |
| `integrated_revision` | admitted integration observation binding producer change, candidate, revision, repository and intended target |
| `lifecycle` | lifecycle receipt binding repository, exact revision and required stage |
| `semantic` | supported admitted guarantee plus witness binding both exact statement identities |

Evidence is a saved observation, never an executed check. Names, checked tasks,
worker success, a bare change identity or another relation's receipt cannot discharge
a relation for which it is not evidence. Whole-change coverage requires a
whole-change receipt; a partial selected-task receipt never substitutes for it, while
a whole-change receipt does cover a selected-task requirement.

Evidence matching is exact on every named field. Missing evidence is pending;
several distinct exact observations are ambiguous and pending; a stale revision,
moved target, out-of-scope repository or missing specification gate is a specific
pending blocker. Stale or out-of-scope evidence adds an
`applicability_reevaluation:<dependency>` trigger. A revision-only edge is not
silently strengthened to require archival.

A semantic dependency additionally names the producer guarantee, consumer assumption,
producer and consumer scopes, and a retained implication/correspondence witness. The
witness must bind the exact admitted guarantee statement and the exact consumer
assumption statement, and the contexts must be compatible (equal policy, environment
and bounds, with repository and intended target matching their endpoints). Matching
names, identical statement text or a receipt identity without the witness is
unsupported.

## Discharge and residual closure

Environment assumptions are declared or reused by semantic dependencies with an
identity, statement, source and predicate atom. A semantic dependency that is
supported discharges its consumer assumption and contributes a deterministic path
`[dependency, guarantee_case, claim]`. Alternatively an exact policy-accepted root
retains the assumption explicitly.

The closure merges identical assumptions referenced by a decomposition and a semantic
dependency, rejects the same id with conflicting content, and reports every
assumption with its original source, status (`discharged`, `policy_accepted`,
`residual`, `pending`, `contradicted`), all discharge paths and the referencing
records. Residual or pending assumptions keep the parent pending; contradicted ones
make it infeasible. Residual trust assumptions from admitted #6 guarantees are
deduplicated into the report's trust set.

## Acyclicity and explanations

The initial justification graph has one node per exact requirement row and edges from
parents to children and producers to consumers. Depth-first traversal under the
admitted depth bound reports a cycle as an explicit `cycle` error naming the
requirement path. Mutual declarations therefore cannot justify one another.

Explanations are deterministic and sorted: decompositions and dependencies by id,
obligations by kind and id, residual assumptions by identity. Each explanation states
the obligation kind, state, assurance, statement identity, source, reason and
discharge paths. `pending`, `supported`, `contradicted`, `infeasible` and
`unsupported` remain separate.

`EvaluationBasis` records the policy, requirement-row, assumption-statement and
context identities used. A caller can supply a previous basis; symmetric differences
produce `policy_changed`, `requirement_changed:<row>`, `assumption_changed:<stmt>` or
`context_changed:<scope>` triggers. Identical admitted inputs and previous basis
replay to the same report identity.

## CLI and authority boundary

`cairn refinement explain --case FILE [--policy FILE]` performs bounded
capability-relative reads only. It admits the policy, deserializes the case, evaluates
and prints one versioned advisory explanation. A pending or unsupported obligation in
an otherwise valid case remains a successful advisory explanation; rejected admission
prints an explicit error and exits non-zero.

The command writes no lifecycle state, creates no registry, executes no checks and
changes no existing command default. A supported argument is not behavioral
requirement satisfaction, a reviewed argument or witness is not proof, an integration
observation is linkage rather than correctness, and no result authorizes a claim,
readiness state, verification role, gate verdict or lifecycle action. Existing task
ordering, archive dependencies, implementation-ready output and cross-repository v1
semantics remain authoritative.

## Tests and evidence

Core tests exercise a native completion pilot with three structural child guarantees,
an integration obligation, a policy root, deterministic replay, assurance
propagation, `any_of` selection, contradictory and unresolved assumptions, a
contradicted child, dependency relation meanings, archive-versus-integration
separation, exact integration evidence, whole-change coverage, verified-specification
evidence without archival, candidate/input/lifecycle distinctions, semantic witness
requirements, residual closure paths, reevaluation triggers, checked-proof admission
and cycle reporting.

CLI tests exercise the actual binary, generated policy, bounded reads, successful
advisory explanations, a pending-but-valid argument, malformed case and policy
rejection, unknown arguments and absence of lifecycle mutation.

No test claims integration correctness, proof soundness, source-code correctness,
behavioral requirement satisfaction or whole-product correctness.
