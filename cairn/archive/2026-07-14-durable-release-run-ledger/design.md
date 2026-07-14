## Design

1. Add a pure `ReleaseRunLedger` core model with run id, lifecycle root identity, policy hash, command identity, selected profile, step rows, artifact refs, diagnostics, blockers, resume hints, and non-claims.
2. Bind run identity with BLAKE3 over canonical ledger material using a named self-hash profile. Do not use remote service IDs as Cairn-owned identity.
3. Model steps as deterministic states: pending, running-observed, passed, failed, skipped, blocked, and superseded. CLI may render these states; core only validates transitions over in-memory values.
4. Add status/explain projections as pure functions that classify current blockers, stale inputs, missing evidence, resumable steps, and terminal verdicts.
5. Keep filesystem discovery, evidence-bundle reads, stdout/stderr, JSON rendering, and mutation planning in `cairn-cli`.
6. Keep publish/promotion/signing adapters out of this change. The ledger can reference external evidence but cannot mutate registries, GitHub releases, remote refs, or caches.
7. Add positive fixtures for complete, blocked, and resumable runs; add negative fixtures for stale policy hash, missing command identity, impossible step transition, weakened non-claim, cross-run artifact substitution, and remote-publish overclaim.

## Non-Goals

- No live publishing, tag creation, GitHub release creation, registry upload, signing, or remote mutation.
- No release-readiness pass from ledger validation alone.
- No external trust decision about SBOM, provenance, or security evidence truth.