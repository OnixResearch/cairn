## Context

The current repository exposes `cairn` as the repo name, native lifecycle directory, Rust package/crate prefix, CLI binary, Nix app/package, README title, Hermes skill source, fixtures, goldens, and accepted requirement IDs. The name still usefully signals “spec,” but it also keeps the project semantically anchored to OpenSpec supersession rather than the broader native lifecycle engine it has become.

`Cairn` is the chosen natural name. It maps to durable trail markers: small, ordered, inspectable artifacts that help operators navigate from proposal through specs, gates, evidence, sync, and archive.

## Goals / Non-Goals

**Goals:**

- Make `Cairn` the canonical public identity for new operators.
- Preserve deterministic lifecycle behavior and FCIS boundaries during the rename.
- Keep compatibility intentional, visible, and test-covered.
- Produce reviewable evidence for all remaining legacy-name occurrences.

**Non-Goals:**

- Changing lifecycle semantics, gate policy, receipt hashing, or Nickel contracts beyond names and compatibility metadata.
- Maintaining permanent dual branding.
- Relying on raw grep counts as proof that the rename is complete.

## Decisions

### 1. Canonical identity becomes `Cairn` / `cairn`

**Choice:** Use `Cairn` for prose/product branding and `cairn` for CLI, packages, crate prefixes, Nix attributes, config tables, and normal runtime docs.

**Rationale:** The name is natural without being soft; it preserves the “guided lifecycle marker” metaphor and fits deterministic infrastructure.

**Alternative:** Keep `cairn` as the technical name and use Cairn only as branding. Rejected because dual identity would make command/docs/tests drift more likely.

### 2. Compatibility is explicit and classified

**Choice:** Legacy `cairn` command/config/layout surfaces may remain temporarily only as explicit compatibility aliases. Receipts and docs must identify canonical `cairn` behavior and classify legacy input as compatibility, not as silent fallback.

**Rationale:** Existing local workflows and archived evidence may still reference `cairn`; breaking everything in one step would obscure review signal. Explicit aliases preserve users while preventing accidental permanent dual naming.

**Alternative:** Hard cutover with no aliases. Rejected as unnecessarily risky for fixtures, archive history, and existing automation.

### 3. Rename proof uses deterministic classification evidence

**Choice:** Add a checked or reproducible rename evidence artifact that classifies remaining `cairn` strings into `needs-fix`, `compatibility-alias`, `historical-archive`, `requirement-id-history`, or `intentional-reference` buckets.

**Rationale:** A raw search will always find historical requirement IDs, archive paths, and compatibility aliases. Review needs semantic classification rather than a misleading zero-match goal.

**Alternative:** Require zero `cairn` matches. Rejected because accepted IDs and archived evidence may intentionally preserve old names.

## Risks / Trade-offs

- **Fixture/golden churn:** Rename touches many stable outputs. Mitigation: update goldens in the same slice and keep focused exact-key assertions.
- **Compatibility ambiguity:** Dual layout/command names can recreate legacy fallback confusion. Mitigation: canonical-over-legacy precedence, explicit receipt fields, and negative tests for ambiguous state.
- **Traceability churn:** Requirement IDs containing `cairn` may persist for history. Mitigation: classify IDs separately and only mint new `cairn.*` IDs for new requirements after rename.
- **Nix attr drift:** Nix package/app/check names can diverge from Cargo names. Mitigation: include `nix flake check` and metadata checks in the task evidence.
