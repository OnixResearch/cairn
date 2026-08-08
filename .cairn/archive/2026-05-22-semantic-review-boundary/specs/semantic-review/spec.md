# Semantic Review Boundary Specification

## Purpose

Defines the `semantic-review-boundary` capability for Cairn semantic trust/review workflows while preserving deterministic receipt and policy boundaries.

## Requirements

### Requirement: The system
r[cairn.semantic_review.semantic_review_boundary.boundary] The system MUST define and enforce the boundary between semantic review production and Cairn receipt validation.

#### Scenario: Boundary Non Execution
r[cairn.semantic_review.semantic_review_boundary.boundary.boundary.non_execution]
- GIVEN a semantic review validation command
- WHEN it runs
- THEN Cairn does not call remote LLMs, network services, or reviewer tools

#### Scenario: Boundary Claims
r[cairn.semantic_review.semantic_review_boundary.boundary.boundary.claims]
- GIVEN semantic review documentation and receipts
- WHEN an operator inspects them
- THEN they state that Cairn validates externally produced evidence rather than proving semantic correctness

### Requirement: The system
r[cairn.semantic_review.semantic_review_boundary.fcis] The system MUST preserve the functional-core/imperative-shell boundary for semantic review support.

#### Scenario: Fcis Pure Core
r[cairn.semantic_review.semantic_review_boundary.fcis.fcis.pure_core]
- GIVEN semantic review validation logic
- WHEN it is implemented
- THEN schema and policy checks live in pure core code without filesystem, process, network, or clock access

#### Scenario: Fcis Cli Shell
r[cairn.semantic_review.semantic_review_boundary.fcis.fcis.cli_shell]
- GIVEN artifact hash validation against a repository root
- WHEN it is implemented
- THEN filesystem reads and JSON presentation remain in the CLI shell
