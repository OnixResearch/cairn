## Context

Cairn's license map grants `MIT OR Apache-2.0` to the reusable core, the gate
contract, templates and the five policy inputs named in
`cairn-policy/LICENSING.md`. The consumer-facing typed Nickel contracts under
`contracts/` and the assurance-targets policy source are newer reusable
surfaces and have no explicit permissive grant; unmarked lifecycle material
defaults to `AGPL-3.0-or-later`.

Octet vendored `contracts/assurance-targets.ncl` from two Cairn revisions,
`3dc654d38404d595ff07ca994925789f45941ffe` and
`f20bcb4aa00268fdaa630cbce07633250f18a681`, and labels it permissively.
Terrane vendors the assurance-targets contract and policy source in the WP8
loop. A grant that named only the current bytes would leave those copies
ambiguous, and editing the vendored bytes to add a header would break their
content identity.

## Decisions

### One explicit grant for the consumer contract surface

**Choice:** Add one `MIT OR Apache-2.0` grant covering `contracts/**` and
`cairn-policy/assurance-targets.ncl`, at every revision of this repository and
for byte-for-byte copies elsewhere, naming the two revisions Octet vendored.

**Rationale:** The grant covers the *old* bytes as well as the current ones, so
consumers can choose MIT or Apache without changing the vendored content
identity. `contracts/**` is the consumer-facing typed Nickel surface; listing
every file individually would grow stale as contracts are added, so the
directory is the unit. The assurance-targets policy source has the same
consumer reuse profile and is named explicitly.

### Keep the grant narrow

**Choice:** Do not touch the existing rows or the historical five-file grant.
State that the new grant does not cover generated JSON exports, including
`cairn-policy/generated/assurance-targets.json`, any other `cairn-policy/` file,
the CLI, the exporter/toolchain, documentation or third-party material.

**Rationale:** A generated JSON export contains material produced by the
exporter/toolchain and has its own inventory treatment; generated content
should not inherit a hand-authored source grant. The rest of the boundary
already has reviewed terms, and broadening the grant beyond what consumers
vendored would relicense material the decision did not ask to relicense.

### Enforce the grant through the existing inventory

**Choice:** Add `contains` rules to `license-policy.tsv` for the new table row,
the covered paths and both vendored revisions in `docs/licensing.md` and
`cairn-policy/LICENSING.md`, and add the new requirement's verification marker
to `tools/check-license-boundary.rs`.

**Rationale:** The existing checker already validates the historical grant with
`contains` rules, so the new grant needs no new rule kind. The negative proof is
the same check on a scratch copy with the row removed.
