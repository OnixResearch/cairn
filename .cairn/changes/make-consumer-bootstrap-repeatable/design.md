# Consumer bootstrap design

## Context

`run_init` creates lifecycle directories. `default_policy_path` still names a path relative to the invoking directory.
The generated-workflow installer already owns project-local guidance manifests and freshness classifications.
The new flow composes those mechanisms and makes policy selection explicit. It does not replace `init` or legacy migration.

## Goals and non-goals

The goal is a repeatable clean-root and upgrade workflow with explicit policy identity, file ownership, and recovery.
The first consumer is the Cairn disposable adoption fixture. Cairn maintainers own its long-term conformance test.
This is not a package manager, ambient configuration resolver, or automatic consumer migration service.

## Decisions

### Contract and policy selection

A typed Nickel bootstrap cohort declares the tool artifact, policy export, evaluator cohort, workflow profile, allowed upgrade relation, managed destinations, and named bounds.
It also declares source provenance, compatibility rules, and non-claims. Checked JSON is the runtime projection.
The existing Nickel Export pin records declared input and exact output identity. Normal lifecycle commands never evaluate Nickel.

Policy selection gives an explicit `--policy` precedence over a checked project policy declaration.
An invalid explicit selection fails without fallback. A project declaration binds a safe relative locator and exact policy identity.
Existing invocations without a project declaration retain their documented default-path behavior.
Root selection continues to use the existing root/store contract. The policy locator resolves beneath that selected root, not an unrelated working directory.

The proposed `setup plan`, `setup apply`, and `setup check` commands use the same core contract.
Apply requires an exact current plan identity and explicit execution. Upgrade is an explicit operation in the plan, not a hidden side effect.

### Functional core and shell

A new narrow verified bootstrap module owns cohort admission, ownership classification, compatibility decisions, transitions, and typed publication plans.
Its inputs include current managed manifests and already-loaded bounded file facts.
The CLI shell owns locks, directory capabilities, exports, observation, publication, and recovery evidence.
Outbound ports describe policy export and managed publication. No port wraps a trivial hash or path calculation.

Cap Root supplies lexical path and leaf admission at the revision recorded in `docs/cairn-utility-program.md`.
Cairn retains capability-root grants, no-follow operations, symlink rejection, substitution checks, and diagnostics.
Durable File Publication supplies one-file publication. It does not supply installation transactions or rollback.

### Publication and recovery

The plan binds source files, checked policy bytes, prior manifest, destination observations, and generated-workflow identities.
Apply takes a project-local lock and observes all preconditions again before a write.
Prepared policy generations stay separate from the currently selected complete policy.
The shell publishes bounded artifacts before it publishes the final installation manifest or selection pointer.

Guidance uses the existing managed-workflow installer. Modified and unmanaged files remain conflicts.
Cairn records intermediate installation state so checks can detect partial publication.
A crash cannot convert prepared artifacts into an admitted complete installation.
Recovery observes exact bytes and either resumes an unchanged owned plan or requests manual repair.
It never deletes unowned files, infers success from a rename alone, or treats unknown durability as durable completion.

### Diagnostics and compatibility

Doctor composes selected-policy compatibility, export freshness, and declared generated-guidance checks.
It reports stable codes, safe artifact references, expected and observed identities, and bounded repair actions.
It does not scan home directories or replace global skills.
The existing `agent workflow check` classifications remain authoritative for generated guidance.

## Stack adoption and ownership

Reuse candidates and exact revisions are recorded in the program index.
The implementation must retain positive and negative consumer replays before adding or updating a dependency.
No ambient sibling path enters Cargo, Nix, runtime policy, or execution.
Source imports must preserve notices and record any OnixResearch owner grant.
A required upstream API change needs a separate producer-owned change before adoption.

## Risks and trade-offs

A new setup path adds managed state. A complete manifest and conservative recovery make that state inspectable.
Existing scripts can depend on the old policy default. An additive project declaration avoids a silent precedence change.
Guidance updates span files. One-file atomic replacement cannot establish whole-installation atomicity.
Unsupported platform publication primitives produce a blocker rather than weaker silent behavior.

## Validation strategy

Establish existing init, policy-selection, generated-workflow, and migration baselines before implementation.
Add pure tests for accepted and rejected cohorts, ownership, stale plans, and recovery transitions.
Add shell tests for clean setup, explicit policy precedence, unrelated working directories, no-follow paths, permission errors, interruptions, and modified guidance.
Retain the original missing-policy failure and explicit-policy success as controls.
Run the complete setup path from a fresh root without global Cairn state.
Run focused tests, Nickel positive and negative fixtures, strict Clippy, existing architecture checks, Cairn gates, and the full Nix check.
Use named policy values for every resource limit and timing budget.

## Migration and completion

Document the additive setup path and the unchanged low-level init path.
Publish current project-local workflows and the checked setup example together.
Do not modify accepted legacy trees automatically.
Completion requires retained runtime evidence and fresh-root reproduction. Planning receipts establish document conformance only.
