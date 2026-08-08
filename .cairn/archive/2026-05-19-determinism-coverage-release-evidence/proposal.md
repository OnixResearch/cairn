# Change: Determinism coverage release evidence

## Why

Cairn now exposes a policy-owned deterministic surface coverage audit, but release and manifest evidence can still omit that audit unless wrappers remember to run it. That leaves a gap between "coverage can be audited" and "coverage was audited for this release/agent evidence chain."

## What Changes

- Add coverage-audit evidence to release-readiness receipts so releases disclose whether deterministic surface coverage passed.
- Extend evidence bundles/manifests to carry optional coverage-audit artifacts with stable hash linkage.
- Reject stale, missing, failing, or malformed coverage-audit evidence deterministically when supplied or required by release-readiness.
- Keep all failure receipts path-safe and secret-safe.

## Capabilities

### Modified Capabilities
- `ironspec-supersession`: release and evidence-chain verification include deterministic coverage-audit evidence.

## Impact

- **Files**: Cairn CLI, policy schemas, docs, tests, fixtures/goldens, accepted specs.
- **APIs**: public receipt fields for release-readiness, evidence bundle verification, and evidence manifest verification/generation may grow with coverage-audit hash fields.
- **Testing**: focused CLI tests plus policy export, formatting, workspace tests, Tracey, release-readiness, coverage-audit, and flake checks.
