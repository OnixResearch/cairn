# ROI 03: Preserves corpus policy inventory

## Why

The Preserves corpus/update rail is now important deterministic operator tooling. Adding it to policy-owned inventory/replay coverage would make future dependency-bump drift harder to overlook.

## What changes

- Add Preserves corpus freshness/update rail to the deterministic surface inventory.
- Add a replay/policy case that executes the dry-run freshness path in-process or through existing deterministic support.
- Update coverage audit and replay tests.

## Non-goals

- Do not run `--execute` in replay or CI.
- Do not duplicate Preserves hashing logic outside the core seam.
