# Cross-repository adoption report

The adoption report answers one question: which downstream adoption stages are still gaps for an upstream Cairn change?

It consumes saved inputs only. It does not scan sibling repositories, fetch remotes, run checks, update pins, or mutate lifecycle files.

## Producer recipe

1. Produce the workspace dependency observations outside Cairn:
   - `stack-deps` catalog schema `onixresearch.stack-dependency-catalog.v2`
   - `stack-deps` audit receipt schema `onixresearch.stack-dependency-receipt.v1`
   - `stack-deps` reconciliation schema `onixresearch.stack-dependency-reconciliation.v2`
   - optional `stack-doctor` receipt schema `onixresearch.stack-health-receipt.v1`
2. Add Cairn lifecycle or completion summaries as `cairn.cross-repo-receipt-summary.v1` files.
3. Add optional Cairn registry snapshot `cairn.global-registry-snapshot.v1` for identity lookup only.
4. Add optional Valence evidence admission `valence.evidence-admission.v1` when roles, spans, and non-claims apply.
5. Compose those facts into a `cairn.adoption.snapshot.v1` file.
6. Run:

```bash
nix run path:$PWD#cairn -- adoption report --root REPO --snapshot adoption-snapshot.json
```

The MCP equivalent is `cairn_adoption_report` with `root` and `snapshot`.

## Authority map

| Input | Owner | What it can prove | What it cannot prove |
|---|---|---|---|
| Cairn lifecycle receipt | Cairn repository | The selected lifecycle receipt exists and has the stated status | Consumer adoption, remote freshness, or release eligibility |
| Cairn completion receipt | Consumer repository | The consumer supplied a saved completion or integration observation | Whole-repository correctness or publication approval |
| `stack-deps` | Workspace dependency tools | Pin state under the producer-owned dependency vocabulary | Compatibility, release readiness, or update authority |
| `stack-doctor` | Workspace health tools | Saved conformance status for declared checks | Current service state or semantic correctness |
| Cairn registry | Cairn registry maintainers | Stable node and edge identity lookup | Evidence satisfaction, catalog authority, or mutation authority |
| Valence admission | Valence | Evidence identity, roles, spans, and non-claims | Behavioral correctness, catalog truth, or release authority |

## Supported versions

- `cairn.adoption.snapshot.v1`
- `cairn.adoption.report.v1`
- `cairn.cross-repo-receipt-summary.v1`
- `cairn.global-registry-snapshot.v1`
- `valence.evidence-admission.v1`
- `onixresearch.stack-dependency-catalog.v2`
- `onixresearch.stack-dependency-receipt.v1`
- `onixresearch.stack-dependency-reconciliation.v2`
- `onixresearch.stack-health-receipt.v1`

The workspace dependency tool source revision for the first cohort is `22515fb7bc01d378312053d389d4bbdd55637a50`.

## Diagnostic actions

All actions are advisory. `effect_authorized` is always `false`.

- `evidence_refresh`: get a fresh saved receipt from the responsible repository or producer.
- `diagnostic_inspection`: inspect the consumer pin or producer observation outside Cairn.
- `unresolved`: resolve contradictory saved observations before you trust the row.

Do not treat a clean registry query, a matching label, or an archived upstream change as adoption completion.

## Bounded-stage interpretation

Each relationship reports four independent stages:

1. `upstream_lifecycle`
2. `consumer_pin`
3. `consumer_compatibility`
4. `integration_evidence`

A row is complete only when every stage is `supported` and the query is not partial. Missing, stale, unavailable, blocked, diverged, conflict, partial, gap, and unknown states are visible non-success states.

A partial report means the explicit bounds omitted some selected input. Raise the bound or narrow the scope, then produce a new snapshot.
