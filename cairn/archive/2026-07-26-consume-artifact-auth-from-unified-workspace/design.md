## Context

Cairn consumes `artifact-auth-core` and `artifact-auth-ed25519` from historical revision `799459346d5416fbd7b9f55840a7371441b55afa`. The unified Artifact workspace publishes both packages plus the independent `artifact-transfer-core` package at Git revision `e41340bec587b6d049b5cc518ec7db925dde84be`.

The reviewed unified source is available from `ssh://git@github.com/OnixResearch/onix-artifact.git`. Nix prefetch reports `sha256-2cM912L2YXVnVX9LquwhAPKyjPP/z/oFQRe7Qq9bHHE=`. Its source archive BLAKE3 is `8fc65f3d7968d76af419f08ac972955514c8256f8aa5ed90815d8bdcced28113`.

## Decisions

### Use the unified Git revision without changing runtime behavior

**Choice:** Change only the immutable source coordinate. Keep the package names, features, adapter code, and product policy unchanged.

**Rationale:** Recursive directory comparison shows both authentication crate directories are unchanged. The `artifact-auth-core` entry-point BLAKE3 remains `839ea3e35429e2a1252020dd779b46ebd185b6911674036820da118d487d1d21`. The Ed25519 entry-point BLAKE3 remains `7fd3987fe3ddc6d30efca37878952135f19905a00b52f349db237054be6b3633`.

### Separate source workspace membership from Cairn's crate graph

**Choice:** Validate the complete three-package source profile while requiring Cairn's Cargo lock to contain only the two authentication packages from that source.

**Rationale:** Unified build infrastructure does not make transfer geometry a Cairn dependency. The source-profile check prevents hidden workspace drift. The consumer-graph check prevents accidental semantic widening.

### Align Cargo and Nix on one source identity

**Choice:** Rename the Nix input from the legacy repository identity to `artifact`, update Cargo Git dependencies to the same SSH repository and revision, and regenerate both lockfiles with their owning tools.

**Rationale:** Source aliases are review surfaces. Keeping the old alias after migration would obscure which repository owns the active packages.

### Keep rollback explicit and bounded

**Choice:** Record historical revision `799459346d5416fbd7b9f55840a7371441b55afa` as rollback evidence only.

**Rationale:** A source-only migration should be reversible without restoring duplicate canonical ownership. Rollback does not authorize floating dependencies or sibling paths.

## Validation

- Baseline and post-change focused `cairn-core` artifact-auth tests.
- Source-pin positive evaluation and negative self-tests.
- Cargo metadata and lock-source review.
- Nix lock, revision, NAR, package-set, license, and source checks.
- Formatting, workspace tests, strict Clippy, Cairn validation, lifecycle gates, traceability, and bounded Nix checks.

## Risks

- **Source package widening:** The unified source contains transfer geometry. Mitigate with an exact two-package consumer-lock assertion.
- **Lock disagreement:** Cargo and Nix can resolve different URLs or revisions. Mitigate with one pure source-pin validator.
- **False behavior parity:** Equal package names do not prove equal code. Mitigate with exact directory comparison, BLAKE3 evidence, and focused tests.
- **Authority widening:** A shared workspace can be mistaken for shared product policy. Mitigate with unchanged adapters and explicit non-claims.
