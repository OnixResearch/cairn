# Design: Determinism coverage release evidence

## Context

`cairn determinism coverage-audit --root .` produces a deterministic receipt that proves policy-owned public deterministic surfaces are covered by replay cases. Release-readiness, bundle verification, and evidence manifest verification already stitch other deterministic artifacts together, but coverage audit is still a separate optional operator step.

## Goals

- Make coverage-audit evidence visible in release-readiness receipts.
- Let evidence bundles and manifests carry coverage-audit artifacts with deterministic hash linkage.
- Preserve compatibility by accepting optional bundle/manifest coverage artifacts while release-readiness always runs the audit itself.
- Emit deterministic issue codes without raw paths, roots, environment values, or secrets.

## Non-Goals

- No cryptographic signing or remote transparency anchoring changes.
- No expansion of replay case semantics beyond the existing coverage audit.
- No network access or wrapper-specific artifact publishing.

## Decisions

### 1. Release-readiness runs coverage audit internally

**Choice:** release-readiness composes the same coverage-audit logic and records `coverage_audit_receipt_hash` and `coverage_audit_valid`.

**Rationale:** release-readiness should not depend on wrapper file paths to prove this local policy invariant.

### 2. Bundles and manifests support coverage audit artifacts

**Choice:** bundle JSON may include `coverage_audit_path` and optional `coverage_audit_receipt_hash`; evidence manifests may include an optional `coverage_audit` role. Verifiers check readability, JSON shape, `valid=true`, and hash linkage when present.

**Rationale:** this lets CI publish durable coverage evidence without making older bundles/manifests invalid.

### 3. Stable public schemas grow explicitly

**Choice:** policy public receipt schemas list coverage audit hash fields so downstream agents can consume them without ad hoc parsing.

**Rationale:** public receipt fields are reviewed policy, not hidden CLI behavior.

## Risks / Trade-offs

- **Policy hash churn**: schema additions may stale goldens and fixtures. Mitigate by regenerating policy/goldens after implementation.
- **Receipt recursion**: release-readiness must record coverage audit hash without embedding the full audit receipt. Mitigate with hash-only linkage.
- **Secret/path leaks**: all coverage evidence failures use stable codes/classes and avoid raw artifact paths.

## Validation Plan

- Add positive/negative CLI tests for release-readiness coverage fields and bundle/manifest coverage linkage.
- Run policy export, formatting, CLI/core/workspace tests, Tracey, release-readiness, coverage-audit, and flake checks.
