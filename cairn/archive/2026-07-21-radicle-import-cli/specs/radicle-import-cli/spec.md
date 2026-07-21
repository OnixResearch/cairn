# Radicle Import CLI Delta

## ADDED Requirements

### Requirement: Lossless import plan
r[cairn.radicle_import_cli.lossless_plan] The system MUST deterministically map every present supported native active-change artifact into a validated `xyz.onixresearch.cairn-change` root without changing its UTF-8 bytes.

#### Scenario: Present artifacts map exactly
r[cairn.radicle_import_cli.lossless_plan.exact]
- GIVEN a native active change with proposal, design, tasks, metadata, and delta-spec artifacts
- WHEN Cairn builds the Radicle import plan
- THEN each present artifact MUST map to its stable COB artifact identifier with exact content bytes
- AND malformed delta-spec identifiers or protocol-limit violations MUST fail before Radicle mutation

### Requirement: Current delegate authority
r[cairn.radicle_import_cli.current_authority] The system MUST authorize Radicle import execution from the repository's current canonical identity document rather than solely from a historical operation identity document or caller assertion.

#### Scenario: Current delegate is authorized
r[cairn.radicle_import_cli.current_authority.allowed]
- GIVEN the selected profile actor is a delegate in the current canonical repository identity document
- WHEN Cairn evaluates import authority
- THEN the authority result MUST identify the actor and checked canonical identity OID

#### Scenario: Non-delegate is rejected
r[cairn.radicle_import_cli.current_authority.denied]
- GIVEN the selected profile actor is absent from the current canonical repository identity document delegates
- WHEN Cairn evaluates import execution
- THEN Cairn MUST reject the mutation before creating a COB operation

### Requirement: Signed typed creation
r[cairn.radicle_import_cli.signed_creation] The system MUST create and reload the import through Radicle's typed in-process store using the checked current identity OID and signing actor.

#### Scenario: Executed import matches preview
r[cairn.radicle_import_cli.signed_creation.match]
- GIVEN an authorized actor and a valid previewed import plan
- WHEN the operator executes the import
- THEN the created object MUST reload as `xyz.onixresearch.cairn-change`
- AND its semantic content root MUST equal the previewed content root

#### Scenario: Concurrent local imports serialize
r[cairn.radicle_import_cli.signed_creation.concurrent]
- GIVEN two local processes attempt to import the same change through the same explicit Radicle home
- WHEN one process holds the change import lock
- THEN the other process MUST fail before signed COB creation
- AND the process holding the lock MUST re-check current authority and existing objects while serialized

### Requirement: Preview-first import command
r[cairn.radicle_import_cli.command] The system MUST expose `cairn radicle import <change>` with explicit repository root, Radicle home, repository ID, policy, and execution-mode inputs.

#### Scenario: Preview does not mutate
r[cairn.radicle_import_cli.command.preview]
- GIVEN a valid active change and explicit Radicle context
- WHEN the operator omits `--execute`
- THEN Cairn MUST emit the import plan without creating or updating Radicle refs

#### Scenario: Hidden profile context is rejected
r[cairn.radicle_import_cli.command.explicit_context]
- GIVEN the operator omits `--rad-home` or `--rid`
- WHEN Cairn parses the import command
- THEN Cairn MUST reject the request rather than infer mutable authority from ambient profile state

### Requirement: Import receipt
r[cairn.radicle_import_cli.receipt] The system MUST emit a deterministic BLAKE3-bound receipt identifying the selected change, repository, actor, canonical identity OID, base Git OID, policy hash, artifact hashes, semantic content root, execution mode, mutation result, and created object when present.

#### Scenario: Blocked execution is visible
r[cairn.radicle_import_cli.receipt.blocked]
- GIVEN import execution is unauthorized or its validated inputs drift
- WHEN Cairn emits the result
- THEN the receipt MUST report no successful mutation and MUST include deterministic blocker diagnostics

### Requirement: Bounded import claims
r[cairn.radicle_import_cli.non_claims] The system MUST state that a passing local import receipt does not prove gate validity, evidence truth, dependency readiness, network publication, peer availability, future authority, source correctness, archive execution, or release eligibility.

#### Scenario: Local creation is not peer publication
r[cairn.radicle_import_cli.non_claims.local_only]
- GIVEN Cairn successfully creates a local signed change COB
- WHEN the receipt and operator documentation describe the result
- THEN they MUST limit the claim to local typed storage under the recorded point-in-time identity document
- AND they MUST NOT claim live peer synchronization or remote trust
