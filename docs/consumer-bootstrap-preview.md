# Consumer bootstrap

Cairn maintainers own the setup command, bootstrap contracts, and consumer fixtures.
Setup installs a checked policy and project-local generated guidance. It does not install the CLI executable.

## Setup procedure

Both roots must exist. The consumer root can be empty.
Use the exact CLI artifact that the source bundle declares.

```text
cairn setup plan --root CONSUMER --source-root SOURCES --bundle bundle.json
cairn setup apply --root CONSUMER --source-root SOURCES --bundle bundle.json
cairn setup apply --root CONSUMER --source-root SOURCES --bundle bundle.json --execute --plan-hash HASH
cairn setup check --root CONSUMER
cairn validate --root CONSUMER
cairn doctor --root CONSUMER
```

First, review the preview paths, identities, ownership actions, and guidance findings.
Then, replace `HASH` with the preview's exact `plan_hash`.
Without `--execute`, apply returns a preview and writes nothing.
The separate `artifact_plan_hash` is not an execution token.

The execution token binds the root's device and inode, source observations, prior selection, desired manifest, and generated-workflow plan.
Apply acquires `.cairn-bootstrap.lock` and rebuilds that plan before publication.
Source, policy, destination, or ownership drift blocks execution.
The lock serializes cooperating Cairn writers. It does not constrain other processes with equal filesystem authority.

## Source bundle

Author the bundle with `contracts/consumer-bootstrap.ncl` and export JSON before runtime use.
`fixtures/consumer-bootstrap/source-bundle.ncl` shows the shape. Its identities are contract fixtures, not installation identities.
The executable replay in `crates/cairn-cli/tests/bootstrap_preview/apply.rs` constructs exact identities for its built CLI.
Its saved export observations are controlled fixtures, not evidence of evaluator execution.

The bundle declares:

- `cohort`: tool, policy, export, evaluator, source, workflow, compatibility, and bounds.
- `export_request`: one saved Nickel Export request.
- `export_manifest`: the saved manifest for exactly one JSON policy export.
- `workflow_source`: a checked project-local guidance declaration.
- `policy_locator`: the checked policy destination.
- `artifacts`: explicit source-to-destination mappings.

The guidance declaration has this shape:

```json
{"schema":"cairn.bootstrap-guidance.v1","adapter":"agent-skills","destination":".agents/skills"}
```

`fixtures/consumer-bootstrap/guidance.ncl` is its maintained Nickel source example.
The existing generated-workflow planner owns adapter admission, rendering, manifests, and freshness categories.
Bootstrap includes its derived files in the installation manifest and execution identity.
The cohort's byte and artifact bounds also limit the complete installation, including generated guidance and retained generations.

All locators are bounded relative paths. Bootstrap metadata and stage names are reserved.
The policy mapping must name the export request's output and contain those exact bytes.
The selected policy must admit the cohort's workflow profile.

Cohort identities use bare BLAKE3 strings:

| Field | Observed bytes |
|---|---|
| `tool` | Running CLI artifact |
| `policy` | Export output |
| `export` | Exact saved manifest |
| `evaluator` | Descriptor serialized with `serde_json::to_vec` |
| `source` | Root export source |
| `workflow` | Guidance declaration |
| `managed_locators` | Explicit artifact destination bytes, in mapping order |

Cairn rebuilds the export manifest from bounded observations and compares producer freshness identities.
Normal commands do not evaluate Nickel, access the network, or scan home directories for guidance.
A matching saved manifest does not prove that an evaluator ran or that the policy is correct.

## Policy selection

CLI validation, doctor, and gates prefer explicit `--policy PATH` over the project declaration.
An invalid explicit selection fails without fallback.
The project declaration, `.cairn/bootstrap/selection.json`, binds the exact complete manifest and selected policy bytes.
Its locator resolves beneath the selected root, not the caller's working directory.
Only an absent declaration permits the existing working-directory default.
Other command families and MCP retain their existing policy loaders.

The complete manifest records managed artifact paths and identities.
Consumer-owned files outside that manifest remain untouched.
Doctor and setup check report generated-workflow freshness only for the declared project-local destination.
Modified, missing-managed, unmanaged, and symlinked guidance block automatic replacement.
Their existing generated-workflow findings or bootstrap ownership errors identify the manual repair category.

## Upgrade

Prepare a new pinned bundle whose `upgrade_from` includes the installed cohort identity.
Run the same plan, reviewed apply, check, and validation commands.
The preview reports `operation: upgrade` and exact desired identities.

A changed selected policy needs a new generation locator.
Bootstrap rejects replacement of the currently selected policy bytes before the final pointer switch.
Old generation artifacts remain owned and checked. Bootstrap does not prune them or migrate consumer-owned policy.
Modified managed files require manual reconciliation before another plan can pass.

## Interrupted publication and recovery

Apply publishes `.cairn/bootstrap/prepared.json` before managed artifacts.
Each file uses the pinned Durable File Publication mechanism with required parent synchronization and bounded stage creation.
The complete manifest follows the artifacts. The selection pointer follows the complete manifest.
Only successful publication permits removal of prepared state and release of the lock.

```text
cairn setup recover --root CONSUMER
```

Recovery is diagnostic and non-mutating. There is no automatic resume, rollback, deletion, or synchronization retry.
Prepared state or a retained lock yields `bootstrap.recovery.manual_repair` with `valid: false` and `blocked: true`.
Checks do not infer durable completion from a visible rename or matching bytes.

If recovery is required, stop competing writers.
Preserve the lock, prepared record, source bundle, existing manifest, and policy generations for review.
Compare the recorded identities with the intended plan and observed files.
Restore an independently checked complete generation through the repository's reviewed recovery procedure.
Do not remove a lock merely because an apply process returned an error.
Do not delete unowned files to make a plan pass.

Manual recovery remains an operator responsibility. The command does not certify a manual repair or grant deletion authority.
A process can die after an artifact rename and before the final manifest. That state remains blocked.
Unknown synchronization and failed publication retain recovery state, not a successful installation receipt.

## Bounds and compatibility

The source read budget is 64 MiB, including repeated reads. Each ordinary artifact has a 16 MiB maximum.
Bundle and request reads have a 64 KiB maximum. CLI identity uses a separate 512 MiB streaming bound.
Cap Root admits lexical paths. Capability-relative no-follow operations retain filesystem authority in the shell.
The publication adapter targets Linux. Unsupported publication mechanics are not replaced with weaker behavior.

Low-level `cairn init` still creates lifecycle directories only. It does not install policy or guidance.
A low-level initialized root without policy retains the original policy-gap failure.
An explicit valid policy remains the positive control.
Legacy-only lifecycle roots still require explicit migration.

A successful apply receipt records local publication observations only.
Setup check reports current identities and guidance freshness, not historical durability, policy correctness, or release eligibility.
