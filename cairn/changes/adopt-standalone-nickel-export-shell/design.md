## Context

Cairn exports Nickel-authored lifecycle policy and lifecycle data, then applies Rust-owned parsing, consistency checks, gates, receipt logic, and release-readiness policy. The standalone `nickel-export` CLI can replace duplicate external-command, exact-identity, diagnostic, and freshness mechanics without becoming the lifecycle authority.

## Decisions

### 1. Cargo/Nix release inputs select one immutable source

Cairn will pin `github.com/OnixResearch/nickel-export` revision `257fafc1c746f1faf156207043a4c826bfb16d49` and the declared Nickel evaluator cohort. Local path overrides remain diagnostic-only and cannot replace the lock or release identity.

### 2. A Cairn-owned wrapper defines invocation authority

The wrapper selects the repository root, evaluator binary, request, dependencies, contract/selector, and generated destination. The standalone shell performs bounded external evaluation, exact identity, structured diagnostics, and check-mode comparison. Cairn code continues to parse and authorize the resulting policy/lifecycle data.

### 3. Lifecycle semantics never move into the shared layer

Policy contracts, defaulting, gate interpretation, task/spec semantics, generated policy destinations, receipt acceptance, replay, and release readiness remain Cairn-owned. Standalone success is necessary only where configured; it is never sufficient for a gate or release decision.

### 4. Dual-run compares current observable behavior

Legacy and standalone commands run with identical source bytes, declared dependency closure, evaluator version, selectors/contracts, formats, and destination expectations. Comparison covers policy export bytes, lifecycle-data bytes, negative Nickel diagnostics, and exact generated-file freshness.

### 5. Rollback is explicit and fail-closed

Current Cairn export commands remain authoritative for at least one complete validation cycle. Any unexplained drift blocks cutover, preserves both outputs and identities as diagnostics, and requires positive and negative regression fixtures before retrying. Rollback cannot authorize stale generated policy or a local path override.

### 6. Pure and imperative responsibilities remain separated

The standalone core owns pure normalization, BLAKE3 identity, admission, receipt/manifest construction, and freshness over explicit bytes. Cairn's shell owns file/process I/O and generated destinations; Cairn's pure cores continue to own lifecycle-policy and gate decisions over parsed values.

## Risks / Trade-offs

- Evaluator-version changes can alter diagnostics or serialization and therefore require an explicit cohort update.
- Dual-run temporarily increases policy/check latency.
- Exact-byte equality does not prove lifecycle-policy correctness or gate soundness.
- The shared shell's path checks do not replace Cairn's destination and repository-root policy.

## Non-Goals

- Moving lifecycle parsing, gates, replay, receipts, or release readiness into `nickel-export`.
- Weakening policy freshness or accepting generated fallback content.
- Evaluating Nickel in ordinary pure lifecycle cores.
- Claiming evaluator equivalence, gate soundness, or release eligibility from export parity.
