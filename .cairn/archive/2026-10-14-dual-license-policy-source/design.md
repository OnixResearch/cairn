## Context

Cairn licenses its CLI under AGPL and its reusable core and templates under
`MIT OR Apache-2.0`. Unmarked lifecycle material defaults to AGPL. Terrane
imports four Nickel policy files plus a generated JSON projection from Cairn
commit `b6759012db4dab0158936e995b7bad39ac10465b` and checks their exact
Git blobs; changing those files to add headers would invalidate that identity.

## Decisions

### Grant a choice for five identified files

**Choice:** Put the explicit grant in `cairn-policy/LICENSING.md`; name each
file and its historical blob. Offer the same choices for the files currently
shipped in Cairn and byte-for-byte copies of the historical revision.

**Rationale:** A new license declaration that covers the *old* bytes is
needed; editing only Cairn's current defaults would leave the pinned consumer
ambiguous. Keep MIT and Apache terms aligned with the existing reusable core.

### Preserve other license boundaries

**Choice:** Reference the grant from `docs/licensing.md` and README, inventory
it in `license-policy.tsv`, and leave the CLI, export toolchain, fixtures, and
other unmarked lifecycle materials unchanged.

**Rationale:** Processing policy with the CLI does not relicense the CLI.
Independent consumers need a specific grant, not a Cairn-wide relicense.

## Risks / Trade-offs

- Commit authorship and license inventory checks do not establish a complete
  copyright chain of title or third-party licensing authority. The license
  grant relies on the repository owner's authorization for the scoped files.
- Keep original license texts and source identities available so consumers
  may choose the terms appropriate to their distribution.
