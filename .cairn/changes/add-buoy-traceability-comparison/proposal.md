# Add pinned Buoy traceability comparison qualification

## Why

Cairn retains a content-bound trace index with a repository-scale live proof, but it has no source-identified comparison against a second implementation of requirement/source binding.
Issue #3 asks for a bounded experiment that pins the reviewed Buoy revision, compares declared structural observations, and proves through positive and negative mutation fixtures that the comparison detects planted drift instead of restating current output.

The pinned upstream consumer package sets `doCheck = false`, and full upstream acceptance requires private Atoll access.
No realized Buoy binary or derivation is available in this environment, so an executed upstream comparison is not reachable here.
This change therefore implements the bounded substitute the issue allows: a versioned declared comparison fixture set, executed native mutation qualification, and an explicit blocked upstream disposition.
It does not claim that Buoy ran.

## What Changes

- Retain one pinned upstream identity: revision, recursive source content hash, Rust toolchain, BLAKE3 feature selection, read-only command surface, machine report schemas, and evidence locations.
- Add a closed `cairn.buoy-comparison.fixtures.v1` manifest with a declared semantic subset and an explicit projection version that preserves original Cairn requirement and source identities.
- Add `traceability compare`, which executes the ordinary read-only trace-index commands on saved temporary fixture copies, compares their observable obligations with independently declared native expectations, and rejects restated mutation observations.
- Add mutation fixtures for missing marker, duplicate marker, moved span, changed requirement body, and schema/version drift, plus Unicode/CRLF and `cfg(not(any()))` expected-divergence cases.
- Retain raw native stdout/stderr/exit observations, per-producer report identities, and a comparison identity bound to source, tool, mapping, policy, and fixture bytes.
- Add a dedicated pinned Nix check and consumer-visible integration tests.
- Record the measured decision to retain the bounded declared comparator without permanent runtime adoption while the pinned artifact stays unrealized.

## Impact

- **Files**: core comparison evaluator, traceability CLI, comparison fixtures, integration tests, Nix check, and change specification.
- **Assurance**: report-only. Structural comparison does not prove behavior, requirement satisfaction, proof soundness, or release eligibility. Existing traceability gate meanings are unchanged.
- **Compatibility**: adds an opt-in command and new fixture artifacts; existing export, verify, show, and live-proof paths keep their current schemas and defaults.
