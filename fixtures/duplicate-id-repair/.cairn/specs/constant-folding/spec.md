# Constant Folding Specification

## Purpose

Define a bounded constant-folding pass over a control-flow graph.

## Requirements

### Requirement: Lattice Properties

r[constfold.finite-height] The lattice MUST have a finite height under the fold order.

#### Scenario: Finite ascent

r[constfold.finite-height.ascent]
- GIVEN a lattice with declared maximum height
- WHEN folded values ascend
- THEN the analysis MUST stop within that height

### Requirement: Transfer Function Lattice

r[constfold.finite-height] The transfer function MUST stay inside the finite lattice.

#### Scenario: Lattice-closed transfer

r[constfold.finite-height.closed]
- GIVEN a transfer function for each instruction
- WHEN it maps an input value
- THEN the output value MUST remain in the lattice

### Requirement: Transfer Function Monotonicity

r[chain.collision-free] The transfer function MUST be monotone under the lattice order.

#### Scenario: Monotone transfer

r[chain.collision-free.monotone]
- GIVEN two ordered input values
- WHEN the transfer function is applied to both
- THEN the outputs MUST keep that order

### Requirement: Chain Collision Freedom

r[chain.collision-free] The fold chains MUST stay collision-free.

#### Scenario: Distinct chains

r[chain.collision-free.distinct]
- GIVEN two fold chains over the same block
- WHEN each chain is assigned a key
- THEN the keys MUST differ
