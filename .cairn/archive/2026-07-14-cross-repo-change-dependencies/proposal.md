## Why

Stack work often depends on lifecycle evidence from another repository: Valence may need an Octet provenance export, Mantle may need a Valence identity sidecar, and Cairn may need release evidence from several siblings. Today those relationships are usually recorded in prose, branch timing, or local paths. That makes ordering easy to miss and hard for agents to validate deterministically.

Cairn should support cross-repo dependencies as explicit evidence links. A downstream change should be able to say "I require this upstream repo/change receipt at this stage/hash/status" and Cairn should fail closed when the presented evidence is missing, stale, failed, mismatched, or unsupported.

## What Changes

- Add a `cross_repo_dependencies` metadata contract for change packages.
- Validate dependencies against supplied Cairn receipt summaries or imported evidence bundles.
- Keep validation pure in `cairn-core`; keep filesystem resolution, bundle loading, and CLI/MCP presentation in `cairn-cli`.
- Report satisfied and blocked dependency rows with deterministic issue codes and a BLAKE3-bound receipt.
- Treat only explicit current passing upstream evidence as satisfying a dependency; blocked, skipped, deferred, timed-out, dry-run, cancelled, infrastructure-failed, malformed, or stale evidence remains non-passing.
- Return bounded actionable dependency diagnostics with stable IDs, status, identity mismatches, remediation, safe reproduction guidance, artifact refs, and non-claims.
- Integrate blocked cross-repo dependencies into validation, drain/release-readiness summaries, and operator docs.
- Preserve a clear non-claim: satisfying a cross-repo dependency proves receipt linkage only, not upstream correctness or downstream integration correctness.

## Impact

- **Files**: Nickel policy/contracts for metadata shape, core dependency validation, CLI/MCP receipt loading and reporting, fixtures, docs, and lifecycle specs.
- **Testing**: Positive fixtures for matching current upstream gate/validation/archive receipts; negative fixtures for every non-pass status, missing or stale evidence, wrong repo/change/kind/subject/policy, unsupported kind, unsafe diagnostics, and hash mismatch.
