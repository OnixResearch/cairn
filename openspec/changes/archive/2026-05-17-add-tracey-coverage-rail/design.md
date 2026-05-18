# Design: Tracey coverage rail

## Approach

Tracey owns interactive coverage UX via `.config/tracey/config.styx`. Cairn adds a small deterministic helper that scans accepted specs and Rust comments for the same marker shape so `nix flake check` can enforce a bootstrap baseline without packaging Tracey.

## Boundaries

- Tracey configuration is repository metadata, not runtime policy.
- Coverage scanning in the Nix check is a repository verification helper, not `cairn-core` logic.
- `cairn-core` remains free of filesystem/process/presentation effects.

## Baseline

The first rail requires every accepted requirement marker to have at least one `impl` or `verify` reference. It does not yet require both for every marker because scenario-level IDs and umbrella requirements need gradual curation.
