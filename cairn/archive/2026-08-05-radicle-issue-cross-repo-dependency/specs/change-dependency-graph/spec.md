# Change Dependency Graph Specification Delta

## ADDED Requirements

### Requirement: Radicle issue receipt kind is supported

r[cairn.radicle_issue_cross_repo.receipt_kind] Cairn MUST accept the `radicle-issue.v1` cross-repo dependency receipt kind in the same allowlist as `proposal_gate`, `design_gate`, `tasks_gate`, `validation_receipt`, `sync_receipt`, and `archive_receipt`.

#### Scenario: Issue linkage with a matching receipt passes

r[cairn.radicle_issue_cross_repo.receipt_kind.valid]
- GIVEN a change declares a cross-repo dependency row with receipt kind `radicle-issue.v1` and an explicit matching issue receipt
- WHEN Cairn validates the dependency
- THEN the row MUST be `satisfied` and MUST preserve the repo identity, issue id, receipt kind, status, and hash constraints.

#### Scenario: Unsupported issue evidence kind fails closed

r[cairn.radicle_issue_cross_repo.receipt_kind.invalid]
- GIVEN a dependency row uses an unsupported issue kind or a malformed issue receipt
- WHEN Cairn validates the dependency
- THEN Cairn MUST emit the stable unsupported-kind issue code and MUST NOT promote the row to `satisfied`.

### Requirement: Issue receipt contract is typed and bounded

r[cairn.radicle_issue_cross_repo.contract] Cairn MUST define a typed `cairn.radicle-issue-receipt.v1` contract that records the upstream repo identity, issue id, COB object id, observed status, and subject hash constraints.

#### Scenario: Receipt binds issue identity and observed state

r[cairn.radicle_issue_cross_repo.contract.identity]
- GIVEN a bounded typed issue observation for one Radicle issue
- WHEN Cairn builds the issue receipt in pure core
- THEN the receipt MUST bind the repo identity, issue id, COB object id, observed status, subject hash, and receipt hash deterministically.

#### Scenario: Issued receipt omits raw private transport data

r[cairn.radicle_issue_cross_repo.contract.privacy]
- GIVEN a bounded issue receipt is built
- WHEN Cairn renders or stores it
- THEN the receipt MUST NOT include credentials, raw action payloads, environment values, or unbounded logs.

### Requirement: Issue reading is a bounded read-only adapter

r[cairn.radicle_issue_cross_repo.adapter_bounded] Cairn MUST build a `radicle-issue.v1` receipt only from a bounded typed issue observation. Cairn MUST NOT reduce the issue into lifecycle state or author an update to it.

#### Scenario: Adapter emits only a bounded typed observation

r[cairn.radicle_issue_cross_repo.adapter_bounded.observe]
- GIVEN a bounded typed issue observation
- WHEN Cairn builds the issue receipt
- THEN Cairn MUST emit only the bounded typed receipt value and MUST NOT mutate any COB or lifecycle file.

#### Scenario: Issue receipt carries explicit non-claims

r[cairn.radicle_issue_cross_repo.adapter_bounded.non_claims]
- GIVEN an issue receipt is produced
- WHEN Cairn documents or renders the supported claim
- THEN the receipt MUST state that it proves bounded local observation of one issue only and does not prove closure authority, peer convergence, remote trust, semantic truth, or release eligibility.

### Requirement: Issue evidence validation is pure core

r[cairn.radicle_issue_cross_repo.validation] Cairn MUST validate `radicle-issue.v1` dependency rows and normalized issue receipt summaries in pure deterministic core logic.

#### Scenario: Matching issue evidence is satisfied

r[cairn.radicle_issue_cross_repo.validation.match]
- GIVEN an issue dependency row and a supplied issue receipt with matching repo, issue id, kind, expected status, and hash constraints
- WHEN the pure cross-repo evaluator runs
- THEN the row status MUST be `satisfied` with no blocker issue.

#### Scenario: Mismatched issue evidence is blocked

r[cairn.radicle_issue_cross_repo.validation.mismatch]
- GIVEN an issue dependency row and missing, stale, wrong-repo, wrong-issue, wrong-kind, unsupported, or hash-mismatched evidence
- WHEN the pure cross-repo evaluator runs
- THEN the row MUST retain a deterministic blocked status with a stable issue code.

### Requirement: Issue linkage final verification

r[cairn.radicle_issue_cross_repo.final_validation] The change MUST include positive and negative verification evidence before archive.

#### Scenario: Positive issue fixtures pass

r[cairn.radicle_issue_cross_repo.final_validation.positive]
- GIVEN a matching issue receipt fixture for the supported kind
- WHEN focused core and CLI tests run
- THEN valid fixtures MUST produce deterministic satisfied-row receipts.

#### Scenario: Negative issue fixtures fail closed

r[cairn.radicle_issue_cross_repo.final_validation.negative]
- GIVEN missing, stale, wrong-repo, wrong-issue, wrong-kind, unsupported-kind, malformed, and hash-mismatch issue fixtures
- WHEN focused core and CLI tests run
- THEN invalid fixtures MUST fail closed with expected issue codes and MUST NOT mutate lifecycle files.

#### Scenario: Repository gates cover the feature

r[cairn.radicle_issue_cross_repo.final_validation.gates]
- GIVEN implementation and documentation are complete
- WHEN Cairn validation, proposal gate, design gate, tasks gate, focused tests, and release-readiness checks run
- THEN the evidence MUST show this change is ready to sync and archive.
