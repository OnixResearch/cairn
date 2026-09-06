# Check evidence producer design

## Context

`agent-receipt complete` owns exact required-check admission. Developer-loop verification owns local-loop conformance.
Both consume saved evidence and leave execution outside their pure validators.
Kiln already consumes pinned Cairn gate and Lattice workflow contracts and preserves native result distinctions.
This package supplies the missing maintained producer path, not a replacement completion rule.

## Goals and non-goals

Cairn maintainers own the producer profile, translation, and consumer fixtures.
Cairn itself is the first local consumer. A disposable Kiln integration fixture is the second consumer boundary.
The design excludes CI state, worker placement, automatic retries, secret resolution, and arbitrary command execution.

## Decisions

### Contract and local plan

A typed Nickel profile declares required check IDs, parameter schemas, exact executable cohort, argv grammar, target, features, environment, named limits, and evidence roles.
The repository-owned completion contract remains the source of required-check selection.
The pure core admits a `CheckEvidencePlan` and computes its BLAKE3 identity.
Producer input cannot replace the required set or waive prerequisites.

Local `run` requires explicit execution and the unchanged reviewed plan hash.
The shell admits exact executable bytes or a declared tool closure before each attempt.
It passes explicit argv, environment, current directory, I/O bounds, deadline, cancellation, and teardown settings to Bounded Exec.
The first local profiles cover reviewed Cargo test and Nix check invocations. They do not accept arbitrary shell text.
A test profile must observe nonempty executed-test coverage or an explicit repository-approved zero-test rationale.
No zero-test rationale applies by default.

### Ownership and components

The verified producer core owns selection binding, applicability, classification, safe diagnostics, and typed publication plans.
A narrow CLI producer shell owns local execution and file publication.
Saved-evidence adapters own producer decoding and call published producer admission contracts where available.
Application-owned ports describe authorized check execution and evidence publication, not trivial internal calculations.

Bounded Exec supplies process mechanics at the program's reviewed revision.
Kiln retains candidate, attempt, CI state, and provider selection.
Lattice retains workflow execution through `lattice-workflow-contract` and its existing Kiln adapter.
Mantle retains build observations through `mantle-build-contract`.
Octet retains source-analysis and checked-path evidence. Valence retains shared identities, roles, spans, and non-claims.
No Cairn dependency imports a producer's process shell into the functional core.

### Adapter admission

The adapter registry admits exact schema versions, cohort identities, status vocabularies, and supported evidence roles.
Unknown or unavailable producer admission contracts block that adapter. They do not select a best-effort JSON parser.
If a producer must add a public contract, its owner needs a separate change and published conformance fixtures before Cairn adopts it.
This package does not modify producer repositories.

Native statuses remain visible. A passed build satisfies only an explicitly selected build obligation.
A pending workflow, skipped check, cancelled attempt, failing payload, or warning-only Octet run cannot satisfy a required passed test or strict lint check.
The strict Octet profile uses the pinned full catalog as errors over workspace, all targets, and compatible features.
No disabled lints, warning budget, or finding baseline can turn an incomplete strict check into acceptance evidence.

### Applicability and publication

The source projection is explicit and consumer-owned. Generated evidence destinations are excluded only by that reviewed projection.
The shell compares source observations before and after local execution. Drift invalidates whole-subject evidence.
Every result binds subject, required selection, policy, dependency set, tool cohort, command, target, features, and attempt.
A cache hit requires the configured prior execution provenance and exact applicability. It is never relabeled as a new execution.
Unsupported cache claims remain non-passing.

Stable diagnostic categories replace raw output in portable receipts. Optional local logs require separate bounded, authorized retention.
The shell publishes complete artifacts before the final manifest through capability-relative paths.
Reuse Durable File Publication for the one-file primitive. Cairn owns the multi-file manifest, lock, interrupted state, and recovery policy.
Partial publication, unknown teardown, and source drift cannot produce a complete successful manifest.
No automatic retry follows an unknown outcome.

## Risks and trade-offs

Command exit status alone can hide empty tests or a failing native payload. Each profile needs a reviewed output grammar and coverage rule.
A producer adapter can accidentally widen claims. Preserve native status and role alongside the Cairn classification.
The maintained local path is deliberately narrow. Unsupported tools require an explicit new profile and conformance evidence.
Producer revisions and notices remain pinned as recorded in `docs/cairn-utility-program.md`.

## Validation strategy

Establish completion, loop, and current gate-interchange baselines before implementation.
Test pure normalization with current and stale identities, wrong roles, failed statuses, duplicate rows, missing coverage, and invalid prerequisite order.
Test local authority denial, explicit environment, timeout, cancellation, flooded streams, descendant cleanup, source drift, and publication interruption.
Retain independent consumer replays through the original completion and loop validators.
Use exact upstream fixtures through producer-owned import tools where available. Do not copy source implementations.
Run the new producer on a fresh Cairn fixture and the pinned disposable Kiln consumer recipe without manual result editing.
Run focused and full repository checks, strict Clippy, architecture checks, Nickel fixtures, and Cairn gates.

## Migration and completion

Existing validation commands remain read-only and byte-compatible.
Publish the producer profile, import contracts, recipes, and positive and negative fixture cohort together.
The separate candidate-assurance and compiler-bound evidence plans remain independent and unclaimed.
Completion requires the existing consumers to accept current generated evidence and reject the full failure cohort.
