# Add Preserves dependency-bump runbook

## Why

Cairn now has an internal Preserves canonicalization seam, checked-in corpus hashes, a dry-run/default corpus update rail, CI drift guards, and operator docs. Future `preserves` crate upgrades still need one explicit maintainer checklist so hash drift is either rejected automatically or refreshed intentionally with reviewable evidence.

## What changes

- Add a dedicated Preserves dependency-bump runbook.
- Link it from the existing Preserves receipt workflow docs and README command list.
- Validate that the runbook preserves the JSON public receipt boundary, uses the dry-run/fail-closed guard first, and permits `--execute` only after intentional drift review.

## Non-goals

- Do not bump `preserves` in this change.
- Do not change public/stored JSON receipt schemas.
- Do not add any automatic `--execute` behavior to CI.
