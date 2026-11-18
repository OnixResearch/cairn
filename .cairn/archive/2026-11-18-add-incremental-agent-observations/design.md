# Incremental observation design

## Context

Compact readiness and targeted context provide small deterministic observations with exact identities.
This change adds continuation and difference behavior for those views and active-change listings.
It does not skip the source observations required by existing freshness and mutation checks.

## Decisions

### Decision: Make continuity explicit and stateless

Use a typed request for a complete view, one page, unchanged comparison, or a difference against a prior observation.
CLI accepts an explicit saved prior-observation path. MCP accepts a bounded inline observation or a safe declared local reference.
Do not recover hidden session state. Ordinary read requests do not write snapshots.
An explicit output option can save the response through the existing guarded shell output path.
Initial support covers active-change listings, selected readiness entries, and targeted context entries, not arbitrary repository files.

### Decision: Bind the complete observation cohort

Identity includes admitted root or store identity, source manifest, policy, evidence, producer contract, view, selectors, and ordering version.
Use existing BLAKE3 helpers with a new domain tag for this schema.
A content digest identifies bytes and does not authorize access.
A prior observation must satisfy the original view schema, its own identity checks, and completeness requirements.
A partial page cannot serve as a complete difference baseline.

### Decision: Use stable whole-entry pages

Nickel owns named page-byte, entry, cursor, snapshot, and aggregate request bounds.
Each cursor binds the complete cohort, last stable key, page contract, and selected limits.
The core sorts by existing stable identities and never splits an entry or argv.
Return total, included, and omitted counts plus an explicit complete or partial state.
A changed cohort invalidates continuation. Replaying an old cursor deterministically repeats its page without mutating cursor state.
A consumer must reject duplicate page application through the page identity; replay itself is not evidence of malicious behavior.

### Decision: Compare exact entries

Differences contain added and changed current entries plus removed prior identities.
Stable IDs preserve identity across content edits. A changed identity is removal plus addition unless an existing authoritative alias contract applies.
Unknown snapshots, corrupt state, missing entries, and contract drift return explicit errors or require a full refresh.
Do not guess an empty baseline or promote incomplete state to unchanged.

### Decision: Keep caches disposable

A cache is optional shell storage of admitted observation bytes. It owns no domain truth.
Keys include the complete cohort rather than source mtime or commit alone.
Warm, cold, corrupt, and absent caches must preserve admission semantics.
Fresh source and evidence checks still run before a current observation or protected mutation.
Existing bounded readers remain authoritative. Repeated reads do not establish an atomic filesystem snapshot.

### Implemented contract

Views are `active_changes`, `readiness` (compact selected entries only), and `context` (exact targeted selectors).
The core adds `incremental_observations` with typed cohort, entry, snapshot, cursor, page, unchanged, and difference values.
Cohort continuation identity hashes root, optional store, source manifest, lifecycle policy, producer policy, evidence, producer contract, view, selectors, and ordering under `cairn.incremental-observation.cohort.v1`.
Difference admission uses a contract identity that excludes source and evidence, because source and evidence drift is what a difference must report.
Policy, root, producer contract, view, selectors, or ordering drift is a mixed cohort.
Snapshots and pages carry explicit schema versions, total/included/omitted counts, completeness state, and domain-hashed receipts.
Cursors are hexadecimal canonical JSON carrying the snapshot identity, last stable identity, page contract, and page limits; they are stateless and replay deterministically.
The CLI takes root-relative prior and output paths through the guarded capability writer.
MCP takes a bounded inline prior or a root-relative `prior_path` and never writes.
Nickel owns all byte, entry, cursor, prior, selector, and page bounds in `cairn-policy/incremental-observations.ncl`; the generated projection is `cairn-policy/generated/incremental-observations.json` with root field `policy`.


## Alternatives rejected

- A hidden server session makes restart and resume depend on ambient mutable state.
- Commit-only invalidation misses dirty files, policy changes, and external evidence.
- Offset-only cursors can mix different source snapshots.
- Automatic snapshot retention or deletion adds a separate storage authority problem.

## Tests and evidence

Baseline complete readiness, context, and active-list observations.
Reconstruct full results from pages and compare exact entries and identities.
Test unchanged inputs, edits, removals, renamed IDs, empty sets, restarted processes, and warm/cold caches.
Reject mixed cohorts, unsafe paths, partial baselines, forged counts, invalid cursors, and oversized entries.
Include repeated-cursor idempotence and duplicate-page consumer controls.
Measure full resumed loops with snapshot and continuation overhead under the shared benchmark.

## Risks and non-claims

A difference can increase total calls for small responses. Preserve full-view fallback and measure complete task cost.
Observation identities do not prove authenticity, truth, secrecy, or future mutation authority.
Lifecycle-core maintainers own continuation semantics. CLI/MCP maintainers own explicit storage and bounded access.
