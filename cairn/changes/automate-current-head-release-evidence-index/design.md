# Design: automated current-HEAD release evidence index

## Context

`docs/release-evidence-current-head.md` documents the manual current-HEAD evidence chain. The workflow is valuable but fragile: it requires several commands, hash extraction, non-self-referential wording, and care around sync/Tracey changes that can stale receipt hashes. A first-class command should make the reviewed path repeatable.

## Goals

- Produce deterministic release evidence index output from a clean local checkout.
- Compose existing trusted rails rather than duplicating validators.
- Bind the payload commit and worktree status explicitly without writing self-referential final commit hashes.
- Keep generated receipt artifacts out of source control unless explicitly requested.
- Fail closed on dirty worktrees, invalid generated receipts, mismatched HEAD binding, or missing required artifact roles.

## Non-goals

- No tag creation or tag movement.
- No network publication, credential handling, or remote mirror trust validation.
- No broad receipt schema rewrite.
- No claims about production readiness beyond local/trusted Cairn evidence rails.

## Proposed shape

Add a narrow CLI surface, likely under `cairn release-evidence index` or an equivalent release-readiness/evidence subcommand. The command accepts `--root`, `--out`, and `--artifact-dir`/`--work-dir` paths. It runs or invokes the existing Rust functions behind:

1. `release-readiness`
2. `evidence export`
3. `evidence verify`
4. `evidence import`
5. `evidence republish`
6. `evidence index`
7. `evidence index verify`

The command then renders a deterministic index with:

- payload commit short hash and branch/worktree status,
- command list,
- artifact role/path/hash table,
- receipt verdict/hash table,
- explicit non-claims,
- generation timestamp avoided or normalized unless policy owns it.

## Risks and mitigations

- **Self-referential commit drift:** never write the evidence-recording commit into generated source-controlled output; bind only the payload commit being evaluated.
- **Stale receipt hashes:** generate after validation inputs are final, and add tests that compare rendered payload commit against live `git rev-parse --short HEAD`.
- **Overclaiming release readiness:** render explicit local/trusted scope and non-claims.
- **Core impurity:** keep git/process/filesystem orchestration in `cairn-cli`; pure formatting helpers may be unit-tested separately.
