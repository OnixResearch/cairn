# Verification guidance adapters and interfaces

Status: implemented and focused-tested. The coordinator retains final verification, sync, archive, and integration authority.

## Delivered capability

The pure adapter admits bounded Octet structural receipts and Valence evidence-admission facts. It binds source identity, target, features, cohort identity, receipt identity, and scope. A passed Octet receipt remains completion-ineligible because it does not prove the pinned full-catalog scope.

When structural evidence is unavailable, the report retains the declared-link fallback. The fallback does not claim a stronger source-span or execution fact.

`cairn verification-impact guidance` and `cairn_verification_impact` provide the same read-only guidance report. They expose exact producer-plan arguments without `--execute`. Both transports reject unknown, repeated, empty, or execution-shaped input.

The control suite covers source deletion, rename, copied marker, missing edge, stale index, cycles, macro text, generated source, target and feature changes, partial plans, Octet substitutions, and a mutation that omits a mandatory check.

The fresh source replay preserves required checks after a source change. It reports stale prior evidence and retained full gates rather than granting whole-subject reuse.

## Focused checks

- `cargo test -p cairn-core verification_impact`: passed.
- `cargo test -p cairn-cli --test verification_impact`: passed.
- `cargo clippy -p cairn-core -p cairn-cli --all-targets --all-features -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.

The tests in `crates/cairn-cli/tests/verification_impact.rs` provide CLI/MCP parity, adapter substitution, control, mutation, and consumer-replay evidence.

## Non-claims

The report does not execute checks, update pins, scan siblings, or mutate lifecycle state. Declared links and structural receipts do not prove semantic source equivalence, execution truth, source correctness, or release eligibility.
