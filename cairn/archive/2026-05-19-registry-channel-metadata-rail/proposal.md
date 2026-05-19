## Why

Evidence registries can now aggregate, verify, select, and produce fetch plans for published evidence indexes. Downstream agents still need to know structural hashes or bundle kinds ahead of time to choose an entry. CI and operators need a deterministic, reviewable label layer such as channel, release, platform, and compatibility class so they can ask for "stable linux evidence" without relying on host-specific naming or out-of-band conventions.

## What Changes

- Add policy-validated registry metadata labels for published evidence entries.
- Extend registry build to accept stable labels and bind them into registry entries and receipts.
- Extend registry select/fetch-plan selectors to filter by labels.
- Reject malformed labels and duplicate/conflicting channel entries with deterministic issue codes.
- Keep receipts portable and secret-safe: labels are stable metadata; raw paths, URLs, environment values, and credentials remain excluded.

## Capabilities

### New Capabilities

- `registry-channel-metadata`: deterministic registry channel/release/platform metadata for agent and CI selection.

### Modified Capabilities

- `evidence-registry`: registry build/verify/select/fetch-plan preserve and validate stable metadata labels.

## Impact

- Files: `crates/cairn-cli/src/main.rs`, `crates/cairn-cli/tests/cli.rs`, `cairn-policy/default.ncl`, generated policy, docs, specs, goldens/fixtures if policy hashes change.
- APIs: public CLI flags on evidence registry build/select/fetch-plan.
- Dependencies: none; Rust/Nickel only.
- Testing: targeted registry evidence-chain tests, policy export freshness, public receipt schema tests, Tracey coverage, strict validation, workspace tests, clippy, flake check.