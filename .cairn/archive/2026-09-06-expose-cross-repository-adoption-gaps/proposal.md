# Expose cross-repository adoption gaps

## Why

An upstream archive does not prove that a downstream repository adopted the change or passed its integration checks.
The workspace already supplies dependency audit, reconciliation, and health receipts through `stack-deps` and `stack-doctor`.
Cairn already supplies registry projections and explicit cross-repository evidence admission.
Operators need one view that connects these facts without replacing their owners.

## What Changes

- Add an explicit saved-input adapter for workspace dependency, reconciliation, and health receipts.
- Combine upstream lifecycle evidence, consumer pin observations, and consumer integration evidence under an exact snapshot selection.
- Report each stage separately, including incompatible, missing, stale, diverged, unavailable, and blocked observations.
- Reuse unified readiness actions and existing registry projections for bounded read-only queries.
- Publish a disposable multi-repository receipt fixture for the Cairn-to-Octet and Octet-to-Lattice relationships.

The proposed command is `cairn adoption report` with explicit input files and scope.
It does not scan sibling repositories, fetch remotes, change dependency pins, or run consumer checks.

## Impact

Maintenance owner: Cairn dependency and registry maintainers.
Current consumers: workspace operators and maintainers of the Cairn, Octet, and Lattice dependency chain.
Immediate outcome: operators distinguish upstream completion from downstream integration and identify the responsible next action.
Durable contribution: a reproducible, snapshot-bound adoption report over existing producer contracts.

The workspace root retains catalog, dependency-plane, observation, and reconciliation authority.
Consumer repositories retain compatibility decisions, exact pins, checks, and release authority.
Valence retains evidence role and identity semantics. Cairn retains lifecycle and report meaning.
The hard local prerequisite is `unify-change-readiness-and-next-actions`.

## Repeatability evidence

Retain saved positive and negative relationship cohorts with exact producer revisions and policy identities.
Show that a completed upstream change with an unchanged consumer pin remains a gap.
Show that a matching pin without current consumer checks is not integration-ready.
Reject tampered, stale, partial, contradictory, unavailable, and wrong-scope observations without hidden filesystem or network fallback.

## Out of Scope

- A replacement for `stack-deps`, `stack-doctor`, Kiln, or the workspace catalog
- Automatic sibling discovery, remote freshness claims, dependency updates, or consumer mutations
- A shared workspace build, release unit, trust domain, or global completion authority
- Treating registry membership, matching versions, or an upstream archive as downstream success
- New Site, Subwayrat, graph database, or scheduling infrastructure
