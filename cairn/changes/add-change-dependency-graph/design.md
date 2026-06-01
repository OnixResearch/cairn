# Design: change dependency graph

## Model

Add `depends_on` to change `metadata.json` as an optional array of change identifiers. Edge direction is `dependent -> dependency`; rendered and receipt output also exposes `blocks` as the reverse adjacency. Dependency identifiers are normalized with the same bounded selector rules used for change names, then matched against active changes and archive entries.

A dependency is satisfied when the referenced change is archived. An active dependency is well-formed but blocks the dependent from sync/archive execution until it archives. Unknown references, self edges, duplicate edges, and cycles are graph issues.

## Pure core

Implement graph construction and validation in `cairn-core` over an in-memory list of change ids, archived ids, and declared dependency ids. The core returns a deterministic report with nodes, edges, issue codes, `blocked_by`, `blocks`, and a topological drain order. Sorting is stable and lexical so receipts are reproducible.

Use Trellis as the primitive source for the graph logic:

- `../trellis/src/topo_sort.rs`: Kahn-style topological ordering and order validity.
- `../trellis/src/cycle_detect.rs`: cycle semantics and acyclic proof shape.
- `../trellis/src/job_dag.rs`: all-dependencies-satisfied and unsatisfied-count readiness shape.

Do not make normal Cairn builds depend on `../trellis`. Adapt the needed pure primitives into Cairn-owned code or add a reviewed portable dependency path only if the release/build contract remains self-contained. Mark the adapted implementation with Tracey references back to this change's requirements.

## CLI and MCP shell

Extend metadata parsing in the CLI shell, then pass plain data into core. Add a read-only graph surface, either `cairn change graph --root PATH` or an equivalent deterministic receipt path, and extend `cairn_drain_plan` with dependency fields:

- `dependencies`: declared ids
- `blocked_by`: unsatisfied dependency ids
- `blocks`: dependent ids
- `drain_order`: dependency-first active change order
- issue objects for unknown, self, duplicate, or cyclic edges

The existing dry-run sync/archive plans include dependency blockers. `--execute` refuses mutation when blockers exist, same as gate blockers.

## Tests and verification

Add positive tests for no dependencies, archived dependencies, and active dependency ordering. Add negative tests for unknown dependency, self dependency, duplicate dependency, and active cycles. Run focused core tests, CLI graph/drain tests, sync/archive blocker tests, Cairn validation, and Tracey coverage.
