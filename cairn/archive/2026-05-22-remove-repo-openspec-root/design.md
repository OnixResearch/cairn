## Context

Runtime commands already select the native `cairn/` layout when both native and legacy roots are present, and legacy `openspec/` is documented as an explicit migration source rather than a runtime fallback. The remaining top-level `openspec/` tree is historical duplicate lifecycle content that now blocks a clean native-only doctor receipt for the Cairn repository itself.

## Decisions

### 1. Delete only the repo-local legacy root

**Choice:** Remove `/openspec` from this repository while preserving `migrate openspec`, legacy fixture construction, compatibility tests, and historical archived references under `cairn/archive/`.

**Rationale:** The product still needs to import external OpenSpec repositories, but the Cairn repo itself should no longer carry an OpenSpec lifecycle root.

### 2. Rename Cairn-owned gate labels

**Choice:** Rename the flake check attr/output currently named `openspec-validate` to a Cairn-native validation name and update README examples to avoid OpenSpec as the primary Cairn identity.

**Rationale:** Local verification labels should describe the current lifecycle authority while migration command examples can still name legacy inputs explicitly.

## Risks / Trade-offs

- Deleting historical duplicate files is noisy in Git; verification focuses on doctor/validate and tests that still exercise import compatibility through fixtures.
- Some historical archives still mention OpenSpec by design; this change avoids rewriting history under native Cairn archives.
