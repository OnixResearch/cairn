# Tasks: Policy-owned workflow profiles

## Phase 1: Baseline and policy

- [x] [serial] B1 Record baseline policy, change creation, scaffold, workflow, gate, task, sync, archive, MCP, validation, and release-readiness results. r[cairn.lifecycle_workflow_profiles.verification]
- [x] [serial] P1 Add typed Nickel contracts for profile IDs, versions, artifacts, stages, task-link classes, review obligations, effects, bounds, and non-claims. r[cairn.lifecycle_workflow_profiles.registry]
- [x] [depends:cairn.lifecycle_workflow_profiles.registry] P2 Register `spec-driven`, `assurance`, and `no-spec-delta` with deterministic generated projections. r[cairn.lifecycle_workflow_profiles.initial_profiles]
- [x] [depends:cairn.lifecycle_workflow_profiles.registry] P3 Add repository profile allowlists and deny ambient or user-global profile authority. r[cairn.lifecycle_workflow_profiles.selection] r[cairn.lifecycle_workflow_profiles.boundary]

## Phase 2: Pure core

- [x] [depends:cairn.lifecycle_workflow_profiles.registry] C1 Add pure profile admission, compatibility fallback, explicit selection, and immutable-selection decisions. r[cairn.lifecycle_workflow_profiles.registry] r[cairn.lifecycle_workflow_profiles.selection]
- [x] [depends:cairn.lifecycle_workflow_profiles.initial_profiles] C2 Add pure profile artifact obligations, stage mapping, apply prerequisites, and assurance checks. r[cairn.lifecycle_workflow_profiles.initial_profiles] r[cairn.lifecycle_workflow_profiles.assurance]
- [x] [depends:cairn.lifecycle_workflow_profiles.no_spec] C3 Add pure `a[...]` acceptance admission, task linkage, no-spec conflict detection, and review linkage. r[cairn.lifecycle_workflow_profiles.no_spec]
- [x] [depends:cairn.lifecycle_workflow_profiles.no_spec_sync] C4 Add pure no-spec sync no-op and archive planning with exact effect and evidence identities. r[cairn.lifecycle_workflow_profiles.no_spec_sync] r[cairn.lifecycle_workflow_profiles.functional_boundary]

## Phase 3: CLI and MCP shells

- [x] [depends:cairn.lifecycle_workflow_profiles.selection] S1 Add explicit `change create --profile` parsing, metadata, profile-specific scaffolding, and unknown-profile failure before writes. r[cairn.lifecycle_workflow_profiles.selection]
- [x] [depends:cairn.lifecycle_workflow_profiles.initial_profiles] S2 Make workflow status, instructions, gates, task validation, and MCP results profile-aware. r[cairn.lifecycle_workflow_profiles.integration]
- [x] [depends:cairn.lifecycle_workflow_profiles.no_spec] S3 Load bounded acceptance and semantic review artifacts for no-spec gates. r[cairn.lifecycle_workflow_profiles.no_spec] r[cairn.lifecycle_workflow_profiles.functional_boundary]
- [x] [depends:cairn.lifecycle_workflow_profiles.no_spec_sync] S4 Add no-spec sync preview and archive recheck without accepted-spec mutation. r[cairn.lifecycle_workflow_profiles.no_spec_sync]

## Phase 4: Verification and documentation

- [x] [parallel] V1 Add positive compatibility, assurance, no-spec, scaffold, gate, review, sync, archive, and repeatability fixtures. r[cairn.lifecycle_workflow_profiles.verification]
- [x] [parallel] V2 Add negative unknown, cyclic, ambient, drifted, conflicting-delta, malformed-acceptance, unlinked-task, stale-review, and partial-mutation fixtures. r[cairn.lifecycle_workflow_profiles.verification] r[cairn.lifecycle_workflow_profiles.boundary]
- [x] [depends:cairn.lifecycle_workflow_profiles.integration] V3 Replay existing changes and receipt goldens through the `spec-driven` compatibility fallback. r[cairn.lifecycle_workflow_profiles.compatibility]
- [x] [depends:cairn.lifecycle_workflow_profiles.initial_profiles] D1 Document profile selection, obligations, assurance, no-spec acceptance, review evidence, sync, archive, compatibility, and non-claims. r[cairn.lifecycle_workflow_profiles.docs]
- [x] [depends:cairn.lifecycle_workflow_profiles.verification] V4 Run policy freshness, formatting, focused tests, Clippy, purity smoke, traceability, lifecycle gates, release readiness, and relevant Nix checks. r[cairn.lifecycle_workflow_profiles.verification] r[cairn.lifecycle_workflow_profiles.functional_boundary]

## Verification Coverage

- `Scenario: Valid profile registry exports deterministically` -> P1, P2, C1, V1
- `Scenario: Invalid profile registry fails admission` -> P1, C1, V2
- `Scenario: Existing change uses the compatibility profile` -> C1, V3
- `Scenario: Explicit profile creates the exact scaffold` -> C1, S1, V1
- `Scenario: Unknown or late profile selection fails before mutation` -> C1, S1, V2
- `Scenario: Assurance profile requires extra obligations` -> C2, S2, V1, V2
- `Scenario: No-spec profile links local acceptance criteria` -> C3, S3, V1
- `Scenario: No-spec conflict fails closed` -> C3, S3, V2
- `Scenario: No-spec sync is an explicit no-op` -> C4, S4, V1
- `Scenario: No-spec archive rechecks evidence` -> C4, S4, V1, V2
- `Scenario: Profile decisions remain pure` -> C1, C2, C3, C4, V4
- `Scenario: Profile claims remain bounded` -> V2, D1

## Completion evidence

Evidence recorded on 2026-08-04:

- Baseline policy, lifecycle, MCP, validation, and readiness receipts passed before implementation.
- Policy export freshness passed after the typed profile registry and generated projection changed.
- Focused core, CLI, MCP, assurance, no-spec, compatibility, stale-review, and negative policy tests passed.
- `cargo fmt --check`, `cargo test --workspace`, and workspace Clippy with warnings denied passed.
- The core-purity scan found no operational file, environment, process, clock, async, or print use in the new core.
- Proposal, design, and tasks gates passed. The delta synced into `cairn/specs/lifecycle-workflow-profiles/spec.md`.
- Traceability and Tracey coverage passed at `1855/1855` referenced requirements.
- MCP smoke and release readiness passed. `onix_release_ready` remained false only because the optional authenticated release profile was not requested.
- Self-dogfood, Tracey, Slotcar, Radicle format compatibility, and `nix flake check path:$PWD -L` passed.
