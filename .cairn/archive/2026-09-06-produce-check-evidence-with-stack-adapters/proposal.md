# Produce check evidence with stack adapters

## Why

Cairn can validate exact required-check coverage, but consumers must still construct producer results and bind execution artifacts themselves.
Custom receipt scripts increase maintenance cost and can confuse a successful process with a successful required check.
The stack already provides bounded execution, CI state, workflows, builds, Rust evidence, and canonical evidence links.

## What Changes

- Add a maintained producer contract and narrow normalization adapters for current Cairn completion and developer-loop inputs.
- Provide an explicit local producer for reviewed Cargo and Nix checks through Bounded Exec.
- Consume saved Kiln, Lattice, Mantle, and Octet observations through their pinned contracts and existing Valence evidence roles.
- Publish versioned producer fixtures and a disposable Kiln consumer recipe.
- Preserve failure, cancellation, skipped work, cache provenance, scope, teardown, and unknown outcomes without optimistic fallback.

The proposed surfaces are `cairn check-evidence plan`, `run`, and `import`.
Only the explicitly approved local `run` path starts checks. Gates and import remain read-only over saved evidence.

## Impact

Maintenance owner: Cairn producer-adapter maintainers.
Current consumers: Cairn developer-loop and completion checks, plus the pinned disposable Kiln consumer fixture.
Immediate outcome: consumers obtain correctly bound evidence without bespoke receipt construction.
Durable contribution: independently tested producer adapters and conformance fixtures for maintained stack contracts.

Kiln retains CI candidate and attempt meaning. Lattice retains workflow execution.
Mantle retains build meaning. Octet retains Rust analysis and check scope. Valence retains shared identity and role semantics.
Cairn retains selected-check policy, interpretation, completion, and release boundaries.
No changes to those producer schemas are assumed or implemented in this package.

## Repeatability evidence

Produce evidence from a known successful check and a known failed check in a fresh consumer fixture.
Retain nonzero, zero-with-failing-payload, skipped, timed-out, cancelled, truncated, stale, and substituted result cases.
Run the generated result through `agent-receipt complete` and developer-loop verification.
A new process wrapper does not count as adoption until these existing consumers accept the exact successful result and reject every failure control.

## Out of Scope

- A Cairn CI scheduler, daemon, retry controller, worker placement system, or secret broker
- Arbitrary shell text or executable discovery through ambient PATH
- Executing checks during normal lifecycle gates
- Treating a build, cached artifact, or opaque producer label as a successful test
- Replacing the separate candidate-assurance or compiler-evidence planning packages
- Claims of sandboxing, source correctness, or release readiness from a process receipt alone
