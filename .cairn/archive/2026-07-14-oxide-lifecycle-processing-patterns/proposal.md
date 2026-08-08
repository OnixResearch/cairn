## Why

Cairn lifecycle work is increasingly multi-step: changes are created, validated, synced, archived, exported as evidence, and sometimes replayed by agents or release tools. Oxide's Steno, RFD API/site, Expectorate, support-bundle tooling, and large systems repositories provide useful patterns for saga-style execution logs, document processors, golden snapshots, bundle inspection, and reference-only system mining.

This change writes a Cairn package for adapting those patterns while preserving Cairn's native lifecycle model and deterministic receipt boundaries.

## What Changes

- Add a reference inventory for Steno, RFD API/site, Expectorate, bundle-cat, support-bundle-viewer, and selected study-only Oxide system repos.
- Define saga-style lifecycle execution logs for multi-step Cairn operations with recovery, compensation, and bounded operator intervention.
- Define RFD-style lifecycle document processing for source revisions, rendered assets, state transitions, search indexes, webhook inputs, and periodic scans.
- Define golden snapshot workflows for deterministic CLI output, receipts, and rendered lifecycle reports.
- Define bundle-inspection requirements for exported Cairn evidence bundles.
- Keep study-only system repos as mining inputs for observability, archive shape, and operator workflows, not implementation dependencies.

## Impact

- Cairn gains a roadmap package for safer multi-step operations and more reviewable lifecycle artifacts.
- Golden snapshots and bundle inspection become first-class validation targets.
- Study-only Oxide systems are captured without committing Cairn to domain-specific storage, VMM, network, OS, or firmware behavior.
