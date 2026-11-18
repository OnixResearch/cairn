# Typed requirement contracts

Cairn already validates requirement identity, strength, statement form and exact
source occurrence. An optional finite semantics profile adds reviewable meaning
to a selected requirement without replacing the registry or the Markdown
specification.

```sh
cairn requirement-contract inspect --root . \
  --requirement cairn.supersede.agent_run_receipts.verification_contract \
  --registry target/requirement-registry.json
cairn requirement-contract inspect --root . \
  --requirement cairn.supersede.agent_run_receipts.verification_contract \
  --registry target/requirement-registry.json \
  --example fixtures/requirement-contracts/example.json \
  --review fixtures/requirement-contracts/review.json
```

`--registry` is produced by `cairn traceability export` and supplies the exact
native requirement row: specification path, specification BLAKE3, byte/line span,
requirement-body BLAKE3 and row identity. `--inventory` overrides the bundled
policy projection; `--policy` selects the lifecycle policy used for the root.

## What the profile contains

Nickel owns the finite profile, its bounds and the five pilot contracts in
`cairn-policy/typed-requirement-contracts.ncl`. The regenerated projection is
`cairn-policy/generated/typed-requirement-contracts.json`.

A contract binds the native row identity and declares:

- finite nominal domains, including an explicit absence sentinel where needed;
- total observable mappings, each with an argument list, result domain,
  observation phase, exact Rust source path and a reviewed projection description;
- an activation condition, a guarantee formula and explicit assumptions;
- the supported portion and the informal remainder in prose.

Supported formula forms are finite equality, membership, Boolean connectives,
quantification over a declared finite domain, and single-step before/after
observations. Arithmetic, temporal operators, transitive closure, recursion and
uninterpreted program behaviour have no interpretation; they are rejected rather
than approximated.

Normalization sorts map entries and connective operands, so identical admitted
input produces identical normalized contracts and identities. Contract,
interpretation, formula and observable-mapping identities are separate BLAKE3
domains.

## Four distinct results

`requirement-contract inspect` keeps these apart and never collapses them:

| Result | Meaning |
|---|---|
| `well_formed` | The formula type-checks against declared domains within bounds. |
| `admitted_review_declaration` | A revision-bound correspondence record matched the normalized contract and an explicitly authorized reviewer identity. |
| `evaluated_example_only` | The formula was evaluated on one supplied total observable table. |
| proof evidence | Not produced by this command. Use the existing evidence boundary. |

A requirement with no contract reports `informal` with legacy behaviour intact.

## Bounds and failure behaviour

Admission enforces the profile's node, depth, domain-value and step limits with
checked arithmetic. Ill-typed expressions, unbound variables, unknown profiles,
unsupported constructs and exhausted bounds return distinct typed outcomes and
never a silent pass. Interpretation is pure: no filesystem, clock, process or
network access.

## Claim boundary

EARS keywords and the vocabulary classifier remain authoring aids. They cannot
approve a correspondence record or prove a proposition.

A well-typed formula does not establish faithful natural-language translation.
A review declaration is an identity-bound record, not a signature or a proof. A
finite example distinguishes candidate meanings; it does not establish an
unbounded property, program correctness or requirement satisfaction. Prose
remains authoritative, and native gates, readiness and verification roles keep
their existing meanings.

The claim-revision pilot covers the supported single-step revision observation
in `WorkflowState`. Per-fact claim-root binding, authorization of the complete
basis, concurrent heads and arbitrary history are named informal remainders
rather than fabricated observables.
