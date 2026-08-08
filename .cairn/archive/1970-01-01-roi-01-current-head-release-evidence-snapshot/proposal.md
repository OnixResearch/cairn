# ROI 01: current-HEAD release evidence snapshot

## Why

Cairn is clean and drained after the Preserves evidence stack. The highest-ROI next step is durable current-HEAD proof rather than another feature slice: generate and validate a local release-evidence snapshot for the exact post-Preserves commit without implying a checked-in page remains perpetually current.

## What changes

- Generate a current-HEAD release evidence index under `target/release-evidence-current-head/`.
- Validate the generated index with the existing release-evidence checker and release-readiness rails.
- If any evidence summary is committed, phrase it as a verified-code snapshot and keep generated receipts/artifacts under `target/` out of source control.

## Non-goals

- Do not move readiness tags.
- Do not commit generated `target/` receipts.
- Do not claim unattended production readiness beyond the validated local evidence chain.
