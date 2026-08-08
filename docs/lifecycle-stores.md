# Lifecycle stores, references, context, and worksets

A lifecycle store is an ordinary repository with a native Cairn lifecycle root. A local store ID gives the repository a stable machine-local name.

This interaction model takes inspiration from the OpenSpec stores beta guide. Cairn keeps its own layout, typed configuration, commands, receipts, and claim boundaries.

Cairn does not adopt the OpenSpec layout, YAML configuration, ambient defaults, or hidden transport behavior.

## Store layout

A lifecycle store contains these artifacts:

```text
.cairn-store/store.ncl
.cairn-store/store.json
.cairn/specs/
.cairn/changes/
.cairn/archive/
```

`.cairn-store/store.ncl` is the typed Nickel source. `.cairn-store/store.json` is the checked JSON projection for runtime use.

The identity binds these facts:

- the identity schema
- the store ID
- an optional bounded remote hint
- the required non-claims
- the BLAKE3 identity of the Nickel source

Normal lifecycle commands read only `.cairn-store/store.json`. They do not evaluate Nickel at runtime.

Use this command to export or check the store projection:

```text
cairn config export --kind store --root PATH [--check] [--execute]
```

The default mode prints a deterministic plan. It does not change the projection.

Use `--check` to compare the current projection with the typed source. This mode is read-only and fails for a stale projection.

Use `--execute` to replace only `.cairn-store/store.json`. Cairn rechecks the source and destination before an atomic same-directory replacement.

This command evaluates the lifecycle-store Nickel contract. Normal lifecycle commands read JSON and do not evaluate Nickel.

## Set up a store

Store setup is preview-first. The planned syntax is:

```text
cairn store setup ID --path PATH [--remote VALUE] [--execute]
```

1. Preview the setup plan.

   ```bash
   cairn store setup shared-planning --path /srv/cairn/shared-planning
   ```

2. Review every planned path and identity.

3. Execute the unchanged plan.

   ```bash
   cairn store setup shared-planning --path /srv/cairn/shared-planning --execute
   ```

If the store needs an informational remote hint, add `--remote VALUE`.

Setup creates only the declared native lifecycle and identity artifacts. It does not initialize VCS, create commits, or contact a remote.

If a destination is unsafe or contains conflicting content, setup fails before mutation. The receipt binds the before and after BLAKE3 file identities.

## Register a store

Registration maps a store ID to one normalized local checkout path. The local locator does not become lifecycle authority.

The planned syntax is:

```text
cairn store register PATH [--id ID] [--execute]
cairn store unregister ID [--execute]
cairn store list
cairn store show ID
cairn store doctor [ID]
```

Preview registration before execution:

```bash
cairn store register /srv/cairn/shared-planning --id shared-planning
cairn store register /srv/cairn/shared-planning --id shared-planning --execute
```

Registration validates these inputs:

- the typed store source
- the current JSON projection
- the native `.cairn/` root
- the exact store identity
- path safety
- conflicts in the local locator

Duplicate IDs, duplicate paths, symlinks, stale plans, and changed identities fail before replacement. The last valid locator state remains unchanged.

List all registrations:

```bash
cairn store list
```

The list is deterministic and sorted. It reports each registered ID and its local display path.

Show one registration:

```bash
cairn store show shared-planning
```

The result reports the local mapping, exact store identity, and current projection facts.

Inspect one store or the complete locator:

```bash
cairn store doctor shared-planning
cairn store doctor
```

Store doctor reports path safety, native-root presence, projection freshness, identity drift, conflicts, and manual repair actions.

Unregister a moved or retired checkout with a preview and an execution:

```bash
cairn store unregister shared-planning
cairn store unregister shared-planning --execute
```

Unregister removes only the local locator row. It does not remove repository files, lifecycle content, or VCS state.

## Local state locations

Cairn keeps locator and workset state below its platform data directory.

```text
<data-dir>/cairn/stores/registry.json
<data-dir>/cairn/worksets/state.json
```

`registry.json` maps each store ID to one normalized checkout path and one exact identity. It is machine-local locator data.

`state.json` contains personal workset definitions. It is not repository configuration.

Cairn uses an exclusive lock, an expected prior BLAKE3 state hash, and atomic replacement for each local-state mutation.

Do not edit these JSON files to repair normal state. Use the preview-first store and workset commands.

These files are separate from the Cairn global registry. Root-local commands do not read the global registry for store selection.

## Select a lifecycle root

Supported root-local commands use these selectors:

```text
--root PATH
--lifecycle-store ID
```

The selectors are mutually exclusive. Cairn rejects a command that contains both selectors before it reads lifecycle content.

Existing `--store PATH` options keep their command-specific meanings. Cairn does not reinterpret them as lifecycle-store IDs.

### Selector precedence

Cairn resolves a lifecycle root in this order:

1. Use an explicit `--root PATH` or `--lifecycle-store ID` selector.
2. Use the nearest native `.cairn/` root inside the current VCS boundary.
3. Use the nearest checked `.cairn/project.json` store pointer inside that boundary.
4. Use the current directory through the classic fallback.

If no VCS boundary exists, implicit discovery inspects only the current directory. It does not select an unrelated ancestor root.

A native root wins over a project pointer. Cairn reports the pointer as stale, but it does not follow or change it.

No machine-wide default lifecycle store exists. Root resolution does not consult the global registry.

Each root-local receipt reports the resolution source and selected identity. If related facts exist, it reports the store ID, project identity, and diagnostics.

For example, fetch a spec from one named store with this exact form:

```text
cairn spec show <spec-id> --lifecycle-store <store-id>
```

## Add a project pointer

A code repository can point to one external lifecycle store. It can also declare direct read-only store references.

The project configuration uses these files:

```text
.cairn/project.ncl
.cairn/project.json
```

`.cairn/project.ncl` is the typed Nickel source. `.cairn/project.json` is the checked runtime projection.

The project source can declare these values:

- one external lifecycle-store pointer
- a sorted set of direct references with store IDs and optional remote hints

Keep machine paths out of the project files. Each machine resolves shared store IDs through its local locator.

Use this command after each project source change:

```text
cairn config export --kind project --root PATH [--check] [--execute]
```

Preview the export without flags. Use `--check` for a read-only freshness check. Use `--execute` to replace only `.cairn/project.json`.

Normal commands reject a missing, malformed, or stale projection. They do not evaluate Nickel.

If the repository has no native lifecycle root, a pointer can select an external store. The resolution receipt reports `project_pointer` as its source.

Duplicate, self-referential, unsafe, malformed, or stale declarations fail closed. Cairn does not select or index those declarations.

## Use direct references

References give operators and agents read-only access to accepted spec metadata. They do not import lifecycle authority.

Cairn resolves only the direct references in the selected project configuration. It does not load references from a referenced store.

Reference indexing reads accepted specs only. It does not read active changes from a referenced store.

## Inspect context

Use `cairn context` with the applicable root selector:

```bash
cairn context --root PATH
cairn context --lifecycle-store ID
```

Context returns a deterministic sorted index with these facts:

- the selected root and resolution source
- the selected store and project identities
- each direct reference and its health
- each accepted spec title and purpose summary
- an exact store-qualified fetch command for each accepted spec

The fetch command has this form:

```text
cairn spec show <spec-id> --lifecycle-store <store-id>
```

Context is read-only. It does not change repositories, locator state, worksets, VCS state, or network state.

## Inspect root and reference health

Use root doctor for the selected root and its direct references:

```bash
cairn doctor --root PATH
cairn doctor --lifecycle-store ID
```

Root doctor reports these facts:

- the selected root
- the resolution source
- the store and project identities
- root health findings
- sorted direct-reference health findings
- exact manual repair actions

A healthy reference reports the observed registered store identity and accepted-spec index. A missing reference reports the missing store ID.

If a bounded remote hint exists, doctor can include one quoted manual clone hint. Doctor never runs the hint.

Store doctor focuses on local locator health:

```bash
cairn store doctor
cairn store doctor ID
```

It reports store IDs, local paths, exact identities, native-root health, projection freshness, and locator conflicts. It includes manual repair actions for invalid entries.

Store doctor does not repair entries automatically.

## Repair local state manually

### Repair a moved checkout

1. Preview the removal of the stale locator row.

   ```bash
   cairn store unregister ID
   ```

2. Execute the reviewed removal.

   ```bash
   cairn store unregister ID --execute
   ```

3. Preview registration at the new path.

   ```bash
   cairn store register NEW_PATH --id ID
   ```

4. Execute the reviewed registration.

   ```bash
   cairn store register NEW_PATH --id ID --execute
   ```

### Repair a missing referenced store

Cairn reports the exact register action. If a remote hint exists, Cairn can also report a quoted clone hint.

Run transport yourself, outside Cairn:

```bash
git clone '<remote-hint>' '<checkout-path>'
```

Then preview and execute registration:

```bash
cairn store register '<checkout-path>' --id '<store-id>'
cairn store register '<checkout-path>' --id '<store-id>' --execute
```

### Repair a stale projection

1. Update the typed `.ncl` source.
2. Preview `cairn config export --kind store|project --root PATH`.
3. Run the same command with `--execute`.
4. Run the same command with `--check`.
5. Rerun the failed store, context, or doctor command.

Do not repair drift by editing the generated JSON projection or local locator directly.

## Use worksets

A workset is a personal ordered group of repository folders. It has one ID, one primary member, and one policy-known opener profile.

Worksets do not select lifecycle roots. They do not add lifecycle meaning to their members.

The planned syntax is:

```text
cairn workset create ID --member PATH [--member PATH ...] --opener ID [--execute]
cairn workset list
cairn workset show ID
cairn workset remove ID [--execute]
cairn workset open ID [--opener ID] [--execute]
```

Preview and create a workset:

```bash
cairn workset create shared-change \
  --member /srv/cairn/shared-planning \
  --member /srv/code/service-a \
  --opener editor

cairn workset create shared-change \
  --member /srv/cairn/shared-planning \
  --member /srv/code/service-a \
  --opener editor \
  --execute
```

The preview reports the ordered members, primary member, opener profile, actions, and identities. Review these facts before execution.

List or show worksets:

```bash
cairn workset list
cairn workset show shared-change
```

Preview an opener plan:

```bash
cairn workset open shared-change
```

The preview emits the exact argv and artifact plan. It does not launch a process.

Execute the reviewed opener plan:

```bash
cairn workset open shared-change --execute
```

Cairn rechecks the workset, member paths, opener profile, and plan identities before launch.

An `--opener ID` value can select another policy-known profile. Local state cannot contain arbitrary executable text or shell interpolation.

Preview and remove a workset:

```bash
cairn workset remove shared-change
cairn workset remove shared-change --execute
```

Removal deletes only the local workset row. It does not delete, change, close, or unregister a member.

## Receipt identities

Store, context, doctor, root, and workset receipts bind local paths in their full receipt hashes.

A separate portable hash excludes absolute host paths. It binds semantic IDs, identities, policies, actions, member labels, references, and required non-claims.

Equivalent semantic state can have the same portable hash on two machines. The full receipt hashes can differ because the local paths differ.

Locator and workset receipts are local-only by default. Release readiness does not consume them as release evidence.

## Claim boundaries

### Transport non-claims

Cairn does not initialize VCS or create commits for a store. It does not clone, fetch, pull, push, branch, merge, or open pull requests.

Store commands do not contact networks, credential stores, or hosting services. A remote hint is bounded informational text only.

Cairn does not infer remote freshness, remote ownership, or peer state. Context and doctor return manual repair actions only.

Workset open launches only a reviewed policy-known opener after explicit `--execute`. It does not run arbitrary operator-configured shell text.

### Evidence non-claims

A valid store identifies one observed native lifecycle root at one local path. It does not prove that the lifecycle content is true or complete.

Registration proves only one local locator mapping. Locator state is not authored policy, lifecycle evidence, or global-registry authority.

A reference index proves only that Cairn read accepted spec metadata from an observed checkout. It does not prove freshness, ownership, implementation, or satisfaction.

A context or doctor receipt reports bounded local observations. It does not prove verifier correctness, evidence truth, or semantic review completeness.

A workset receipt proves a local member configuration and an opener plan or process observation. It does not prove editor state or saved work.

### Authority non-claims

A project pointer selects a root. It does not grant trust, policy authority, or repository ownership.

References are not dependencies, evidence inputs, requirements, policies, trust statements, or approval records. Referenced content cannot satisfy local requirements.

References do not affect validation, gates, sync, archive, dependency checks, evidence status, or lifecycle readiness. Cairn does not traverse transitive references.

Worksets do not select roots, change member content, or grant authority. An opener result does not prove that every member opened or remained open.

### Release non-claims

No store, pointer, reference, context, doctor, or workset result proves release eligibility.

Passing receipts in a referenced store do not satisfy local release-readiness inputs. Ready changes in a referenced store do not make the selected project ready.

Local locator and workset receipts are not release evidence by default. Cairn does not create tags, publish releases, or upload evidence through these commands.

Existing lifecycle, evidence, and release validators retain all authority.

<!--
r[impl cairn.lifecycle_stores.docs]
r[related cairn.lifecycle_stores.identity]
r[related cairn.lifecycle_stores.locator]
r[related cairn.lifecycle_stores.commands]
r[related cairn.lifecycle_stores.root_resolution]
r[related cairn.lifecycle_stores.root_cli]
r[related cairn.lifecycle_stores.project_config]
r[related cairn.lifecycle_stores.references]
r[related cairn.lifecycle_stores.context]
r[related cairn.lifecycle_stores.transport_boundary]
r[related cairn.lifecycle_stores.worksets]
r[related cairn.lifecycle_stores.receipts]
r[related cairn.lifecycle_stores.boundary]
r[related cairn.lifecycle_stores.functional_boundary]
-->
