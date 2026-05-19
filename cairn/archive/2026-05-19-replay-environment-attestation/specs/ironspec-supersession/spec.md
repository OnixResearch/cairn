## MODIFIED Requirements

### Requirement: Determinism replay oracle

The system MUST emit a stable, policy-owned environment attestation in replay receipts so agents and CI can prove which reviewed execution facts shaped replay without leaking host-specific paths, raw environment values, or secrets.

#### Scenario: Replay receipt includes environment attestation

r[cairn.supersede.replay_environment_attestation.policy]
r[cairn.supersede.replay_environment_attestation.receipt]

- GIVEN a repository with generated replay policy
- WHEN `cairn determinism replay --root <root>` runs successfully
- THEN the receipt includes `environment_attestation`
- AND the attestation includes stable Cairn version, generated policy hash, entropy policy hash, selected cases, selected groups, replay scope, policy fact disclosures, and an attestation hash
- AND repeated and cross-root replay produce the same attestation hash for equivalent selected cases/groups
- r[task cairn.supersede.replay_environment_attestation.policy]
- r[task cairn.supersede.replay_environment_attestation.receipt]
- r[verify cairn.supersede.replay_environment_attestation.policy]
- r[verify cairn.supersede.replay_environment_attestation.receipt]

#### Scenario: Environment attestation is secret safe

r[cairn.supersede.replay_environment_attestation.secret_safe]

- GIVEN host environment variables and checkout paths may contain sensitive values
- WHEN replay renders environment attestation or deterministic rejection receipts
- THEN the attestation exposes reviewed source names, classes, handling modes, package/version identity, and hashes only
- AND it does not expose raw environment values, usernames, absolute checkout roots, or machine-specific temporary paths
- r[task cairn.supersede.replay_environment_attestation.receipt]
- r[verify cairn.supersede.replay_environment_attestation.secret_safe]

### Requirement: Agent receipt replay evidence

Agent receipt validation SHOULD accept optional replay environment attestation hash evidence and report or reject malformed attestation evidence deterministically without making the field mandatory for older wrappers.

#### Scenario: Agent receipt binds optional replay attestation hash

r[cairn.supersede.replay_environment_attestation.agent]

- GIVEN an agent-run receipt with replay evidence from a passing `agent` replay group
- WHEN the replay evidence includes `replay_environment_attestation_hash`
- THEN validation requires the value to be a BLAKE3 hex hash
- AND validation reports the optional attestation hash in the replay evidence summary
- AND malformed optional attestation hash evidence emits a deterministic issue code
- r[task cairn.supersede.replay_environment_attestation.agent]
- r[verify cairn.supersede.replay_environment_attestation.agent]

#### Scenario: Replay attestation change is verified and archived

r[cairn.supersede.replay_environment_attestation.docs]
r[cairn.supersede.replay_environment_attestation.verification]
r[cairn.supersede.replay_environment_attestation.archive]

- GIVEN implementation, docs, schema, tests, and fixtures are updated
- WHEN the change is completed
- THEN Cairn validation, Tracey coverage, policy export checks, focused/full Rust tests, release-readiness, and replay smokes pass before archive
- r[task cairn.supersede.replay_environment_attestation.docs]
- r[task cairn.supersede.replay_environment_attestation.verification]
- r[task cairn.supersede.replay_environment_attestation.archive]
- r[verify cairn.supersede.replay_environment_attestation.docs]
- r[verify cairn.supersede.replay_environment_attestation.verification]
- r[verify cairn.supersede.replay_environment_attestation.archive]
