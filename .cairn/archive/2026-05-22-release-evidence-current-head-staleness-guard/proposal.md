# Proposal: Release evidence current-head staleness guard

## Summary

Add a first-class guard for checked-in release evidence pages so a document that claims to describe the current repository head cannot silently drift behind the live commit.

## Motivation

`docs/release-evidence-current-head.md` is committed documentation, but a committed file cannot safely remain "current HEAD" after later implementation or documentation commits. The current page names an older payload commit, which makes the title and operator workflow easy to misread even though the original payload evidence was valid when generated.

Cairn should make this invariant machine-checkable:

- committed evidence pages are immutable/audited payload snapshots, or
- if a page claims current-head status, it must match the live clean HEAD at check time.

## Scope

- Add a local, deterministic `release-evidence check-index` command that parses a release-evidence Markdown index and emits a receipt.
- Detect stale `current-head` claims by comparing the parsed payload commit to live git HEAD.
- Validate that the payload commit exists and is an ancestor of live HEAD.
- Update docs/help so checked-in release evidence is described as an audited payload snapshot unless it is freshly generated as an uncommitted/CI artifact.
- Wire focused tests and release-evidence verification so stale checked-in current-head claims fail closed.

## Non-goals

- No remote publication, upload, signing, credential, or tag authority.
- No mutation of evidence files by the checker.
- No attempt to prove public unattended production readiness.
