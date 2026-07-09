# Workspace Contract Matrix

Cairn owns the workspace contract matrix as a lifecycle inventory for cross-repo contract edges. Provider repos still own local artifact semantics, and consumer repos own integration checks. Cairn validates only that reviewed edges are declared, current, covered by positive and negative evidence, and bounded by explicit non-claims.

Run validation with:

```bash
cairn workspace-contract verify --root . --input fixtures/workspace-contract-matrix/positive/representative-edges.json
```

Each row names an edge ID, provider repo, consumer repo, artifact family, schema or contract ID, repo-relative artifact path, BLAKE3 artifact identity, validation command, positive fixture, negative fixture or failure class, owner, and non-claims.

## Release readiness

`workspace_contract_matrix_gate.mode` supports `disabled`, `optional`, and `required`.

- `disabled`: release readiness skips the matrix.
- `optional`: an absent receipt is contextual only; a supplied receipt must be valid to be accepted.
- `required`: an absent or invalid matrix receipt fails release readiness.

## Non-claims

A passing matrix proves declared edge health only. It does not prove provider correctness, does not prove consumer behavioral correctness, does not prove verifier soundness, or does not prove release eligibility by itself.
