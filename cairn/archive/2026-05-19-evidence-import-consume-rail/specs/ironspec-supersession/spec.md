## ADDED Requirements

### Requirement: Evidence Import Consume Rail

Cairn MUST provide a first-class evidence import command that verifies and materializes a deterministic local copy of an exported evidence bundle for downstream CI or reviewer consumption.

r[cairn.supersede.evidence_import_consume_rail.command]

#### Scenario: Import command verifies before copying

- GIVEN an evidence directory produced by `cairn evidence export`
- WHEN an operator runs `cairn evidence import --root . --bundle <dir> --into <dir>`
- THEN Cairn MUST first verify the bundle using the same deterministic checks as `cairn evidence verify`
- AND Cairn MUST emit a deterministic import receipt with the nested verification receipt hash.

r[cairn.supersede.evidence_import_consume_rail.artifacts]

#### Scenario: Import materializes declared artifacts only

- GIVEN a valid exported evidence bundle with an export receipt artifact index
- WHEN the bundle is imported into a local evidence directory
- THEN Cairn MUST copy only files declared by the export receipt artifact index
- AND Cairn MUST preserve declared relative artifact paths
- AND Cairn MUST record each imported artifact role, relative path, and BLAKE3 content hash in sorted order.

r[cairn.supersede.evidence_import_consume_rail.failures]

#### Scenario: Import rejects unsafe or undeclared bundle content

- GIVEN an exported evidence directory with a failed verifier, unsafe declared path, missing artifact, hash mismatch, or undeclared extra file
- WHEN an operator imports the bundle
- THEN Cairn MUST emit a deterministic failure receipt
- AND Cairn MUST use stable issue codes without leaking absolute bundle paths, raw host environment values, credentials, or secrets.

r[cairn.supersede.evidence_import_consume_rail.gates]

#### Scenario: Import receipt is portable across directories

- GIVEN the same valid evidence bundle imported into two distinct local directories
- WHEN Cairn emits import receipts for both imports
- THEN the portable receipt hashes MUST match
- AND the receipt MUST normalize bundle and destination output locations.

r[cairn.supersede.evidence_import_consume_rail.archive]

#### Scenario: Import rail is archived with deterministic evidence

- GIVEN the evidence import change is complete
- WHEN the change is synced and archived
- THEN the accepted spec, archive, tests, and docs MUST retain requirement traceability for the import command, artifact rules, deterministic failures, verification gates, and completion evidence.
