## Why

Cairn owns lifecycle gates, release-readiness, evidence export, sync/archive planning, and deterministic receipts. The dpub repository is a useful reference for durable publish runs, status/explain/resume UX, SBOM/provenance/security evidence, and sanitized public release-history artifacts. Cairn should adapt those ideas into Cairn-owned release-run ledgers without adopting remote publishing authority.

Reference source: `https://github.com/joweeba/dpub`.

## What Changes

- Add a durable release-run ledger model for Cairn lifecycle/release workflows.
- Record run identity, command identity, policy hash, input artifact identities, step states, resumable blockers, evidence bundle refs, and explicit non-claims.
- Add `status` and `explain`-ready projections over already-loaded ledger data.
- Preserve remote-changing publish, GitHub release, registry upload, and signing authority as out-of-scope unless later changes define guarded adapters.

## Impact

- **Surfaces**: release evidence core, release-readiness UX, evidence export/verify, operator docs.
- **Non-claims**: The ledger proves receipt/run structure and hash linkage only; it does not publish artifacts, prove release eligibility, or trust remote services.
- **Validation**: pure ledger validation tests, positive/negative fixtures, CLI status/explain smoke, Cairn gates/validate.