# Design: Electrical Circuit Primitive Receipt Validation

## Context

Electrical artifacts cross schematic capture, netlists, PCB tools, symbolic analysis, simulators, and lab measurements. The first Cairn implementation slice intentionally validates only deterministic primitive evidence: topology shape, terminal-to-net bindings, dimensioned values, canonical graph hashes, and explicit non-claims.

## Approach

The validator is a functional core in `cairn-core` over `serde_json::Value`. It computes a canonical BLAKE3 hash using the same deterministic JSON hashing used by other Cairn receipt families, after stripping display-only and hash fields. The CLI shell reads a bounded JSON artifact, emits a stable JSON report, and exits non-zero for invalid receipts.

Policy registration advertises the schema, command, hash field, hash material, and non-claims so generated policy consumers can discover the receipt family without inspecting Rust code.

## Validation Boundaries

- Required schema is `electrical.circuit-primitive-graph.v1`.
- A topology must include an explicit reference node and every component terminal must bind to a declared net.
- Passive/source/switch/vendor/behavioral primitives must use deterministic kind and terminal metadata.
- Dimensioned values must carry numeric magnitude plus SI unit metadata.
- Receipts must explicitly deny simulation correctness, analog equivalence, electrical safety, regulatory compliance, layout correctness, and manufacturability.

## Risks

Overclaiming is the primary risk. The validator therefore treats missing non-claims as invalid and does not infer physical safety or analog behavior from primitive graph well-formedness.
