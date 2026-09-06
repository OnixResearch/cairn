# Consumer bootstrap preview

`cairn setup plan` is a read-only artifact preview. It does not install Cairn or authorize an installation.
Cairn maintainers own this command and the `bootstrap_preview` consumer test.

```text
cairn setup plan --root EXISTING_ROOT --source-root SOURCE_ROOT --bundle bundle.json
```

Both roots must exist. The bundle locator is relative to `SOURCE_ROOT`.
The command rejects `--execute`, unknown options, duplicate options, and missing values.
Apply, upgrades, installation checks, and recovery are not implemented by this command.

## Inputs

Author the bundle with `contracts/consumer-bootstrap.ncl` and its `source_bundle` contract. Export it to JSON before runtime use.
`fixtures/consumer-bootstrap/source-bundle.ncl` shows the structure. Its identities are contract fixtures, not usable installation identities.

The bundle declares:

- `cohort`: the existing checked bootstrap cohort.
- `export_request`: a saved Nickel Export request.
- `export_manifest`: a saved canonical Nickel Export manifest with exactly one JSON export.
- `workflow_source`: bytes whose identity matches the cohort's workflow field.
- `policy_locator`: the destination of the checked policy.
- `artifacts`: exact source-to-destination mappings.

All locators are bounded relative paths. Bootstrap state destinations are reserved.
The policy mapping must name the export request's output and contain those exact bytes.
The policy must admit the declared workflow profile.

Cohort identities are bare BLAKE3 strings:

| Field | Observed bytes |
|---|---|
| `tool` | The currently running CLI artifact |
| `policy` | The export output |
| `export` | The exact saved manifest file |
| `evaluator` | The descriptor serialized with `serde_json::to_vec` |
| `source` | The root export source |
| `workflow` | The declared workflow file |
| `managed_locators` | Each destination's UTF-8 bytes, in artifact order |

The producer's manifest retains its own tagged identities and schema.
Cairn rebuilds the manifest from bounded source, dependency, and output observations, then uses the published producer freshness comparison.
Cairn does not evaluate Nickel during preview. A matching saved manifest does not prove that an evaluator ran.
The producer's non-claim appears unchanged in `producer_non_claim`.

Source and destination reads use Cap Root lexical admission and capability-relative no-follow reads.
The source read budget is 64 MiB, including repeated reads. Each ordinary artifact has a 16 MiB maximum.
Bundle and request reads have a 64 KiB maximum. The running CLI identity has a separate 512 MiB streaming bound.
Cohort bounds can further restrict desired artifacts. No source or destination file is written.

## Results and limits

A valid preview includes exact desired identities, artifact actions, the cohort identity, bundle identity, and a receipt hash.
`artifact_plan_hash` identifies the pure artifact plan. It is not an apply token and does not bind installation root authority.
A future installation plan must bind the root grant, generated-guidance plan, complete state, and exact rechecks before mutation.

The result always reports:

```json
{
  "dry_run": true,
  "mutated": false,
  "installation_receipt": false,
  "execution_supported": false,
  "execution_blockers": ["bootstrap.setup.installation_shell_incomplete"]
}
```

An existing selection, complete manifest, or prepared state blocks this fresh-root preview.
Unmanaged destinations, stale exports, cohort drift, unsupported policy profiles, symlinks, and unsafe paths also block it.
Workflow bytes are identity-bound only. Generated-guidance compatibility and installation freshness remain unchecked.

## Project policy resolution

CLI validation, doctor, and gates select an explicit `--policy PATH` before a checked project declaration.
An invalid explicit selection cannot fall back. These commands report the selected source and exact policy identity.
The project declaration is `.cairn/bootstrap/selection.json`. It binds `.cairn/bootstrap/complete.json` and the selected policy bytes.
Only an absent declaration permits the existing working-directory default. Dangling symlinks and failed reads are not absence.
Other command families and MCP still use their existing policy loaders.

Low-level `cairn init` still creates lifecycle directories only. It does not install a policy or generated guidance.
Selection metadata and preview receipts do not prove durability, installation completion, consumer correctness, or release readiness.
