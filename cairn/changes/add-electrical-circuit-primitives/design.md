## Context

Electrical/circuit evidence crosses several tool boundaries: schematics, SPICE-like netlists, PCB layouts, symbolic analysis, simulation traces, and lab measurements. The first Cairn slice should define the reviewed primitive contract and receipt boundaries, not choose a simulator or layout tool.

## Goals

- Make the primitive graph deterministic and portable across repository roots and tool output locations.
- Keep a small closed vocabulary for first-class electrical concepts while allowing opaque vendor/tool annotations as non-hash display metadata.
- Preserve polarity, reference-node, unit, and model-caveat data needed for review.
- Fail closed when a primitive receipt tries to claim simulator, safety, or manufacturing correctness without a stronger validator.

## Model

A circuit primitive receipt is a deterministic graph with:

- **Topology primitives**: `node`, `port`, `terminal`, `net`, `branch`, `reference-node`, and `measurement-point`.
- **Component primitives**: passive two-terminal primitives (`resistor`, `capacitor`, `inductor`), ideal/declared source primitives (`voltage-source`, `current-source`), switching primitives, and explicitly caveated behavioral/vendor-model components.
- **Metadata primitives**: SI unit/dimension values, polarity/orientation, tolerance, operating region caveats, model provenance, and source-tool identity.
- **Evidence fields**: canonical primitive hash, graph hash, normalization version, source artifact hash, and explicit non-claims.

## Boundaries

- Cairn owns lifecycle review, deterministic receipt structure, hashes, gate coverage, and non-claim validation.
- Domain tools own extraction, simulation, ERC/DRC, layout, vendor-model interpretation, and physical measurement.
- Matching primitive hashes identify reviewed artifact evidence only; they do not prove circuit behavior, safety, compliance, or manufacturability.

## Alternatives

- **Put all circuit semantics in a simulator.** Rejected because Cairn needs tool-neutral evidence receipts before choosing simulators.
- **Treat netlists as unstructured blobs.** Rejected because agents need stable nodes, ports, units, and model caveats for reviewable changes.
- **Include PCB layout primitives now.** Deferred; this change only leaves room for physical/layout evidence without claiming it.
