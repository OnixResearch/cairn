# Developer Loop Profiles Specification

## Purpose

Defines the `developer-loop-profiles` capability.

## Requirements

### Requirement: Developer-loop profiles are policy-owned

r[cairn.developer_loop_profiles.policy] Cairn MUST define typed Nickel developer-loop profiles that bind profile identity, repository and subject scope, ordered required check identities, target and toolchain requirements, a named latency budget, declared effect posture, cache policy, safe diagnostics, exact full-gate references, limits, and explicit non-claims.

#### Scenario: Valid profile exports deterministically

r[cairn.developer_loop_profiles.policy.valid]
- GIVEN a complete developer-loop profile satisfies the Nickel contract
- WHEN Cairn exports runtime policy
- THEN the generated profile and BLAKE3 profile identity MUST be deterministic
- AND runtime Rust MUST consume the checked generated projection without evaluating Nickel.

#### Scenario: Invalid profile fails policy admission

r[cairn.developer_loop_profiles.policy.invalid]
- GIVEN a profile has a duplicate ID, duplicate check, unknown check, invalid budget, unsafe posture promotion, incomplete cache policy, missing full-gate reference, or missing non-claim
- WHEN policy validation runs
- THEN export MUST fail with a deterministic policy issue.

### Requirement: Developer-loop plans are deterministic and read-only

r[cairn.developer_loop_profiles.plan] Cairn MUST derive a deterministic read-only developer-loop plan from one admitted profile, current subject facts, exact probe or check descriptors, generated policy, target, and toolchain identities.

#### Scenario: Plan is deterministic and read-only

r[cairn.developer_loop_profiles.plan.read_only]
- GIVEN identical profile, subject, policy, target, toolchain, and check inputs
- WHEN plan derivation runs under different checkout roots
- THEN portable plan fields and hashes MUST match
- AND no check, process, network request, clock read, cache mutation, or repository mutation MUST occur.

### Requirement: Developer-loop CLI exposes inspection and verification

r[cairn.developer_loop_profiles.cli] Cairn MUST expose read-only `developer-loop list`, `developer-loop plan PROFILE`, and `developer-loop verify PROFILE --receipt FILE` commands with deterministic receipts and stable issue codes.

#### Scenario: Operator inspects one profile

r[cairn.developer_loop_profiles.cli.inspect]
- GIVEN a supported profile and repository root
- WHEN the operator lists or plans developer-loop profiles
- THEN Cairn MUST show the selected checks, named budget, declared effect posture, cache policy, full-gate references, and non-claims.

#### Scenario: Developer-loop commands do not execute checks

r[cairn.developer_loop_profiles.cli.no_execution]
- GIVEN a profile references executable probe or repository-check descriptors
- WHEN the operator runs list, plan, or verify
- THEN Cairn MUST NOT execute those descriptors
- AND explicit probe or repository shells MUST remain the only execution owners.

### Requirement: Developer-loop receipts bind current evidence

r[cairn.developer_loop_profiles.receipt] Cairn MUST validate externally produced developer-loop receipts against the current subject, profile, policy, target, toolchain, selected checks, producer, result statuses, named budget, cache status, diagnostics, and non-claims.

#### Scenario: Current receipt meets its named budget

r[cairn.developer_loop_profiles.receipt.current]
- GIVEN a receipt binds the current identities, includes every required passing check, and reports an observed duration within the selected named budget
- WHEN receipt verification runs
- THEN Cairn MAY report developer-loop profile conformance
- AND it MUST preserve the receipt's measurement class and bounded timing non-claims.

#### Scenario: Stale or over-budget receipt fails

r[cairn.developer_loop_profiles.receipt.stale]
- GIVEN a receipt has a stale subject, profile, policy, target, toolchain, check identity, non-passing required result, or observed duration beyond the selected budget
- WHEN receipt verification runs
- THEN Cairn MUST fail with stable expected/actual diagnostics
- AND it MUST identify the exact next check or regeneration command.

### Requirement: Cache-hit claims bind complete inputs

r[cairn.developer_loop_profiles.cache] A developer-loop cache-hit claim MUST bind a BLAKE3 cache-key identity over the profile, subject, dependency set, target, toolchain, policy, and selected checks. Cairn MUST treat cache storage, eviction, and build correctness as external responsibilities.

#### Scenario: Complete cache hit is admitted

r[cairn.developer_loop_profiles.cache.complete]
- GIVEN a receipt reports a cache hit with every required cache-key input and matching BLAKE3 identity
- WHEN cache evidence validation runs
- THEN the hit MAY contribute to developer-loop conformance with explicit external-cache non-claims.

#### Scenario: Incomplete cache hit is rejected

r[cairn.developer_loop_profiles.cache.incomplete]
- GIVEN a cache-hit receipt omits or mismatches any profile, subject, dependency, target, toolchain, policy, or check identity
- WHEN cache evidence validation runs
- THEN Cairn MUST reject the cache-hit claim as stale or incomplete.

### Requirement: Fast-loop and full-gate status remain separate

r[cairn.developer_loop_profiles.full_gate] A passing developer-loop receipt MUST NOT satisfy release readiness, full repository checks, proof obligations, source correctness, or lifecycle archive requirements unless separate current evidence satisfies those gates.

#### Scenario: Fast-loop pass leaves full gate outstanding

r[cairn.developer_loop_profiles.full_gate.separate]
- GIVEN every developer-loop check passes
- AND no current full-gate or release-readiness evidence is supplied
- WHEN Cairn reports loop status
- THEN developer-loop conformance MAY pass
- AND full-gate and release status MUST remain explicitly outstanding.

### Requirement: Developer-loop diagnostics are actionable and safe

r[cairn.developer_loop_profiles.diagnostics] Developer-loop findings MUST include stable codes, check identities, safe repository-relative spans when known, expected and actual identity summaries, bounded remediation, and one exact next command. Findings MUST exclude raw logs, prompts, credentials, environment values, absolute host paths, and unbounded output.

#### Scenario: Safe finding gives one exact next command

r[cairn.developer_loop_profiles.diagnostics.safe]
- GIVEN receipt verification finds a stale check identity
- WHEN Cairn renders the finding
- THEN it MUST identify the stale field and provide one exact safe recheck command.

#### Scenario: Unsafe diagnostic content is supplied

r[cairn.developer_loop_profiles.diagnostics.unsafe]
- GIVEN a receipt includes credentials, raw prompts, raw logs, environment values, absolute private paths, or unbounded output
- WHEN receipt verification runs
- THEN Cairn MUST reject the unsafe diagnostic content.

### Requirement: Developer-loop claims remain bounded

r[cairn.developer_loop_profiles.boundary] Developer-loop profiles, plans, receipts, and documentation MUST distinguish declared, externally enforced, observed, unavailable, and not-proven effect posture and MUST NOT claim command hermeticity, network denial, mutation absence, clock accuracy, future speed, source correctness, proof soundness, full-gate success, or release eligibility.

#### Scenario: Declared posture is promoted to enforcement

r[cairn.developer_loop_profiles.boundary.overclaim]
- GIVEN a receipt has only a declared no-network or read-only posture
- WHEN it claims enforced sandbox or mutation evidence
- THEN Cairn MUST reject the receipt with a stable posture-overclaim issue.

### Requirement: Cairn dogfoods one developer-loop profile

r[cairn.developer_loop_profiles.dogfood] Cairn MUST maintain one repository-local developer-loop profile with a named budget, exact focused checks, current identity inputs, and explicit full-gate references after the generic contract is verified.

#### Scenario: Cairn dogfoods a current profile

r[cairn.developer_loop_profiles.dogfood.current]
- GIVEN the Cairn-local profile and current repository subject
- WHEN the repository-owned execution path produces a loop receipt and Cairn verifies it
- THEN the receipt MUST identify every selected focused check and budget result
- AND it MUST keep full workspace and release gates separate.

### Requirement: Developer-loop profiles have complete verification and documentation

r[cairn.developer_loop_profiles.verification] The change MUST include positive and negative policy, plan, receipt, cache, budget, posture, diagnostic, cross-root, and overclaim fixtures plus focused and repository validation.

#### Scenario: Verification closes the change

r[cairn.developer_loop_profiles.docs]
- GIVEN maintainers intend to sync and archive developer-loop profiles
- WHEN focused tests and lifecycle gates run
- THEN policy freshness, pure-core tests, CLI tests, positive fixtures, negative fixtures, dogfood evidence, documentation, traceability, Cairn validation, and proposal/design/tasks gates MUST pass
- AND documentation MUST state that Roc supplied design inspiration rather than compiler or runtime authority.
