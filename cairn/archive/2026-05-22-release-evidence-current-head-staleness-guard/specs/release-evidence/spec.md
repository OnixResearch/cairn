# Release Evidence Specification Delta

## ADDED Requirements

### Requirement: Release evidence current-head staleness guard

r[release_evidence.current_head_staleness_guard.command] The system MUST provide a deterministic local checker for generated release-evidence index documents so stale current-head claims fail closed.

#### Scenario: Parser extracts generated payload fields

r[release_evidence.current_head_staleness_guard.parser]
- GIVEN a generated release-evidence Markdown index
- WHEN the checker reads the payload block
- THEN it extracts the payload commit, branch, clean marker, upstream, and ahead/behind fields
- AND missing or malformed payload fields are reported as deterministic validation issues

#### Scenario: Git checks bind payload to repository state

r[release_evidence.current_head_staleness_guard.git_checks]
- GIVEN a parsed release-evidence index and repository root
- WHEN the checker validates the payload commit
- THEN it verifies the commit exists locally
- AND it verifies the payload commit is an ancestor of the live HEAD
- AND current-head mode additionally requires the payload commit to equal the live HEAD

#### Scenario: Stale current-head claims fail closed

r[release_evidence.current_head_staleness_guard.tests]
r[release_evidence.current_head_staleness_guard.fail_closed]
- GIVEN a checked index whose path or mode claims current-head status
- WHEN its payload commit differs from live HEAD
- THEN the checker emits `valid=false`, `verdict=fail`, and a deterministic stale-current-head issue

#### Scenario: Release readiness includes checked-in evidence freshness

r[release_evidence.current_head_staleness_guard.release_readiness]
- GIVEN the conventional checked-in release evidence page exists
- WHEN `release-readiness` runs
- THEN the readiness receipt includes the index freshness check
- AND stale current-head claims contribute to a failing release-readiness verdict

#### Scenario: Operator docs distinguish current artifacts from audited snapshots

r[release_evidence.current_head_staleness_guard.operator_docs]
- GIVEN release evidence documentation is reviewed
- WHEN it describes generated current-head evidence and checked-in evidence pages
- THEN it says fresh current-head evidence is generated into local or CI artifacts
- AND checked-in evidence pages are audited payload snapshots unless explicitly refreshed for the current payload

#### Scenario: Package can be archived after verification

r[release_evidence.current_head_staleness_guard.archive_ready]
- GIVEN the checker, tests, readiness wiring, and docs pass focused and repository verification
- WHEN the operator syncs and archives the package
- THEN the accepted specs include the staleness guard requirements
- AND no active package remains for this slice
