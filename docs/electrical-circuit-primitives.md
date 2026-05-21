# Electrical Circuit Primitive Receipts

This note is the maintained implementation/reference surface for Cairn's reviewed electrical circuit primitive contracts.

## Primitive contract coverage

- r[impl electrical_circuit_primitives.topology] Topology evidence uses deterministic nodes, nets, terminals, reference nodes, and measurement points.
- r[impl electrical_circuit_primitives.topology.reference_node] Circuit primitive receipts require an explicit `reference_node` that matches a declared node.
- r[impl electrical_circuit_primitives.topology.connection_arity] Component terminal bindings are reviewable and missing required bindings fail validation.
- r[impl electrical_circuit_primitives.components] The first-pass vocabulary is closed around passive components, declared sources, switches, and caveated model primitives.
- r[impl electrical_circuit_primitives.components.passive_primitives] Passive resistor/capacitor/inductor entries carry deterministic kind, terminal bindings, and value metadata.
- r[impl electrical_circuit_primitives.components.model_caveats] Behavioral/vendor/semiconductor models require provenance/caveats rather than physical-equivalence claims.
- r[impl electrical_circuit_primitives.units] Values use deterministic dimension metadata instead of untyped display strings.
- r[impl electrical_circuit_primitives.units.dimensioned_values] Dimensioned values include normalized numeric value, SI dimension, display unit, and normalization version.
- r[impl electrical_circuit_primitives.receipts] Circuit primitive graph receipts carry graph/source hashes, normalization metadata, tool identity, and caveats.
- r[impl electrical_circuit_primitives.receipts.portable_hashes] Graph hash material strips display-only host paths and hash fields while preserving reviewed primitive content.
- r[impl electrical_circuit_primitives.non_claims] Receipt validation preserves explicit non-claims for simulation, analog equivalence, safety, compliance, layout, and manufacturability.
- r[impl electrical_circuit_primitives.non_claims.no_safety_claim] Missing safety/compliance/layout/manufacturability non-claims are validation failures.

## Validator coverage

- r[impl electrical_circuit_primitive_receipts.validator] Circuit primitive receipt validation is implemented as deterministic Rust validation over JSON receipt content.
- r[impl electrical_circuit_primitive_receipts.validator.core] The core validator checks schema, topology, bindings, values, hashes, and non-claims.
- r[impl electrical_circuit_primitive_receipts.validator_cli] The CLI surface is `cairn evidence circuit-primitive validate <receipt.json>`.
- r[impl electrical_circuit_primitive_receipts.validator.cli] The CLI emits stable JSON with validity, verdict, issues, computed graph hash, and receipt hash.
- r[impl electrical_circuit_primitive_receipts.fixtures] Positive and negative fixtures live under `fixtures/electrical-circuit-primitives/`.
- r[impl electrical_circuit_primitive_receipts.fixtures.positive] `positive-rc-filter.json` covers a valid passive RC topology.
- r[impl electrical_circuit_primitive_receipts.fixtures.negative] `missing-terminal.json` and `safety-overclaim.json` prove fail-closed issue reporting.
- r[impl electrical_circuit_primitive_receipts.policy] Generated policy metadata advertises the circuit primitive receipt family.
- r[impl electrical_circuit_primitive_receipts.policy.family] The policy family entry records schema, validate command, graph hash field, hash material, and required non-claims.

## Non-claims

Circuit primitive validation is evidence of reviewed representation only. It is not simulation proof, analog equivalence proof, electrical safety certification, regulatory compliance evidence, layout correctness proof, or manufacturability review.
