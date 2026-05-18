## Context

The current CLI already scans active changes and accepted spec file paths for `status`, `validate`, `sync`, and `archive`. It lacks explicit read commands that match the everyday OpenSpec discovery workflow.

## Goals / Non-Goals

**Goals:**
- Provide stable JSON receipts for listing and showing active changes.
- Provide stable JSON receipts for listing and showing accepted specs.
- Keep read model decisions pure and deterministic.
- Preserve the strict FCIS boundary.

**Non-Goals:**
- Markdown rendering or human table output.
- Full markdown requirement parsing.
- Archived change browsing.
- Mutation behavior.

## Decisions

### 1. Reuse repository snapshots

**Choice:** Extend the existing shell discovery snapshot with spec source metadata and pass plain data into pure read-model builders.

**Rationale:** The CLI already owns filesystem traversal, and pure core can sort/filter/shape deterministic models without reading files.

**Alternative:** Add direct file reads in core. Rejected because it violates the FCIS boundary guarded by Slotcar.

**Implementation:** CLI reads `spec.md` content and paths; core builds `SpecSummary`, `SpecDetail`, `ChangeList`, and `ChangeDetail` models from supplied strings/status data.

### 2. Stable machine receipts only

**Choice:** New commands emit JSON only, matching existing `status`, `sync`, and `archive` behavior.

**Rationale:** `cairn` is being built for deterministic automation first; table output can be added later as a presentation mode.

**Alternative:** Mirror upstream CLI human output. Rejected for this slice because it increases surface area without improving automation proof.

## Risks / Trade-offs

**Spec ID ambiguity** → Use accepted spec domain IDs derived from `openspec/specs/<domain>/spec.md`; reject missing IDs with a clear error.

**Large spec content** → `spec show` returns the markdown content for deterministic inspection. Future slices can add bounded/ranged reads if needed.

**Style drift** → Add tests/goldens and run Slotcar, clippy, and flake checks.
