# Design: Split agent receipt core modules

## Boundaries

- `agent_receipt/mod.rs`: public facade, completion-gate orchestration, public re-exports, facade tests, and Tracey implementation refs.
- `types.rs`: public issue/report DTOs.
- `constants.rs`: public schema constants and private policy tables.
- `hash.rs`: canonical agent-run receipt hash plus receipt-hash validation helper.
- `schema.rs`: required receipt field validation.
- `plan.rs`: no-mutation/side-effect plan linkage checks.
- `verification.rs`: successful-verdict verification evidence checks.
- `nondeterminism.rs`: captured/forbidden entropy checks.
- `replay.rs`: replay-evidence hash, verdict, case, and group checks.
- `fixture.rs`: deterministic positive fixture generation.
- `common.rs`: private issue construction and BLAKE3 helper.

## Behavior preservation

The facade continues to export the same public API consumed by `crates/cairn-core/src/lib.rs` and `crates/cairn-cli/src/agent_receipt/validation.rs`. Internal helpers are module-private or `pub(super)`, and the split does not alter issue codes, messages, schema constants, hash material, or fixture content.

## Verification

Focused core tests exercise the canonical fixture, positive completion gate, and existing negative cases. Repository gates prove formatting, workspace tests, clippy, native Cairn validation, Tracey coverage, and flake checks remain green.
