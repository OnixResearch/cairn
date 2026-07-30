# Lifecycle Stores Specification

## Purpose

Defines the `lifecycle-stores` capability.

## Requirements

### Requirement: Lifecycle store identity is typed and deterministic

r[cairn.lifecycle_stores.identity] Cairn MUST define lifecycle stores through typed Nickel identity source and a checked JSON runtime projection. The identity MUST bind schema, store ID, optional remote hint, non-claims, and BLAKE3 source identity. Cairn MUST export store and project projections through `cairn config export --kind store|project --root PATH [--check] [--execute]` only.

#### Scenario: Valid store identity exports deterministically

r[cairn.lifecycle_stores.identity.valid]
- GIVEN a valid store identity satisfies the typed Nickel contract
- WHEN the operator exports its runtime projection
- THEN equivalent source values MUST produce the same canonical JSON bytes and BLAKE3 identity
- AND ordinary lifecycle commands MUST read the checked projection without evaluating Nickel.

#### Scenario: Stale or malformed store identity fails

r[cairn.lifecycle_stores.identity.invalid]
- GIVEN a store identity has an invalid ID, unsafe remote text, missing non-claim, malformed source, or stale runtime projection
- WHEN Cairn admits or resolves the store
- THEN Cairn MUST reject the identity with deterministic diagnostics
- AND it MUST NOT register or select the store.

### Requirement: The local store locator is explicit and transactional

r[cairn.lifecycle_stores.locator] Cairn MUST keep machine-local store paths in a canonical locator separate from lifecycle authority and the global registry. Locator mutations MUST use preview, expected prior identity, locking, recheck, and atomic replacement.

#### Scenario: Registration commits one checked mapping

r[cairn.lifecycle_stores.locator.register]
- GIVEN a valid native store, safe checkout path, current identity projection, and matching prior locator identity
- WHEN the operator executes registration
- THEN Cairn MUST atomically map one store ID to one checkout and exact store identity
- AND the receipt MUST distinguish local locator state from lifecycle evidence.

#### Scenario: Duplicate or stale registration fails without replacement

r[cairn.lifecycle_stores.locator.conflict]
- GIVEN an ID or checkout already maps differently, the plan is stale, the path is a symlink, or identity changed
- WHEN Cairn previews or executes locator mutation
- THEN Cairn MUST emit a stable conflict diagnostic
- AND the last valid locator state MUST remain unchanged.

### Requirement: Store commands are preview-first

r[cairn.lifecycle_stores.commands] Cairn MUST expose store setup, register, unregister, list, show, and doctor commands. Every command that changes repository or local state MUST require explicit `--execute` after a deterministic preview.

#### Scenario: Setup creates native store artifacts

r[cairn.lifecycle_stores.commands.setup]
- GIVEN a valid store ID, safe empty destination, and optional bounded remote hint
- WHEN the operator executes a reviewed setup plan
- THEN Cairn MUST create native lifecycle and store identity artifacts only at declared safe paths
- AND the mutation receipt MUST bind before and after BLAKE3 file identities.

#### Scenario: Blocked setup or unregister does not mutate

r[cairn.lifecycle_stores.commands.blocked]
- GIVEN setup would overwrite content or unregister execution observes changed locator state
- WHEN the operator requests execution
- THEN Cairn MUST fail before mutation
- AND it MUST preserve repository files and the prior locator.

### Requirement: Root resolution is deterministic and bounded

r[cairn.lifecycle_stores.root_resolution] Cairn MUST resolve root-local lifecycle commands from explicit selectors, bounded native-root discovery, checked project pointers, or the current-directory fallback. Cairn MUST NOT use global-registry state or a machine-wide default store.

#### Scenario: Explicit store selection resolves deterministically

r[cairn.lifecycle_stores.root_resolution.explicit]
- GIVEN one registered current store and a supported root-local command with `--lifecycle-store ID`
- WHEN root resolution runs
- THEN Cairn MUST select the registered native root and bind the store identity in the resolution result
- AND the result MUST not depend on unrelated current-directory or global-registry state.

#### Scenario: Conflicting selectors fail closed

r[cairn.lifecycle_stores.root_resolution.conflict]
- GIVEN one command includes both `--root PATH` and `--lifecycle-store ID`
- WHEN argument and root resolution validation runs
- THEN Cairn MUST reject the command before reading lifecycle content or mutating state.

#### Scenario: Native root wins over a project pointer

r[cairn.lifecycle_stores.root_resolution.native_precedence]
- GIVEN the current VCS boundary contains native `cairn/` content and a checked external-store pointer
- WHEN Cairn resolves an implicit root
- THEN Cairn MUST select the nearest native root
- AND it MUST report the pointer as stale without following or changing it.

#### Scenario: Discovery does not cross an unknown boundary

r[cairn.lifecycle_stores.root_resolution.boundary]
- GIVEN the current directory has no known VCS boundary and no explicit selector
- WHEN Cairn resolves a root
- THEN it MUST inspect only the current directory before using the classic fallback
- AND it MUST NOT select an unrelated ancestor lifecycle root.

### Requirement: Store-qualified CLI syntax preserves existing flags

r[cairn.lifecycle_stores.root_cli] Policy-listed root-local commands MUST accept `--lifecycle-store ID`. Existing `--root PATH` and command-specific `--store PATH` meanings MUST remain unchanged.

#### Scenario: Store-qualified spec fetch is explicit

r[cairn.lifecycle_stores.root_cli.spec_fetch]
- GIVEN a registered lifecycle store contains an accepted spec
- WHEN the operator runs `cairn spec show <spec-id> --lifecycle-store <store-id>`
- THEN Cairn MUST read only the selected store and report the named root-selection source.

#### Scenario: Existing backing-store option remains local

r[cairn.lifecycle_stores.root_cli.compatibility]
- GIVEN a collaboration or global-registry command uses its existing `--store PATH` option
- WHEN argument parsing runs after lifecycle-store support is added
- THEN Cairn MUST preserve the existing backing-file meaning
- AND it MUST NOT reinterpret that path as a lifecycle-store ID.

### Requirement: Project configuration uses checked typed declarations

r[cairn.lifecycle_stores.project_config] Cairn MUST support typed project declarations for one external store pointer and direct lifecycle-store references. Normal commands MUST consume checked runtime projections and reject source-projection drift.

#### Scenario: Pointer selects an external store without local lifecycle content

r[cairn.lifecycle_stores.project_config.pointer]
- GIVEN a repository has no native lifecycle root and has a current checked pointer to a registered store
- WHEN a supported root-local command resolves implicitly within that repository
- THEN Cairn MUST select the named store and report `project_pointer` as the resolution source.

#### Scenario: Invalid project projection fails closed

r[cairn.lifecycle_stores.project_config.invalid]
- GIVEN project source or projection is malformed, stale, duplicated, self-referential, or outside policy bounds
- WHEN pointer or reference loading runs
- THEN Cairn MUST emit deterministic project-configuration diagnostics
- AND it MUST NOT select or index the declared stores.

### Requirement: References are direct and read-only

r[cairn.lifecycle_stores.references] Cairn MUST resolve only direct declared store references for context and health inspection. Reference presence MUST NOT affect lifecycle, dependency, evidence, requirement, gate, or release decisions.

#### Scenario: Direct references produce a bounded accepted-spec index

r[cairn.lifecycle_stores.references.index]
- GIVEN a checked project declaration names registered current stores with accepted specs
- WHEN Cairn builds working context
- THEN it MUST emit a sorted bounded index of store identities, spec titles, purpose summaries, and exact fetch commands
- AND it MUST NOT read referenced active changes or transitive references.

#### Scenario: References do not affect lifecycle authority

r[cairn.lifecycle_stores.references.non_authority]
- GIVEN a referenced store contains matching requirement names, passing receipts, or ready changes
- WHEN local validation, gates, sync, archive, dependency checks, or release readiness run
- THEN those commands MUST ignore the reference as authority
- AND explicit existing evidence inputs MUST remain necessary.

### Requirement: Context and doctor report root and reference health

r[cairn.lifecycle_stores.context] Cairn MUST expose read-only context and doctor results that bind the selected root, resolution source, store identity, direct references, findings, and exact repair actions.

#### Scenario: Healthy context is inspectable

r[cairn.lifecycle_stores.context.healthy]
- GIVEN a selected root and all direct references are current and registered
- WHEN the operator runs context or doctor
- THEN Cairn MUST emit deterministic sorted health facts and accepted-spec fetch commands
- AND no repository, locator, workset, VCS, or network mutation may occur.

#### Scenario: Missing reference provides a bounded repair action

r[cairn.lifecycle_stores.context.missing]
- GIVEN a direct reference is not registered on the current machine
- WHEN doctor checks reference health
- THEN Cairn MUST report the missing store and one exact manual register action
- AND it MAY report a quoted manual clone hint only when a bounded remote hint exists.

### Requirement: Store and context commands do not own VCS transport

r[cairn.lifecycle_stores.transport_boundary] Store, context, doctor, reference, and workset commands MUST NOT clone, fetch, pull, push, branch, merge, open pull requests, or infer remote freshness. Opener execution MUST require a reviewed policy profile and explicit `--execute`.

#### Scenario: Store setup never runs VCS transport

r[cairn.lifecycle_stores.transport_boundary.store]
- GIVEN setup or registration includes remote metadata
- WHEN the command previews or executes
- THEN Cairn MUST treat the remote as bounded informational text
- AND it MUST NOT execute VCS, network, credential, or hosting-service operations.

#### Scenario: Context never repairs a missing store automatically

r[cairn.lifecycle_stores.transport_boundary.context]
- GIVEN doctor or context observes a missing referenced store with a remote hint
- WHEN health inspection completes
- THEN Cairn MUST return a manual repair action only
- AND it MUST NOT create directories, clone repositories, or change locator state.

### Requirement: Worksets are personal non-authoritative folder groups

r[cairn.lifecycle_stores.worksets] Cairn MUST model a workset as local state with an ID, ordered unique members, one primary member, and one policy-known opener. Worksets MUST NOT select lifecycle roots or change member content.

#### Scenario: Workset open requires explicit execution

r[cairn.lifecycle_stores.worksets.open]
- GIVEN a valid workset and supported opener profile
- WHEN the operator runs workset open without `--execute`
- THEN Cairn MUST emit the exact argv and artifact plan without launching a process
- AND explicit execution MUST recheck workset, path, profile, and plan identities.

#### Scenario: Arbitrary opener commands are rejected

r[cairn.lifecycle_stores.worksets.opener_reject]
- GIVEN local workset state contains an unknown opener, shell text, interpolation, or changed member path
- WHEN workset planning or execution runs
- THEN Cairn MUST reject the workset with deterministic diagnostics
- AND it MUST NOT launch a process or change any member.

#### Scenario: Workset removal preserves members

r[cairn.lifecycle_stores.worksets.remove]
- GIVEN a valid workset references existing store and code repositories
- WHEN the operator executes workset removal
- THEN Cairn MUST remove only the local workset row
- AND it MUST NOT delete, close, modify, or unregister any member.

### Requirement: Store decisions preserve the functional-core boundary

r[cairn.lifecycle_stores.functional_boundary] Cairn MUST keep identity admission, state transitions, root resolution, reference indexing, context results, opener planning, and hash material pure. The CLI MUST own files, paths, locks, Nickel export, process launch, output, and exits.

#### Scenario: Pure decisions remain in the functional core

r[cairn.lifecycle_stores.functional_boundary.pure]
- GIVEN normalized store, project, locator, reference, and workset values
- WHEN Cairn evaluates admission, resolution, context, or a transition plan
- THEN the result MUST be deterministic without filesystems, environments, clocks, processes, networks, mutation, or output effects.

#### Scenario: Shell rechecks every effect

r[cairn.lifecycle_stores.functional_boundary.shell]
- GIVEN a reviewed locator, setup, configuration-export, workset, or opener plan
- WHEN the operator requests execution
- THEN the shell MUST reload and recheck exact inputs under the applicable lock before any effect
- AND a mismatch MUST block all planned effects.

### Requirement: Receipts distinguish local display and portable identity

r[cairn.lifecycle_stores.receipts] Store, root, context, doctor, and workset receipts MUST bind local display facts in full receipt hashes. Separate portable hashes MUST exclude absolute host paths and bind semantic IDs, identities, actions, policy, and non-claims.

#### Scenario: Local display paths do not change portable identity

r[cairn.lifecycle_stores.receipts.portable]
- GIVEN equivalent store and workset state exists under different safe checkout paths
- WHEN Cairn emits local receipts
- THEN full receipt hashes MAY differ because local path facts differ
- AND portable hashes MUST match when semantic identities and actions match.

#### Scenario: Local receipt is not promoted to release evidence

r[cairn.lifecycle_stores.receipts.local_only]
- GIVEN a valid locator or workset receipt contains local display paths
- WHEN Cairn reports its evidence class
- THEN it MUST label the receipt local-only unless an explicit later profile normalizes and validates it
- AND release readiness MUST NOT consume it by default.

### Requirement: Lifecycle-store claims remain bounded

r[cairn.lifecycle_stores.boundary] Store, pointer, reference, context, doctor, and workset outputs MUST state their bounded claims. They MUST NOT claim remote freshness, ownership, trust, requirement truth, implementation, dependency satisfaction, lifecycle readiness, editor state, or release eligibility.

#### Scenario: Reference or workset output overclaims authority

r[cairn.lifecycle_stores.boundary.overclaim]
- GIVEN a context, doctor, or workset result labels discovery or launch state as trusted, satisfied, ready, or released
- WHEN output validation runs
- THEN Cairn MUST reject the claim promotion with a stable diagnostic.

#### Scenario: Bounded result remains valid

r[cairn.lifecycle_stores.boundary.bounded]
- GIVEN a result reports only observed local identity, availability, index, plan, or process facts with required non-claims
- WHEN output validation runs
- THEN Cairn MAY report the bounded result as valid
- AND existing domain validators MUST retain all authority.

### Requirement: Lifecycle stores have complete verification and documentation

r[cairn.lifecycle_stores.verification] The change MUST include positive and negative policy, core, CLI, state, path, resolution, reference, workset, transport, receipt, and claim fixtures. It MUST pass focused and repository lifecycle checks before archive.

#### Scenario: Positive and negative suites close the change

r[cairn.lifecycle_stores.verification.suites]
- GIVEN valid fixtures and malformed, stale, conflicting, unsafe, ambient, transport, and overclaim fixtures
- WHEN focused tests and repository gates run
- THEN valid cases MUST produce deterministic passing results
- AND invalid cases MUST fail with stable diagnostics and no partial mutation.

#### Scenario: Documentation preserves the source and claim boundaries

r[cairn.lifecycle_stores.docs]
- GIVEN an operator reads the lifecycle-store guide
- WHEN setup, registration, pointers, references, context, doctor, worksets, precedence, and file locations are described
- THEN the guide MUST identify the OpenSpec stores beta guide as interaction-model inspiration
- AND it MUST preserve Cairn layout, configuration, command, transport, evidence, and authority boundaries.
