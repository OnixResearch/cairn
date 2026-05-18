## Context

EARS is a requirements authoring pattern, not a lifecycle engine. `cairn` should use the pattern as a quality constraint inside its specs while remaining responsible for repository lifecycle, deterministic receipts, and FCIS enforcement.

## Goals / Non-Goals

**Goals:**
- Detect specs whose requirements lack RFC 2119 strength words such as MUST/SHALL/SHOULD/MAY.
- Detect scenarios that do not include GIVEN, WHEN, and THEN clauses.
- Validate both accepted specs and active change delta specs.
- Preserve deterministic issue ordering.
- Keep validation decisions pure and testable.

**Non-Goals:**
- Implement a full natural-language EARS parser.
- Reject every possible prose variation.
- Add interactive remediation or auto-rewrites.
- Replace existing OpenSpec-compatible markdown layout.

## Decisions

### 1. EARS-style structural validation, not grammar parsing

**Choice:** Treat EARS as structural checks over markdown headings and scenario clauses: requirements need explicit normative strength, scenarios need GIVEN/WHEN/THEN.

**Rationale:** This captures the high-value testability property without adding brittle NLP or broad parsing dependencies.

**Alternative:** Full EARS grammar parsing. Rejected for this slice because current specs use OpenSpec-style markdown scenarios, and broad parsing would slow supersession progress.

### 2. Pure core returns issue strings

**Choice:** Add pure validation helpers that consume caller-provided spec sources and markdown strings and return deterministic issue strings.

**Rationale:** This keeps FCIS intact and matches the current validation receipt model.

**Alternative:** Put validation in the CLI while reading files. Rejected because policy decisions belong in the core.

### 3. CLI shell supplies accepted and delta spec markdown

**Choice:** CLI reads accepted `openspec/specs/**/spec.md` and active change `specs/**/spec.md` files, passes content into core validation, and renders JSON.

**Rationale:** Filesystem traversal and JSON are shell responsibilities.

## Risks / Trade-offs

**False positives on legacy prose** → Keep requirements conservative and document supported structure; fixtures cover valid/invalid cases.

**Receipt drift** → Update canonical goldens/tests in the same commit.

**Overfitting to current fixtures** → Validate both accepted specs and active change delta specs, with unit tests independent of fixture paths.
