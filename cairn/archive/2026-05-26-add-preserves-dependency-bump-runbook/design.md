# Design: Preserves dependency-bump runbook

Add `docs/preserves-dependency-bumps.md` as a maintainer checklist for dependency upgrades that may affect Preserves canonical packed bytes.

The runbook covers:

1. Preconditions: clean worktree, identify the dependency change, and keep JSON public receipt semantics unchanged.
2. Dry-run first: run the existing corpus update rail without `--execute` and both Nix CI guards.
3. No-drift path: if the corpus is clean, run focused checks and commit the dependency bump without corpus hash churn.
4. Intentional-drift path: review the cause, run `--execute`, inspect only `expected_hash` changes, then run focused and workspace verification.
5. Landing evidence: Cairn validate/gates/Tracey, relevant tests, and no secrets in fixtures/receipts.

Tracey coverage is attached to the existing Preserves corpus integration test because the runbook is a documentation contract for already-existing rails.
