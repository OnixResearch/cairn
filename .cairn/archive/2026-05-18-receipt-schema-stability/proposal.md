## Why

Cairn receipts are now deterministic and hash-linked, so downstream tools can begin depending on their JSON shape. Without an explicit schema stability rail, a future field rename or removal could silently break operators, release automation, or evidence comparison while existing hash/reproducibility tests still pass.

## What Changes

- r[cairn.supersede.receipt_schema] Define required public receipt fields for gate, sync, archive, and migration receipts.
- Add regression coverage that fails on accidental top-level receipt field removals or renames.
- Document the stable receipt contract where operators already learn about receipt hashes.

## Impact

- **Files**: CLI integration tests, README/docs, Cairn supersession spec.
- **Testing**: Native Cairn gates, Tracey coverage, workspace Rust tests, Slotcar, and flake checks.
