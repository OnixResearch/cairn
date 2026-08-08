# Ast Grep Lifecycle Gate Specification

## Purpose

Define Cairn's lifecycle contract for validating ast-grep rule-test and scan receipts without promoting syntax-aware matches into stronger claims.

## Requirements

### Requirement: Lifecycle gate profile
r[cairn.ast_grep_gate.profile] Cairn MUST define an ast-grep lifecycle gate profile with policy modes for inventory-only, advisory warning, blocking error, and codemod-review evidence.

#### Scenario: Repository selects advisory mode
- GIVEN a repository policy declares the ast-grep gate as advisory
- WHEN Cairn evaluates an ast-grep scan receipt with findings
- THEN Cairn MUST report the findings as advisory diagnostics and MUST NOT fail release readiness solely because advisory findings exist.

### Requirement: Receipt validation
r[cairn.ast_grep_gate.receipt] Cairn MUST validate ast-grep receipts for tool identity, rule bundle BLAKE3 identity, repository scope, command identity, output format, findings summary, severity posture, and non-claim labels.

#### Scenario: Valid ast-grep receipt is accepted
- GIVEN an ast-grep receipt includes tool identity, BLAKE3 rule bundle identity, declared scope, structured findings summary, and non-claims
- WHEN Cairn validates the receipt
- THEN validation MUST accept the receipt according to the declared policy mode.

### Requirement: Shell-owned execution
r[cairn.ast_grep_gate.shell_boundary] Cairn MUST keep ast-grep process execution, filesystem walking, stdout/stderr capture, and raw report loading outside pure validation cores.

#### Scenario: CLI captures scan output
- GIVEN a CLI command or repository script runs ast-grep
- WHEN Cairn receives the parsed receipt
- THEN pure validation MUST operate only on in-memory receipt data and MUST NOT spawn ast-grep or read repository files.

### Requirement: Malformed receipts fail closed
r[cairn.ast_grep_gate.negative_fixtures] Cairn MUST reject malformed or overclaiming ast-grep receipts with deterministic diagnostics.

#### Scenario: Receipt overclaims proof status
- GIVEN an ast-grep receipt claims source correctness, proof discharge, requirement satisfaction, or release eligibility
- WHEN Cairn validates the receipt
- THEN validation MUST fail closed and identify the overclaiming field.

### Requirement: Valence-compatible evidence fields
r[cairn.ast_grep_gate.valence_compat] Cairn SHOULD preserve optional source-span, rule id, finding identity, and verification-role fields when ast-grep receipts are intended for Valence-compatible evidence export.

#### Scenario: Receipt includes source spans
- GIVEN an ast-grep scan receipt includes file spans and verification roles
- WHEN Cairn validates the receipt
- THEN validation SHOULD preserve those fields for downstream evidence wrapping while retaining Cairn non-claims.

### Requirement: Gate validation evidence
r[cairn.ast_grep_gate.validation] Cairn MUST validate ast-grep gate changes with positive fixtures, negative malformed fixtures, policy export checks, lifecycle validation, and Cairn proposal/design/tasks gates.

#### Scenario: Gate implementation is reviewed
- GIVEN the ast-grep gate profile or receipt validator changes
- WHEN validation evidence is assembled
- THEN the evidence MUST include accepted positive fixtures, rejected negative fixtures, policy freshness, and lifecycle gate results.
