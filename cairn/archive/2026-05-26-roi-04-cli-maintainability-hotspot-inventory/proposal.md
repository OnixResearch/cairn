# ROI 04: CLI maintainability hotspot inventory

## Why

After v0.1.0, the highest-risk refactors are remaining large CLI/core modules. Before moving code, a small inventory gives future Cairn packages narrow seams and verification commands.

## Scope

- Record the current largest source/test hotspots.
- Identify candidate future splits and focused checks.
- Avoid behavior changes in this slice.

## Non-goals

- No module moves, public API changes, fixture churn, or broad formatting-only rewrites.
