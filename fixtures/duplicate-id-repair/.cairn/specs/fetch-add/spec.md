# Fetch Add Specification

## Purpose

Define speculative fetch-add execution for a bounded operand pair.

## Requirements

### Requirement: Commutativity

r[fetch-add.associativity] The system MUST return the same fetched value for swapped operands.

#### Scenario: Swapped operands

r[fetch-add.associativity.commutative]
- GIVEN two operands in different speculation slots
- WHEN the operand order is swapped
- THEN the fetched value MUST be equal

### Requirement: Associativity

r[fetch-add.associativity] The system MUST group fetched additions consistently.

#### Scenario: Grouped additions

r[fetch-add.associativity.grouped]
- GIVEN three operands across two speculation slots
- WHEN the additions are grouped differently
- THEN the fetched value MUST be equal
