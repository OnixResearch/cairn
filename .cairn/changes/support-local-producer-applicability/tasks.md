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

## Maintained consumer cohort integration

- [ ] [serial] [task:consumer-grammars] Add exact closed maintained Cargo workspace-test, Clippy, formatting and Nix flake-check grammars with process-only non-test evidence and independent manifest admission. r[cairn.check_evidence_producer.maintained_consumer_commands]
- [ ] [serial] [task:consumer-verification] [after:consumer-grammars] Verify the new command paths and negative controls together with unchanged required producer/cohort gates; retain observed blockers before sync or archive. r[cairn.check_evidence_producer.maintained_consumer_commands]

Integration work does not mark the historical validation task complete. Builds, tests, lint and formatters are deferred to the integration owner; no new passing gate, sync, archive, readiness or publication claim exists.


## Integration verification commands (not executed here)

Run from the integrated Cairn worktree with the freshly built CLI, never the retained historical producer binary. Preserve a bounded explicit invocation PATH so the existing 4,096-byte per-environment-value check is not bypassed or accidentally exceeded.

```sh
env PATH=/run/current-system/sw/bin:/etc/profiles/per-user/brittonr/bin RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop --no-update-lock-file -c cargo test -p cairn-core check_evidence
env PATH=/run/current-system/sw/bin:/etc/profiles/per-user/brittonr/bin RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop --no-update-lock-file -c cargo test -p cairn-cli --test check_evidence_local --test check_evidence_nix
nix run path:$PWD#cairn -- policy export --check
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- traceability coverage --root . --json
nix run path:$PWD#cairn -- gate proposal support-local-producer-applicability --root .
nix run path:$PWD#cairn -- gate design support-local-producer-applicability --root .
nix run path:$PWD#cairn -- gate tasks support-local-producer-applicability --root .
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
nix develop --no-update-lock-file -c cargo octet check --output-format json --artifact-dir target/producer53-octet
nix build path:$PWD#checks.x86_64-linux.octet --no-link -L
nix flake check path:$PWD -L
```

The integration owner must additionally exercise all four maintained consumer profiles through the actual `source`, `dependencies`, `plan`, `run`, `verify`, and original completion commands against the reviewed ChaosControl source, with new BLAKE3 attempt/artifact identities. Require `valid=true`, complete publication, exact required results and dependencies; an exit-zero non-passing report is not evidence of success. Retain the independent native and KVM release-matrix obligations; neither a flake process observation nor a portable completion report replaces them.

No commands in this section ran in the producer integration worktree. The historical receipts remain byte-for-byte reused evidence for their recorded source and cohort only. Environmental timeouts remain unresolved until fresh commands finish. The two current license links are already present in baseline source; their fresh gate verdict remains unobserved. Octet's six recorded noncanonical source-map paths remain an explicit blocker unless a scoped source repair is independently verified.

