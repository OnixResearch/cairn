# Check evidence producer

## Available boundary

Cairn currently provides pure producer admission and normalization, plus read-only CLI planning.
Local execution, saved-producer imports, and durable publication are not available in this build.
The active change remains incomplete.

Cairn producer-adapter maintainers own this boundary.
The current consumer is Cairn's authoritative completion validator.
The maintained tests generate result rows from explicit observations and replay those rows through the original validator.
These tests do not constitute a live producer or Kiln adoption pilot.

## Plan a check

1. Author the profiles through `contracts/check-evidence-producer.ncl`.
2. Export the profiles with Nickel.
3. Supply the repository-owned completion contract and declared applicability.
4. Run the read-only plan command:

```console
cairn check-evidence plan --root REPOSITORY \
  --contract contract.json --profiles profiles.json \
  --applicability applicability.json
```

The plan admits exactly one profile per required check.
It preserves the completion owner's selection and command identities.
It orders prerequisites before dependent checks and binds the complete plan with BLAKE3.
Unknown, duplicate, malformed, stale-policy, and substituted inputs fail admission.
The shell bounds each input file to 1 MiB and uses Cairn's existing capability-root reader.

The applicability input contains these fields:

- `subject_hash`: the declared final subject identity;
- `policy_hash`: the current generated policy identity;
- `dependency_hashes`: sorted, unique dependency identities;
- `source_projection_hash`: the consumer-owned projection identity, including reviewed exclusions.

Each digest uses 64 lowercase hexadecimal characters.
Planning does not observe the source filesystem or executable bytes.
The receipt explicitly reports `subject_observed: false` and `checks_executed: false`.
It grants no execution authority.

## Profile contract

The two fixed grammars are `cargo_test` and `nix_check`.
Cargo selects the workspace, all targets, a target triple, locked offline dependencies, and an explicit feature set without default features.
Nix selects an offline flake check without lock updates for an explicit system.
A Nix check is a build obligation, not a test-coverage claim.
Neither grammar accepts trailing shell text or ambient executable discovery.

Profiles bind exact executable and cohort hashes, environment values, working root, and named execution bounds.
Limits cap execution at one hour, teardown at one minute, and each captured stream at 1 MiB.
No cache waiver or zero-test rationale exists in this version.
The synthetic profiles under `fixtures/check-evidence/` are admission fixtures, not installed executables or approved live cohorts.

## Pure observation boundary

`AttemptObservation` is a Cairn application-port type, not a producer wire schema.
It deliberately lacks `Deserialize`.
Only a future execution or pinned producer-admission adapter can supply observed facts to this port.
No CLI command accepts user-authored observations through this type.

Normalization preserves the supplied producer identity, native status, non-claims, role, attempt identity, and evidence hash.
Only the local port's exact `passed` status can produce a passing row.
Future upstream adapters must retain native meaning without feeding unknown vocabularies into this local convention.

A zero process exit does not establish success.
Test observations need a passing payload, nonzero executed-test coverage, current applicability, and complete teardown.
Failure, timeout, cancellation, overflow, unknown execution, missing teardown, and unsupported cache claims remain non-passing.
Changed sources, commands, executable bytes, dependency identities, or tool cohorts fail applicability.
Diagnostics use a finite vocabulary instead of raw producer output.

`publication_decision` requests a final manifest only after exact, ordered, passing results and durable artifact readback facts.
It performs no I/O and does not prove those facts are true.
The shell still needs the admitted one-file publication mechanism, output-root authority, serialization, interruption handling, and recovery.

## Explicit blockers

`check-evidence run` rejects every request, including `--execute`.
This build does not contain the reviewed Bounded Exec dependency.
There is no copied runner, arbitrary-command escape hatch, or fallback to `std::process::Command`.

`check-evidence import` rejects every request.
This build does not contain pinned producer admission adapters.
A generic JSON parser must not replace those contracts.

The worker assignment forbids Nix pin and lock changes.
Cairn's current manifests contain none of the new process, publication, or producer-contract dependencies.
The coordinator must authorize matching Cargo/Nix dependency admission before the execution and import work can proceed.
Existing Nix names still refer to Slotcar/Tiger Style and predate canonical Octet integration.
This increment changes no pins, locks, lint defaults, or gate scope.

See the active change's `evidence/implementation-first-pass.md` for exact inspected revisions, contracts, and remaining conformance work.
