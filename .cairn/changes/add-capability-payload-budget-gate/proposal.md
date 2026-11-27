# Why

Two independent investigations — our 40-day quantitative agent audit in `onix-insights-mneme/18-opus-fable-engine-local-lm-guardrails-audit-2026-11-26.adoc` (`$28.7k` Opus guardrail churn, `35.8%` single-case micro-refusal commits in `memory-ir`, `53.3%` premature `NO-GO` kills in `rsi-lab`, receipt bureaucracy in `aspen`) and the external architectural review in `onix-insights-mneme/19-external-review-onix-architecture-and-blind-spots-2026-11-26.adoc` ("Specification Inflation / AI-Era Bureaucracy Trap", citing 5-repo cross-contract ceremony around a 2-`i64` workload) — converge on the same failure mode:

When autonomous coding swarms make Rust boilerplate, Nickel policies, JSON receipt schemas, and cross-repo contracts nearly free to author, the ratio of meta-plumbing to actual downstream capability explodes without any mechanical brake.

# What Changes

- Add an **opt-in `capability_payload_budget` policy contract** in `crates/cairn-cli/src/{policy.rs, cross_repo_dependencies.rs, readiness.rs}` that activates *only* when a change declares cross-repo dependencies (`>= 2` repositories) or adds new verification/receipt/guardrail plumbing:
  - Requires the proposal to name a concrete **downstream capability payload** (a non-toy end-to-end workload, real user-visible capability, or measured defect/failure mode it unblocks) and a bounded cross-repo touch count (`max_cross_repo_targets`).
  - Emits a deterministic diagnostic warning or fail-closed policy error (configurable per repo policy, default-off until enabled) when a change introduces cross-repo plumbing/receipts without naming a concrete capability payload or exceeds `max_cross_repo_targets` without an explicit rationale.
  - Leaves ordinary single-repo feature and bugfix Cairns completely untouched (zero added friction or ceremony).
- Add targeted unit and fixture tests covering compliant cross-repo changes, missing-payload ceremony rejection, single-repo zero-friction bypass, and explicit budget-override justification.
