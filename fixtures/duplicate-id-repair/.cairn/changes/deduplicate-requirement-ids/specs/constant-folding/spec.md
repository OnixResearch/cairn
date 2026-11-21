## MODIFIED Requirements

### Requirement: Transfer Function Lattice

r[constfold.lattice] The transfer function MUST stay inside the finite lattice.

#### Scenario: Lattice-closed transfer

r[constfold.finite-height.closed]
- GIVEN a transfer function for each instruction
- WHEN it maps an input value
- THEN the output value MUST remain in the lattice

### Requirement: Transfer Function Monotonicity

r[constfold.monotone] The transfer function MUST be monotone under the lattice order.

#### Scenario: Monotone transfer

r[chain.collision-free.monotone]
- GIVEN two ordered input values
- WHEN the transfer function is applied to both
- THEN the outputs MUST keep that order
