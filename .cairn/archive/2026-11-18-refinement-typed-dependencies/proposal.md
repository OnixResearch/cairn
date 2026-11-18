# Add refinement and typed dependency contracts

## Why

Cairn records task order, archive prerequisites, requirement identities and scoped
claims, but it cannot explain why decomposed child guarantees establish a parent
requirement, nor which producer guarantee discharges a consumer assumption.
Issue #7 and Roadmap #4 require that argument to be inspectable: exact parent and
child revisions, environment assumptions, selected alternatives, integration
obligations, a separately justified feasibility disposition, and typed dependency
relations whose evidence sources remain distinct.

Without that, completing every child task, matching requirement names, reusing a
receipt identity or presenting a bare integration claim can look like a
composition proof it is not.

## What Changes

- Add an opt-in versioned decomposition record binding exact parent and child
  requirement revisions, environment assumptions, selected `any_of` alternatives,
  finite `all_of`/conditional applicability, integration obligations and an
  implication relation.
- Keep unassessed, reviewed, bounded-check and checked-proof assurance distinct,
  propagate the weakest applicable assurance, and require a policy-admitted
  checker for any checked-proof claim.
- Add a separately justified feasibility disposition: contradictory or unresolved
  assumptions or children cannot yield a vacuous parent conclusion.
- Add typed dependency contracts alongside legacy `depends_on` for ordering/archive,
  verified-specification, verified-candidate, candidate-input, integrated-revision,
  lifecycle and semantic relations, each with a named meaning and authoritative
  evidence source.
- Require semantic dependencies to name the producer guarantee, consumer
  assumption, compatible context and a retained implication/correspondence
  witness; matching names or receipt identities alone never discharge.
- Discharge assumptions through admitted guarantees or policy-accepted roots and
  report residual assumption closure with original sources and discharge paths.
- Require an acyclic initial justification graph and reject cycles with a specific
  path.
- Expose deterministic read-only explanations for pending, supported, contradicted,
  infeasible and unsupported obligations, plus reevaluation triggers for
  requirement, policy, assumption and context changes.
- Add a read-only `cairn refinement explain --case FILE [--policy FILE]` command.
- Put the admitted relations, assurances, feasibility methods, roots, checkers and
  bounds in a standalone Nickel export; keep the core pure and the shell bounded.

## Impact

Current consumers: maintainers reviewing why a composition argument does or does
not follow, Kiln dependency consumers, and the kernel-qualification workstream.
Immediate outcome: one replayable parent/child composition argument and its
unresolved-premise report, plus the Kiln dependency fixture semantics.
Durable contribution: a bounded assume-guarantee contract that later workflow and
truth-maintenance work can consume without creating another scheduler or gate.
Maintenance owner: Cairn verified-core maintainers own identity, rule and discharge
semantics; CLI shell maintainers own bounded reads and presentation.

Task ordering, archive dependencies, implementation-ready output, cross-repository
v1 semantics, native applicability, readiness states, verification roles and the
`Pass | Block` gate keep their current meanings and authority.

## Out of Scope

- A live multi-repository execution or integration observation provider
- Automatic theorem proving, refinement inference or proof-search
- Cyclic assume-guarantee induction or mutual justification
- Selecting work, claiming tasks, merging changes or owning integration order
- Reinterpreting a saved observation as integration correctness or behavioral
  requirement satisfaction
- Changing legacy `depends_on`, task ordering, archive receipts or cross-repo v1
  wire formats

Dependencies: `add-typed-requirement-contracts`, `add-scoped-claim-derivations`.
Implementation remains open.
