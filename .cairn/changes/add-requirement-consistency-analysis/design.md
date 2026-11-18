# Requirement consistency analysis design

## Ownership and API

`cairn-policy/consistency-analysis.ncl` owns the enabled query vocabulary and
every bound. Its entire standalone root exports to
`cairn-policy/generated/consistency-analysis.json`, which the CLI reads under the
selected root (or accepts through `--analysis-policy`). Rust owns the versioned
wire model and pure analysis in `cairn_core::requirement_consistency`, also
exported at the crate root. No literal duplicate of the policy exists in source.

Public entry points are `admit_consistency_policy(policy)`,
`analyze_consistency(policy, case)` and `replay_witness(policy, case, witness)`.
Errors are `Unsupported`, `InvalidInput`, `ScopeMismatch`, `BoundExhaustion` or
`Serialization`, serialized as tagged explicit outcomes. No error becomes a
status, a false value or a fallback policy.

`ConsistencyCase` offers contracts as exact
`ContentBoundRequirementRow` + `SemanticContract` pairs plus an explicit
`selected` order, subject/policy/target/environment identities, authorized
correspondence reviewers, caller-named native obligations and a producer
identity. Admission reuses `normalize_contract` and
`admit_correspondence_review`, so a contract is analyzed only at its exact
admitted revision; a present-but-invalid review is rejected rather than ignored.
The core does not verify native registry freshness: the shell does, and core
callers remain responsible for that admission.

## Finite model and search

The model of a query is one complete total observable table for every selected
contract — exactly the artifact the typed-contract evaluator accepts. Each table
row is one independent cell. Quantifiers are evaluated in place against the
contract's declared domains, with contract-local variables, so identical variable
names in different contracts are never conflated and no alpha-renaming is
claimed.

Cross-contract identity is never inferred. Equal observable names in two contracts
denote one shared observation only when the case declares an explicit
`ObservablesCorrespondence` naming both members, and admission requires their
declared argument and result domain values to agree positionally. A declared
correspondence makes the two contracts share one model cell, so a genuine
cross-contract inconsistency (for example `verdict = a` against `verdict = b`)
becomes detectable; without a declaration the analysis reports independence
instead of inventing an identity, and that fact is part of the retained scope.

Search is a deterministic depth-first enumeration. Evaluation is strong Kleene
three-valued: an unassigned cell makes the goal unknown, and the first unknown
cell in evaluation order is the branch point. Because table cells carry no
cross-cell constraints, branching over all values of that cell covers every
model, so `inconsistent`/`vacuous` are only reported after the complete declared
space was explored. Domains cannot be empty: typed-contract admission already
rejects empty domains, and the analysis returns `unsupported` if one is ever
presented. Unconstrained cells in a found model are completed by the first
declared value, which cannot change the truth of a goal that was already true for
all completions.

Every search charges evaluations and states against the policy bounds. An
exhausted bound yields `undetermined`; nothing is reported as a verdict. A found
model yields `consistent` and is retained as total tables with their example,
contract and model identities. `max_cells` bounds the complete table size, so the
witness stays small, replayable and cheap to validate.

## Queries and statuses

`assumptions_satisfiable` conjoins every selected contract's assumptions.
`activation_reachable` conjoins one contract's assumptions with its activation.
`guarantees_compatible` conjoins all selected assumptions, activations and
guarantee fragments. `guarantee_constraining` conjoins everything except one
guarantee fragment and its negation, so a found model is a behavior that
withholding the guarantee admits; weakening to `true` is the same test.

Statuses describe the queried formula: `consistent` (a witness was found),
`inconsistent` (complete search, no model), `vacuous` (the implication's
antecedent has no model), `undetermined` (bound exhausted) and `unsupported`
(construct or query outside the enabled analysis). A disabled query kind yields
`unsupported` without searching. `SearchBasis` records the kind, the explored
states and evaluations, the model cell count and every declared domain
cardinality, so an absence-of-counterexample result keeps its complete finite
domain and exploration basis.

Selected guarantees are the normalized top-level conjuncts of each contract's
guarantee; a non-conjunctional guarantee is one fragment. Fragment references are
`requirement::assumptions[i]`, `requirement::activation` and
`requirement::guarantee[i]`, with a BLAKE3 identity over the exact expression.

## Nonvacuity diagnostics

The analysis reports separate findings instead of collapsing causes:

- contradictory assumptions, with pairwise conflicting subsets whose `minimal`
  flag is only set after both singletons were separately shown satisfiable;
- unreachable activation — reported only when the contract's own assumptions are
  satisfiable, so it stays distinct from contradiction;
- vacuous requirement set when the shared assumptions and activations have no
  model;
- conflicting guarantees with the same pairwise minimality discipline;
- redundant-within-model guarantees, stated explicitly as not useless or
  removable from their prose;
- constraining guarantees with the distinguishing behavior retained;
- single-instance quantified claims, naming the variable, the domain and the
  fragment;
- quantified domains that no observable of that contract accepts as an argument,
  so observable projections cannot distinguish states differing only in that
  domain;
- incomplete analysis wherever a query was undetermined or unsupported.

Findings carry the exact scope contracts (row, contract, interpretation and
mapping identities), the formula fragments with their identities, the declared
domains, the retained conflicting subsets, the witness where one exists, and the
caller-named native obligations with disposition `unresolved`. No finding
resolves an obligation, changes a disposition or becomes proof evidence.

## Replay

`replay_witness` re-admits the policy and the case, requires the witness to name
one exact planned query, requires the table set and contract identities to be the
selected set, re-validates every table through `evaluate_example`, and requires
the analysis evaluator's assumptions/activation/guarantee booleans to agree with
the typed-contract evaluator. It then re-evaluates the named query over the
retained cells, recomputes the model identity and returns an explicit
`replays: false` rejection for any mismatch. Replay never panics and never
upgrades a mismatch into a status.

## Identities

Every BLAKE3 preimage is a UTF-8 domain separator, zero byte and canonical JSON.
Domains are `cairn.requirement-fragment.v1`, `cairn.requirement-consistency-query.v1`,
`cairn.requirement-consistency-witness.v1`, `cairn.requirement-consistency-input.v1`
and `cairn.requirement-consistency-report.v1`. The report identity hashes the
complete report with an empty `report_blake3`; the witness identity hashes the
query label and the total tables. `.cairn`-independent determinism follows from
sorted maps, sorted domain values, sorted fragments and goal-directed branching in
canonical order.

## CLI

`cairn requirement-analysis inspect --case FILE --registry FILE [--root DIR]
[--policy FILE] [--analysis-policy FILE] [--witness FILE]` performs bounded reads
only. `--registry` is the native content-bound registry; the shell verifies it
against current accepted specifications and lifecycle policy bytes, and rejects
any case contract whose native row is absent or stale. `--analysis-policy`
overrides the checked Nickel export; otherwise the root's
`cairn-policy/generated/consistency-analysis.json` is read and its absence is an
explicit error. Output states `admitted_proof_evidence: not_supplied` and
`lifecycle_authority: none`, and never mutates the lifecycle.

## Tests

Permanent tests exercise actual core and CLI consumers: satisfiable constraints
with a replayed model, contradictory assumptions with a minimal conflicting pair,
identical guarantee text with vacuous versus reachable activation, single-instance
and unobservable domains, model identity and table tampering, rejected stale
native rows and invalid policies, exhausted bounds and disabled queries, read-only
inspection, and the five packaged pilot contracts with their exact revisions.
They do not claim executed evidence until the parent runs them.
