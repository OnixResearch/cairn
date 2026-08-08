## Context

Cairn already owns lifecycle gates, traceability, release readiness, evidence export, and stack-provenance readiness. A workspace-level contract matrix fits Cairn's role: it does not own upstream semantics, but it can validate that reviewed contract edges are declared, current, and bounded by explicit non-claims.

## Design

### Matrix schema

Author the matrix contract in Nickel and export deterministic JSON for runtime validation. Each row includes:

- edge ID;
- provider repo and artifact family;
- consumer repo and consumed role;
- schema or contract ID;
- artifact BLAKE3 or manifest digest;
- validation command;
- positive fixture path;
- negative fixture path or failure class;
- owner/reviewer field;
- required non-claim boundary.

### Validator

The core validator is pure over parsed matrix rows and supplied artifact identities. The CLI shell loads files, expands safe repo-relative paths, computes BLAKE3, and prints a stable receipt. Validation fails for missing owners, unsupported repos, missing positive or negative coverage, stale identities, unsafe paths, and weakened non-claims.

### Release-readiness integration

Add an optional/required policy mode for workspace contract matrix validation. Required mode fails release readiness when a configured edge is missing or stale. Optional mode records the matrix receipt as context.

## Alternatives

### Keep matrix in README prose

Rejected. Prose is useful for orientation but cannot fail closed on stale identities or missing negative fixtures.

### Centralize all contracts in Cairn

Rejected. Provider repos should still own their contract semantics. Cairn only owns the cross-repo edge inventory and validation receipt.

## Risks

- **Matrix becomes stale**: require BLAKE3 identities and validation commands.
- **Cairn overclaims upstream correctness**: preserve non-claims that the matrix verifies declared edge health only.
- **Maintenance burden**: start with a small required edge set and allow optional rows for emerging contracts.
