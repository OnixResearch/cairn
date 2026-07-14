## Why

Cairn's workspace contract matrix compares declared provider/consumer edges with observed artifact identities. The current helper builds plain ordered maps keyed by `edge_id`; that is deterministic, but it still stores the key separately from values that already carry the same edge id and can hide duplicate observations through overwrite-style collection.

`iddqd` fits this boundary because workspace contract edges and observations are identity-bearing records. Adopting an ordered ID map in the pure validation core makes duplicate handling explicit and keeps matrix diagnostics deterministic.

## What Changes

- Use iddqd-backed ordered identity maps for workspace contract edge and observation indexes.
- Reject duplicate edge ids and duplicate observed artifact edge ids before stale-identity comparison.
- Keep public receipt schemas, matrix JSON shape, and release-readiness modes unchanged.
- Add positive and negative fixtures for unique observations, duplicate observations, missing observations, stale identities, and weakened non-claims.

## Impact

- Workspace contract validation stops relying on duplicated map keys for identity-bearing records.
- Existing matrix and receipt consumers keep the same serialized contract.
- This is a focused iddqd pilot in Cairn's release-evidence core, avoiding unrelated requirement-registry work that is already covered by active marker-core changes.
