# Change: Add Tracey coverage rail

## Why

Cairn already uses stable `r[...]` requirement identifiers, but there is no first-class way to connect accepted requirements to implementation and verification references.

## What Changes

- Add Tracey workspace configuration for accepted Cairn specs and Rust sources/tests.
- Add implementation and verification annotations for current core/CLI coverage.
- Add a deterministic coverage summary helper and Nix check that can tighten coverage without requiring Tracey to be vendored into Cairn.

## Impact

- Operators can run Tracey queries from the repository root.
- Flake verification gains a lightweight traceability rail.
- Tracey remains optional tooling; Cairn runtime does not depend on Tracey.
