# Fetch Execute Policy and Documentation Delta

## ADDED Requirements

### Requirement: Registry fetch execution policy surface
r[cairn.supersede.fetch_execute_policy_docs.policy] The system MUST publish `evidence registry fetch-execute` as a reviewed deterministic receipt surface in Cairn-owned policy.

#### Scenario: Receipt schema catalog includes fetch execution
r[cairn.supersede.fetch_execute_policy_docs.policy.schema]
- GIVEN the Nickel-authored Cairn policy source
- WHEN the generated policy is exported
- THEN the receipt schema catalog includes `evidence registry fetch-execute`
- AND the schema lists stable top-level fields for plan/source/out placeholders, linked receipt hashes, materialized artifact summaries, validity, issues, verdict, and receipt hash

#### Scenario: Deterministic surface inventory includes fetch execution
r[cairn.supersede.fetch_execute_policy_docs.policy.inventory]
- GIVEN the deterministic surface inventory in Cairn policy
- WHEN the coverage audit evaluates public receipt-producing commands
- THEN `evidence registry fetch-execute` is present with a stable command id, replay coverage expectations, receipt command, and determinism claim

### Requirement: Registry fetch execution operator documentation
r[cairn.supersede.fetch_execute_policy_docs.docs] The system MUST document the operator-facing registry fetch execution flow and its non-claims.

#### Scenario: Operator flow documents local materialization
r[cairn.supersede.fetch_execute_policy_docs.docs.flow]
- GIVEN an operator has a verified registry fetch-plan receipt and a local/offline artifact source directory
- WHEN they read Cairn operator documentation
- THEN the docs show how to run `evidence registry fetch-execute`
- AND they explain that the command materializes planned artifacts using safe relative paths and BLAKE3 verification before and after copying

#### Scenario: Documentation avoids network and credential overclaims
r[cairn.supersede.fetch_execute_policy_docs.docs.non_claims]
- GIVEN fetch execution is a local/offline verification and materialization command
- WHEN docs describe the command
- THEN they state that Cairn does not perform network transport, credential handling, or remote trust validation in this flow
