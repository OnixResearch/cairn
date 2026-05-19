## ADDED Requirements

### Requirement: Evidence Import Republish Rail

Cairn MUST provide a first-class evidence republish command that verifies an imported evidence directory and materializes a deterministic re-published evidence bundle.

r[cairn.supersede.evidence_import_republish_rail.command]

#### Scenario: Republish command verifies imported evidence

- GIVEN an evidence directory produced by `cairn evidence import`
- WHEN an operator runs `cairn evidence republish --root . --imported <dir> --out <dir>`
- THEN Cairn MUST verify the imported evidence before copying artifacts
- AND Cairn MUST write a deterministic `evidence-republish-receipt.json` into the output directory
- AND the receipt MUST use stable placeholders for input and output paths.

r[cairn.supersede.evidence_import_republish_rail.artifacts]

#### Scenario: Republish materializes declared imported artifacts only

- GIVEN an imported evidence directory with `evidence-import-receipt.json`
- WHEN Cairn republishes the evidence
- THEN Cairn MUST copy only artifact paths declared by the import receipt
- AND Cairn MUST preserve each declared artifact role, relative path, and hash
- AND Cairn MUST bind the republish receipt to the import receipt hash.

r[cairn.supersede.evidence_import_republish_rail.failures]

#### Scenario: Republish rejects invalid imported evidence

- GIVEN missing, tampered, duplicate, extra, unsafe, or path-traversing imported artifacts
- WHEN republish is attempted
- THEN Cairn MUST refuse to materialize a successful republish
- AND Cairn MUST emit deterministic issue codes without leaking host-specific paths, environment values, credentials, or secrets.

r[cairn.supersede.evidence_import_republish_rail.gates]

#### Scenario: Republish rail gates pass

- GIVEN the republish rail is implemented
- WHEN final project gates run
- THEN policy export checks, Rust tests, clippy, Cairn validation, Tracey coverage, coverage audit, release-readiness, evidence export, evidence verify, evidence import, evidence republish, whitespace checks, and Nix flake checks MUST pass.

r[cairn.supersede.evidence_import_republish_rail.archive]

#### Scenario: Republish rail is archived with deterministic evidence

- GIVEN the republish rail has passed final gates
- WHEN the change is completed
- THEN the change MUST be synced into accepted specs, archived with the current date, committed, and pushed.
