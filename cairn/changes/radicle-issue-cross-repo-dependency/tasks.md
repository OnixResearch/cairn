# Tasks: Radicle issue cross-repo linkage

## Phase 1: Policy and contract

- [ ] [serial] B1 Record baseline cross-repo receipts, supported receipt kinds, validation output, and release readiness. r[cairn.radicle_issue_cross_repo.receipt_kind] [depends:cairn.cross_repo_change_dependencies.contract]
- [ ] [serial] P1 Add generated policy for the `radicle-issue.v1` receipt kind and the `cairn.radicle-issue-receipt.v1` contract. r[cairn.radicle_issue_cross_repo.contract] r[cairn.radicle_issue_cross_repo.receipt_kind]
- [ ] [depends:cairn.radicle_issue_cross_repo.contract] P2 Export deterministic issue receipt policy and reject unknown kinds, malformed hashes, and unsafe identities. r[cairn.radicle_issue_cross_repo.contract]

## Phase 2: Pure core

- [ ] [depends:cairn.radicle_issue_cross_repo.receipt_kind] C1 Add `radicle-issue.v1` to the supported receipt-kind allowlist. r[cairn.radicle_issue_cross_repo.receipt_kind]
- [ ] [depends:cairn.radicle_issue_cross_repo.contract] C2 Add pure issue receipt parsing, bounded normalization, and status mapping. r[cairn.radicle_issue_cross_repo.contract]
- [ ] [depends:cairn.radicle_issue_cross_repo.validation] C3 Confirm the pure cross-repo evaluator satisfies and blocks `radicle-issue.v1` rows through exact matching. r[cairn.radicle_issue_cross_repo.validation]

## Phase 3: Read-only adapter

- [ ] [depends:cairn.radicle_issue_cross_repo.contract] A1 Add a bounded read-only `radicle_issue` decoder in `cairn-radicle-cob` that emits an issue receipt from a local COB store. r[cairn.radicle_issue_cross_repo.adapter_bounded] r[cairn.radicle_issue_cross_repo.contract]
- [ ] [depends:cairn.radicle_issue_cross_repo.adapter_bounded] A2 Bind the COB object id, observed status, subject hash, and receipt hash in the emitted receipt with explicit non-claims. r[cairn.radicle_issue_cross_repo.adapter_bounded]

## Phase 4: CLI shell

- [ ] [depends:cairn.radicle_issue_cross_repo.shell] S1 Load `radicle-issue.v1` evidence only from explicit `--cross-repo-evidence` files or imported bundles. r[cairn.radicle_issue_cross_repo.shell]
- [ ] [depends:cairn.radicle_issue_cross_repo.shell] S2 Keep the no-fetch default and confirm no node, peer, or remote behavior is added. r[cairn.radicle_issue_cross_repo.shell]
- [ ] [depends:cairn.radicle_issue_cross_repo.validation] S3 Surface unsatisfied issue rows as read-only blockers in validation and release-readiness plans without mutation. r[cairn.radicle_issue_cross_repo.validation]

## Phase 5: Verification and documentation

- [ ] [parallel] V1 Add positive open and closed issue receipt fixtures for satisfied rows. r[cairn.radicle_issue_cross_repo.final_validation]
- [ ] [parallel] V2 Add negative missing, stale, wrong-repo, wrong-issue, wrong-kind, unsupported-kind, malformed, and hash-mismatch issue fixtures. r[cairn.radicle_issue_cross_repo.final_validation]
- [ ] [depends:cairn.radicle_issue_cross_repo.docs] D1 Document issue linkage metadata, the receipt schema, status mapping, no-fetch behavior, and non-claims. r[cairn.radicle_issue_cross_repo.docs]
- [ ] [depends:cairn.radicle_issue_cross_repo.final_validation] V3 Run policy freshness, formatting, focused tests, Clippy, traceability, lifecycle gates, release readiness, and relevant Nix checks. r[cairn.radicle_issue_cross_repo.final_validation]

## Verification Coverage

- `Scenario: Issue linkage with a matching receipt passes` -> C1, S1, V1
- `Scenario: Unsupported issue evidence kind fails closed` -> C1, C2, V2
- `Scenario: Receipt binds issue identity and observed state` -> C2, A1, A2
- `Scenario: Issued receipt omits raw private transport data` -> C2, A2
- `Scenario: Adapter observes only typed COB histories` -> A1, A2
- `Scenario: Issue receipt carries explicit non-claims` -> A2, D1
- `Scenario: Matching issue evidence is satisfied` -> C2, C3, V1
- `Scenario: Mismatched issue evidence is blocked` -> C3, S3, V2
- `Scenario: Explicit issue receipt is summarized` -> S1, V1
- `Scenario: No implicit issue fetch occurs` -> S2, D1
- `Scenario: Positive issue fixtures pass` -> V1, V3
- `Scenario: Negative issue fixtures fail closed` -> V2, S3, V3
