## Phase 1: Spec and design

- [x] [serial] Define read parity requirements for change/spec list/show receipts.
- [x] [serial] Record FCIS design for read model discovery and presentation.

## Phase 2: Implementation

- [x] [serial] Add pure core read models for change and spec summaries/details.
- [x] [depends:core-read-models] Add CLI `change list`, `change show`, `spec list`, and `spec show` commands.
- [x] [depends:cli-read-commands] Add CLI integration tests and canonical JSON goldens.
- [x] [depends:cli-read-commands] Update README and self-dogfood coverage.

## Phase 3: Verification

- [x] [depends:tests-docs] Run cargo tests, policy freshness, fmt, clippy, Slotcar, core purity search, OpenSpec validation, and flake checks.
- [x] [depends:verification] Sync accepted spec and archive the completed change.
