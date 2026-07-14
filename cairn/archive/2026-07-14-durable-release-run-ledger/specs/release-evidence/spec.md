# Release Evidence Specification

## Purpose

Add a durable release-run ledger for Cairn-owned lifecycle and release evidence workflows.

## Requirements

### Requirement: Durable release-run ledger model
r[cairn.release_evidence.durable_run_ledger.model] Cairn MUST define a pure durable release-run ledger model with run identity, lifecycle root identity, policy hash, command identity, profile, step rows, artifact references, diagnostics, blockers, resume hints, and non-claim boundaries.

#### Scenario: Complete run ledger validates
r[cairn.release_evidence.durable_run_ledger.fixtures.positive]
- GIVEN a release-run ledger records a complete local release-readiness workflow with matching policy hash, command identity, step rows, artifact references, diagnostics, and non-claims
- WHEN ledger validation runs
- THEN Cairn MUST accept the ledger and preserve the run identity as a local evidence artifact.

### Requirement: Ledger validation fails closed
r[cairn.release_evidence.durable_run_ledger.validation] Cairn MUST reject release-run ledgers with stale identity, malformed state transitions, missing command identity, missing artifact references, or weakened boundaries.

#### Scenario: Invalid ledger is rejected
r[cairn.release_evidence.durable_run_ledger.fixtures.negative]
- GIVEN a release-run ledger has a stale policy hash, missing command identity, impossible step transition, weakened non-claim, cross-run artifact substitution, or remote publish overclaim
- WHEN ledger validation runs
- THEN Cairn MUST fail with deterministic diagnostics naming the rejected ledger field.

### Requirement: Status and explain projections are deterministic
r[cairn.release_evidence.durable_run_ledger.status_explain] Cairn SHOULD expose deterministic status and explain projections over already-loaded release-run ledgers.

#### Scenario: Blocked run explains next action
r[cairn.release_evidence.durable_run_ledger.shell]
- GIVEN a release-run ledger has blocked or resumable steps
- WHEN an operator asks for status or explain output
- THEN Cairn SHOULD report blockers, stale inputs, missing evidence, resumable steps, terminal verdict, and next recommended local action without mutating remote services.

### Requirement: Ledger validation remains local evidence only
r[cairn.release_evidence.durable_run_ledger.final_validation] Cairn MUST state that passing release-run ledger validation proves only ledger syntax, identity linkage, transition validity, and non-claim preservation.

#### Scenario: Ledger does not publish or promote
- GIVEN a release-run ledger validates
- WHEN release-readiness or evidence export consumes it
- THEN the ledger MUST NOT by itself publish artifacts, create tags, create GitHub releases, upload registry packages, sign artifacts, prove release eligibility, prove evidence truth, or trust external services.