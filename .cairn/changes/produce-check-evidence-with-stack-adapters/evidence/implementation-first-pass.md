# Producer implementation first pass

## Scope and baseline

The dedicated worktree started clean at `f23a5b0b8581387962e2726c36763dde494094d1`.
The baseline preceded core edits.
Cairn producer-adapter maintainers own the new core and CLI planning shell.
The immediate outcome is deterministic producer admission and fail-closed normalization for the existing completion consumer.
The durable contribution is a typed Nickel contract, fixed command grammars, and direct positive/negative consumer tests.
This is a tested partial implementation, not completion of the change.

Baseline command prefix:

```console
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop --option min-free 0 -c
```

| Command after prefix | Result |
|---|---|
| `cargo test -p cairn-cli --test agent_completion --test developer_loop` | PASS: 5 completion tests and 2 loop tests |
| `cargo test -p cairn-gate-contract` | PASS: 9 unit tests and 1 producer-fixture test |

The completion baseline includes current evidence, stale-subject rejection, legacy non-authority, and CLI/MCP parity.
The loop baseline includes a current receipt and stale-subject rejection.
The gate baseline includes passed/blocked results and identity, target, policy, schema, and non-claim substitution controls.

## Published source inspection

All revisions below match `docs/cairn-utility-program.md`.
Research checkouts remain under the assigned worktree's ignored `.pi/` directory.
They are not product dependencies, copied implementations, or fixture provenance claims.
No producer repository was modified.

| Owner | Acquired source and revision | Exact inspected contract | License |
|---|---|---|---|
| Bounded Exec | `https://git.onix.computer/z2CpqLFpdP36fZXYUK5ZNWxMibpCo.git`, `29dac88ecded94457572db3fdfaaaab95fa91525` | `crates/bounded-exec/src/lib.rs`: `RunRequest`, `CommandSpec`, `ExecutionOutput`, `EnvironmentMode`, `TerminationScope` | AGPL-3.0-or-later |
| Kiln | `rad://z2wsvXm5S2sJGvuV1k5JHiwi1PbKE`, `5d99f4e9bb954743443e340baece31256b4c53f1` | `crates/kiln-core/src/workflow.rs`: `WorkflowEffectPlan`, `WorkflowObservation`, `WorkflowNodeState`; `docs/cairn-gate.md` | MIT OR Apache-2.0 |
| Lattice | `git@github.com:OnixResearch/lattice.git`, `70496e67c7fd4a8b05914161a8e09de2759bebc8` | `lattice-workflow-contract`: request v2, observation v1, `validate_observation(profile, request, accepted, observation)` | MIT |
| Mantle | `git@github.com:OnixResearch/mantle.git`, `0bbab77e23ab95caa4e4184b9945cfaa5c8040e2` | `mantle-build-contract`: request/observation v1, `validate_observation(request, expected_builder, observation)` | AGPL-3.0-or-later |
| Octet | `git@github.com:OnixResearch/octet.git`, `890f424cd7b1b2015f39c0eb5747f1bd1e576028` | `standards/receipts/contracts.ncl`: `octet-acceptance-receipt/v1`, `octet-provenance-record/v1`; root workspace manifest | AGPL-3.0-or-later |
| Valence | `git@github.com:OnixResearch/valence.git`, `e40c76b4d2070a29636e00c85c0dff93f03dba2f` | `valence-core/src/stack_provenance_adapter.rs`: `StackProvenanceSidecar`, `validate_stack_provenance_sidecar`; `stack_role_registry.rs` | AGPL-3.0-or-later |

Unauthenticated GitHub raw URLs returned 404. SSH acquisition succeeded at every listed GitHub revision.
The attempted Kiln HTTPS mirror returned repository-not-found. Native Radicle acquisition succeeded at the exact requested revision.
These transport failures do not imply missing source or missing contracts.

OnixResearch owner-authorized reuse is the basis for future adoption of exclusively owned source.
No source implementation or upstream fixture was imported in this increment.
Future admission must preserve the listed public licenses and all third-party or contributor notices.
The owner grant changes neither public licenses nor evidence semantics.

## Contract findings and integration blockers

1. Bounded Exec supplies the required library mechanism, not a standalone receipt CLI.
   Its request exposes clear/inherited environment modes, null/byte stdin, limits, and process-group scope.
   Its output preserves completion, exit, signal, captured streams, and disposition.
   The worker cannot add matching Cargo/Nix dependencies because the assignment forbids pin and lock changes.
   This is a consumer integration blocker, not a missing upstream API.
2. Kiln owns candidate, pipeline, attempt, effect, provider, and CI outcome identities.
   `WorkflowNodeState::is_successful` includes `Skipped`.
   A consumer must retain that state and refuse to interpret it as executed required-test coverage.
   The existing gate adapter consumes Cairn revision `15f00875562025e7ea7e0d1f4af24d1a2e2ac06f`, not this new producer.
3. Lattice observation admission requires the profile, request, and accepted execution record, not an observation JSON object alone.
   Its node success convention also includes `Skipped`.
   The observation contract contains no executed-test count or Cargo payload verdict.
   A completed workflow therefore cannot alone establish the proposed nonempty-test obligation.
   Cairn needs a separately admitted node receipt with a reviewed test-coverage contract before it can create passing test rows.
4. Mantle preserves `Success`, `Failed`, `Cancelled`, `TimedOut`, and `Unknown`.
   Cache kinds are `None`, `Local`, and `Substitution`, with explicit source identity rules.
   Admission binds the request, expected builder, required products, receipt, non-claims, and observation identity.
   It contains no test-coverage contract. A successful build cannot satisfy a test obligation.
5. Octet's inspected acceptance contract preserves status, stages, warnings, errors, commands, evidence, and provenance scope.
   Shape admission alone does not establish full-catalog deny-all scope across workspace, targets, and compatible features.
   No strict-check adapter was admitted. The coordinator owns the separate Octet repair.
6. Valence sidecar and role validators establish identity/linkage under their own non-claims, not check execution.
   No new direct Valence dependency was added. Existing Cairn evidence paths remain unchanged.
7. Durable File Publication is not present in this worktree's Cargo dependency graph.
   Actual artifact publication needs matching dependency admission, capability-relative output authority, interrupted-state handling, and recovery tests.

The worktree retains its historical Slotcar/Tiger Style Nix names.
Canonical Octet integration, Nix pins, locks, lint defaults, and gate scope are unchanged.

## Implemented and withheld behavior

The pure core admits exact required selection, derives stable dependency order, binds profiles and applicability, and normalizes supplied observations.
The core returns completion rows without changing the existing completion validator.
It checks explicit execution intent, reviewed plan identity, executable recheck facts, and prior prerequisite results.
It requests final-manifest publication only for complete current passing results and exact durable readback facts.
These are decisions over supplied facts, not real process or filesystem observations.

The CLI implements bounded, capability-relative planning with current policy validation.
It reports that source observation, execution, and publication did not occur.
Run and import fail explicitly without effects. No generic producer decoder or local process fallback exists.

## Verification of this increment

All Cargo commands used the baseline prefix and the `cairn-rest-checks-1295` Pueue group.
Persistent logs are under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.

| Check | Verdict and scope | Log |
|---|---|---|
| `cargo test -p cairn-core` | PASS: 574 unit tests, including 13 producer tests, plus 12 doc tests | `evidence-core-verified.log` |
| `cargo test -p cairn-cli --test check_evidence --test agent_completion --test developer_loop` | PASS: 2 producer CLI tests, 5 completion tests, 2 loop tests | `evidence-cli-verified.log` |
| `cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings` | PASS | `evidence-clippy-last.log` |
| `cargo fmt --all -- --check` | PASS | `evidence-format-last.log` |
| `cargo run -q -p cairn-cli --bin cairn -- policy export --check` | PASS: generated policy is fresh | `evidence-policy-verified.log` |
| `cargo run -q -p cairn-cli --bin cairn -- gate tasks produce-check-evidence-with-stack-adapters --root .` | PASS: structural task gate, next ready task `core` | `evidence-task-gate.json` |
| Nickel profile export and byte comparison | PASS: re-export matches `fixtures/check-evidence/profiles.json` | Pueue task 1822 |
| Nickel negative bounds fixture | Expected rejection: zero `deadline_ms` breaks `C.bounds` | `evidence-nickel-negative.log` |
| Direct core effect scan | No filesystem, environment, process, clock, async, unsafe, or print matches in the new module | Source inspection |

The Rust tests cover denial, empty tests, failed payloads, skipped/native failure statuses, timeout, cancellation, overflow, stale identities, substitutions, and teardown failure.
They also cover prerequisite denial, duplicate prior results, retry refusal, build/test separation, and partial or substituted artifact readback.
These cases use supplied observations. They do not claim that a real child timed out or that descendant cleanup occurred.
The CLI tests cover unchanged deterministic plans, current policy, malformed-input redaction, strict options, and unavailable execution/import without fallback.
The task gate proves document conformance only. Eight tasks remain unchecked.

Initial development checks caught a shell `Path` type mismatch, incorrect display-path conversion, and Clippy findings in new tests and argument parsing.
Those findings were corrected without suppressions. The passing checks above followed those corrections.
Full workspace tests, the complete Nix check set, canonical strict Octet, and live producer conformance remain unclaimed.

## Remaining work

- Admit matching published Cargo/Nix dependencies and upstream conformance fixtures.
- Implement the Bounded Exec shell with exact executable rechecks, authority, source projection, cancellation, and output parsing.
- Select an actual reviewed Cargo output/coverage grammar. The synthetic profile is not a live executable cohort.
- Add upstream-native decoders that call the producer-owned admission functions and retain original schemas and identities.
- Implement authorized durable publication and interruption recovery through the admitted one-file primitive.
- Exercise real denied, flooded, timed-out, cancelled, descendant-cleanup, teardown-failure, and publication-interruption controls.
- Produce a fresh Cairn run and a pinned disposable Kiln run without hand-authored observations.
- Replay generated live evidence through both completion and developer-loop validators.
- Run full repository, canonical strict Octet, lifecycle, and conformance gates before completion.

No sync, archive, push, or worktree removal occurred.
