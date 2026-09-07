# guide2-2746 round-two evidence

## Scope

Worktree: `/home/brittonr/git/OnixResearch/.worktrees/cairn-drain-guidance-20260906`
Branch: `drain/utility-guidance-20260906`
Starting tip: `0a6e5eb`.
No sync, archive, push, or worktree removal was performed.

## Implemented in this pass

- Completed the `evidence` acceptance slice.
  - Reused the existing `check_evidence` applicability validator through public pure-core exports.
  - Added whole-subject evidence freshness evaluation to `VerificationImpactInput` and `VerificationImpactReport`.
  - Derived the current applicability from the admitted after-source identity, check-map policy identity, dependency identities, and source projection identity.
  - Added prior-evidence refresh verdicts with exact stale reasons for subject, policy, dependency, and projection drift.
  - Added controls proving an unlinked source edit still makes prior whole-subject evidence stale.
- Updated `contracts/verification-impact.ncl` and fixtures for applicability, dependency hashes, and prior evidence.
- Added a first read-only CLI/MCP guidance shell:
  - CLI: `cairn verification-impact guidance --root ROOT --input FILE [--change CHANGE] [--contract FILE --profiles FILE --applicability FILE]`.
  - MCP: `cairn_verification_impact` with inline admitted source-comparison input.
  - The wrapper emits unified-readiness and `cairn check-evidence plan` requests only; it records `checks_executed: false` and `mutation_authorized: false`.

## Task state changes

- Marked `evidence` complete after direct positive/negative applicability controls and Nickel fixture controls passed.
- Left `adapters`, `interfaces`, `controls`, `mutation`, `consumer`, and `verification` open. The interface shell exists, but the `interfaces` task remains open because adapter composition and full parity controls are not complete.

## Commands and results

All commands used the required PATH prefix and `TMPDIR=/var/tmp`. Cargo commands used `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c ...`.

| Pueue task | Command summary | Exit | Result |
|---|---:|---:|---|
| 1233 | `git status --short --branch && git log --oneline -5` | 0 | Worktree on `drain/utility-guidance-20260906`, ahead 3 at `0a6e5eb`; three check-evidence files were already modified. |
| 1234 | `git diff -- check_evidence/*` | 0 | Showed public pure exports for `valid_applicability` and `whole_subject_evidence_current`. |
| 1250 | `cargo test -p cairn-core verification_impact` | nonzero | Exposed stale prior-evidence fixture ordering and stale-reason ordering failures; fixed in this pass. |
| 1255 | `cargo test -p cairn-core verification_impact` | nonzero | One remaining expected-order mismatch; fixed in this pass. |
| 1264 | `cargo test -p cairn-core verification_impact` | 0 | 19 verification-impact tests passed. |
| 1275 | `cargo test -p cairn-cli verification_impact` | nonzero | Initial CLI wrapper compile failed because `canonical_json_hash` expected `Value`; fixed. |
| 1285 | `cargo test -p cairn-cli verification_impact` | 0 | CLI wrapper action test passed for both `cairn` and `ironspec` bins. |
| 1294 | `cargo fmt --all` | 0 | Formatting applied. |
| 1296 | focused core/CLI/Nickel/fmt battery | 0 | 19 core tests passed, CLI action test passed, positive/negative Nickel verification-impact fixtures passed, fmt check passed. |
| 1302 | focused clippy before lifetime fix | nonzero | Clippy rejected needless explicit lifetime in CLI wrapper; fixed. |
| 1307 | `cargo fmt --all -- --check && cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings` | 0 | Focused fmt and strict Clippy passed. |
| 1308 | `nix run path:$PWD#cairn -- validate --root .` | 0 | Lifecycle validation passed; 4 tasks done, 6 open. Nix emitted repeated max-link warnings before local validation completed. |
| 1322 | Nickel positive/negative fixture battery including prior evidence | 0 | `nickel-verification-impact-fixtures-pass`. |
| 1325 | `nix run path:$PWD#cairn -- gate tasks derive-change-verification-guidance --root .` | 0 | Tasks gate passed; `adapters` is now the only ready task. |

Observed non-blocking environment warnings: `warning: unknown experimental feature 'wasm-builtin'`; transient SSH lookup failures for pinned dependencies during Nix commands; Nix store `.links` max-link warnings before validation succeeded.

## Remaining gaps / next actions

1. `adapters`: compose Octet structural and Valence span evidence into verification-impact input/report semantics. Existing `check_evidence` adapters and tests are available, but this pass did not yet attach their admitted relationships to the guidance report.
2. `interfaces`: complete CLI/MCP parity over a fresh valid source-comparison fixture, including exact `cairn check-evidence plan` request fields and no-execution assertions through both transports.
3. `controls`: run and retain transport-level controls for deletion, rename, copied-marker, missing-edge, stale-index, cycle, macro, generated-source, target, and feature cases.
4. `mutation`: retain an unsafe-omission mutant that fails acceptance and direct positive controls for mandatory-check preservation.
5. `consumer`: replay fresh Cairn source comparisons and document selected checks, refresh needs, fallback scope, and outstanding full gates.
6. `verification`: run the full frozen battery and prepare sync/archive dry-run previews if the coordinator authorizes those preview commands.
