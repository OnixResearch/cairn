# Radicle Import Cli Specification

## Purpose

Defines the `radicle-import-cli` capability.

## Requirements

### Requirement: Lossless native-change import plan
r[cairn.radicle_import_cli.lossless_plan] The system MUST deterministically map every present supported native active-change artifact into a validated `xyz.onixresearch.cairn-change` root without changing its UTF-8 bytes and without requiring Radicle runtime context.

#### Scenario: Exact supported artifacts are preserved
r[cairn.radicle_import_cli.lossless_plan.exact]
- GIVEN a native active change contains supported proposal, design, tasks, metadata, and delta-spec files
- WHEN Cairn plans a COB-only import
- THEN every present supported file MUST appear exactly once in root state with byte-identical UTF-8 content
- AND the plan MUST bind an explicit workspace identity and typed Git base revision.
