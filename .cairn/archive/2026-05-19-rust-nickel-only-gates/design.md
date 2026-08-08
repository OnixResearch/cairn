# Design: Rust/Nickel-only gates

## Overview

Cairn keeps deterministic project logic in the Rust CLI and Nickel policy. The CLI owns portable receipts and deterministic checks; Nickel owns declarative policy data. Nix checks may orchestrate commands, but they should not depend on checked-in Python or shell helper scripts.

## Policy export

`cairn policy export --check` shells only to `nickel export --format json` for Nickel evaluation, then performs semantic checks in Rust:

- policy JSON parses through the existing core policy parser;
- receipt hash policy remains BLAKE3 with stable canonicalization;
- lifecycle stages, marker contracts, artifact dependencies, receipt schema keys, and deterministic surface inventory are unique and internally consistent;
- invalid Nickel fixtures still fail semantic validation;
- generated policy freshness is checked against `cairn-policy/generated/cairn-policy.json`.

## Tracey bootstrap coverage

`cairn tracey coverage --root PATH` scans accepted spec markdown for `r[...]` requirements and scans Rust/Nickel source for implementation, verification, dependency, and related-reference markers. The command exits nonzero for missing or dangling refs and can emit JSON for receipts.

## Rename classification

`cairn rename-classification` keeps the historical compatibility scan in Rust and exits nonzero when `needs-fix` occurrences remain.

## CI integration

Nix checks call the built Cairn CLI and Nickel instead of Python helpers. The dev shell no longer includes Python for Cairn maintenance.
