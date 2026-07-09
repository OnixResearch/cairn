# Verification

## Baseline

Transcript: `/tmp/onix-workspace-contract-baseline-20260709.txt`

- Proposal, design, and tasks gates passed before implementation.
- `nix run path:$PWD#cairn -- validate --root .` passed.
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-workspace-contract nix develop -c cargo test -p cairn-core workspace_contract_matrix -- --nocapture` passed with `0` matching pre-change tests.

## Focused implementation checks

Transcripts:

- `/tmp/onix-workspace-contract-first-check-20260709.txt`
- `/tmp/onix-workspace-contract-focused-20260709.txt`
- `/tmp/onix-workspace-contract-policy-check-20260709.txt`
- `/tmp/onix-workspace-contract-release-readiness-20260709.txt`

Results:

- Formatting passed.
- Focused workspace contract core tests passed (`7` tests).
- Positive representative matrix fixture passed and emitted observed BLAKE3 artifact identities.
- Negative fixtures for missing owner, unsafe path, stale identity, missing negative coverage, and weakened non-claims failed closed.
- `policy export --check` initially reported stale generated policy; `policy export --output cairn-policy/generated/cairn-policy.json` refreshed it and the recheck passed.
- Release-readiness smoke with a valid matrix receipt passed in default disabled mode and included workspace contract matrix context.

A mid-change release-readiness test sweep was intentionally not final evidence because traceability fails before this change is synced/accepted; final validation after archive reruns the relevant suite.

## Archive and final validation

Transcripts:

- `/tmp/onix-workspace-contract-gate-sync-archive-20260709.txt`
- `/tmp/onix-workspace-contract-post-archive-repair-20260709.txt`
- `/tmp/onix-workspace-contract-final-validation-2-20260709.txt`

Results:

- Proposal, design, and tasks gates passed.
- Cairn validation passed before sync, after sync, after archive, after accepted-spec repair, and in final validation.
- Sync did not merge the workspace contract matrix delta into the long-lived `release-evidence` accepted spec; the accepted spec was repaired by appending the archived delta.
- Final validation passed policy export refresh/check, `cargo fmt --check`, focused core tests, focused release-readiness tests, scoped clippy for `cairn-core`/`cairn-cli`, positive/negative workspace matrix fixture validation, release-readiness smoke with a matrix receipt, Cairn validation, and traceability coverage.
- Refreshing the generated policy updated the agent receipt fixture's replay policy hash and self hash so MCP smoke and release-readiness remain current.

## Non-claims

This proves matrix declaration completeness, artifact identity freshness, fail-closed fixture behavior, and release-readiness mode wiring. It does not prove provider correctness, consumer behavioral correctness, verifier soundness, or release eligibility by itself.
