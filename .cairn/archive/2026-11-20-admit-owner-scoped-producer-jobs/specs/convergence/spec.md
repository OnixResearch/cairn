## MODIFIED Requirements

### Requirement: Bounded and bound inputs

r[cairn.convergence.inputs] `cairn convergence inspect` MUST read every input bounded and without following links, MUST reject schema identifiers the policy does not admit, and MUST bind the registry, targets, contracts, decisions, evidence bundles, Mantle build records and bridge handoffs together before it reports any state.

The handoff `job.json` MUST declare a non-empty `owner`, and its `schema_version` MUST equal the producer-job schema the policy admits for that owner. A missing owner, an owner the policy does not admit, and a `schema_version` that does not match the owner's admitted schema MUST each fail closed with a stable issue code, and no affected row MUST be reported `current`.

#### Scenario: Bounded inputs are admitted and identified
- GIVEN a case file naming a registry, exported targets and, per contract, the contract, Octet decision, evidence bundle, bridge handoff directory, producer output and Mantle build record
- WHEN the command runs
- THEN every input MUST be read within its byte bound and bound by its recorded identity
- AND the receipt MUST record the BLAKE3 identity of each input.

#### Scenario: Linked, oversized or unknown inputs fail closed
- GIVEN an input that is a symbolic link, exceeds its bound, escapes its root or carries a schema identifier the policy does not admit
- WHEN the command runs
- THEN Cairn MUST reject the input with an explicit issue code
- AND no affected row MUST be reported `current`.

#### Scenario: Producer jobs are admitted by their own owner
- GIVEN a policy whose owner-scoped producer-job map admits Octet's and Trellis's job schemas, and a handoff `job.json` that names an owner the policy admits with that owner's schema
- WHEN the command runs
- THEN the job MUST be admitted and its `job_id` MUST still bind the target's `evidence_job.job_id`
- AND a job with a missing owner, an unknown owner, or a schema that does not match its owner's admitted schema MUST be rejected with `convergence.missing_producer_owner`, `convergence.unknown_producer_owner` or `convergence.producer_schema_mismatch` respectively.
