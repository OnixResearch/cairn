## ADDED Requirements

### Requirement: Evidence Publish Index Rail

Cairn MUST provide first-class deterministic evidence index generation and verification commands for published evidence bundles.

r[cairn.supersede.evidence_publish_index_rail.command]

#### Scenario: Index command catalogs a bundle

- GIVEN an evidence bundle produced by export, import, or republish
- WHEN an operator runs `cairn evidence index --root . --bundle <dir> --out <file>`
- THEN Cairn MUST detect the bundle kind deterministically
- AND Cairn MUST write a portable index receipt to the requested output file
- AND the index MUST include artifact roles, relative paths, BLAKE3 hashes, and bound receipt-chain hashes.

r[cairn.supersede.evidence_publish_index_rail.verify]

#### Scenario: Index verify rechecks indexed evidence

- GIVEN an evidence index file
- WHEN an operator runs `cairn evidence index verify --root . --index <file>`
- THEN Cairn MUST validate the index self hash
- AND Cairn MUST re-detect and re-hash the referenced bundle
- AND Cairn MUST compare the current bundle kind, artifacts, and receipt-chain hashes with the index.

r[cairn.supersede.evidence_publish_index_rail.failures]

#### Scenario: Index commands reject ambiguous or tampered evidence

- GIVEN missing, ambiguous, unsafe, tampered, or path-traversing bundle/index inputs
- WHEN index generation or verification is attempted
- THEN Cairn MUST emit deterministic failure receipts
- AND Cairn MUST use stable issue codes without leaking host paths, raw environment values, credentials, or secrets.

r[cairn.supersede.evidence_publish_index_rail.gates]

#### Scenario: Index rail gates pass

- GIVEN the index rail is implemented
- WHEN final project gates run
- THEN policy export checks, Rust tests, clippy, Cairn validation, Tracey coverage, coverage audit, release-readiness, evidence export, evidence verify, evidence import, evidence republish, evidence index, evidence index verify, whitespace checks, and Nix flake checks MUST pass.

r[cairn.supersede.evidence_publish_index_rail.archive]

#### Scenario: Index rail is archived with deterministic evidence

- GIVEN the index rail has passed final gates
- WHEN the change is completed
- THEN the change MUST be synced into accepted specs, archived with the current date, committed, and pushed.
