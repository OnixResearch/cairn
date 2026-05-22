# Automate current-HEAD release evidence index

## Why

The checked-in current-HEAD release evidence page proved the release/readiness and evidence publication chain, but the page is manually assembled and can become stale as soon as the evidence-recording commit lands. Operators need a first-class, deterministic rail that regenerates the same bounded index for the actual clean HEAD without hand-copying hashes from `target/` receipts.

## What Changes

- Add a release-evidence index generation command or subcommand that orchestrates the existing release-readiness, evidence export, verify, import, republish, index, and index-verify rails into one deterministic local bundle.
- Emit a bounded Markdown/JSON index that binds the payload commit from `git rev-parse --short HEAD`, clean/synced worktree evidence, generated artifact roles, receipt hashes, commands, and explicit non-claims.
- Keep generated bulky receipt artifacts under an operator-chosen output directory such as `target/`, while making the durable index safe to review or check in.
- Preserve existing release-readiness and evidence command semantics; this change only automates their composition and index rendering.

## Scope

In scope: CLI orchestration, deterministic receipt/index rendering, positive and negative tests for stale/dirty state handling, docs, policy/surface inventory updates if a new public receipt surface is added.

Out of scope: moving readiness tags, remote mirror publication, credential/transport validation, external trust claims, or changing existing evidence receipt schemas except additively.

## Verification

Validate with native Cairn gates, Tracey coverage, focused release/evidence tests, format/lint, and a local generated index whose payload commit matches `git rev-parse --short HEAD`.
