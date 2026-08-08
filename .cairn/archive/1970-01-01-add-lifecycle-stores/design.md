# Design: Lifecycle stores, references, context, and worksets

## Context

Cairn currently selects repository state through `--root PATH`, with `.` as the default. Several unrelated command families already use `--store PATH` for a backing artifact.

Cairn also has an explicit global registry. Its accepted contract forbids silent use by root-local lifecycle commands.

The OpenSpec stores beta guide combines four ideas:

- a standalone planning repository with a stable name;
- committed project pointers and read-only references;
- read-only context and health inspection; and
- personal worksets for related folders.

Cairn can adapt these ideas without adopting OpenSpec layout, YAML configuration, ambient defaults, or hidden VCS behavior.

## Goals

- Make a native Cairn lifecycle repository addressable by a stable local ID.
- Preserve explicit root selection and deterministic source reporting.
- Let repositories declare direct read-only lifecycle references.
- Give agents one bounded command for discovering referenced accepted specs.
- Let operators keep personal folder worksets without adding lifecycle meaning.

## Non-Goals

- Synchronize VCS repositories or validate remote ownership.
- Make local locator state authoritative lifecycle evidence.
- Let reference presence satisfy a dependency or gate.
- Add transitive reference traversal.
- Add arbitrary shell command configuration.

## Decisions

### Decision: Define a lifecycle store as a native Cairn repository

**Choice:** A lifecycle store contains a native `cairn/` root and these identity artifacts:

```text
.cairn-store/store.ncl
.cairn-store/store.json
cairn/specs/
cairn/changes/
cairn/archive/
```

The Nickel file is the authored source. The JSON file is the checked runtime projection.

The identity includes a schema, store ID, optional remote hint, required non-claims, and a BLAKE3 source identity. Named policy bounds limit IDs and remote hints.

Normal lifecycle commands read only the checked JSON projection. This explicit command evaluates Nickel and detects stale projections:

```text
cairn config export --kind store|project --root PATH [--check] [--execute]
```

The default mode previews the projection. `--check` detects drift without mutation. `--execute` rechecks inputs and atomically writes one projection.

**Rationale:** This preserves Nickel authoring without adding runtime Nickel execution to ordinary lifecycle commands.

A store remains an ordinary VCS repository. Cairn does not initialize VCS, create commits, or contact a remote.

### Decision: Keep local locator state separate from the global registry

**Choice:** Store registration uses canonical local state at this logical location:

```text
<data-dir>/cairn/stores/registry.json
```

The state maps one store ID to one normalized checkout path and one exact store identity. It is machine-local locator data, not authored policy or lifecycle evidence.

Registry mutations use preview-first plans, an expected prior BLAKE3 state hash, an exclusive lock, and atomic replacement. Duplicate IDs, duplicate paths, stale plans, symlinks, and changed identities fail before replacement.

**Rationale:** Named local lookup needs machine paths. The accepted global registry has different catalog and authority boundaries.

The locator is read only after an explicit named selector or committed project pointer requires it. Root-local commands do not consult it otherwise.

### Decision: Use `--lifecycle-store`, not `--store`

**Choice:** Policy lists the root-local lifecycle commands that accept:

```text
--root PATH
--lifecycle-store ID
```

The options are mutually exclusive. Existing meanings of `--store PATH` remain unchanged for collaboration and registry commands.

**Rationale:** Reusing `--store` creates an ambiguous global flag and breaks existing command contracts.

### Decision: Resolve roots with bounded and visible precedence

**Choice:** The shell collects bounded path and local-state facts. A pure resolver then applies this order:

1. Use an explicit `--root PATH` or `--lifecycle-store ID` selector.
2. Use the nearest native `cairn/` root within the current VCS boundary.
3. Use the nearest checked `.cairn/project.json` store pointer within that boundary.
4. Use the current directory under the existing classic fallback.

If both explicit selectors occur, resolution fails. If no VCS boundary exists, implicit discovery inspects only the current directory.

A native root wins over a project pointer. Cairn reports the stale pointer but does not change or follow it.

No machine-wide default store exists in this change. The resolver does not read global-registry state.

Every root-local receipt gains a stable resolution block. It names the source, store ID when present, store identity, project identity, and diagnostics.

**Rationale:** Explicit selectors remain strongest. Bounded discovery helps subdirectory use without crossing an unrelated repository boundary.

### Decision: Author project pointers and references in typed Nickel

**Choice:** A code repository can contain:

```text
.cairn/project.ncl
.cairn/project.json
```

The checked projection can declare one external lifecycle store pointer and a sorted set of direct references. Each reference contains a store ID and an optional remote hint.

`cairn config export --kind project` writes or checks the JSON projection through a preview-first flow. Normal commands reject missing, malformed, or stale projections.

A repository with native `cairn/` content remains local. Its pointer does not override that root.

**Rationale:** Shared declarations belong in the repository. Machine paths do not.

### Decision: Keep references direct, read-only, and non-authoritative

**Choice:** `cairn context` resolves only direct references through the local locator. It indexes accepted specs and returns each title, purpose summary, store identity, and exact fetch command.

The fetch form is:

```text
cairn spec show <spec-id> --lifecycle-store <store-id>
```

`cairn doctor` reports selected-root health and direct-reference health. Missing registration can include a bounded manual clone hint when a remote hint exists.

Reference indexing does not read active changes. It does not recursively load references from referenced stores.

References never affect validation, gates, sync, archive, release readiness, dependency evaluation, requirement satisfaction, or evidence status.

**Rationale:** Reference discovery helps operators and agents. It cannot become implicit authority.

### Decision: Make setup and registration preview-first

**Choice:** Add these commands:

```text
cairn store setup ID --path PATH [--remote VALUE] [--execute]
cairn store register PATH [--id ID] [--execute]
cairn store unregister ID [--execute]
cairn store list
cairn store show ID
cairn store doctor [ID]
```

`setup` plans native lifecycle and identity artifacts. It never initializes VCS or performs transport.

`register` checks the store source, runtime projection, native root, identity, path safety, and local locator conflicts. `unregister` removes only the locator row.

All mutation execution rechecks exact inputs and the expected prior locator hash under the lock.

**Rationale:** Store commands change repository or machine state. Cairn must expose each effect before execution.

### Decision: Model worksets as personal effect plans

**Choice:** Worksets use canonical local state at this logical location:

```text
<data-dir>/cairn/worksets/state.json
```

A workset contains an ID, an ordered non-empty member set, one primary member, and one policy-known opener profile.

Add these commands:

```text
cairn workset create ID --member PATH [--member PATH ...] --opener ID [--execute]
cairn workset list
cairn workset show ID
cairn workset remove ID [--execute]
cairn workset open ID [--opener ID] [--execute]
```

`open` emits an exact argv and artifact plan by default. `--execute` launches only a policy-known opener through the imperative shell.

Opener profiles use named bounds and exact argv construction. They do not use shell interpolation or arbitrary executable text from local state.

Removing a workset changes only local workset state. It never deletes, changes, or closes member repositories.

**Rationale:** Personal folder groups improve operator flow. They have no lifecycle or authority meaning.

### Decision: Separate portable identity from local display facts

**Choice:** Store, context, doctor, and workset receipts identify local paths as display facts. Their full receipt hashes bind those local facts.

A separate portable hash binds store IDs, store identities, project identities, reference IDs, member labels, policies, actions, and non-claims. It excludes absolute host paths.

Receipts state whether they are local-only or portable. Local locator and workset receipts do not become release evidence by default.

**Rationale:** Operators need exact local paths. Cross-machine identity must not depend on home-directory layout.

### Decision: Preserve the functional-core boundary

**Choice:** `cairn-core` owns these pure operations:

- identity and projection admission;
- locator and workset transition planning;
- root resolution over supplied observations;
- direct-reference normalization and indexing;
- context and doctor result construction;
- opener plan construction; and
- canonical BLAKE3 identity material.

`cairn-cli` owns path discovery, VCS-boundary observation, capability roots, locks, atomic writes, Nickel export, process launch, JSON output, and exit status.

**Rationale:** All decisions remain testable without files, processes, environment access, clocks, or networks.

## Rejected Alternatives

### Reuse `--store`

This conflicts with existing collaboration and global-registry meanings. The change uses `--lifecycle-store`.

### Use the global registry as the locator

This introduces ambient cross-domain state into root-local commands. The change uses a narrow local locator only when a declaration requests it.

### Use paths only

Paths preserve explicitness but do not provide portable shared names. Store IDs provide names while registrations keep paths local.

### Add a global default store

A default hides the root source in ambient machine state. The first Cairn slice omits this behavior.

## Test Design

Positive tests cover setup, registration, explicit selectors, bounded native-root discovery, project pointers, direct references, context indexing, doctor output, workset plans, and opener execution.

Negative tests cover:

- duplicate IDs and duplicate checkout paths;
- conflicting explicit selectors;
- missing, moved, or symlinked paths;
- malformed or stale Nickel projections;
- changed store identity after registration;
- stale locator and workset write plans;
- local-root and stale-pointer coexistence;
- unknown stores and duplicate or self references;
- unsafe remote text and unsafe path components;
- reference claim promotion and transitive traversal;
- accidental global-registry consultation;
- hidden VCS or network execution;
- arbitrary opener text and shell interpolation;
- workset execution without `--execute`; and
- deletion or mutation of workset members.

Cross-root fixtures compare portable hashes while local display paths differ. Positive and negative suites assert deterministic issue ordering and no partial mutation.

## Risks and Trade-offs

- Root resolution changes default behavior in repository subdirectories. Receipts always name the selected source and root.
- Checked projections add one generated artifact per authored configuration. Explicit freshness checks prevent silent source drift.
- Local registries can become stale after a checkout moves. Doctor reports a bounded unregister and register repair path.
- Store references can look authoritative. Context and receipt text keep discovery separate from evidence and gates.
- Editor launch behavior varies. Cairn claims only the planned argv and observed process result.

## Claim Boundary

A valid store proves a bounded native lifecycle identity at one observed local path. Registration proves only one local locator mapping.

A valid reference index proves that Cairn read accepted spec metadata from the observed checkout. It does not prove truth, freshness, ownership, implementation, or satisfaction.

A workset receipt proves local member configuration and a bounded opener plan or process observation. It does not prove editor state or saved work.

No store, pointer, reference, context, doctor, or workset result proves dependency satisfaction, evidence truth, lifecycle readiness, or release eligibility.

## Source Adaptation

The interaction model adapts the OpenSpec stores beta user guide. Cairn retains its own native layout, configuration format, commands, policy, receipts, and claim boundaries.
