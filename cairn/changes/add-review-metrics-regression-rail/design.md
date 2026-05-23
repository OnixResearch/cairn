# Design: Review metrics regression rail

## Context

Cairn's semantic-review domain already validates review receipts, aggregates ledger analytics, checks completeness links, and defines review gate policy profiles. The missing piece is a deterministic bridge from repeated sanitized findings to regression fixtures and operator guidance.

Downstream projects currently implement this bridge ad hoc. Clankers has a project-local `scripts/check-openspec-review-gates.rs` and fixture corpus. The reusable Cairn shape should encode the invariant, not the project-specific category vocabulary.

## Decisions

### 1. Registry is the source of truth

Each metrics-derived regression category is represented by one registry entry containing:

- stable category code;
- metrics category/stage/source tuple;
- diagnostic code;
- human label;
- requirement or scenario id;
- required operator-guidance anchor;
- required positive fixture id/path;
- required negative fixture id/path;
- sanitized evidence snapshot id/path.

Docs, fixtures, tasks, and receipts are checked against this registry rather than maintained as loose parallel lists.

### 2. Fixtures prove behavior, metrics explain priority

Metrics snapshots select and justify the next category. Enforcement comes from deterministic positive and negative fixtures. A category is not considered covered until both fixture directions exist and the negative fixture fails with the expected diagnostic.

### 3. Cairn owns the generic invariant, projects own categories

Cairn should expose a generic validator/receipt contract that downstream projects can feed with their registry plus fixture roots. Clankers can then remain a consumer while keeping Clankers-specific diagnostics local.

### 4. Secret-free evidence boundary

The rail validates that evidence snapshots name safe category metadata and provenance, but do not contain credentials, tokens, provider payload bodies, private transcript dumps, raw hidden prompts, or account identifiers.

## Implementation Sketch

A future implementation should add pure core validation for a review metrics registry DTO and a thin CLI command such as `cairn review metrics-regression --registry <path> --fixtures <dir> --root <root>`. The command emits a deterministic receipt with category coverage rows, drift issues, fixture pass/fail summaries, and a stable receipt hash.

## Verification Plan

- Unit tests for pure registry validation and fail-closed issue codes.
- CLI tests with one positive fixture set and multiple negative drift cases.
- Policy/schema docs covering registry fields and secret-free evidence boundaries.
- `nix run path:$PWD#cairn -- validate --root .`
- proposal/design/tasks gates for this change.
- `git diff --check`.
