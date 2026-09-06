# Evidence producer continuation 2384

## Scope and commits

This pass used only the assigned evidence worktree. It preserved the existing commits and finished the inherited source edits as tested increments.
Cairn producer-adapter maintainers own the capability. The immediate outcome is a tested Cargo producer, conservative Nix observations, and saved Mantle admission.
The durable contribution is repeatable consumer fixtures, bounded diagnostics, and preservation of observations after failed source rechecks or publication.
This report does not close the change. All eight incomplete tasks remain unchecked.

| Commit | Subject |
|---|---|
| `ce89af405834d19652f08d60d9ce0475d47410e7` | Reject foreign loop receipts without panicking |
| `4f85bd989a9f1a6deac1e1fb4364b8c252164dfe` | Preserve bounded check evidence across failed attempts |
| `ffd28201ac80bff8a5fea271d4aa6e61d23ea5bc` | Withhold Nix completion without admitted payloads |

The third commit supersedes the second commit's optimistic local Nix payload classification.
A successful Nix process now remains non-passing for completion because no admitted native build-payload decoder exists.
The native process outcome remains visible. No build result becomes executed-test coverage.

The inherited Cargo/Nix admissions use Bounded Exec `29dac88ecded94457572db3fdfaaaab95fa91525`, Durable File Publication `951c27f59003cea9bfdb40ed4d89653d50fada1f`, and Mantle `0bbab77e23ab95caa4e4184b9945cfaa5c8040e2`.
This pass added no further dependencies and changed no existing producer or compiler pin.
Cargo and Nix lockfiles were already generated in the inherited edits. This pass did not edit them manually.
Producer source, schemas, licenses, and non-claims remain unchanged. No source implementation was copied.

## Baseline and repairs

Before source edits, the focused core baseline passed 14 producer tests.
The CLI baseline passed completion and planning tests, but its live Cargo replay failed at the developer-loop receipt path boundary.
A separate Nix baseline also failed. Its fixture used a builder string without dependency context.
Cargo stopped at each failed integration binary, so those initial commands did not execute every later test binary.
The separate developer-loop core baseline passed eight tests before its repair.

The repaired replay exposed a second failure: foreign receipt schemas triggered a developer-loop debug assertion.
The validator now returns the existing schema and hash rejection diagnostics. Valid loop policy and completion requirements did not change.
The Nix fixture now supplies explicit store context with `builtins.appendContext` and requests a local build.
Both successful and failing Nix processes execute in the maintained fixture. Neither supplies an admitted completion payload.

Additional repairs and controls:

- Source disappearance now preserves child facts and rejected observations in an incomplete attempt. It does not fabricate a source identity or completion row.
- Artifact collisions preserve existing bytes. Other available artifact paths retain observations and `incomplete.json`.
- Runner errors no longer claim observed execution coverage.
- Cargo summary admission rejects incomplete elapsed fields and a missing final newline.
- Source prechecks reject non-regular members, symbolic links, invalid projections, and oversized files.
- Mantle controls recompute native identities before substituting schema, request, builder, and cache fields. Admission still rejects those substitutions.
- Live Cargo controls cover denial, empty tests, failed tests, zero exit with a failing payload, timeout, flood, cancellation, source drift, and descendant cleanup.
- The cancellation fixture asserts the exact `cancelled` result. The escaped-pipe fixture preserves cleanup uncertainty and waits for its finite descendant.

## Commands and observed verdicts

All Cargo commands used the private Pueue group `continue-2384-evidence` and this prefix:

```console
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c
```

Logs remain under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`. No older log was overwritten.

| Command after the prefix | Observed verdict | Log |
|---|---|---|
| `cargo test -p cairn-core` | PASS: 576 unit tests and 12 doc tests | `continue-2384-evidence-core-final.log` |
| `cargo test -p cairn-cli --test check_evidence --test check_evidence_local --test check_evidence_nix --test check_evidence_saved --test agent_completion --test developer_loop` | PASS: 19 integration tests | `continue-2384-evidence-final-tests.log` |
| `cargo test -p cairn-cli --bin cairn check_evidence` | PASS: one source-boundary unit test | `continue-2384-evidence-shell-unit-final.log` |
| `cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings` | PASS after all source repairs | `continue-2384-evidence-final-clippy.log` |
| `cargo fmt --all -- --check` | PASS after all source repairs | `continue-2384-evidence-final-format.log` |
| `target/debug/cairn policy export --check` | PASS: generated policy is fresh | `continue-2384-evidence-policy-verified.log` |

The final integration cohort contains five completion, two planning, seven live Cargo, two live Nix, one saved Mantle, and two developer-loop tests.
The unchanged completion consumer accepts generated current Cargo results and rejects the live failure controls.
The developer-loop consumer rejects Cargo rows with a schema diagnostic. This is not a positive developer-loop producer replay.

Additional commands from the worktree:

```console
nix build path:$PWD#checks.x86_64-linux.check-producer-source-pins --no-link --option min-free 0 -L
target/debug/cairn gate tasks produce-check-evidence-with-stack-adapters --root .
git diff --check
```

The source-pin check passed its positive and substituted-pin assertions (`continue-2384-evidence-pins.log`).
The task gate returned `PASS`, eight unchecked tasks, and `core` as the next ready task (`continue-2384-evidence-tasks.json`).
This gate is structural evidence only. The final whitespace check passed.
A source scan found no filesystem, process, environment, clock, printing, unsafe, or async operations in the producer core.

Earlier logs retain the failed baselines, two Nix fixture diagnostics, and the Clippy allocation finding.
The allocation finding was repaired without an allowance. Direct policy export outside the dev shell failed to locate a required program.
The dev-shell replay then reported fresh policy. Process exit alone was not used as evidence that a payload passed.

## Exact remaining gaps

1. Source, input, cancellation, and publication readback paths still use metadata prechecks rather than atomic no-follow opens.
   Concurrent substitution can defeat those prechecks. Special-file substitution can also defeat bounded-time assumptions.
   This pass does not claim race-resistant no-follow loading or hostile same-authority safety.
2. Local Nix execution has no admitted native build-payload decoder. Exit zero remains a process observation only and cannot satisfy completion.
3. Developer-loop production lacks fmt, lifecycle-validation, traceability, and duration facts for the existing `cairn-local` profile.
   Its positive producer replay remains unavailable. No loop schema or gate was weakened to accept Cargo rows.
4. Mantle admission is read-only native contract validation. It does not create completion rows or provide test coverage and Cairn command/applicability bindings.
5. Lattice, Kiln, Octet, and Valence adapters remain unavailable. The earlier cohort probe reports Lattice's BLAKE3 `^1.8.3` conflict with Mantle's `=1.8.2`.
   That probe was inspected, not rerun. No pin was changed to conceal the conflict.
6. The pinned disposable Kiln consumer replay and producer-owned fixture cohort remain incomplete. Current saved tests construct native contract values, not imported upstream fixture provenance.
7. Nonempty dependency observation sets remain blocked. Source-closure completeness and tool-closure attestation remain consumer obligations.
8. Publication has no automated recovery or manifest-consumer admission command. Journal and partial files can remain after unavailable publication.
   Standalone completion rows do not prove complete bundle publication. Only the final manifest closes the producer attempt.
9. Full workspace tests, all Nix checks, strict Octet acceptance, and complete cross-platform or adversarial race controls were not established.
   The coordinator owns the separate Octet repair. No gate, lint scope, allowance, or warning budget was weakened.

The code worktree was clean after `ffd28201ac80bff8a5fea271d4aa6e61d23ea5bc`, before this report was added.
No incomplete source edit was discarded. No sync, archive, push, worktree removal, or agent spawn occurred.
