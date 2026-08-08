## Why

Cairn/Valence-style evidence currently has lifecycle rails for software artifacts, but it does not have a reviewed primitive contract for electrical engineering or circuit artifacts. Circuit work needs a small deterministic vocabulary before agents can safely exchange schematic, netlist, or analysis receipts without implying simulator correctness or physical safety.

## What Changes

- Define a native Cairn specification domain for electrical-engineering circuit primitives.
- Establish canonical primitive families for nodes, ports, nets, branches, components, sources, references, and measurement points.
- Require unit/dimension metadata, polarity/orientation metadata, deterministic serialization, and BLAKE3 content hashes for portable receipts.
- Preserve explicit non-claims: primitive agreement is not proof of simulation correctness, layout correctness, electrical safety, regulatory compliance, or manufacturability.

## Non-Goals

- Implement a circuit simulator, SPICE parser, PCB layout engine, ERC/DRC engine, or hardware safety certification.
- Replace KiCad, SPICE, vendor models, or lab instrumentation.
- Claim analog equivalence or physical correctness from matching primitive hashes alone.

## Impact

- **Files**: Adds native Cairn change artifacts under `cairn/changes/add-electrical-circuit-primitives/`.
- **APIs**: Future Cairn validators may consume circuit primitive receipts, but this change first defines the lifecycle contract.
- **Testing**: Cairn validate plus proposal/design/tasks gates for the native change package.
