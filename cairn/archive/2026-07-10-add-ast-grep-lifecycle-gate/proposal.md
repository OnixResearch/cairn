## Why

Cairn owns lifecycle gates, deterministic receipts, and release evidence boundaries. ast-grep can provide fast syntax-aware scans for repository rule packs, but the findings need a Cairn-native gate contract so lifecycle packages can distinguish inventory, warning, error, codemod, and non-claim evidence. Cairn should define how repositories run ast-grep in lifecycle validation without implying source correctness.

## What Changes

- Add a Cairn lifecycle gate profile for ast-grep rule-test and scan receipts.
- Define receipt fields for ast-grep tool identity, rule bundle BLAKE3 hash, repository scope, command, output format, findings summary, and non-claims.
- Keep rule execution shell-owned while pure validation cores check parsed receipt shape and policy conformance.
- Allow repositories to use the gate as inventory-only, advisory, or blocking according to policy.

## Impact

- **Surfaces**: lifecycle validation, release evidence, agent receipt bundles, policy schema, gate diagnostics.
- **Non-claims**: Cairn does not treat ast-grep findings as proof of source correctness, release eligibility, or requirement satisfaction.
- **Validation**: fixture receipts, malformed receipt tests, Cairn gates, policy export checks, and focused lifecycle validation.
