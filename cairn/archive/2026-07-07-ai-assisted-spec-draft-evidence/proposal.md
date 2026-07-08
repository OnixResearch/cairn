# Change: AI-assisted spec draft evidence

## Why

The Kani paper reports an effective workflow where an AI assistant drafts contracts and harnesses, but the resulting specifications are trusted only after tool checks and human review. Cairn should make that boundary explicit for lifecycle evidence.

## What Changes

- Add optional AI-assisted draft provenance for lifecycle specs, contracts, harnesses, and evidence notes.
- Require machine-checked receipts or human-review acceptance before AI-drafted artifacts can satisfy verification tasks.
- Record failed or revised draft checks as diagnostics, not trusted evidence.
- Add positive and negative fixtures for accepted checked drafts and unsupported AI-only claims.

## Impact

- **Files**: lifecycle policy vocabulary, gate checks, receipt models, fixtures, and docs.
- **Consumers**: agent-authored Kani contracts or Cairn specs can be reviewed without pretending AI output is proof.
- **Non-claims**: Cairn records draft provenance and validation status; it does not certify AI reasoning.
