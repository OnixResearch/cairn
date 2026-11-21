## MODIFIED Requirements

### Requirement: Commutativity

r[fetch-add.commutativity] The system MUST return the same fetched value for swapped operands.

#### Scenario: Swapped operands

r[fetch-add.associativity.commutative]
- GIVEN two operands in different speculation slots
- WHEN the operand order is swapped
- THEN the fetched value MUST be equal
