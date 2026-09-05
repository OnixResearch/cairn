# Design

## Boundary and reuse

Cairn owns the Nix composition and operator guidance.
Reuse the existing Octet `lib.mkConsumerCheck` and `packages.<system>.cargo-octet` contracts.
The selected source is `OnixResearch/octet` at `2f8213a88781d9d38bb81319d06a50805d48ccd3`.
Its Nix NAR identity is `sha256-OvqB00mDIsGDOcuK5jvPJUModVz9WbIBuM/l06TABHc=`.
Nix requires SHA-256 for this identity. New evidence identities use BLAKE3.

Source inspection confirms canonical Octet package exports and precedence for Octet config tables at this revision.
The revision already supplies the legacy packages as aliases of the canonical package.
Do not upgrade its lint catalog or compiler inputs as part of this rename.
Do not add sibling paths, copied producer code, or runtime dependencies.

## Changes

Rename the direct flake input and check to `octet`.
Pin the existing revision explicitly in `flake.nix`, then let `nix flake lock` generate the lock.
Select `cargo-octet` in the development shell.
Rename the workspace metadata and lint config tables without changing their values.

Change the required command emitted by the CLI shell to `checks.x86_64-linux.octet`.
Keep receipt fields, other command entries, and the non-execution boundary unchanged.
The pure lifecycle core requires no behavioral change.

Update maintained README and workflow guidance.
Do not rename recorded log files, edit archived commands, or rewrite old signed or hash-bound receipts.
The saved function-address smoke receipt remains a historical producer observation.
Its documentation must distinguish it from a newly generated report.

## Verification

Establish the existing Rust-test baseline before product edits.
Use a pure Nix integration test over actual inputs and config values, plus negative mutations for each former name and source drift.
Exercise the canonical command through the release-readiness integration tests.
Retain the existing negative release-readiness tests.
Sync the reviewed spec through an unblocked Cairn plan before the full tests that require accepted-spec traceability.
This sync accepts requirements, not completed implementation or successful producer execution.
Run focused Nix checks and attempt the actual Octet build.
A failure before consumer analysis remains an external blocker, not a passing lint result.

## Risks and non-claims

Transitive locked inputs can retain old names because their owners control those inputs.
A changed lock-node label is not an unrelated source upgrade.
The comparison must bind source revisions and NAR identities, not incidental node labels.
This change intentionally removes the direct legacy Nix check name from current Cairn outputs.
Historical receipts retain their original command text and do not authorize new execution.
Passing naming controls does not prove lint execution, Rust correctness, or completion of the utility changes.
