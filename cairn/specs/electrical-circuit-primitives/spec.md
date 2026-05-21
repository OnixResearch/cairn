# Electrical Circuit Primitives Specification

## Purpose

Defines deterministic electrical circuit primitive evidence contracts for reviewed topology, component, unit, receipt, and non-claim boundaries.

## Requirements

### Requirement: Circuit topology primitives
r[electrical_circuit_primitives.topology] The system MUST define a deterministic topology vocabulary for circuit evidence graphs that includes nodes, ports, terminals, nets, branches, reference nodes, and measurement points.

#### Scenario: Reference node is explicit
r[electrical_circuit_primitives.topology.reference_node]

- GIVEN a circuit primitive graph with one or more connected nets
- WHEN Cairn validates or renders the graph receipt
- THEN the receipt identifies the reference node or reports that no reference node has been declared.

#### Scenario: Connection arity is reviewable
r[electrical_circuit_primitives.topology.connection_arity]

- GIVEN a component primitive with declared ports or terminals
- WHEN the primitive is included in a circuit graph
- THEN the receipt records each terminal-to-net binding and fails closed for missing or duplicate required terminal bindings.

### Requirement: Electrical component primitives
r[electrical_circuit_primitives.components] The system MUST define a closed first-pass component vocabulary for passive two-terminal primitives, ideal declared sources, switching primitives, and explicitly caveated behavioral or vendor-model components.

#### Scenario: Passive primitives carry typed values
r[electrical_circuit_primitives.components.passive_primitives]

- GIVEN a resistor, capacitor, or inductor primitive
- WHEN the primitive is serialized into a receipt
- THEN the receipt records its primitive kind, terminal bindings, SI-dimensioned value, tolerance when known, and polarity/orientation where applicable.

#### Scenario: Behavioral and vendor models are caveated
r[electrical_circuit_primitives.components.model_caveats]

- GIVEN a behavioral source, semiconductor model, or vendor-provided component model
- WHEN the primitive is serialized into a receipt
- THEN the receipt records model provenance and caveats instead of treating the model as a proven physical equivalent.

### Requirement: Dimensioned electrical values
r[electrical_circuit_primitives.units] The system MUST require electrical primitive values to carry deterministic unit and dimension metadata instead of untyped display strings.

#### Scenario: SI dimensions are machine-readable
r[electrical_circuit_primitives.units.dimensioned_values]

- GIVEN a primitive value such as resistance, capacitance, inductance, voltage, current, frequency, or power
- WHEN the value appears in a circuit primitive receipt
- THEN the receipt records a normalized numeric value, SI dimension metadata, display unit, and normalization version.

### Requirement: Circuit primitive receipts
r[electrical_circuit_primitives.receipts] The system MUST define portable receipt evidence for circuit primitive graphs that includes canonical primitive hashes, graph hashes, source artifact hashes, normalization metadata, tool identity, and explicit caveats.

#### Scenario: Primitive and graph hashes are portable
r[electrical_circuit_primitives.receipts.portable_hashes]

- GIVEN the same logical circuit primitive graph emitted under two repository roots or temporary output directories
- WHEN canonical primitive and graph hashes are computed
- THEN hash material excludes display-only absolute paths and ambient entropy while preserving source artifact hashes and reviewed primitive fields.

### Requirement: Circuit evidence non-claims
r[electrical_circuit_primitives.non_claims] The system MUST preserve explicit non-claims that circuit primitive agreement is evidence of reviewed representation only and is not proof of simulation correctness, analog equivalence, electrical safety, regulatory compliance, physical layout correctness, or manufacturability.

#### Scenario: Safety and compliance are not implied
r[electrical_circuit_primitives.non_claims.no_safety_claim]

- GIVEN a circuit primitive receipt with matching canonical hashes
- WHEN Cairn renders, validates, or republishes the receipt
- THEN the receipt MUST NOT claim electrical safety, regulatory compliance, manufacturability, layout correctness, or analog behavioral correctness unless separate reviewed evidence is present.
