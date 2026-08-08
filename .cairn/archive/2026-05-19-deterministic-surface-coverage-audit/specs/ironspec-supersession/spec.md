## ADDED Requirements

### Requirement: Deterministic Surface Coverage Audit Command r[cairn.supersede.determinism_surface_coverage_audit.cli]

Cairn SHALL provide a deterministic `cairn determinism coverage-audit --root .` command that audits policy-owned replay coverage for public deterministic surfaces.

#### Scenario: Audit emits pass/fail receipt

r[cairn.supersede.determinism_surface_coverage_audit.cli]

- GIVEN Cairn's generated policy
- WHEN the operator runs `cairn determinism coverage-audit --root .`
- THEN Cairn SHALL emit a deterministic JSON receipt with verdict, issues, current replay policy hash, surface inventory hash, covered surfaces, missing surfaces, and receipt hash.

### Requirement: Policy-Owned Deterministic Surface Inventory r[cairn.supersede.determinism_surface_coverage_audit.inventory]

Cairn SHALL declare deterministic public receipt surfaces and their required replay coverage in generated policy.

#### Scenario: Inventory declares coverage expectations

r[cairn.supersede.determinism_surface_coverage_audit.inventory]

- GIVEN a public deterministic CLI or MCP receipt surface
- WHEN that surface is expected to support agent or CI determinism claims
- THEN the policy inventory SHALL name the surface id, command label, surface kind, required replay cases, required replay groups, and determinism claim category.

### Requirement: Missing Coverage Failure Semantics r[cairn.supersede.determinism_surface_coverage_audit.failures]

Cairn SHALL fail deterministically when an inventoried surface lacks policy replay coverage.

#### Scenario: Missing case or group is reported safely

r[cairn.supersede.determinism_surface_coverage_audit.failures]

- GIVEN a surface inventory entry with a missing replay case, missing replay group, or group that omits a required case
- WHEN Cairn audits coverage
- THEN the receipt SHALL set `valid=false`, exit nonzero, and report stable issue codes without raw roots, paths, environment values, credentials, or host-specific values.

### Requirement: Deterministic Surface Coverage Documentation r[cairn.supersede.determinism_surface_coverage_audit.docs]

Cairn SHALL document coverage audit usage and scope.

#### Scenario: Operator can distinguish audit scope

r[cairn.supersede.determinism_surface_coverage_audit.docs]

- GIVEN Cairn's README and agent receipt documentation
- WHEN an operator reads the deterministic evidence flow
- THEN the docs SHALL explain that coverage-audit checks policy replay catalog completeness and does not by itself run replay or prove all workspace behavior.

### Requirement: Deterministic Surface Coverage Tests r[cairn.supersede.determinism_surface_coverage_audit.tests]

Cairn SHALL test passing and failing deterministic surface coverage audit behavior.

#### Scenario: Tests cover real and broken inventories

r[cairn.supersede.determinism_surface_coverage_audit.tests]

- GIVEN Cairn's CLI test suite
- WHEN coverage audit tests run
- THEN tests SHALL prove the real policy passes and synthetic missing coverage produces deterministic, path-safe failure receipts.

### Requirement: Deterministic Surface Coverage Gates r[cairn.supersede.determinism_surface_coverage_audit.gates]

Cairn SHALL pass proposal, design, and tasks gates for the deterministic surface coverage audit change.

#### Scenario: Change artifacts are gate-ready

r[cairn.supersede.determinism_surface_coverage_audit.gates]

- GIVEN proposal, design, tasks, and spec delta artifacts
- WHEN Cairn gates each stage
- THEN each gate receipt SHALL be valid.

### Requirement: Deterministic Surface Coverage Lifecycle r[cairn.supersede.determinism_surface_coverage_audit.archive]

Cairn SHALL sync and archive the deterministic surface coverage audit change after verification.

#### Scenario: Archived change retains review evidence

r[cairn.supersede.determinism_surface_coverage_audit.archive]

- GIVEN implementation and verification are complete
- WHEN the change is synced and archived
- THEN the accepted spec and archive SHALL retain traceable requirements and verification references.
