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

## Maintained consumer command extension (2026-10-20)

Reuse producer commit `35e6bd4c4d6e9d5a3639a0e70ca922faf3b94a48` without rewriting its retained evidence. The integration baseline is `1cacafebbff7b2716a953961a1ebf52bafb352e7`; unrelated detached source-review work remains intact.

ChaosControl's maintained portable completion commands require four additional closed profile kinds: `cargo_workspace_test`, `cargo_clippy`, `cargo_fmt`, and `nix_flake_check`. Their exact argument vectors are respectively `test --workspace --all-targets --locked --no-fail-fast`, `clippy --workspace --all-targets --locked -- -D warnings`, `fmt --all -- --check`, and `flake check -L`. Do not inject offline, target, feature, or default-feature options. Empty features are required; target remains a bound applicability label. Existing `cargo_test` and native `nix_check` grammars and payload admission remain unchanged.

Workspace tests retain strict executed-test summary admission. Formatting, Clippy, and flake checks establish only a bounded, naturally exited process observation with complete streams and teardown; they retain null native build payload, false payload admission, and zero executed tests. Their closed process-check role must not be promoted to native build or test evidence. Independent manifest verification must enforce these distinctions and all existing applicability, executable, cohort, command, publication and prerequisite bindings.

The maintained flake command requires 10,800 seconds. Any extension of the existing 3,600-second maximum must be explicit and restricted to `nix_flake_check`; existing kinds, stream bounds and teardown bounds remain unchanged. KVM release matrix eligibility and all native readiness gates remain independent required consumer rails, not fabricated portable completion rows.

The integration owner's explicit decision retains the already-declared 10,800-second consumer bound through a separately named flake-kind-only maximum of 10,800,000 ms. All other kinds retain 3,600,000 ms. This does not increase the consumer release bound or the legacy producer bounds. The only additional control-bearing environment value is the exact reviewed `NIX_CONFIG` string `min-free = 0\nbuilders =\nsecret-key-files =\nmax-jobs = 1\n`, on the new flake kind only; it modifies the child environment, never host settings.


The historical traceability failure names two license requirements whose exact verification links already exist in baseline `tools/check-license-boundary.rs`; preserve those source-review changes. The retained Octet diagnostic identifies six noncanonical rustc source-map paths from relative includes/module paths. Repair only demonstrated source causes without suppressing findings, changing gate scope, dependencies or limits. Current historical receipts prove neither gate success on this baseline nor completed validation.

