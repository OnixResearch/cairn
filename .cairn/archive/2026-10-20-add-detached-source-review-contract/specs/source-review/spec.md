# Detached source-review contract

## ADDED Requirements

### Requirement: Exact bounded source-review contract
r[cairn.source_review.contract] Cairn MUST expose a closed source-review subject,
claim, verification policy, signing request and attachment for the
`mantle-release-source` v1 `source` projection. The signed claim MUST bind archive,
optional tree and revision facts, claim identity/root/revision, review-policy
identity, disposition and mandatory non-claims. Logical identities MUST use the
schema domain plus NUL plus recursively key-sorted compact JSON; the exact input
file's artifact identity MUST remain separate. DTO parsing MUST reject duplicate
and unknown fields, unsupported schemas/projections, malformed digests, inputs
above 1 MiB, and review/key/author collections above 64 entries.

#### Scenario: Exact source facts retain one logical identity
r[cairn.source_review.contract.valid]
- GIVEN a supported source subject and closed claim
- WHEN canonicalization processes equivalent object-key orderings
- THEN logical identities and unsigned statement bytes remain equal
- AND a changed archive, tree, revision, claim root or policy cannot verify under the old binding

#### Scenario: Malformed or unsupported carriers fail closed
r[cairn.source_review.contract.invalid]
- GIVEN duplicate/unknown JSON fields, unsupported schema or projection, malformed digest, or an exceeded bound
- WHEN parsing or validation admits the carrier
- THEN it returns an explicit failure without approval attribution

### Requirement: Independently authenticated role-separated attribution
r[cairn.source_review.authentication] Cairn MUST authenticate external detached
Ed25519 signatures through Artifact Auth over its exact canonical statement bytes.
Signing requests MUST reconstruct those bytes without accepting private keys.
Verification MUST bind the explicit trusted full-key identity, purpose, generation,
currentness observation and verifier context; apply author exclusions; reject
unknown, stale, revoked or superseded keys; and count distinct approved full keys
against the declared threshold. Duplicate full review keys MUST fail even under
other labels. Authenticated `needs_revision` rows MUST NOT count as approval.

#### Scenario: Distinct authorized reviewers reach the selected threshold
r[cairn.source_review.authentication.valid]
- GIVEN valid detached signatures from distinct currently authorized full keys
- WHEN verification independently authenticates their exact source-review statements
- THEN approved attribution counts each full key once regardless of display labels
- AND reports accepted manual review only when the selected threshold is reached

#### Scenario: Altered, stale, excluded or rejected attribution grants no approval
r[cairn.source_review.authentication.invalid]
- GIVEN a forged signature, substituted binding, duplicate key, stale/revoked/untrusted/excluded signer, or needs-revision disposition
- WHEN verification evaluates the attachment against the explicit policy
- THEN invalid authentication fails closed
- AND valid needs-revision attribution remains separate from approval and cannot satisfy quorum

### Requirement: Bounded read-only operator CLI
r[cairn.source_review.shell] Cairn MUST provide `source-review signing-request`,
`assemble` and `verify` using the existing capability-relative bounded reader under
`--root`. The shell MUST reject unknown/duplicate options, confinement escapes and
oversized inputs. It MUST emit exact raw unsigned bytes when requested, verify
assembly cryptographically, derive verification reports independently, and return
nonzero for insufficient approval while preserving its machine report. It MUST NOT
generate keys, sign, discover trust, read currentness from an ambient clock, or
mutate the supplied source or evidence files.

#### Scenario: External operator signature completes the CLI round trip
r[cairn.source_review.shell.valid]
- GIVEN an explicit source claim, trusted-key policy and externally produced signature
- WHEN the operator requests bytes, assembles the signature and verifies the attachment
- THEN verification returns the matching manual-review attribution and artifact identity
- AND no private key or signing operation is accepted by the CLI

#### Scenario: Malformed requests and unavailable approvals fail closed
r[cairn.source_review.shell.invalid]
- GIVEN a substituted request, tampered signature, excluded/revoked signer, confinement escape or insufficient threshold
- WHEN the operator invokes the CLI
- THEN it exits unsuccessfully without reporting accepted approval

### Requirement: Preserve producer and consumer non-claims
r[cairn.source_review.boundary] Cairn MUST document the exact producer/consumer
contract and its required non-claims. Source-review attribution MUST NOT grant
build-witness or release-signature authority or prove review quality, source/build
correctness, reproducibility, witness independence, deployment safety or release
eligibility. Tests MUST label deterministic keys as test-only and MUST NOT present
their signatures as genuine review approval. Consumers retain their own source,
claim-root, currentness and release-policy decisions.

#### Scenario: Valid attribution preserves explicit limits
r[cairn.source_review.boundary.valid]
- GIVEN a cryptographically valid source-review attachment
- WHEN a report or operator workflow describes its authority
- THEN it reports manual-review attribution with every required non-claim
- AND does not represent test signatures as real approval or build/release authority

#### Scenario: Weakened claims or cross-role purpose is rejected
r[cairn.source_review.boundary.invalid]
- GIVEN missing non-claims or a key policy permitting a different signing purpose
- WHEN the source-review validator evaluates the carrier
- THEN it rejects the weakened boundary rather than promoting another signature role
