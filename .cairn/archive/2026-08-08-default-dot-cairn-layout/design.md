## Context

Cairn centralizes most lifecycle paths through pure `cairn-core` helpers. Some CLI modules still use direct path constants for bounded mutation and collaboration operations.

The stable receipt layout name is `cairn`. The directory name is not the same compatibility surface as that receipt value.

## Decisions

### Decision: Make `.cairn/` the only canonical runtime root

**Choice:** Change `LifecycleLayout::Native` path helpers to `.cairn/`. Update all maintained runtime paths, policy roots, fixtures, goldens, and current documentation. Classify `.cairn/` as a native lifecycle root only when it contains at least one lifecycle subdirectory. A directory that contains only lifecycle-project pointer files remains project configuration.

**Rationale:** One canonical root keeps command behavior deterministic. The subdirectory check preserves existing lifecycle-store pointer selection. It also preserves the current rule that compatibility content is migration-only.

### Decision: Model the old directory as an explicit migration source

**Choice:** Add a `LegacyCairn` layout rooted at `cairn/`. Add `cairn` to the CLI and MCP migration source enums. Migration remains preview-first, copies files, blocks conflicts, and does not delete source content.

**Rationale:** Existing users get a bounded upgrade path without an implicit fallback.

### Decision: Preserve the stable layout receipt value

**Choice:** Keep `layout_name(LifecycleLayout::Native)` equal to `cairn`. Use `legacy-cairn` for the migration source layout value.

**Rationale:** The folder change does not require an unrelated break in stable receipt consumers. Exact action paths still disclose the directory change.

### Decision: Keep path selection in the functional core

**Choice:** Put canonical and legacy roots in pure lifecycle helpers. Imperative CLI modules consume these values or named constants for capability-relative file operations.

**Rationale:** Pure path planning remains testable without filesystem setup. CLI code retains all filesystem effects.

## Risks / Trade-offs

- Existing automation that reads `cairn/` directly must move to `.cairn/`.
- Hidden directories need explicit flags in some file discovery tools.
- Historical archived text can retain old paths because it describes past evidence. Current policy, specs, tests, and operator documents must use `.cairn/`.
- The migration copies source files and leaves the old directory for deliberate operator removal.
