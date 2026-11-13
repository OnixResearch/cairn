## Context

The existing pinned englishc adapter checks explicit files, emits advisory JSON v1, and never rewrites source. Its source positions and machine behavior are public contracts. Its grammar findings are not lifecycle evidence.

## Decisions

### Decision: Preserve active contracts explicitly

**Choice:** Keep default JSON v1. Select enriched JSON v2 with `--report-version 2`; other new formats consume the same enriched diagnostic model. Machine mode continues to accept only JSON presentation.

**Rationale:** Existing consumers retain their report shape while new consumers can bind exact inputs and configuration.

### Decision: Separate IO, pure analysis, and presentation

**Choice:** Reuse capability-relative no-follow reads, existing source traversal bounds, pinned englishc rule metadata, Cairn traceability marker parsing, Nickel export parity, and BLAKE3 identities. Pure document projection preserves every byte offset and newline. Renderers cannot publish or mutate.

**Rationale:** Reuse the existing authorities instead of adding a second grammar or EARS validator.

### Decision: Recompute before declaring a saved report current

**Choice:** Verify the v2 schema/cohort/profile/bounds/selection, read original current input bytes under confinement, rerun the same analysis, and compare the complete canonical report including its hash. Hashes bind content but do not establish authenticity or approval.

**Rationale:** A fresh source hash alone must not authorize a stale or altered suggestion.

## Risks / Trade-offs

- Technical grammar and meaning-risk heuristics can miss defects or produce false positives; all suggestions require review.
- Format projection is bounded and not a complete Markdown/AsciiDoc parser; unsupported syntax remains ordinary input.
- Output limits fail atomically instead of hiding truncated findings; callers may need smaller selections.
- Saved diagnostics may disclose excerpts. Export is operator-controlled; no network or publication is performed.
