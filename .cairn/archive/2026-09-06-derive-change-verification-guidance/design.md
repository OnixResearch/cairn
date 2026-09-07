# Change verification guidance design

## Context

Cairn's trace index records exact marker bytes, source hashes, spans, and declared requirement links.
The current index does not prove dependency closure or identify executed tests.
Developer-loop profiles name focused checks but do not derive a selection from source changes.
The new contract connects these existing facts without weakening full-gate or final-subject rules.

## Goals and non-goals

Cairn traceability maintainers own the impact model and declared check map.
The first consumer is Cairn's own developer-loop profile.
The goal is explainable focused work and evidence refresh, not proof that omitted work is safe.
No compiler, source scanner, execution scheduler, or cache admission engine moves into Cairn.

## Decisions

### Admitted inputs

The first interface accepts explicit before and after source manifests, their exact identities, current requirement registries, trace indexes, and a checked Nickel check map.
The manifests bind the selected source projection and producer scope.
The shell admits paths through existing capability-relative loaders. The pure core derives the path delta from the admitted manifests.
A supplied changed-path assertion must match that delta. It cannot hide a deletion or an extra changed path.

The check map binds stable requirement IDs to check IDs, target and feature scope, declared dependency edges, and mandatory checks.
It records incomplete scopes explicitly. Missing graph coverage is not an empty dependency set.
Input limits and diagnostic bounds are named policy values.

### Pure selection model

The verified impact module unions relevant before and after links so deletions and marker moves remain visible.
It computes affected requirements and declared checks with bounded causal paths.
Renames use exact supplied identity evidence. Names alone do not establish source equivalence.
Mandatory checks remain in every applicable selection.
Unknown, incomplete, or unsupported coverage expands the recommendation to the broader configured loop or full set.
Conflicting identities and invalid graph shapes block admission rather than invent an impact result.

The report contains the source comparison, requirement links, selected checks, full required set, uncertainty, refresh needs, and non-claims.
Deterministic order and BLAKE3 framing bind the complete semantic result.

### Stack evidence adapters

Reuse the current content-bound registry and trace-index contracts without parsing requirement meaning again.
Octet owns optional structural dependency observations. Its declared source-token or stronger scope stays explicit.
Valence owns source-span and evidence identity admission. Recorded-only links never become behavioral verification.
The implementation must inspect the exact pinned producer contracts and run their positive and negative consumer vectors.
If no compatible dependency artifact exists, the first implementation uses declared links and conservative fallback.
It does not add an implicit compiler adapter or make unmerged compiler-evidence work a hidden dependency.

### Readiness and producer integration

The package follows unified readiness and the check-evidence producer.
The proposed `verification-impact` CLI and MCP tool return the same semantic report and typed next actions.
An action can request a producer plan for selected checks. It cannot authorize a run.
The full gate remains outstanding unless its own complete current evidence satisfies the original contract.

Refresh decisions call existing applicability validators.
A changed final subject invalidates whole-subject completion evidence even if the changed path lacks markers.
The initial feature grants no incremental reuse exemption. Any later exemption needs a separate reviewed contract and stronger evidence.

## Risks and trade-offs

Conservative fallback can select many checks. That cost is preferable to an unsupported omission.
A source diff is meaningful only inside its declared projection. Report out-of-scope and incomplete observation explicitly.
Feature, target, generated-source, macro, or provider-cohort changes can invalidate structural evidence.
A recommendation and a completed check are separate records with separate authority.

## Validation strategy

Establish trace-index, requirement-registry, loop, and applicability baselines before implementation.
Add direct-value tests for exact selections, causal paths, input reordering, renamed and deleted files, moved markers, and unrelated declared requirements.
Negative tests cover omitted paths, stale manifests, mismatched roots, invalid spans, unknown check IDs, graph cycles, feature drift, and opaque macro coverage.
Retain a mutation control that incorrectly treats missing coverage as independence. Required-check preservation must reject that mutant.
Run CLI/MCP parity and no-execution tests against fresh bounded fixtures.
Run focused tests, producer conformance vectors, Nickel fixtures, strict Clippy, Slotcar, Cairn gates, and full Nix checks.

## Migration and completion

The first feature is additive and advisory. Existing full verification remains authoritative.
Document the declared-link limit and the exact fallback behavior in the command guide.
Record observed local-loop work and selected checks without claiming an unmeasured speedup.
Completion requires retained safe-omission controls and fresh-source reproduction, not a smaller check count.
