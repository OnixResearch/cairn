# ROI 02: Preserves release-readiness summary

## Why

Preserves now has an internal hash seam, corpus rails, CI guards, and operator docs, but release-readiness receipts do not summarize whether those rails are present or fresh. Surfacing a concise Preserves readiness section would make operator evidence more self-contained.

## What changes

- Add a release-readiness summary section for internal Preserves receipt/hash readiness.
- Include corpus freshness/CI guard status or deterministic references to existing rails.
- Preserve public/stored JSON receipt semantics and release-readiness schema stability guarantees.

## Non-goals

- Do not promote Preserves hashes into stored agent-run receipt schemas.
- Do not run `--execute` from release-readiness.
- Do not replace JSON `receipt_hash` compatibility.
