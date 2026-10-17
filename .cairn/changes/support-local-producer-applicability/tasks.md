# Local producer applicability tasks

## Foundation

- [x] [serial] [task:foundation] Record the existing producer baseline and pass proposal, design, and tasks gates before implementation. r[cairn.check_evidence_producer.dependency_observation] r[cairn.check_evidence_producer.native_nix_build]

## Implementation and evidence

- [x] [parallel] [task:dependencies] [after:foundation] Implement bounded dependency projection observation and exact rechecks in execution, publication, and verification with positive and negative conformance cases. r[cairn.check_evidence_producer.dependency_observation]
- [x] [parallel] [task:nix] [after:foundation] Cut over to real Nix builds, admit native payloads, and preserve build-only meaning through original completion and manifest consumers with positive and negative conformance cases. r[cairn.check_evidence_producer.native_nix_build]
- [ ] [serial] [task:validation] [after:dependencies] [after:nix] Exercise the actual CLI paths, run existing required repository checks, and retain bounded evidence and documentation before reviewed sync and archive. r[cairn.check_evidence_producer.dependency_observation] r[cairn.check_evidence_producer.native_nix_build]

Foundation commands, verdicts, baseline failures, and evidence boundaries are recorded in `evidence/foundation.md`.

## Observed verification boundary

The actual local producer suite passed all 12 cases, including dependency drift, special-file and byte bounds, publication failures, cancellation, and the isolated developer-loop consumer. All three native Nix cases passed: real realization, failed realization, and an exit-zero empty build set. Independent CLI and read-only MCP controls also passed; no completion rows were hand-authored. Exact commands, report fields, executable identity, and raw-log identities are retained in `evidence/producer-receipt.ncl` and its observation input.

The validation task remains unchecked. The unchanged full workspace command reached the later `evidence_registry` executable but exceeded its 600-second outer bound; no complete workspace verdict exists. Packaged policy freshness and the Nix flake check timed out while waiting for the global garbage-collector lock. Native validation passed; native traceability retained `cairn.license_boundary.policy_reuse_grant` and `cairn.license_boundary.policy_reuse_scope` as missing, and Octet rejected noncanonical finding paths. Clippy with warnings denied and formatting passed. No required check, policy, dependency, or bound was weakened. No archive, main integration, release readiness, or full ChaosControl MCP completion is claimed.
