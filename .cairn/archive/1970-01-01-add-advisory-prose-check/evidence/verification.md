# Advisory prose integration observations

## Observed checks

- `cargo test -p cairn-cli --test prose`: seven actual-engine consumer tests passed.
- Pinned Rust 1.94.1 `cargo clippy --workspace --all-targets --features redb-registry -- -D warnings`: passed.
- `cargo fmt --all --check`: passed.
- `cairn policy export --check` with the pinned Nickel 1.17.0 cohort and flake-pinned nickel-export: policy fresh.
- The repository license-boundary checker accepted all 26 inventory rules.
- Proposal, design, and tasks gates passed; accepted-spec traceability covered 2359 declared IDs after synchronization. This is linkage, not proof of satisfaction.
- `nix build .#cairn -o target/prose-package`: passed for x86_64 Linux. The package retains the upstream MIT notice under `share/licenses/cairn/english-rs-LICENSE`.
- `nix flake check . -L` with local builders, automatic GC disabled for the invocation, the public Nix cache, and an isolated evaluator cache: all 27 x86_64 Linux checks passed, including workspace tests, Clippy, Octet, policy, traceability, and self-dogfood. Incompatible aarch64 checks were omitted; no all-systems claim is made.
- The full gate evaluated the implementation snapshot before archive/evidence-only updates. Native validation and traceability also passed after archiving the completed change.

## Live packaged CLI

From a temporary working directory outside the source checkout, the packaged binary ran with an initially empty ambient PATH and no CAIRN_POLICY override. The package supplied its bundled policy.

Input bytes were `He are late.\n` in `notes.md`. `cairn --machine prose check --root ROOT notes.md` returned success while retaining upstream E0001 at error severity. All suggestions required author review. Source bytes and the native validation receipt were unchanged. The actual inner report is retained in `packaged-report.json`.

A separate temporary-root scenario observed identical native proposal-gate and validation receipts before and after the prose check. Explicit file-count admission accepted 64 files and rejected 65 without emitting a success report. Temporary fixtures were removed.

## Claim boundary

These are local integration observations for the pinned englishc revision `336ed696eec73e5b9858c5d7952b54c8e93b9df9`. The saved language diagnostic is example output, not requirement, implementation, readiness, or release evidence. No grammar suggestion was applied. The prose command did not create a release tag or publish anything.
