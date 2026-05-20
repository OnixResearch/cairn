# ironspec-supersession delta: registry mirror verification rail

## ADDED Requirements

### Requirement: Registry Mirror Verification Command

r[cairn.supersede.registry_mirror_verification.command] The system MUST provide a read-only CLI command that verifies a local mirror directory contains a registry artifact and every index declared by that registry through safe relative references.

#### Scenario: Valid local registry mirror

- GIVEN a registry artifact and mirrored index artifacts rooted under a local mirror directory
- WHEN the operator runs `cairn evidence registry mirror-verify --root <root> --registry <registry.json> --mirror <mirror-dir>`
- THEN the command SHALL verify the registry and every declared mirrored index
- AND it SHALL emit a valid deterministic mirror verification receipt.

### Requirement: Registry Mirror Verification Receipt

r[cairn.supersede.registry_mirror_verification.receipt] The system MUST emit a portable registry mirror verification receipt that records stable registry/index hashes, entry counts, label summaries, and normalized root/mirror fields without exposing absolute paths, secrets, URLs, or environment values.

#### Scenario: Portable mirror proof

- GIVEN the same registry mirror content copied under two different absolute directories
- WHEN mirror verification runs for each copy
- THEN the stable mirror receipt hashes SHALL match
- AND display-only path fields SHALL remain normalized placeholders.

### Requirement: Registry Mirror Failure Receipts

r[cairn.supersede.registry_mirror_verification.failures] The system MUST reject missing, unsafe, unreadable, or tampered mirrored registry/index artifacts with stable machine-readable issue codes and without mutating the mirror.

#### Scenario: Missing mirrored index

- GIVEN a registry whose declared index file is absent from the mirror directory
- WHEN mirror verification runs
- THEN the receipt SHALL be invalid
- AND it SHALL include a deterministic issue code identifying the missing mirrored index.

#### Scenario: Tampered mirrored index

- GIVEN a registry whose declared index file exists in the mirror directory but its content hash no longer matches the registry declaration
- WHEN mirror verification runs
- THEN the receipt SHALL be invalid
- AND it SHALL include a deterministic issue code identifying the mirrored index hash mismatch.
