## Phase 1: Policy, schema, and pure inputs

- [x] [serial] Define the Nickel-owned authoritative agent verification contract, pass-only status vocabulary, bounded diagnostic contract, and versioned generated schema using named policy fields. r[cairn.supersede.agent_run_receipts.verification_contract]
- [x] [serial] Add typed normalized contract, required-check, prerequisite, execution-result, final-subject, artifact-reference, and diagnostic DTOs in the pure agent receipt core. r[cairn.supersede.agent_run_receipts.execution_boundary]
- [x] [serial] Define canonical BLAKE3 material for registry, selection, command, subject, execution evidence, artifacts, aggregate input, and completion receipt identities. r[cairn.supersede.agent_run_receipts.freshness]
- [x] [serial] Add the authoritative schema/profile migration and classify legacy v1 success receipts as inspectable `legacy_unscoped` evidence rather than completion. r[cairn.supersede.agent_run_receipts.schema_migration]

## Phase 2: Authoritative completion core

- [x] [serial] Implement pure exact-coverage validation for missing, duplicate, unknown, command-mismatched, and prerequisite-incomplete check rows. r[cairn.supersede.agent_run_receipts.pass_only]
- [x] [serial] Make `passed` the only completion-eligible status and retain every other supported status in deterministic non-passing sets and diagnostics. r[cairn.supersede.agent_run_receipts.pass_only]
- [x] [serial] Validate registry, selection, policy, final-subject, command, evidence, execution-receipt, and artifact freshness without relying on clocks or checkout paths. r[cairn.supersede.agent_run_receipts.freshness]
- [x] [serial] Derive required/passed/non-passing sets, `valid`, verdict, diagnostics, and completion receipt hash exactly once from the final issue set. r[cairn.supersede.agent_run_receipts.completion_authority]
- [x] [serial] Normalize actionable diagnostic rows with stable IDs, bounded source spans, remediation, reproduction argv, safe artifact refs, hashes, severity, and non-claims; reject unsafe payload fields. r[cairn.supersede.agent_run_receipts.actionable_diagnostics]

## Phase 3: Shell and public surface integration

- [x] [serial] Add bounded explicit CLI loading for repository-owned verification contracts and saved execution receipts, passing normalized values to the pure core without launching checks. r[cairn.supersede.agent_run_receipts.execution_boundary]
- [x] [serial] Extend CLI agent receipt validation with an authoritative completion mode whose structured receipt is the decision authority and whose automation behavior cannot confuse command success with completion success. r[cairn.supersede.agent_run_receipts.completion_authority]
- [x] [serial] Route MCP agent completion validation through the same contract, result, freshness, diagnostic, and finalization core as CLI validation. r[cairn.supersede.agent_run_receipts.schema_migration]
- [x] [serial] Update public Rust exports and compatibility shims so legacy inspection is explicit and new wrappers use the contract-aware completion API. r[cairn.supersede.agent_run_receipts.schema_migration]

## Phase 4: Fixtures, docs, and closeout

- [x] [parallel] Add positive fixtures for single and multi-check contracts, prerequisite closure, exact command matching, final-subject freshness, safe diagnostics, and stable cross-root hashes. r[cairn.supersede.agent_run_receipts.authoritative_verification.positive]
- [x] [parallel] Add negative fixtures for missing/self-authored contracts, missing/duplicate/unknown checks, every non-pass status, failed prerequisites, stale subject, registry/selection/policy drift, command mismatch, malformed hashes, unsafe diagnostics, contradictory outcomes, tampering, and legacy v1 success. r[cairn.supersede.agent_run_receipts.authoritative_verification.negative]
- [x] [serial] Update agent receipt docs, wrapper flow, schema examples, status semantics, completion citation guidance, compatibility notes, and bounded non-claims. r[cairn.supersede.agent_run_receipts.completion_authority]
- [x] [serial] Refresh generated policy, JSON fixtures, replay cases, plan fixtures, public schema checks, and Preserves/receipt hash goldens as one reviewed compatibility change. r[cairn.supersede.agent_run_receipts.schema_migration]
- [x] [serial] Run focused core/CLI/MCP tests, policy freshness, determinism replay, public schema checks, Cairn validation and proposal/design/tasks gates, Tracey coverage, Slotcar, and release-readiness checks before sync/archive. r[cairn.supersede.agent_run_receipts.authoritative_verification]
