## Design

Convert `steel_orchestration.rs` into `steel_orchestration/`:

- `mod.rs` remains the imperative shell: parse the subcommand, load policy paths via existing CLI support helpers, call pure receipt builders, and print JSON.
- `profiles.rs` owns default profile constants, fallback profile construction, and profile selection.
- `evaluation.rs` owns script-envelope reading, hash checks, Rust-side action validation, redacted action summaries, and fallback/outcome classification.
- `receipts.rs` owns deterministic receipt JSON construction, default-promotion evidence checks, and receipt-hash materialization.

## Boundaries

- No new Steel execution authority.
- No public JSON contract changes.
- No changes to policy format, host function allowlists, or receipt schema constants.
- Filesystem reads remain limited to existing policy/script inputs; host effects remain reported as not executed.

## Verification

Run the focused CLI Steel tests and module tests, then native Cairn validation/Tracey coverage. Use the full flake gate before landing.
