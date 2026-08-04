# Design: Developer-loop profiles

## Context

Cairn already separates policy, pure decisions, command execution, and evidence. Probe declarations describe explicit commands. `probe run` is separate from default gates. Saved receipts let gates consume observations without hiding execution in the pure core.

A developer-loop profile should compose those existing pieces. It should tell an operator which small checks apply now, how results bind to current content, and which full gate remains outstanding.

The profile must not claim that an external command obeyed its declared network, mutation, or effect posture. Strong enforcement requires separate mechanism evidence.

## Decisions

### Decision: Author profiles in typed Nickel

**Choice:** Add a policy-owned developer-loop profile with:

- profile identity and purpose;
- repository and subject scope;
- ordered required check or probe IDs;
- exact target and toolchain identity requirements;
- named latency-budget identity and configured bound;
- declared network, mutation, and effect posture;
- cache policy and cache-key input roles;
- safe diagnostic policy;
- exact full-gate references; and
- required non-claims.

**Rationale:** These values are reviewable configuration with cross-field constraints. Runtime Rust consumes only the checked generated projection.

### Decision: Plan and verification are read-only

**Choice:** Add these commands:

```text
cairn developer-loop list --root PATH
cairn developer-loop plan PROFILE --root PATH
cairn developer-loop verify PROFILE --receipt FILE --root PATH
```

`list` and `plan` emit deterministic read-only receipts. `verify` validates one already-produced loop receipt. None of these commands execute the selected checks.

**Rationale:** Existing probe and repository shells already own explicit execution. The developer-loop surface should not create a second command runner.

### Decision: Reuse probe declarations

**Choice:** Required check IDs reference existing policy-known probe declarations or explicit repository check descriptors. Plans preserve exact argv identity, working-root label, expected status class, and safe artifact roles without embedding raw logs.

**Rationale:** Reuse prevents command drift and keeps execution evidence compatible with current probe receipts.

### Decision: Validate current subject and tool identities

**Choice:** A passing loop receipt binds the current repository subject identity, profile and policy identities, target, toolchain, selected checks, result statuses, and producer identity.

**Rationale:** A quick result is useful only for the exact content and tools that produced it.

Stale content, profile, policy, target, toolchain, or check identities fail closed.

### Decision: Budgets are named policy, not magic constants

**Choice:** Each profile references a named latency budget defined in Nickel. The receipt carries the producer-observed duration and measurement class. The pure core compares the supplied observation with the selected budget.

**Rationale:** Repositories have different workloads and machines. Reviewable named budgets are clearer than one hard-coded duration.

A passing comparison proves only that the supplied observation fits the configured budget. It does not prove clock accuracy, future speed, or performance on another machine.

### Decision: Cache evidence binds complete inputs

**Choice:** Cache status uses a closed class such as disabled, miss, or hit. A hit requires a BLAKE3 cache-key identity over the profile, subject, dependency set, target, toolchain, policy, and selected checks.

**Rationale:** A fast stale result is worse than a slower current result. Incomplete or mismatched cache-key material denies a cache-hit claim.

Cairn validates receipt identity only. It does not own cache storage or eviction.

### Decision: Effect posture remains a declaration

**Choice:** Profiles classify intended network, mutation, filesystem, process, and other effect posture. Receipts distinguish declared, externally enforced, observed, unavailable, and not-proven states.

**Rationale:** A local check label cannot prove sandbox or network behavior. The state vocabulary prevents a declared posture from becoming an enforcement claim.

Profiles can require separate enforcement evidence. When they do, missing or failing evidence blocks verification.

### Decision: Fast and full gates stay separate

**Choice:** Every profile names the full checks that remain required. A passing fast-loop receipt always reports release and full-gate status as outstanding unless separate current evidence proves them.

**Rationale:** Fast feedback improves iteration but has smaller scope. It must not weaken lifecycle or release evidence.

### Decision: Use existing safe diagnostic conventions

**Choice:** Loop findings contain stable codes, check IDs, safe repository-relative spans, expected and actual identity summaries, bounded remediation, and one exact next command.

Raw logs, prompts, credentials, environment values, absolute host paths, and unbounded output remain prohibited.

**Rationale:** Cairn already uses this diagnostic shape. A new generic diagnostic system would duplicate established behavior.

### Decision: Preserve functional core and imperative shell

**Choice:** Profile admission, plan derivation, cache-key material, budget comparison, receipt validation, and issue ordering remain pure over in-memory inputs.

Policy and receipt file reads, capability roots, stdout, stderr, and JSON presentation remain in `cairn-cli`.

**Rationale:** Developer speed must not weaken the verified-core boundary.

## Test Design

Positive tests cover sorted profile listing, deterministic planning, a current passing receipt, budget compliance, cache disabled, cache miss, a complete cache hit, and safe next-command diagnostics.

Negative tests cover unknown profiles, missing checks, duplicate checks, stale subjects, stale policy or tools, over-budget results, invalid duration classes, incomplete cache keys, unsafe diagnostic fields, absent required enforcement evidence, and release overclaims.

Cross-root fixtures prove portable plan and receipt hashes when display-only root paths differ.

## Risks and Trade-offs

- Profile maintenance can duplicate repository scripts. Exact probe references and freshness checks reduce drift.
- Timing observations vary. Receipts preserve measurement class and avoid reproducibility claims.
- A fast profile can grow into another full gate. Policy requires a bounded check set and explicit full-gate references.
- Users can mistake declarations for enforcement. Closed posture states and mandatory non-claims keep the difference visible.

## Claim Boundary

A passing developer-loop receipt proves profile conformance, current supplied identities, selected check statuses, budget comparison, cache-key shape, and diagnostic safety. It does not prove source correctness, complete test coverage, command hermeticity, network denial, mutation absence, clock accuracy, future performance, proof soundness, full-gate success, or release eligibility.
