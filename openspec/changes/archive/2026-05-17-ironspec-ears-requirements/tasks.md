## Phase 1: Spec and design

- [x] [serial] Define EARS-style validation requirements for accepted and delta specs.
- [x] [serial] Record FCIS design for pure markdown validation and CLI filesystem reads.

## Phase 2: Implementation

- [x] [serial] Add pure core validation helpers for normative requirement strength and GIVEN/WHEN/THEN scenario completeness.
- [x] [depends:core-validation] Extend CLI `validate` to include accepted-spec and active-delta-spec issues in deterministic JSON receipts.
- [x] [depends:cli-validation] Add unit tests, CLI integration coverage, and README/self-dogfood updates.

## Phase 3: Verification

- [x] [depends:tests] Run formatting, tests, clippy, Slotcar, OpenSpec validation, and flake checks.
- [x] [depends:verification] Sync accepted spec, archive the change, and commit the verified slice.
