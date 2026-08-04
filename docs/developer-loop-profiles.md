# Developer-loop profiles

Cairn can describe a fast, local verification loop as an inspectable policy value.

A developer-loop profile names the small checks that should run now, the target and toolchain they bind to, a named latency budget, a declared effect posture, a cache policy, and the full gate that remains outstanding.

## Inspect profiles

```bash
cairn developer-loop list
cairn developer-loop plan cairn-local
```

`list` shows every admitted profile. `plan` derives the ordered steps, named budget, declared posture, cache policy, full-gate references, and a deterministic `plan_hash`.

Both commands are read-only. They do not execute the selected checks.

## Verify a loop receipt

```bash
cairn developer-loop verify cairn-local \
  --receipt loop-receipt.json \
  --subject-hash <BLAKE3_SUBJECT> \
  --dependency-hash <BLAKE3_DEP> \
  --root .
```

`verify` checks that the externally produced receipt binds the current subject, policy, target, toolchain, profile identity, every required check, an observed duration inside the named budget, and a safe diagnostic and posture surface.

A passing receipt means loop profile conformance only. It does not satisfy release readiness, full repository checks, proofs, source correctness, or archive requirements.

## Named budgets

Each profile references one named budget defined in policy. The receipt carries the producer-observed duration and a measurement class such as `wall`.

A passing comparison proves only that the supplied observation fits the configured budget. It does not prove clock accuracy, future speed, or performance on another machine.

## Cache identity

A cache-hit claim must bind a BLAKE3 identity over the profile, subject, dependency set, target, toolchain, policy, and selected checks. Cairn validates that identity shape only; cache storage, eviction, and build correctness remain external.

## Effect posture is declared, not enforced

Profiles classify their declared effect posture. Receipts record a posture state such as `declared`, `externally_enforced`, `observed`, `unavailable`, or `not_proven`.

A declared read-only posture does not prove command hermeticity, network denial, mutation absence, or clock accuracy. A claim of enforced posture without enforcement evidence is rejected as an overclaim.

## Cairn dogfoods a local profile

Cairn ships one `cairn-local` profile that binds the `fmt`, `validate`, and `traceability` checks to a named budget and to the full repository gates.

The full-gate status of a passing loop receipt always stays `outstanding` unless separate current evidence satisfies the gate.

## Claim boundary

A passing developer-loop receipt proves profile conformance, current supplied identities, selected check statuses, budget comparison, cache-key shape, and diagnostic safety. It does not prove source correctness, complete test coverage, command hermeticity, network denial, mutation absence, clock accuracy, future performance, proof soundness, full-gate success, or release eligibility.

Roc supplied design inspiration for fast application feedback. Cairn does not copy Roc's compiler architecture or promise one universal latency target.

<!-- r[impl cairn.developer_loop_profiles.verification] -->
<!-- r[impl cairn.developer_loop_profiles.docs] -->
<!-- r[verify cairn.developer_loop_profiles.boundary] -->
