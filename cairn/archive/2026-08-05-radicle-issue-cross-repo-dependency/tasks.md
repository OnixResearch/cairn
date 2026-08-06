# Tasks: Radicle issue cross-repo linkage

## Phase 1: Policy and contract

- [x] [serial] B1 Record baseline cross-repo receipts, supported receipt kinds, and validation output. r[cairn.radicle_issue_cross_repo.receipt_kind] [depends:cairn.cross_repo_change_dependencies.contract]
- [x] [serial] P1 Add the `radicle-issue.v1` receipt kind to the Nickel policy allowlist. r[cairn.radicle_issue_cross_repo.receipt_kind]
- [x] [depends:cairn.radicle_issue_cross_repo.receipt_kind] P2 Export deterministic generated policy and reject unknown kinds by admission. r[cairn.radicle_issue_cross_repo.receipt_kind.invalid]

## Phase 2: Pure core

- [x] [depends:cairn.radicle_issue_cross_repo.receipt_kind] C1 Add `radicle-issue.v1` to the supported receipt-kind allowlist in `cross_repo_dependencies.rs`. r[cairn.radicle_issue_cross_repo.receipt_kind]
- [x] [depends:cairn.radicle_issue_cross_repo.contract] C2 Add the typed `cairn.radicle-issue-receipt.v1` contract and a pure receipt builder from a bounded typed observation. r[cairn.radicle_issue_cross_repo.contract] r[cairn.radicle_issue_cross_repo.contract.identity] r[cairn.radicle_issue_cross_repo.contract.privacy]
- [x] [depends:cairn.radicle_issue_cross_repo.adapter_bounded] C3 Add the bounded read-only issue receipt builder with explicit non-claims, no reduction, and no write path. r[cairn.radicle_issue_cross_repo.adapter_bounded] r[cairn.radicle_issue_cross_repo.adapter_bounded.observe] r[cairn.radicle_issue_cross_repo.adapter_bounded.non_claims]
- [x] [depends:cairn.radicle_issue_cross_repo.validation] C4 Confirm the pure cross-repo evaluator satisfies and blocks `radicle-issue.v1` rows through exact matching. r[cairn.radicle_issue_cross_repo.validation] r[cairn.radicle_issue_cross_repo.validation.match] r[cairn.radicle_issue_cross_repo.validation.mismatch]

## Phase 3: Verification

- [x] [parallel] V1 Add positive fixtures for a matching issue linkage producing a satisfied row. r[cairn.radicle_issue_cross_repo.final_validation.positive]
- [x] [parallel] V2 Add negative fixtures for stale, wrong-repo, wrong-issue, wrong-kind, unsupported-kind, malformed, and hash-mismatch issue evidence. r[cairn.radicle_issue_cross_repo.final_validation.negative]
- [x] [depends:cairn.radicle_issue_cross_repo.final_validation] V3 Run policy freshness, formatting, focused tests, Clippy, traceability, lifecycle gates, release readiness, and relevant Nix checks. r[cairn.radicle_issue_cross_repo.final_validation.gates]

## Verification Coverage

- `Scenario: Issue linkage with a matching receipt passes` -> C1, C2, V1
- `Scenario: Unsupported issue evidence kind fails closed` -> P2, V2
- `Scenario: Receipt binds issue identity and observed state` -> C2, V1
- `Scenario: Issued receipt omits raw private transport data` -> C2, V2
- `Scenario: Adapter emits only a bounded typed observation` -> C3, V1
- `Scenario: Issue receipt carries explicit non-claims` -> C3, V2
- `Scenario: Matching issue evidence is satisfied` -> C2, C4, V1
- `Scenario: Mismatched issue evidence is blocked` -> C4, V2
- `Scenario: Positive issue fixtures pass` -> V1, V3
- `Scenario: Negative issue fixtures fail closed` -> V2, V3
- `Scenario: Repository gates cover the feature` -> V3
