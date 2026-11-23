# Buoy comparison acceptance state

Status: **complete for the bounded declared comparison scope; ready to sync and archive.**

## Verified

- `cargo test -p cairn-core --lib traceability` → **17/17 pass** (including 3/3 `traceability_comparison` unit tests).
- `cargo test -p cairn-cli --test buoy_comparison` → **4/4 pass**:
  - `comparison_rejects_unknown_manifest_schema`
  - `comparison_executes_native_fixtures_and_retains_raw_observations`
  - `comparison_rejects_restated_observations_and_unsupported_pass`
  - `comparison_identity_binds_source_mapping_fixture_and_policy`
- `cargo test -p cairn-cli --test trace_index` → **4/4 pass** (existing trace-index paths remain independent and unchanged).
- Executed the full `buoy-traceability-comparison` check flow from `flake.nix` (`cairn traceability compare --root . --fixtures fixtures/buoy-comparison/v1`):
  - `valid: true`, `verdict: "pass"`, `complete: false`, `upstream_executed: false`, `artifact_available: false`, `upstream.disposition: "blocked"`, `planted_drift_detected: true`, `restated_observation_rejected: true`, `restated_native_expectation_rejected: true`, 11 cases, 0 issues.
  - Fixture SHA-256 checksums unchanged before/after execution.
  - Mutated restated-observation fixture rejected (`valid: false`, `comparison.mutation_restated`).

## Historical defects resolved

- Two compile defects were fixed during initial acceptance (`domain_hash_json` input type) and inverted/shared `restated_observation_rejected` / `restated_native_expectation_rejected` flags were separated (`b9a8d4a`).
- `comparison_identity_binds_source_mapping_fixture_and_policy` passes cleanly on current `main`.
- Added `r[impl ...]` and `r[verify ...]` annotations for `cairn.trace_index.buoy_comparison.{pinned_identity,projection,mutation,boundary}` so repository traceability coverage remains 100% after spec sync.

## Claim boundary

Upstream Buoy was **not executed**: `upstream.executed=false`, `artifact_available=false`, `disposition="blocked"`, and `complete=false`. All upstream observations remain declared fixtures from pinned revision `01096fbdd0b9bf39b2fec468f90e7b1db958f615`. Structural comparison does not prove Buoy execution, cross-implementation agreement, requirement satisfaction, implementation correctness, proof soundness, or release eligibility.
