# Design: Complete Preserves content-hash binding and readiness

## Context

`preserves_content_hash` is optional and additive, computed over receipt content excluding self-referential hash fields. The first implementation proved the core helper and direct verifier. This change finishes the readiness slice by making the field visible on durable evidence/replay/release receipt families and by restoring release-readiness checks.

## Decisions

### 1. Helper-owned binding

**Choice:** Extend the common CLI receipt helpers so target receipt builders can add `preserves_content_hash` before computing the existing `receipt_hash`.

**Rationale:** Keeps binding deterministic and avoids repeated ad-hoc hash insertion logic.

### 2. Additive policy schema entries

**Choice:** Add `preserves_content_hash` to policy top-level key and hash-field declarations for target receipt commands.

**Rationale:** The generated policy remains the reviewable receipt schema authority.

### 3. Fixture freshness is deterministic

**Choice:** Refresh the positive agent receipt fixture's `replay_policy_hash` and resulting `receipt_hash` from current generated policy and core hash rules.

**Rationale:** MCP smoke should prove structured validation, not fail because checked-in fixture evidence references an old policy hash.

### 4. Tracey markers match accepted requirements

**Choice:** Remove dangling implementation markers and add verification markers for accepted validation requirements on commands/tests that exercise them.

**Rationale:** Tracey coverage should pass with no active-change-only exemptions after the accepted specs are synced.

## Risks / Trade-offs

- Adding `preserves_content_hash` changes receipt hashes on target commands. Acceptable: this is the additive field's purpose, and `receipt_hash` remains present.
- Broad binding across every CLI receipt is deferred; this slice targets durable replay/evidence/release identity surfaces.
