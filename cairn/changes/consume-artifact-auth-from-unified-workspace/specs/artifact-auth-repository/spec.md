## MODIFIED Requirements

### Requirement: Independent workspace packages own the neutral contract

r[cairn.artifact_auth.repository_extraction.repository] Cairn MUST consume product-neutral authentication mechanics from the independently versioned `artifact-auth-core` and `artifact-auth-ed25519` packages in `OnixResearch/onix-artifact`, and those packages MUST retain their own lifecycle, documentation, release profile, source identity, and `MIT OR Apache-2.0` boundary without acquiring product authority.

#### Scenario: Unified source preserves package independence

r[cairn.artifact_auth.repository_extraction.repository.neutral]
- GIVEN one reviewed immutable Artifact workspace revision with authentication and transfer packages
- WHEN Cairn validates the source package graph and package metadata
- THEN the authentication packages retain their narrow dependency edge and no transfer, binding, product, or authority dependency enters them.

#### Scenario: Product-coupled or widened source is rejected

r[cairn.artifact_auth.repository_extraction.repository.product_dependency]
- GIVEN a source package imports a product crate, adds an unreviewed cross-mechanism dependency, uses a workspace-relative consumer path, or lacks exact provenance and compatible licensing
- WHEN source admission runs
- THEN Cairn rejects the source before dependency or release evidence is accepted.

### Requirement: Cutover uses immutable source and compatibility evidence

r[cairn.artifact_auth.repository_extraction.cutover] Cairn MUST consume one immutable reviewed Artifact workspace revision with aligned Cargo and Nix identities, MUST keep its resolved source package set limited to the two authentication packages, MUST preserve a bounded historical-source rollback, and MUST reject floating, workspace-relative, widened, or mismatched release dependencies.

#### Scenario: Exact unified source permits receiver migration

r[cairn.artifact_auth.repository_extraction.cutover.positive]
- GIVEN the unified revision is remotely available, Cargo and Nix agree, authentication crate bytes match the historical source, focused behavior tests pass, and only two authentication packages enter Cairn's graph
- WHEN cutover admission runs
- THEN Cairn may use the unified source while retaining all product authority and the historical rollback reference.

#### Scenario: Drift or graph widening blocks migration

r[cairn.artifact_auth.repository_extraction.cutover.negative]
- GIVEN source bytes, revision, repository, NAR hash, license, package profile, Cargo lock set, Nix lock, adapter behavior, or required non-claims drift
- WHEN cutover admission runs
- THEN Cairn keeps the migration blocked and does not treat equal package names as compatibility proof.
