# Convergence Specification

## Purpose

Defines the `convergence` capability.

## Requirements

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

### Requirement: Realization binding

r[cairn.convergence.realization] Cairn MUST report a target `current` only when the Mantle build record shows its job's root output admitted under the case's trusted key, that output is the directory the bridge consumed, and the bridge observation binds the registry revision, the evidence job and the receipt.

#### Scenario: A bound realization is current
- GIVEN an admitted root output and a bridge observation whose `packaging.source_revision` equals the registry revision, whose `producer_job_identity` is the BLAKE3 of the handoff `job.json`, and whose `receipt_identity` is the BLAKE3 of the handoff `external-receipt.json`
- WHEN Octet's decision passes the target's clause
- THEN the target MUST be reported `current`
- AND the receipt MUST name the job, revision, receipt and output identities it bound.

#### Scenario: A missing or mismatched realization is not current
- GIVEN no Mantle record for the job, an output signed by a key outside the trusted set, an observation made at another revision, or a job identity that does not match `job.json`
- WHEN the command runs
- THEN the target MUST be reported `unrealized` or `stale`
- AND it MUST NOT be reported `current`.

### Requirement: Octet decision is the only acceptance

r[cairn.convergence.acceptance] Cairn MUST treat only Octet's requirement decision as acceptance and MUST NOT treat a Mantle success, a cache hit, `proof_acceptance` or `externally_reported_outcome` as acceptance.

#### Scenario: A passed decision clause is accepted
- GIVEN an Octet decision with a passing aggregate whose clause for the target is `passed`, bound by `contract_blake3` to the contract file and by `evidence_blake3` to the evidence bundle
- WHEN the command evaluates the target
- THEN acceptance MUST come from that clause
- AND the bridge's relayed outcome MUST NOT change it.

#### Scenario: A relayed outcome without a passing decision is not accepted
- GIVEN a bridge observation that relays a supported outcome for a reused signed output, but an Octet clause that is not `passed` or a decision made for another contract
- WHEN the command evaluates the target
- THEN the target MUST be reported `failed` or `stale`
- AND it MUST NOT be reported `current`.

### Requirement: Closed state vocabulary

r[cairn.convergence.states] Cairn MUST report each registry row in exactly one of `untargeted`, `not_projected`, `current`, `unrealized`, `stale`, `failed`, `unsupported` or `incomplete`, and every mapping MUST fail closed.

#### Scenario: Each condition maps to its state
- GIVEN rows with no target, a `reviewed` target, a bound passing target, a missing realization, a binding mismatch, a reported failure, an Octet `unsupported` result and a reached bound
- WHEN the command runs
- THEN each row MUST receive exactly the state its condition names
- AND the vocabulary MUST NOT grow without a new receipt schema.

#### Scenario: An unrecognised outcome is never current
- GIVEN an Octet clause status or a Mantle field value that the policy does not admit
- WHEN the command maps it
- THEN Cairn MUST report `unsupported` or `incomplete`
- AND it MUST NOT report `current`.

### Requirement: Completeness

r[cairn.convergence.completeness] The receipt MUST list every registry row, MUST set `targets_current` only when every targeted row is `current` or `not_projected`, MUST set `coverage_complete` only when no row is `untargeted`, and MUST set `complete` only when both hold.

#### Scenario: Full coverage with current targets is complete
- GIVEN a registry whose every row has a target and whose every projected target is `current`
- WHEN the command runs
- THEN `targets_current`, `coverage_complete` and `complete` MUST all be true.

#### Scenario: Partial targeting is not complete
- GIVEN a specification with several rows of which only one is targeted and `current`
- WHEN the command runs
- THEN the other rows MUST be listed as `untargeted`
- AND `coverage_complete` and `complete` MUST be false while `targets_current` is true.

### Requirement: Read-only inspection

r[cairn.convergence.inspection] `cairn convergence inspect` MUST be read-only: it MUST NOT spawn a process, use the network, start Nickel, Mantle or Octet, or create, change or remove any file under the root.

#### Scenario: The root is unchanged
- GIVEN a root tree and a valid case
- WHEN the command runs
- THEN the root tree MUST be byte-identical afterwards
- AND the command MUST write the receipt only to standard output.

#### Scenario: Unknown options and a missing case fail
- GIVEN an unknown option or a missing `--case`
- WHEN the command runs
- THEN it MUST exit non-zero with a usage error
- AND it MUST NOT read any other input.

### Requirement: Native authority is preserved

r[cairn.convergence.authority] Convergence states MUST remain advisory, and the `Pass | Block` gate, readiness states, verification roles and `implementation-ready` output MUST keep their meanings and bytes.

#### Scenario: A complete receipt grants no authority
- GIVEN a receipt with `complete: true`
- WHEN lifecycle gates, readiness or verification roles are evaluated
- THEN their results MUST be unchanged
- AND the receipt MUST state that it is neither release eligibility nor proof of correctness.

#### Scenario: An incomplete receipt blocks nothing
- GIVEN a receipt with `unrealized` or `stale` targets
- WHEN lifecycle gates are evaluated
- THEN no existing gate MUST change its outcome because of the receipt
- AND the command MUST NOT write lifecycle state.

### Requirement: Deterministic receipts

r[cairn.convergence.determinism] Identical inputs MUST produce byte-identical `cairn.convergence.v1` receipts and receipt identities.

#### Scenario: Two runs agree
- GIVEN the same case and inputs
- WHEN the command runs twice
- THEN both receipts and both receipt BLAKE3 identities MUST be identical.

#### Scenario: A changed input changes the receipt
- GIVEN one changed byte in any bound input
- WHEN the command runs again
- THEN the receipt identity MUST change
- AND a row whose `current` state depended on the old input MUST NOT stay `current`.

### Requirement: Retained conformance corpus

r[cairn.convergence.conformance] Cairn MUST retain a positive case built from an exercised Mantle realization and Octet decision and one negative case per binding check, each with its source revisions recorded.

#### Scenario: The positive case reports the pilot current
- GIVEN the retained Octet pilot case
- WHEN the command runs
- THEN the pilot row MUST be `current`, every other row MUST be `untargeted`, `targets_current` MUST be true, and `coverage_complete` and `complete` MUST be false.

#### Scenario: Each negative case reports its designed state
- GIVEN a retained negative case for one binding check
- WHEN the command runs
- THEN the affected row MUST report the state the design names for that check
- AND no negative case MUST report `current`.

### Requirement: MCP inspection parity

r[cairn.convergence.mcp_inspect] The read-only MCP tool `cairn_convergence_inspect` MUST accept `case`, `root` and `policy` with the same meanings as the `cairn convergence inspect` flags and MUST return the same `cairn.convergence.v1` receipt for the same inputs. It MUST share the command's inspect path rather than duplicate its logic, MUST NOT write a file, spawn a process, use the network or start Nickel, Mantle or Octet, MUST read every input bounded and without following links, and MUST reject unknown arguments before reading any input.

#### Scenario: The MCP receipt equals the CLI receipt

- GIVEN a convergence case that the command admits
- WHEN the caller invokes `cairn_convergence_inspect` with the same `case`, `root` and `policy`
- THEN `structuredContent` MUST equal the receipt the command prints
- AND its `receipt_blake3` MUST equal the command's `receipt_blake3`
- AND a repeated call MUST return an identical result.

#### Scenario: Rejections are explicit and read nothing further

- GIVEN a request missing `case`, a request with an unknown argument, an input with a schema the policy does not admit, or a symbolic link among the case inputs
- WHEN the tool is invoked
- THEN the missing or unknown argument MUST be rejected before any input is read
- AND the unadmitted schema or linked input MUST yield the command's `status: "rejected"` receipt with its issue code
- AND no row MUST be reported `current`.
