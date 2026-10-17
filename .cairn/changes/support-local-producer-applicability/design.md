# Design

## Context

The existing producer binds completion selection, source projection, policy, executable bytes, tool cohort, explicit environment, and execution bounds.
Its application-port applicability already includes a canonical dependency identity set, but the shell rejects nonempty sets instead of observing them.
The Nix profile runs `nix eval`; the shell and final-manifest consumer intentionally withhold build success.

## Decisions

### Decision: Observe dependency files, not supplied digest assertions

Use an explicit sorted, unique repository-relative dependency-file projection.
Each dependency identity is the canonical BLAKE3 identity of its path and observed byte hash under a versioned dependency domain.
Use the existing capability-relative no-follow reader and existing per-file and aggregate source bounds, with at most 64 dependency files.
Expose a read-only observation command; require the projection for nonempty applicability and require exact observed identity equality before execution, between checks, after each child, before publication, and during verification.
Empty dependency sets remain valid only when observation agrees; supplying a projection never exempts a declared identity.
The consumer owns dependency-closure review. File identity is not package-resolution or remote-source attestation.

### Decision: Admit a native Nix build result, not an evaluator value

Cut over the fixed Nix grammar to `nix build --offline --no-link --json --file ./check.nix`.
The consumer owns the expression and includes it and its build inputs in the reviewed source projection.
Use the native JSON array emitted by Nix after realizing derivation outputs; reject empty, malformed, truncated, opaque-store-path-only, duplicate, unknown-shape, or incomplete rows.
Retain admitted derivation/output identities and supported native timing fields in the facts artifact.
Require successful bounded process disposition, admitted payload, complete teardown, unchanged applicability, and durable publication independently.
Nix remains build-role evidence with zero executed-test claims; a realization can reuse the Nix store and does not attest fresh builder execution or oracle truth.
The read-only manifest consumer must independently re-admit the retained native payload.
Keep all required check commands and prerequisite edges in the completion owner's contract; changed commands require a newly reviewed plan.

## Verification

Establish the unchanged focused producer baseline before implementation.
Retain positive and negative dependency observations, including drift before execution and after execution, unavailable files, invalid projections, and no-follow failures.
Use a real Nix build and failing build with disposable inputs; an evaluated integer or caller-authored success boolean must never pass.
Replay successful producer rows and failure controls through the original completion and manifest consumers.
Run the existing workspace tests, denied-warning Clippy, formatting, policy freshness, lifecycle checks, and Nix checks without suppressing failures.
Record commands, source/cohort identities, verdict fields, and remaining blockers before lifecycle completion.

## Risks / Trade-offs

Observation cannot prevent a same-authority adversary from modifying files between observations.
Dependency projection completeness and check meaning remain consumer-owned review obligations.
The admitted Nix JSON surface is deliberately bounded and pinned to the maintained native shape; unsupported shape changes fail closed.
Sandboxed conformance tests need an isolated writable test store; they must not modify host Nix configuration.
No changes to saved producer adapters, unrelated lifecycle work, host configuration, release acceptance, or historical evidence are authorized.
