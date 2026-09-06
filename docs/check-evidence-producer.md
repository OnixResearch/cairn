# Check evidence producer

## Available boundary

Cairn provides pure admission, Linux bounded execution, durable local publication, and native saved Mantle admission.
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
Source prechecks reject symbolic-link components, non-regular members, and oversized files.
These prechecks are not atomic no-follow opens. Concurrent path substitution remains an open hardening task.
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
Nix selects an offline flake check without lock updates for an explicit system.
A Nix check is a build obligation, not test coverage. Its process success does not establish nonempty executed-test coverage.

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
Readback still uses metadata prechecks rather than atomic no-follow opens.

If an attempt stops with only a journal or partial files, inspect that directory before recovery.
This version has no automated recovery command. A new attempt requires a separately authorized output directory.
Only a complete manifest closes the publication attempt. Standalone completion rows do not prove bundle publication.
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

## Published dependency cohort and notices

| Dependency | Immutable revision | Imported scope | Public license |
|---|---|---|---|
| Bounded Exec | `29dac88ecded94457572db3fdfaaaab95fa91525` | `bounded-exec` and its core | AGPL-3.0-or-later |
| Durable File Publication | `951c27f59003cea9bfdb40ed4d89653d50fada1f` | Published one-file crate and Linux adapter | MIT OR Apache-2.0 |
| Mantle | `0bbab77e23ab95caa4e4184b9945cfaa5c8040e2` | `mantle-build-contract` only | AGPL-3.0-or-later |

Cargo and Nix acquire complete pinned sources through their published Git transports.
No producer implementation is copied. Upstream source distributions retain their license and contributor notices.
The owner-authorized reuse grant applies only to exclusively OnixResearch-owned material. It does not change public licenses or third-party rights.
`nix/check-producer-source-pins.nix` checks matching Cargo/Nix revisions and rejects a substituted revision.
The existing Octet/Slotcar compiler and check pin remains unchanged.

Lattice, Kiln, Octet, and Valence adapters remain unavailable.
The reviewed Lattice contract requires BLAKE3 1.8.3, while the reviewed Mantle contract requires exactly 1.8.2.
A joint dependency-resolution probe records the resulting blocker separately from runtime conformance.
See the active change's implementation reports for commands, observed verdicts, and remaining work.
