# Implement electrical circuit primitive receipts

## Why

Cairn has reviewed requirements for electrical circuit primitives, but the contract is only useful if operators can validate portable receipt artifacts without relying on a simulator, EDA tool, or host-specific paths.

## What Changes

- Add a pure Rust validator for `electrical.circuit-primitive-graph.v1` receipts.
- Add `cairn evidence circuit-primitive validate <receipt.json>`.
- Add positive and negative fixtures covering topology, terminal bindings, dimensioned values, graph hashes, and explicit non-claims.
- Register the receipt family in the generated Cairn policy.

## Non-Goals

- Circuit simulation or equivalence checking.
- Safety, regulatory, PCB layout, or manufacturability validation.
- Vendor model semantic validation.
