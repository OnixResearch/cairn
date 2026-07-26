## Phase 1: Source and behavior baseline

- [x] [serial] Record the historical and unified source revisions, repository coordinates, Nix NAR hash, source archive BLAKE3, exact authentication crate parity, baseline tests, retained Cairn authority, and rollback source. r[cairn.artifact_auth.repository_extraction.repository] r[cairn.artifact_auth.repository_extraction.cutover]

## Phase 2: Receiver-local migration

- [x] [serial] Update Cargo and Nix source declarations to the unified workspace revision without changing authentication APIs, features, adapter behavior, or product policy. r[cairn.artifact_auth.repository_extraction.repository] r[cairn.artifact_auth.repository_extraction.cutover]
- [x] [depends:artifact-unified-source] Regenerate Cargo and Nix locks with their owning tools and confirm both resolve one immutable source identity. r[cairn.artifact_auth.repository_extraction.cutover]
- [x] [parallel] Strengthen positive and negative source-pin checks for the complete source workspace, exact Cairn consumer package set, source and license drift, floating revisions, sibling paths, duplicate packages, and Cargo/Nix mismatch. r[cairn.artifact_auth.repository_extraction.cutover]

## Phase 3: Evidence and closeout

- [x] [parallel] Run post-change authentication and source-admission checks, record rollback and non-claims, and prove that transfer and binding packages do not enter Cairn's crate graph. r[cairn.artifact_auth.repository_extraction.cutover]
- [x] [serial] Run formatting, workspace tests, strict Clippy, lifecycle validation and gates, traceability, policy freshness, and bounded Nix checks; then sync accepted requirements and archive only after all evidence agrees. r[cairn.artifact_auth.repository_extraction.cutover] r[cairn.artifact_auth.repository_extraction.repository]
