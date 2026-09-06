# Check evidence producer

## Available boundary

Cairn provides pure admission, Linux bounded execution, durable local publication, manifest verification, and saved Mantle, Octet, Lattice, Kiln, and Valence admission.
The active change remains incomplete. Cairn producer-adapter maintainers own this boundary.
The current live consumer is Cairn's original completion validator.
The maintained disposable Cargo and Nix fixtures need no manual result edits.

## Plan and execute a local check

1. Author profiles through `contracts/check-evidence-producer.ncl`.
2. Export the profiles with Nickel.
3. Supply the repository-owned completion contract and applicability.
4. Supply a sorted, unique JSON array of repository-relative source files in `projection.json`.
5. Observe that exact file projection:

```console
cairn check-evidence source --root REPOSITORY --projection projection.json
```

The source receipt reports `subject_hash` and `source_projection_hash` for the applicability input.
The projection limits are 4,096 files, 16 MiB per file, and 64 MiB in total.
Source reads reject symbolic-link components, non-regular members, and oversized files.
On Linux x86_64 and aarch64, capability-relative `O_PATH | O_NOFOLLOW` opens hold each component before metadata inspection.
A symlink descriptor fails admission. Only a regular-file descriptor is reopened through `/proc/self/fd` for bounded reads.
This fail-closed equivalent prevents symlink substitution and avoids device or FIFO read effects before file-type admission.
Missing procfs support and unsupported platform ABIs fail closed.
Contract, profile, applicability, projection, and saved-observation inputs use this mechanism, as do cancellation controls and publication readback.
Producer policy loading uses the same descriptor reader with a 4 MiB bound and the original `validate_policy_json` function.
Absolute policy paths start at `/`. Relative policy paths retain their current-working-directory meaning.
Policy read and validation errors omit paths, contents, and parser diagnostics.
The selected file set does not prove source-closure completeness.
Consumers retain projection review, including build inputs and exclusions for evidence and build output.

6. Run the read-only plan command:

```console
cairn check-evidence plan --root REPOSITORY \
  --contract contract.json --profiles profiles.json \
  --applicability applicability.json
```

The plan preserves the completion owner's exact selection, command identities, and prerequisite order.
Planning does not observe executable bytes. It grants no execution authority.
Each input file has a 1 MiB bound.

7. Create a separate, empty output directory under explicit operator authority.
8. Create a one-byte cancellation control file with the content `0` under the repository root.
9. Execute the reviewed plan with a fresh attempt identity:

```console
cairn check-evidence run --execute --root REPOSITORY \
  --contract contract.json --profiles profiles.json \
  --applicability applicability.json --projection projection.json \
  --plan-hash REVIEWED_HASH --attempt ATTEMPT_BLAKE3 \
  --cancel-file cancel --out OUTPUT_DIRECTORY
```

A nonzero cancellation byte, missing control, or unreadable control requests cancellation.
The control watcher polls every 10 ms. Bounded Exec owns process-group termination and bounded teardown.
A failed or uncertain attempt has no automatic retry.

## Profile and observation contracts

The fixed grammars are `cargo_test` and `nix_check`. Neither accepts shell text or arbitrary trailing arguments.
Cargo selects workspace tests, all targets, an explicit target and feature set, locked offline dependencies, and no default features.
Nix evaluates a fixture-owned expression file offline for an explicit working root, so the same profile runs in a shell and inside a sandboxed check.
A Nix check is a build obligation, not test coverage.
The local Nix adapter records process success but has no admitted native build-payload decoder.
It therefore emits no passing completion row, even after exit zero.
Its process success also does not establish nonempty executed-test coverage.

Profiles bind a Nix-store executable, byte identity, declared cohort identity, explicit environment, working root, and named bounds.
The environment is cleared before explicit entries are applied. Profiles must not contain secrets.
The cohort label is a binding, not independent attestation of a complete tool closure.
Execution rechecks executable bytes before each child and after each result.
The child working directory refers to the held repository directory through a Linux file descriptor.

The source projection is observed before execution, between attempts, and before final publication.
Nonempty dependency identity sets remain blocked because a dependency-observation contract is absent.
The mechanism does not prevent a hostile same-authority process from changing files between observations.

Cargo coverage admission pairs stable libtest `running N tests` announcements with complete result summaries.
All admitted summaries must pass. Executed-test coverage excludes ignored, measured, and filtered tests.
Empty, malformed, unmatched, failing, or truncated summaries cannot pass.
The final line needs a newline and a complete elapsed-time field.
Custom harness output can imitate this grammar. Summary admission does not prove oracle truth.

`AttemptObservation` remains a non-deserializable Cairn application-port type, not an upstream wire schema.
Diagnostics retain bounded hashes, byte counts, statuses, and non-claims instead of raw child output.
Timeout, cancellation, overflow, failed payloads, source drift, and unknown cleanup remain non-passing.
A successful process is not proof of sandboxing, source correctness, or release eligibility.

## Publication and recovery

Durable File Publication supplies exclusive staging, exact no-replace rename, payload synchronization, and parent synchronization.
Cairn publishes `attempt.json` before execution. That journal prevents concurrent reuse and implicit retries in the same output directory.
Cairn publishes native facts, normalized observations, and `results.json` before the final manifest.
The manifest binds artifact hashes after bounded readback. Failed attempts receive `incomplete.json` when publication remains available.
Source disappearance preserves executed facts and rejected observations without completion rows.
Publication collisions preserve existing bytes and retain observations through other available artifact paths.
A runner error does not establish execution coverage.
Any publication error withholds the final manifest. Existing destination bytes remain unchanged.
Readback uses the same no-follow descriptor mechanism as source reads.
The manifest also binds the attempt journal. Pure publication admission rejects results from different attempts, even when their plan identities match.

If an attempt stops with only a journal or partial files, inspect that directory before recovery.
A new attempt requires a separately authorized output directory. Cairn never retries, removes partial files, or completes an interrupted manifest automatically.
The read-only verifier returns `inspect_and_use_new_output_root` for missing, malformed, changed, or incomplete artifacts.
Only an admitted complete manifest closes the publication attempt.

```console
cairn check-evidence verify --root REPOSITORY --out OUTPUT_DIRECTORY \
  --contract contract.json --profiles profiles.json \
  --applicability applicability.json --projection projection.json \
  --plan-hash REVIEWED_HASH --attempt ATTEMPT_BLAKE3
```

Verification rechecks current source and policy, exact artifact bytes, journal linkage, normalized observations, native payload facts, and the original completion contract.
It admits at most 67 artifacts, 4 MiB per artifact, and 64 MiB in total.
Verification proves presented linkage and admitted payload shape, not execution truth. Standalone completion rows do not prove bundle publication.
The shell can return a non-passing JSON receipt with exit zero. Consumers must inspect its `valid` field.
Parent synchronization is an observed system-call result, not a power-loss correctness guarantee.

## Original consumer replay

```console
cairn agent-receipt complete --contract CONTRACT \
  --results OUTPUT_DIRECTORY/results.json --subject-hash SUBJECT
```

The original completion validator accepts current passing Cargo rows and rejects the real failure controls.
The developer-loop validator rejects these rows as loop receipts with a schema diagnostic.
Foreign receipt schemas no longer trigger a debug assertion.
Its `cairn-local` profile requires fmt, lifecycle validation, and traceability facts that Cargo/Nix profiles do not supply.
No adapter fabricates those facts or weakens the loop validator.

## Native saved admission

```console
cairn check-evidence import mantle --root SAVED_DIRECTORY \
  --request request.json --builder builder.json --observation observation.json
```

These files use the published `mantle-build-contract` types. The builder file contains its native identity string.
Cairn calls `validate_observation` and preserves native status, cache facts, identities, and non-claims.
Native admission does not create completion rows or grant execution authority.
The Mantle contract lacks executed-test coverage and Cairn command/applicability binding. Both remain explicit blockers.
Unknown producer adapters fail closed. No generic decoder supplies missing producer facts.

```console
cairn check-evidence import octet --root SAVED_DIRECTORY --observation observation.json
```

The Octet adapter translates the published `octet-acceptance-receipt/v1` JSON contract in Cairn's pure core.
It admits a bounded subset with integer artifact sizes, closed fields, and at most 64 rows per native array.
It preserves native status, failure counts, evidence tiers, and coverage-scope identities. It omits raw logs, diagnostics, and paths from its receipt.
The source contract does not bind full-catalog deny-all scope. Passing, failed, blocked, and warning-bearing receipts therefore remain ineligible for completion.
This translation uses no Octet source implementation or Cargo dependency. The pinned contract revision is `890f424cd7b1b2015f39c0eb5747f1bd1e576028`.
The source contract retains its AGPL-3.0-or-later notice. This consumer translation does not change upstream licenses or claims.

```console
cairn check-evidence import lattice --root SAVED_DIRECTORY \
  --profile profile.json --request request.json \
  --accepted accepted.json --observation observation.json
cairn check-evidence import kiln --root SAVED_DIRECTORY \
  --plan plan.json --observation observation.json
cairn check-evidence import valence --root SAVED_DIRECTORY --observation sidecar.json
```

Lattice admission checks the original profile, request, accepted record, observation, and framed identities.
It checks ordered capabilities, graph membership and cycles, controls, native states, receipts, and ordered bounded logs.
Cairn admits at most 64 graph nodes, edges, and log references, within the producer's declared limits.
The original published producer fixture and its rejection controls accompany a separate native-library fixture replay.
Skipped nodes remain natively successful. They never establish executed-test coverage.

Kiln admission calls the pinned `kiln-core::workflow::admit_workflow_observation` function after the original nominal constructors admit JSON values.
Its native status and CI outcome remain separate. The adapter adds finite input bounds but does not replace CI decisions.
The published Kiln workflow DTO has no non-claim field. Cairn states the consumer boundary explicitly instead of inventing native claims.

Valence admission implements the bounded intersection of the published Nickel sidecar contract and Rust sidecar validator.
It preserves binary and parent artifact identities, source identity hashes, claim scope, and the reviewed stack and chain non-claims.
Unreviewed extra non-claim text is unsupported and rejected, so arbitrary input text cannot enter the safe receipt.
No measured binary bytes or executed tests are present. This adapter does not perform built-binary thread verification.

All three adapters return empty completion rows and `completion_eligible: false`.
Native identity admission proves neither producer authenticity nor current Cairn command applicability.
See [fixture provenance](../fixtures/check-evidence/README.md) for immutable revisions, original fixtures, and repeatable commands.

## Published dependency cohort and notices

| Dependency | Immutable revision | Imported scope | Public license |
|---|---|---|---|
| Bounded Exec | `29dac88ecded94457572db3fdfaaaab95fa91525` | `bounded-exec` and its core | AGPL-3.0-or-later |
| Durable File Publication | `951c27f59003cea9bfdb40ed4d89653d50fada1f` | Published one-file crate and Linux adapter | MIT OR Apache-2.0 |
| Mantle | `0bbab77e23ab95caa4e4184b9945cfaa5c8040e2` | `mantle-build-contract` only | AGPL-3.0-or-later |
| Kiln | `5d99f4e9bb954743443e340baece31256b4c53f1` | `kiln-core` workflow admission and nominal values | MIT OR Apache-2.0 |

Cargo and Nix acquire complete pinned sources through their published Git transports.
No producer implementation is copied. Upstream source distributions retain their license and contributor notices.
The owner-authorized reuse grant applies only to exclusively OnixResearch-owned material. It does not change public licenses or third-party rights.
`nix/check-producer-source-pins.nix` checks matching Cargo/Nix revisions and rejects a substituted revision.
The existing Octet/Slotcar compiler and check pin remains unchanged.

The Lattice native fixture helper has a separate locked Cargo graph. Its producer requires BLAKE3 1.8.3 or later; Mantle requires exactly 1.8.2.
The product JSON translator preserves the pinned Lattice framing without joining those dependency graphs.
Kiln's published source is `https://seed.radicle.garden/z2wsvXm5S2sJGvuV1k5JHiwi1PbKE.git`.

The Nix consumer packaging adapter limits Mantle package discovery to `crates/mantle-build-contract`.
It retains the complete pinned checkout for workspace inheritance and uses Cargo's original package admission and Crane's inheritance resolver.
It compares the packaged source directory and license against the immutable input. No producer source or lint command changes.
The source-pin check also rejects changed revisions and substituted workspace inheritance metadata.
See the active change's implementation reports for commands, observed verdicts, and remaining work.
