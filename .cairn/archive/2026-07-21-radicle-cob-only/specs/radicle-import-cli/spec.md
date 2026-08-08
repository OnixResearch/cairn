# Radicle Import CLI Specification

## MODIFIED Requirements

### Requirement: Lossless native-change import plan
r[cairn.radicle_import_cli.lossless_plan] The system MUST deterministically map every present supported native active-change artifact into a validated `xyz.onixresearch.cairn-change` root without changing its UTF-8 bytes and without requiring Radicle runtime context.

#### Scenario: Exact supported artifacts are preserved
r[cairn.radicle_import_cli.lossless_plan.exact]
- GIVEN a native active change contains supported proposal, design, tasks, metadata, and delta-spec files
- WHEN Cairn plans a COB-only import
- THEN every present supported file MUST appear exactly once in root state with byte-identical UTF-8 content
- AND the plan MUST bind an explicit workspace identity and typed Git base revision.

## REMOVED Requirements

### Requirement: Current canonical authority
r[cairn.radicle_import_cli.current_authority] This identity-document authority requirement MUST be removed from the COB-only provider.

### Requirement: Signed typed creation and reload
r[cairn.radicle_import_cli.signed_creation] This profile-backed creation requirement MUST be removed from the COB-only provider.

### Requirement: Preview-first public command
r[cairn.radicle_import_cli.command] This `cairn radicle` command requirement MUST be removed in favor of `cairn cob`.

### Requirement: Deterministic import receipt
r[cairn.radicle_import_cli.receipt] This identity-document receipt requirement MUST be removed in favor of COB-only receipts.

### Requirement: Explicit claim boundary
r[cairn.radicle_import_cli.non_claims] This profile-specific claim requirement MUST be removed in favor of the COB-only claim boundary.
