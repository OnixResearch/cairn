## Why

Cairn provides deterministic lifecycle gates, explicit probes, onboarding templates, focused checks, and complete release evidence. Repositories still describe their quickest local check paths through unrelated scripts and README sections.

There is no portable policy contract that names a developer loop, its selected checks, subject and tool identities, declared effect posture, cache identity, repository-owned latency budget, and relationship to the full gate.

Roc treats fast application feedback as a primary product requirement. Cairn can adapt that goal as an inspectable lifecycle profile without copying Roc's compiler architecture or promising one universal latency target.

## What Changes

- Add typed Nickel developer-loop profiles with exact check, subject, toolchain, target, policy, budget, cache, effect-posture, and full-gate references.
- Add read-only `cairn developer-loop list`, `plan`, and `verify` commands.
- Reuse existing probe declarations and saved receipts. Default lifecycle gates still do not execute commands.
- Validate externally produced loop receipts for exact profile identity, current subject, selected checks, observed duration, cache status, safe diagnostics, and bounded non-claims.
- Keep a fast-loop pass separate from release readiness, proof, source correctness, and full-gate success.
- Dogfood one Cairn-local profile after the generic contract and fixtures pass.
- Add positive and negative fixtures for current, stale, incomplete, over-budget, unsafe, and overclaiming receipts.

## Impact

- **Policy**: typed Nickel profile and receipt vocabularies with deterministic generated projection.
- **Core**: pure profile parsing, plan derivation, receipt validation, cache-key checks, budget evaluation, and diagnostics.
- **CLI**: bounded profile discovery, receipt loading, and JSON or compact text presentation.
- **Developer experience**: one inspectable fast-loop contract with an exact path to the full gate.
- **Consumers**: repositories can adopt profiles later without changing Cairn release-readiness semantics.

## Out of Scope

- Executing arbitrary commands from default gates.
- Building a compiler, build scheduler, cache service, sandbox, network monitor, or command supervisor.
- Defining one universal duration budget for all repositories or machines.
- Treating declared no-network or read-only posture as enforced security evidence.
- Letting fast-loop success satisfy release readiness or replace repository-owned full checks.
