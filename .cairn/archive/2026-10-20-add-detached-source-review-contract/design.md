## Context

Cairn already separates loaded decision inputs from filesystem/process effects.
Mantle needs an explicit reviewed-source producer, not a locally invented
interpretation of a generic semantic-review status. This change adds a separate
contract without changing existing semantic-review receipts or their authority.

## Decisions

### Decision: Exact detached Artifact Auth bytes

**Choice:** The pure `cairn_core::source_review` API constructs a domain-separated
claim over the `mantle-release-source` v1 `source` projection, exact archive/tree
and revision facts, existing claim identity/root/revision, review-policy digest,
disposition and mandatory non-claims. It emits the exact canonical Artifact Auth
statement bytes; external operators sign those bytes. Assembly reconstructs the
request and verifies the detached Ed25519 signature before returning a carrier.

**Rationale:** A JSON status, key display label or producer assertion must not
become signature authority. The API accepts no private key and reads no clock,
environment, filesystem or network. Artifact Auth owns signature authentication;
Cairn owns this source-review profile and its exact linkage and role rules.

### Decision: Explicit currentness, distinct identities and bounded inputs

**Choice:** Verification consumes an explicit closed policy with full trusted-key
identities, generations, currentness observations, verifier context, author-key
exclusions and an approval threshold. Source-review-only purposes are required.
Unknown, revoked, superseded or stale key observations fail closed; current and
verification-overlap observations are admitted by policy. Duplicate full review
keys fail rather than increasing quorum under a different label. Authenticated
`needs_revision` rows remain attribution, not approval.

**Rationale:** Trust discovery and freshness remain operator facts, not ambient
authority. Source review cannot acquire build-witness or release-signature roles.
At most 64 reviews, trusted keys and author identities are admitted. DTO parsing
rejects duplicate/unknown fields and inputs above 1 MiB before processing them.

### Decision: Read-only capability-confined shell

**Choice:** `signing-request` emits a JSON request or exact raw unsigned bytes;
`assemble` verifies an external signature and optional additional signed rows;
`verify` independently validates an attachment and emits a derived report plus
the exact input-file BLAKE3 identity. All file inputs use the existing bounded
capability-relative reader under `--root`. Insufficient approval produces a
machine report and nonzero exit, while rejected-review attribution is exportable.

**Rationale:** The CLI reuses existing root confinement and JSON presentation. It
does not introduce another signer, policy store, scheduler or publication layer.
Logical identities use schema-domain + NUL + sorted compact JSON; an artifact's
exact file-byte identity is deliberately distinct from its logical identity.

## Risks / Trade-offs

- Correct signatures authenticate supplied attribution only. A caller still owns
  the selected source, claim root, key policy and freshness of its observations.
- Test keys exercise cryptographic and CLI behavior only; they cannot satisfy a
  real source-review requirement or count as release witnesses.
- This initial stable profile admits one exact source projection. Unknown schema
  versions and other projections fail closed rather than receiving guessed
  compatibility. Consumers must pin the producer and verify their own linkage.
- No existing generic semantic-review status is promoted into this contract.

