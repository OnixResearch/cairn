# License consumer contracts

## Why

Cairn already offers its reusable core, gate contract, templates and five named
policy inputs permissively, but its consumer-facing typed Nickel contracts and
`cairn-policy/assurance-targets.ncl` carry only the unmarked AGPL default. A
consumer that vendors those contracts — Terrane imports the assurance-targets
contract and policy source, and Octet vendored `contracts/assurance-targets.ncl`
at earlier revisions — therefore has no published permissive grant to cite.
Octet's existing permissive label for that file is accurate only if Cairn
publishes the grant.

## What Changes

- Add one explicit `MIT OR Apache-2.0` grant for `contracts/**` and
  `cairn-policy/assurance-targets.ncl` to the license map in
  `docs/licensing.md`.
- State in the prose that the grant covers those paths at every revision of this
  repository, including the copies Octet vendored at
  `3dc654d38404d595ff07ca994925789f45941ffe` and
  `f20bcb4aa00268fdaa630cbce07633250f18a681`, and byte-for-byte copies of those
  files in other repositories.
- Mirror the grant in `cairn-policy/LICENSING.md`.
- Add the grant's row, covered paths and vendored revisions to
  `license-policy.tsv` so `tools/check-license-boundary.rs` enforces them.
- Record the grant in `CHANGELOG.md`.

## Impact

- **Current consumers**: Terrane vendors the assurance-targets contract and
  policy source; Octet vendored `contracts/assurance-targets.ncl` at earlier
  revisions; any other repository that copies those exact files.
- **Immediate outcome**: those consumer-facing paths have a published permissive
  grant and a machine-checked inventory; Octet's existing label is accurate.
- **Durable contribution**: one scoped grant for the consumer contract surface
  instead of a repository-wide relicense.
- **Maintenance owner**: Cairn maintainers own the scoped grant and its
  inventory; consumers own how they cite it.
- **Files**: `docs/licensing.md`, `cairn-policy/LICENSING.md`,
  `license-policy.tsv`, `tools/check-license-boundary.rs` (requirement marker),
  `CHANGELOG.md`.
- **Testing**: the license-boundary script and its positive and negative
  transcript, `policy export --check`, `validate` and the lifecycle gates.

The grant does not extend to the generated JSON exports (including
`cairn-policy/generated/assurance-targets.json`), any other `cairn-policy/`
file, the CLI, the exporter/toolchain, repository documentation or third-party
material. Prior grants remain valid. This change renames no file, changes no
runtime behavior and adds no gate.
