# Compact readiness design

## Context

`ReadinessReport` already owns stage states, blockers, exact action arguments, and identities.
`run_readiness` exposes full JSON or a human summary. The human summary is not a machine contract.
This change adds a deterministic view, not another readiness decision engine.

## Decisions

### Decision: Add a separate explicit view

Add `--view full|agent` to `cairn readiness CHANGE` and the matching MCP `view` field.
The default remains full. The agent view is JSON-only and works inside the existing machine envelope.
Unknown views and agent-view requests with the human summary format fail explicitly.
Use a new versioned projection schema. Preserve the original full report and its receipt hash as the referenced authority input.
The projection receives its own BLAKE3 identity over the view contract, full receipt identity, and selected entries.

### Decision: Keep mandatory decisions visible

Every agent view includes report validity, lifecycle classification, every stage state, and mutation_authorized.
It also includes policy, source, and evidence identities plus counts of blockers and actions.
Selected actions retain their entire argv, prerequisites, cause IDs, target, expected observation, and advisory label.
No text shortening may remove these meanings. Transport success remains in the machine envelope, not an inferred lifecycle status.

### Decision: Bound optional detail, not authority

Nickel owns named byte and entry limits and deterministic selection order.
The request can lower a limit but cannot exceed policy. A limit below the mandatory projection size fails admission.
Optional entries use whole-entry selection. Return included and omitted counts and typed detail selectors.
A nonempty omitted set never appears complete. No action may contain a truncated argv or prerequisite list.
Initial selection follows existing stage and action order, not a model-generated importance score.

### Decision: Recompute before exact detail retrieval

Add detail arguments to the same readiness route: `--detail-kind`, `--detail-id`, and `--expected-receipt`.
All three arguments are required together. MCP exposes the same strict selector object.
Supported kinds are blocker, action, and stage. Stage IDs use existing stable stage names.
The shell gathers a fresh report and compares its full receipt identity before selecting detail.
A changed report returns stale detail. Unknown selectors fail without a new store, network lookup, or automatic mutation.
The later incremental-observation package owns optional saved snapshots and paging across requests.

## Architecture and reuse

Pure view admission and selection consume the existing report and explicit policy.
CLI/MCP shells own argument handling, bounded observation, and JSON rendering.
Reuse BLAKE3 identity helpers, the original readiness validators, and machine failure classes.
Keep full report hashes outside view-specific formatting changes.

## Alternatives rejected

- Reusing the human summary loses machine fields and exact actions.
- Shortening field names saves little while increasing ambiguity.
- Truncating serialized JSON creates invalid or misleading results.
- A permanent receipt database is unnecessary for fresh identity-checked detail reads.

## Tests and evidence

Run baseline readiness and machine tests before core changes.
Compare full and agent views over successful, blocked, unavailable, and identity-conflicted fixtures.
Add budget-boundary, whole-entry, stale-detail, unknown-ID, partial-selector, unsafe-path, and no-effect controls.
Prove full receipt identity remains equal and CLI/MCP semantic projections agree.
Measure complete diagnostic loops, including follow-up detail calls, with the shared benchmark.

## Risks and non-claims

A small projection can cause extra reads. The benchmark must count them.
Bounded repeated reads do not provide an atomic filesystem snapshot.
Neither compact output nor a stored identity proves correctness, evidence truth, or future mutation authority.
Lifecycle-core maintainers own projection invariants. Shell maintainers own transport parity.
