# Evidence producer pass 2746

## Scope and ownership

Cairn producer-adapter maintainers own these changes. The immediate outcome is safer local reads, final-manifest admission, and bounded Octet contract translation.
The durable contribution is executable positive and negative controls through the maintained producer and original completion consumer.
This pass does not complete the change. It changes only the assigned worktree and preserves every producer pin.
No source implementation was copied. No dependency, lockfile, gate, lint allowance, producer repository, or original consumer validator changed.
No sync, archive, push, agent spawn, or worktree removal occurred.

## Core task review

The pre-edit baseline passed all 14 producer-core tests. The expanded cohort passed all 18 tests.
The `core` task is now complete for the admitted decision surface. Seven tasks remain open.

| Decision surface | Positive and negative controls |
|---|---|
| Plan admission | Exact selection and dependency-first order; unknown, duplicate, missing prerequisite, stale policy/subject, command substitution, bounds, environment, target, and feature controls |
| Status normalization | Original completion consumer accepts current nonempty tests; denial, timeout, cancellation, overflow, unknown cleanup, empty tests, failed payload, skipped/pending/warning statuses, wrong roles, and cache claims cannot pass |
| Applicability | Subject, projection, policy, dependency, executable, cohort, command, and plan substitutions reject |
| Publication | Current durable artifact readback passes; missing, changed, non-durable, failed, reordered, duplicate, and mixed-attempt results withhold a final manifest |
| Manifest consumer | Exact journal, artifact bytes, observation receipts, native payload facts, current plan, and original completion contract pass; missing, changed, rehashed non-passing, foreign, and interrupted artifacts reject |
| Recovery | Only admitted complete manifests return `complete`; every other observed state requires inspection and a separately authorized new output root |

The review found and fixed a mixed-attempt publication gap. Results from different attempts previously passed when their plan identities matched.
The new manifest consumer also rejects rehashed non-passing observations behind nominally passing completion rows.
These decisions consume supplied facts. They do not prove execution truth or grant recovery mutation authority.

## Local read boundary

`crates/cairn-cli/src/check_evidence/reads.rs` uses the existing admitted `cap-std` dependency.
On Linux x86_64 and aarch64, each capability-relative component opens with `O_PATH | O_NOFOLLOW`.
The shell rejects symlink descriptors and non-directory parents before traversal.
It reopens only a verified regular-file descriptor through `/proc/self/fd`, while the original descriptor remains held.
This is a fail-closed equivalent to an ordinary no-follow read open. It avoids FIFO or device read effects before type admission.
Unsupported ABIs and unavailable procfs fail closed. No new external dependency supplies this mechanism.
Source projections, JSON inputs, cancellation controls, and publication readback use this path.
Tests cover regular files, bounds, leaf and parent symlinks, sockets, directories, traversal, missing members, and concurrent symlink substitution.

Root selection remains explicit ambient operator authority. The mechanism does not provide hostile same-authority protection, source-closure completeness, or immutable regular-file contents.
Nix-store executable authority still relies on the existing exact byte prechecks and postchecks plus the store immutability assumption.
It does not claim descriptor-based process execution or independent tool-closure attestation.

## Publication and consumer replay

`check-evidence verify` is read-only. It rechecks current source and policy before pure manifest admission.
It binds the journal, exact artifact inventory, raw BLAKE3 identities, normalized observations, local producer revision, payload facts, and original completion results.
It admits at most 67 artifacts, 4 MiB per artifact, and 64 MiB in total.
The live Cargo fixture passes both the original completion validator and this manifest consumer without manual result edits.
The same fixture rejects changed results, missing journals, and interrupted manifests. Verification never retries or deletes output.
The original loop validator still rejects Cargo result rows. That negative replay is not a positive developer-loop producer replay.

## Saved JSON contract investigation

The published source snapshots remain at the revisions recorded in `implementation-first-pass.md` and `docs/cairn-utility-program.md`.
Contract translation does not require a joint producer Cargo graph. The Lattice/Mantle BLAKE3 version conflict therefore does not itself block JSON translation.

| Producer and exact revision | Inspected contract and outcome |
|---|---|
| Mantle `0bbab77e23ab95caa4e4184b9945cfaa5c8040e2` | Existing native `validate_observation` admission remains unchanged. It preserves statuses, cache identities, and required non-claims. It supplies no Cairn command/applicability binding or executed-test coverage. |
| Octet `890f424cd7b1b2015f39c0eb5747f1bd1e576028` | Added pure translation of `standards/receipts/contracts.ncl`, `octet-acceptance-receipt/v1`. Closed typed fields, integer artifact sizes, bounded strings/arrays, native statuses, stage/command failures, evidence tiers, and scope identities survive translation. Raw diagnostics and paths do not enter the safe receipt. |
| Lattice `70496e67c7fd4a8b05914161a8e09de2759bebc8` | `validate_observation` needs profile, request, accepted record, node/state consistency, bounded ordered logs, and framed identities. JSON translation is feasible, but this pass does not implement that complete validator or its producer fixture cohort. A node receipt identity alone does not establish test coverage. |
| Kiln `5d99f4e9bb954743443e340baece31256b4c53f1` | `admit_workflow_observation` binds the active plan, candidate, attempt, provider, node receipts, logs, and native outcome. `Skipped` remains natively successful but not an executed test. Full JSON translation and the pinned disposable consumer replay remain unimplemented. |
| Valence `e40c76b4d2070a29636e00c85c0dff93f03dba2f` | `validate_stack_provenance_sidecar` checks schema, identity roles, parent artifacts, and non-claims. JSON translation can preserve linkage but cannot create check execution. A complete translated adapter and upstream fixture replay remain unimplemented. |

The Octet translation is Cairn-owned code over the published contract, not copied producer implementation.
The inspected source contract retains AGPL-3.0-or-later. Existing source distributions retain all original notices.
Its Nickel contract permits broader values than Cairn's bounded subset. Admission reports shape conformance, not producer authenticity or content truth.
It lacks an exact full-catalog deny-all obligation binding. Every admitted Octet receipt remains completion-ineligible, including `passed` receipts.
A warning-only report or a passing outer status with failed native stages cannot become a strict lint result.
The implementation and verification trace markers name this real translator. Marker coverage does not close the remaining adapter task.

There is no demonstrated producer-owned blocker to native JSON admission for Lattice, Kiln, or Valence.
Their missing translators are Cairn work, not an excuse to change producer pins.
Passing test translation additionally needs a reviewed node/payload coverage contract and exact Cairn applicability mapping.
The existing Lattice workflow receipt identity and Valence linkage sidecar cannot supply those missing execution facts.

## Checks and logs

All Cargo checks use the private Pueue group `finish-2746-evidence` and this prefix:

```console
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c
```

Logs remain under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`, with the `finish-2746-evidence-` prefix.
No older log was overwritten.
The host PATH lost `env` and `nix` during the pass. Two commands exited 127 before execution.
Subsequent commands prepend existing coreutils and Nix store binaries, then use the required Cargo prefix unchanged.
The failed logs are `core.log` and `local.log`. Their successful replays are separate files.

| Check | Observed result | Log suffix |
|---|---|---|
| Producer-core baseline | PASS: 14 tests | `baseline.log` |
| Expanded producer core | PASS: 18 tests | `core-expanded.log` |
| Producer shell unit tests | PASS: 2 tests | `shell-expanded.log` |
| Producer/completion/loop integration cohort | PASS: 20 tests | `integration-expanded.log` |
| Focused Clippy before final translator additions | PASS, not final-increment evidence | `focused-clippy.log` |

Final battery results and commits follow below after their commands finish.

## Remaining work

- Finish producer-owned fixture intake and full Lattice, Kiln, and Valence native JSON admission without schema or pin changes.
- Establish the pinned Kiln consumer replay through its original validators.
- Add reviewed profiles for fmt, lifecycle validation, traceability, and measured duration before a positive `cairn-local` replay.
- Admit native Nix payload evidence before any Nix process success can satisfy completion.
- Keep nonempty dependency observation sets unavailable until their exact observation contract exists.
- Complete the remaining shell/adapter failure cohort and independent consumer evidence before closing the ordered tasks.
- The local/publication work in this pass is tested partial progress on later tasks, not task completion out of order.
