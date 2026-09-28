# Agent efficiency measurement design

## Context

Existing Cairn replay checks establish deterministic receipt behavior. They do not measure an agent's complete context or establish task correctness.
The readiness study in `docs/change-readiness.md` counts diagnostic calls only.
This change supplies the common measurement boundary for the other packages in `docs/agent-efficiency-roadmap.md`.

## Decisions

### Decision: Import observations, not model execution

Add `cairn agent-benchmark validate --input FILE` and `cairn agent-benchmark compare --input FILE` as bounded offline commands.
Typed Nickel contracts define scenario manifests, run observations, paired requests, and result schemas.
The shell reads explicit files and emits JSON. The core admits values, classifies supplied oracle observations, and aggregates checked counters.
Agent hosts retain credentials, model calls, clocks, transcript capture, cancellation, and live-run authority.
No new runtime dependency on an agent framework is required.

### Decision: Bind measurement provenance

Each run binds the product revision, selected source projection, policy, fixture, oracle, agent configuration, model, tool catalog, and host-presentation identities.
The scenario manifest names the allowed candidate difference and the run budget.
Accounting uses either explicit provider usage or an immutable tokenizer implementation and exact exposed content.
Provider reasoning counts declare whether output totals already include them. The core never sums inclusive counters twice.
Cached input remains distinct from uncached input. Duplicate call identities fail admission.
Missing token observations remain unavailable. Byte counts never become token estimates.

### Decision: Keep failures in the cost

Repository-owned oracles distinguish completed, correctly blocked, falsely completed, failed, and incomplete runs.
The report shows outcome counts and token totals for all attempts, not only successful attempts.
Tokens per correctly completed change use all declared attempted-run costs and a nonzero correct-completion denominator.
Correct-blocked efficiency has a separate denominator and label.
An absent denominator, incomplete accounting, incomparable cohort, or correctness regression withholds an improvement verdict.

### Decision: Reuse owned mechanisms

Reuse the existing test-harness scenario classes, completion validator fixtures, canonical BLAKE3 identities, and deterministic comparison patterns.
Add scenario-specific outcome assertions where lifecycle conformance alone cannot establish the intended result.
Keep benchmark data separate from lifecycle authority and existing receipt schemas.
Review a published tokenizer component before selecting one. Record its immutable revision and exact encoding when tokenizer measurements are used.

## Bounds and failure behavior

Nickel owns named limits for runs, calls, input bytes, text fields, counters, and repetition budgets.
Every aggregate uses checked arithmetic. Invalid schemas, unknown outcomes, impossible usage counts, and truncated sequences fail closed.
Bounded no-follow readers reject unsafe paths. Public summaries contain no raw prompts, credentials, environment values, or host paths.
A redacted summary proves declared accounting and linkage only. It does not prove transcript truth or provider honesty.

## Alternatives rejected

- Character-to-token ratios cannot establish exact token cost.
- Success-only averages discard expensive failures and reward false completion.
- A new model runner duplicates agent-host authority and increases scope.
- One serialized payload benchmark cannot establish end-to-end usability.

## Tests and evidence

Baseline existing readiness, MCP, and harness fixtures before implementation.
Add positive and negative core tests for counting conventions, duplicate calls, overflow, missing usage, and cohort identity drift.
Add CLI controls for unsafe files, oversized inputs, malformed JSON, and omitted observation records.
Retain successful and blocked fresh-root scenarios plus false-success and interrupted-run controls.
Capture deterministic representation replays separately from explicitly authorized live-host paired runs.
Run the repository checks named in the roadmap and retain source-bound reports. No current token improvement is claimed.

## Risks and claim boundary

A dishonest producer can fabricate usage or oracle observations. Admission cannot prove observation truth.
A narrow fixture can miss useful behavior. Reports must name their scenario scope and unresolved coverage.
Cairn verification maintainers own fixture and accounting review. Host owners retain live measurement quality.
