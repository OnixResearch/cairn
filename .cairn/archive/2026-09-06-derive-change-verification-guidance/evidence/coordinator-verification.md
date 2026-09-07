# Coordinator verification and completion

Change: `derive-change-verification-guidance`.

## Delivered scope

The change provides pure conservative source-impact guidance, current-evidence applicability admission, bounded Octet and Valence structural adapters, CLI/MCP parity, mandatory-check preservation, adversarial controls, and fresh source replays.

The report issues exact producer-plan requests but does not execute checks. It retains declared-link fallback when stronger structural evidence is unavailable.

## Frozen-source battery

| Check | Result |
|---|---|
| `cargo test --workspace --all-targets --all-features` | Exit 0 |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Exit 0 |
| `cargo fmt --all -- --check` | Exit 0 |
| `cairn policy export --check` | Exit 0 |
| `cairn validate --root .` | Exit 0 |
| `cairn traceability coverage --root . --json` | Exit 0; no missing/dangling markers |
| `nix flake check "path:$PWD" --option min-free 0 -L` | Exit 0 |

Receipts are retained under `.pi/cairn-rest-1295/guide3-2746-battery-*`.

## Lifecycle review

The reviewed sync plan is unblocked. The accepted spec already contains the implementation markers after execution. Archive execution follows this review.

## Non-claims

The warning-level slotcar result is not strict Octet acceptance. The guidance does not prove source equivalence, test sufficiency, execution truth, correctness, or release eligibility. It never executes a requested producer plan.
