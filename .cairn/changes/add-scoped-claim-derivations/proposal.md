# Add scoped claim derivations

## Why

Cairn records claims, evidence references, findings and verification-obligation decisions, but it does not explain why one proposition follows from another under explicit assumptions.
Issue #6 and Roadmap #4 require the argument behind a claim to be inspectable: which native-admitted conclusions were used, under which rule, subject, environment, bounds and residual trust assumptions.
Without that, a structural trace link, a saved check row and a reviewed correspondence can be presented as if they were the same kind of fact.

## What Changes

- Add a versioned scoped-claim API that binds an exact native requirement row, interpretation identity, contract identity, subject, environment, bounds, claim kind and proposition.
- Add a small closed derivation-rule vocabulary (`reviewed_completion_v1` in v1) with explicit premises, conclusion, rule version, applicability, assurance scope and residual assumptions.
- Re-admit native producer inputs through existing validators: the authoritative agent-completion evaluator and the ast-grep receipt validator.
- Preserve unresolved, supported, refuted and disputed claim states, including disagreements between applicable support and counterevidence.
- Explain unresolved premises, counterevidence, residual assumptions and shared oracle/trust dependencies without inferring actor independence.
- Add a read-only `cairn claim-derivation explain --case FILE [--policy FILE]` command that explains one actual native completion case.
- Put bounds and the admitted vocabulary in a standalone Nickel policy export; keep the core pure and the shell limited to bounded file reads.

## Impact

Current consumers: maintainers reviewing why a completion decision does or does not follow, and the kernel-qualification workstream that needs an explicit rule/conclusion surface.
Immediate outcome: one native completion case is explained with exact premises, rule identity and residual assumptions.
Durable contribution: a bounded acyclic derivation format that later composition rules and truth maintenance can reuse without becoming a second gate.
Maintenance owner: Cairn verified-core maintainers own identity/rule semantics; CLI shell maintainers own bounded reads and presentation.

Native `Pass | Block`, readiness states, verification roles and applicability validators keep their current meanings and authority.
This change introduces no lifecycle authorization, no alternate requirement registry and no new completion decision.

## Out of Scope

- Inventing negative propositions or refuting a composite judgment from one failing premise
- Combining different revisions, subjects, environments or bounds without an admitted relation
- Treating recorded-only structural evidence as behavior, oracle soundness or requirement satisfaction
- Statistical independence claims from actor or artifact counts
- Automatic rule synthesis, proof search, truth maintenance or releasing any gate authority

Dependency: `add-typed-requirement-contracts`. Implementation remains open.
