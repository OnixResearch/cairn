# Lifecycle Workflow Profiles Specification

## Purpose

Defines the `lifecycle-workflow-profiles` capability.

## Requirements

### Requirement: Workflow profiles are typed policy values

r[cairn.lifecycle_workflow_profiles.registry] Cairn MUST define workflow profiles through typed Nickel policy and checked runtime projections. Each profile MUST bind its version, artifact graph, stage mapping, apply prerequisites, task-link classes, review obligations, receipt fields, and non-claims.

#### Scenario: Valid profile registry exports deterministically

r[cairn.lifecycle_workflow_profiles.registry.valid]
- GIVEN profiles have unique IDs, complete stage mappings, acyclic artifact graphs, safe paths, and known obligation classes
- WHEN policy export and runtime admission run
- THEN equivalent profile values MUST produce the same canonical projection and identities
- AND ordinary lifecycle commands MUST not evaluate Nickel at runtime.

#### Scenario: Invalid profile registry fails admission

r[cairn.lifecycle_workflow_profiles.registry.invalid]
- GIVEN a profile has a duplicate ID, unknown artifact, cycle, unsafe path, missing stage mapping, or unsupported task-link class
- WHEN policy export or runtime admission runs
- THEN Cairn MUST reject the profile with stable diagnostics
- AND no change may select or inherit that invalid profile.

### Requirement: Profile selection is explicit and compatible

r[cairn.lifecycle_workflow_profiles.selection] Cairn MUST support explicit profile selection during change creation. Existing changes without profile metadata MUST use the `spec-driven` compatibility profile, and ambient user or environment state MUST NOT select a profile.

#### Scenario: Explicit profile creates the exact scaffold

r[cairn.lifecycle_workflow_profiles.selection.create]
- GIVEN a known repository-allowed profile and a valid new change name
- WHEN the operator runs `cairn change create NAME --profile PROFILE`
- THEN Cairn MUST write the selected profile ID and version into change metadata
- AND it MUST create only the artifact paths declared by that profile.

#### Scenario: Existing change uses the compatibility profile

r[cairn.lifecycle_workflow_profiles.selection.compatibility]
- GIVEN an existing active or archived change has no workflow profile field
- WHEN profile-aware status, gates, sync, archive, or MCP inspection runs
- THEN Cairn MUST select `spec-driven`
- AND documented existing behavior and receipt meaning MUST remain unchanged.

#### Scenario: Unknown or late profile selection fails before mutation

r[cairn.lifecycle_workflow_profiles.selection.invalid]
- GIVEN a profile is unknown, disallowed, selected through ambient state, or changed after substantive authoring begins
- WHEN selection or change creation runs
- THEN Cairn MUST reject the selection with a stable diagnostic
- AND it MUST NOT create, delete, or reinterpret lifecycle artifacts.

### Requirement: Initial profiles provide compatibility and stronger obligations

r[cairn.lifecycle_workflow_profiles.initial_profiles] Cairn MUST register `spec-driven`, `assurance`, and `no-spec-delta` as initial policy-owned profiles. Profile artifact and gate obligations MUST be inspectable through the artifact workflow contract.

#### Scenario: Spec-driven profile preserves current behavior

r[cairn.lifecycle_workflow_profiles.initial_profiles.spec_driven]
- GIVEN a change selects or inherits `spec-driven`
- WHEN Cairn scaffolds and evaluates its lifecycle
- THEN proposal, delta specs, design, and tasks MUST retain their current dependencies and gate meanings
- AND sync and archive MUST retain their accepted-spec behavior.

#### Scenario: Profile obligations appear in workflow output

r[cairn.lifecycle_workflow_profiles.integration]
- GIVEN any admitted profile is selected for a change
- WHEN workflow status, instructions, gates, or MCP inspection run
- THEN each surface MUST use the same profile version, artifact graph, and obligation facts
- AND a surface MUST NOT silently substitute obligations from another profile.

### Requirement: Existing lifecycle compatibility is preserved

r[cairn.lifecycle_workflow_profiles.compatibility] Workflow profiles MUST be additive for existing changes. The `spec-driven` fallback MUST preserve documented artifact, gate, task, sync, archive, MCP, receipt-field, and receipt-meaning compatibility.

#### Scenario: Existing artifacts and receipts remain stable

r[cairn.lifecycle_workflow_profiles.compatibility.existing]
- GIVEN an existing change or archive has no profile metadata and unchanged lifecycle bytes
- WHEN profile-aware commands evaluate the change through the `spec-driven` fallback
- THEN documented command fields, artifact obligations, gate meanings, and mutation plans MUST remain unchanged
- AND profile metadata MUST not be written unless an explicit supported mutation requests it.

### Requirement: Assurance profile adds reviewable obligations

r[cairn.lifecycle_workflow_profiles.assurance] The `assurance` profile MUST extend spec-driven work with policy-defined risk, verification, and semantic review artifacts. Apply and archive prerequisites MUST bind the current identities of those artifacts and receipts.

#### Scenario: Complete assurance evidence satisfies structural obligations

r[cairn.lifecycle_workflow_profiles.assurance.complete]
- GIVEN risk and verification artifacts pass structural admission and required review receipts bind all current lifecycle sources
- WHEN assurance gates and apply preflight run
- THEN Cairn MUST report the assurance obligations as structurally satisfied
- AND existing implementation and release checks MUST remain separately required.

#### Scenario: Missing or stale assurance evidence blocks progress

r[cairn.lifecycle_workflow_profiles.assurance.blocked]
- GIVEN a risk artifact, verification artifact, required review, or source identity is missing, malformed, or stale
- WHEN a dependent gate, sync, or archive preflight runs
- THEN Cairn MUST block the operation with stable diagnostics
- AND it MUST perform no partial mutation.

### Requirement: No-spec profile uses local acceptance evidence

r[cairn.lifecycle_workflow_profiles.no_spec] The `no-spec-delta` profile MUST require `spec_effect = "none"`, a bounded rationale, stable change-local `a[...]` acceptance criteria, linked tasks, and a current policy-known semantic review receipt. It MUST forbid delta-spec files.

#### Scenario: No-spec profile links local acceptance criteria

r[cairn.lifecycle_workflow_profiles.no_spec.valid]
- GIVEN a no-spec change has a rationale, valid local acceptance criteria, tasks linked to those criteria, no delta-spec files, and a current classification review
- WHEN profile-aware validation and tasks gates run
- THEN Cairn MUST report the local acceptance and review obligations as satisfied
- AND it MUST NOT treat `a[...]` criteria as accepted `r[...]` requirements.

#### Scenario: No-spec conflict fails closed

r[cairn.lifecycle_workflow_profiles.no_spec.conflict]
- GIVEN a no-spec change contains a delta-spec file, missing rationale, malformed acceptance ID, unlinked task, stale review, or spec-mutating action
- WHEN validation, a gate, sync, or archive preflight runs
- THEN Cairn MUST block the operation with stable diagnostics
- AND no accepted spec or archived change may be mutated.

### Requirement: No-spec sync and archive are explicit

r[cairn.lifecycle_workflow_profiles.no_spec_sync] Cairn MUST represent no-spec sync as an explicit no-op plan that binds effect, rationale, acceptance, review, policy, and input identities. Archive execution MUST recheck that plan and every profile prerequisite.

#### Scenario: No-spec sync is an explicit no-op

r[cairn.lifecycle_workflow_profiles.no_spec_sync.preview]
- GIVEN a no-spec change passes its profile-aware gates and has current review evidence
- WHEN the operator previews sync
- THEN Cairn MUST emit an unblocked plan with no accepted-spec write action and `spec_effect` set to `none`
- AND the receipt MUST bind acceptance and review identities with required non-claims.

#### Scenario: No-spec archive rechecks evidence

r[cairn.lifecycle_workflow_profiles.no_spec_sync.archive]
- GIVEN an approved no-spec sync plan and complete tasks exist
- WHEN the operator executes archive
- THEN Cairn MUST reload and recheck profile, rationale, acceptance, review, task, policy, and source identities before mutation
- AND any mismatch MUST preserve the active change and accepted specs unchanged.

### Requirement: Repository policy narrows profile authority

r[cairn.lifecycle_workflow_profiles.boundary] Repository policy MAY allow a subset of generated profiles or require profiles for named effect classes. User-global files, environment variables, agent prompts, and remote packages MUST NOT define or weaken profile authority.

#### Scenario: Ambient profile source is ignored

r[cairn.lifecycle_workflow_profiles.boundary.ambient]
- GIVEN user-global or environment state names a different or weaker workflow profile
- WHEN Cairn selects a profile for a repository change
- THEN Cairn MUST ignore that ambient value
- AND it MUST report only explicit change metadata and admitted repository policy as selection sources.

#### Scenario: Profile conformance does not prove semantic claims

r[cairn.lifecycle_workflow_profiles.boundary.nonclaim]
- GIVEN a no-spec or assurance profile passes every structural and evidence check
- WHEN Cairn emits status, gate, sync, or archive results
- THEN it MUST NOT claim zero behavior effect, risk-analysis truth, test sufficiency, code correctness, or release eligibility
- AND existing domain checks and reviewers MUST retain their authority.

### Requirement: Profile decisions preserve the functional-core boundary

r[cairn.lifecycle_workflow_profiles.functional_boundary] Cairn MUST keep profile admission, selection, artifact obligations, task linkage, conflict detection, and sync or archive plan semantics pure. The CLI MUST own policy, metadata, artifact, review, file, hash, output, and mutation effects.

#### Scenario: Profile decisions remain pure

r[cairn.lifecycle_workflow_profiles.functional_boundary.pure]
- GIVEN admitted profiles, change metadata, normalized artifacts, tasks, and review facts
- WHEN Cairn evaluates profile obligations or a lifecycle plan
- THEN the result MUST be deterministic without filesystems, environments, processes, clocks, networks, mutation, or output effects.

#### Scenario: Shell rechecks profile effects

r[cairn.lifecycle_workflow_profiles.functional_boundary.shell]
- GIVEN a reviewed profile-aware sync or archive plan
- WHEN the operator requests execution
- THEN the shell MUST reload and recheck exact policy, profile, artifact, task, review, and destination facts
- AND a mismatch MUST block every planned effect.

### Requirement: Profile verification and documentation are complete

r[cairn.lifecycle_workflow_profiles.verification] The change MUST include positive and negative policy, core, CLI, MCP, scaffold, gate, task, review, sync, archive, compatibility, and mutation tests.

#### Scenario: Positive and negative suites close the profiles

r[cairn.lifecycle_workflow_profiles.verification.suites]
- GIVEN valid initial profiles and unknown, cyclic, ambient, drifted, conflicting, malformed, stale, and partial-mutation fixtures
- WHEN focused tests and repository lifecycle checks run
- THEN valid cases MUST produce deterministic profile-aware results
- AND invalid cases MUST fail with stable diagnostics and no partial mutation.

### Requirement: Workflow profile documentation is complete

r[cairn.lifecycle_workflow_profiles.docs] Cairn MUST document profile selection, initial obligations, assurance, no-spec acceptance, review evidence, sync, archive, compatibility, and non-claims.

#### Scenario: Operator can select a profile without hidden authority

r[cairn.lifecycle_workflow_profiles.docs.selection]
- GIVEN an operator reads the workflow-profile guide
- WHEN the operator compares available profiles
- THEN the guide MUST show each artifact, gate, task-link, review, sync, and archive obligation
- AND it MUST state that profile conformance does not prove the underlying semantic claims.
