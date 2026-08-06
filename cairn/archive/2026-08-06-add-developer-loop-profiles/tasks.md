# Tasks: Developer-loop profiles

## Phase 1: Policy and baseline

- [x] [serial] I1 Record baseline policy freshness, developer-experience tests, probe behavior, validation, release readiness, and lifecycle gate results. r[cairn.developer_loop_profiles.verification]
- [x] [serial] I2 Add typed Nickel contracts for developer-loop profiles, latency budgets, effect posture, cache policy, full-gate references, receipt fields, limits, and required non-claims. r[cairn.developer_loop_profiles.policy] r[cairn.developer_loop_profiles.boundary]
- [x] [depends:cairn.developer_loop_profiles.policy] I3 Export deterministic runtime profile data and reject duplicate IDs, unknown checks, invalid bounds, unsafe posture promotion, incomplete cache roles, and missing full-gate references. r[cairn.developer_loop_profiles.policy] r[cairn.developer_loop_profiles.verification]

## Phase 2: Pure core and read-only CLI

- [x] [depends:cairn.developer_loop_profiles.policy] I4 Add pure profile admission, sorted listing, deterministic plan derivation, named-budget comparison, cache-key validation, posture classification, receipt validation, and issue ordering in `cairn-core`. r[cairn.developer_loop_profiles.plan] r[cairn.developer_loop_profiles.receipt] r[cairn.developer_loop_profiles.cache]
- [x] [depends:cairn.developer_loop_profiles.plan] I5 Add read-only `developer-loop list`, `plan`, and `verify` CLI commands with capability-root reads and deterministic JSON or compact presentation. r[cairn.developer_loop_profiles.cli] r[cairn.developer_loop_profiles.boundary]
- [x] [depends:cairn.developer_loop_profiles.plan] I6 Reuse exact policy-known probe or repository-check descriptors and keep execution outside the developer-loop commands. r[cairn.developer_loop_profiles.plan] r[cairn.developer_loop_profiles.boundary]
- [x] [depends:cairn.developer_loop_profiles.receipt] I7 Emit actionable safe findings with stable codes, expected/actual identity summaries, bounded remediation, and one exact next command. r[cairn.developer_loop_profiles.diagnostics]

## Phase 3: Dogfood, verification, and docs

- [x] [depends:cairn.developer_loop_profiles.cli] I8 Add one Cairn-local developer-loop profile with a named repository-owned budget and exact full-gate references after the generic fixtures pass. r[cairn.developer_loop_profiles.dogfood]
- [x] [parallel] V1 Add positive policy, plan, receipt, budget, cache-disabled, cache-miss, complete-cache-hit, cross-root, and safe-diagnostic fixtures. r[cairn.developer_loop_profiles.verification]
- [x] [parallel] V2 Add negative unknown-profile, duplicate-check, missing-check, stale-subject, stale-policy/tool, over-budget, invalid-measurement, incomplete-cache, unsafe-diagnostic, missing-enforcement, and release-overclaim fixtures. r[cairn.developer_loop_profiles.verification] r[cairn.developer_loop_profiles.boundary]
- [x] [depends:cairn.developer_loop_profiles.dogfood] V3 Run the Cairn-local profile through the repository-owned execution path, save its loop receipt, and verify that full-gate status remains separate. r[cairn.developer_loop_profiles.dogfood] r[cairn.developer_loop_profiles.full_gate]
- [x] [serial] I9 Document profile authoring, execution separation, cache identity, named budgets, Roc design inspiration, exact next actions, and non-claims. r[cairn.developer_loop_profiles.docs] r[cairn.developer_loop_profiles.boundary]
- [x] [depends:cairn.developer_loop_profiles.verification] V4 Run policy freshness, formatting, focused core/CLI tests, Clippy, core purity smoke, traceability, Cairn validation, proposal/design/tasks gates, release readiness, and relevant Nix checks. r[cairn.developer_loop_profiles.verification]

## Verification Coverage

- `Scenario: Valid profile exports deterministically` -> I2, I3, V1
- `Scenario: Invalid profile fails policy admission` -> I3, V2
- `Scenario: Plan is deterministic and read-only` -> I4, I5, V1
- `Scenario: Developer-loop commands do not execute checks` -> I5, I6, V2
- `Scenario: Current receipt meets its named budget` -> I4, V1
- `Scenario: Stale or over-budget receipt fails` -> I4, V2
- `Scenario: Complete cache hit is admitted` -> I4, V1
- `Scenario: Incomplete cache hit is rejected` -> I4, V2
- `Scenario: Fast-loop pass leaves full gate outstanding` -> I8, V3
- `Scenario: Safe finding gives one exact next command` -> I7, V1
- `Scenario: Unsafe or overclaiming finding is rejected` -> I7, V2
- `Scenario: Cairn dogfoods a current profile` -> I8, V3
- `Scenario: Documentation preserves the claim boundary` -> I9, V4
