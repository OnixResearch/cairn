## Context

Each repair block copies its accepted block byte for byte and changes only the
`r[...]` line.

## Decisions

### 1. Target the duplicated block by its unique heading

**Choice:** Use one MODIFIED operation per duplicated block, named by heading.
**Rationale:** The duplicated ID is not a usable target, and the heading is
unique in each accepted spec.
