# Incremental agent observations

`cairn observe` provides a stateless continuation contract for three bounded
views:

- `active` — the complete active-change listing.
- `readiness CHANGE` — the selected stage, blocker, and action entries of the
  compact readiness view (full stage decisions remain detail-only).
- `context` — exact targeted-context selections for one admitted lifecycle
  artifact.

Every request is a complete view, one page, an unchanged comparison, or an
exact difference against an explicitly supplied prior observation. Cairn does
not retain hidden session state, and ordinary reads do not write snapshots.

## Grammar

```text
cairn observe active [--root PATH] [--policy PATH]
  [--page-entries N] [--page-bytes N] [--cursor HEX]
  [--prior ROOT_RELATIVE_FILE] [--mode complete|page|unchanged|difference]
  [--output ROOT_RELATIVE_FILE]

cairn observe readiness CHANGE [--root PATH] [--policy PATH]
  [--max-bytes N] [--max-entries N]
  [--page-entries N] [--page-bytes N] [--cursor HEX]
  [--prior ROOT_RELATIVE_FILE] [--mode complete|page|unchanged|difference]
  [--output ROOT_RELATIVE_FILE]

cairn observe context --scope accepted|active|archived --artifact ID
  [--root PATH] [--change NAME] [--spec ID]
  (--requirement-id ID | --task-id ID | --heading PATH)...
  [--page-entries N] [--page-bytes N] [--cursor HEX]
  [--prior ROOT_RELATIVE_FILE] [--mode complete|page|unchanged|difference]
  [--output ROOT_RELATIVE_FILE]
```

MCP exposes the same request as `cairn_observe`. It accepts a bounded inline
`prior` object or a root-relative `prior_path`; it never writes an output file.
`--output` and `prior_path` are root-relative normal paths. Cairn rejects
absolute paths, `..` components, and symbolic-link destinations or sources.

## Identity

A snapshot binds a cohort: admitted root, optional store, source manifest,
lifecycle policy hash, producer policy hash, evidence hash, producer contract,
view, selectors, and ordering version. The continuation identity includes all
of those fields and is carried by page cursors and unchanged comparisons.

Difference admission binds the contract identity instead: root, store, policy
hashes, producer contract, view, selectors, and ordering. Source and evidence
drift is exactly what a difference reports. A different root, policy, view, or
selector set is a mixed cohort and requires a complete refresh, never a claimed
difference or `unchanged`.

The new BLAKE3 domains are:

- `cairn.incremental-observation.cohort.v1`
- `cairn.incremental-observation.contract.v1`
- `cairn.incremental-observation.entry.v1`
- `cairn.incremental-observation.snapshot.v1`
- `cairn.incremental-observation.page.v1`
- `cairn.incremental-observation.unchanged.v1`
- `cairn.incremental-observation.difference.v1`

A content digest identifies bytes. It is not authentication and does not
authorize access or mutation.

## Pages and replay

Entries are whole values ordered lexicographically by stable identity under
`lexicographic-identity-byte-order.v1`. Pages never split an entry. The page
byte budget covers the serialized `entries` array, including its brackets.

Each page reports total, included, omitted, remaining, and an explicit
`complete` or `partial` state. A cursor carries the complete cohort and snapshot
identity, the last stable identity, the page contract, and the page limits.
Replaying the same cursor against an unchanged cohort returns the same page and
the same page hash; continuing state does not exist outside the cursor. Applying
the same page twice is a consumer error detected through `page_hash`.

A changed source, policy, evidence, view, selector, or ordering identity makes
the cursor stale. Cairn never mixes entries from two source states.

## Baselines and differences

A saved complete snapshot is a baseline. Partial, truncated, malformed, or
hash-mismatched prior observations are explicit errors, never an empty
baseline. A snapshot must satisfy its recorded counts, ordering, entry hashes,
cohort identity, contract identity, and receipt hash before use.

Differences return added and changed current entries plus removed prior
identities. `unchanged` is true only when the entries and the complete cohort
identity agree; source or evidence drift is visible through differing snapshot
hashes even when no entry changed. Stable identities survive content edits; a
changed identity is a removal plus an addition. Omitting pages, treating
unavailable state as unchanged, or forging counts is rejected by the same
schema admission.

## Authority and limits

Saved observations are disposable shell storage with no domain authority.
Fresh mutation preflight, evidence checks, and lifecycle gates still run before
any mutation. A repeated bounded read does not establish an atomic filesystem
snapshot. Observation identities do not prove source truth, secrecy,
authenticity, or future mutation authority.
