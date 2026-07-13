## Tasks

- [x] [serial] Add `cap-std` only to CLI/support crates that own Cairn filesystem effects. r[cairn.release_evidence.cap_std_boundary.dependency]
  - Evidence 2026-07-13: workspace dependency wiring is consumed only by `crates/cairn-cli`; `cairn-core` has no `cap-std` dependency.
- [x] [serial] Introduce typed capability roots for repository, evidence bundle, registry, release, replay, and scan roots. r[cairn.release_evidence.cap_std_boundary.root_wrappers]
  - Evidence 2026-07-13: `support/capability.rs` defines the six root kinds plus bounded relative read, write, traversal, copy, and removal operations.
- [x] [serial] Convert targeted CLI/support filesystem opens to capability-relative operations while keeping `cairn-core` pure. r[cairn.release_evidence.cap_std_boundary.conversion]
  - Evidence 2026-07-13: production filesystem operations in `evidence*`, `evidence_registry*`, `release_evidence*`, `support/scan.rs`, and `determinism/replay/{execution,render}.rs` use capability roots; remaining `std::fs` calls in those trees are test fixture setup only.
- [x] [serial] Add positive fixtures for valid repository-relative artifact and evidence paths. r[cairn.release_evidence.cap_std_boundary.tests.positive]
  - Evidence 2026-07-13: capability, evidence-import traversal, replay-copy, release-evidence, registry-chain, and determinism tests cover accepted in-root paths.
- [x] [serial] Add negative fixtures for traversal, absolute paths, missing roots, symlink escapes, and replay/scan escape attempts. r[cairn.release_evidence.cap_std_boundary.tests.negative]
  - Evidence 2026-07-13: focused tests reject missing roots, `../`, absolute paths, scan-root symlink escapes, evidence-bundle symlinks, and replay-copy symlinks without materializing escaped content.
- [x] [serial] Document the filesystem authority boundary and evidence/release-readiness non-claims. r[cairn.release_evidence.cap_std_boundary.docs]
  - Evidence 2026-07-13: `README.md`, the change design, and `CAPABILITY_PATH_BOUNDARY` state the local-authority boundary and explicit evidence/replay/lifecycle/release non-claims.
- [x] [serial] Run focused evidence/replay/scan tests, Cairn validation, Tracey coverage, and gates. r[cairn.release_evidence.cap_std_boundary.validation]
  - Evidence 2026-07-13: formatting, capability/import/replay/release tests, all 7 determinism integration tests, the complete evidence-registry chain, policy freshness, Cairn validation, 1250/1250 traceability, and proposal/design/tasks gates passed. Focused Clippy passed with only the three separately staged `native/authoring.rs` findings allowlisted; unfiltered Clippy reports those same pre-existing findings. Repository-wide Tiger Style still reports the existing broad backlog (12,308 findings), including pre-existing replay module size/naming findings.
