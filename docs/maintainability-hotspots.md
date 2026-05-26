# Maintainability hotspot inventory

This advisory inventory was generated after the v0.1.0 tag to guide future Cairn packages. It does not move modules, change public behavior, or claim refactor completion.

## Largest Rust files sampled

| Path | Lines |
| --- | ---: |
| `crates/cairn-core/src/verified/semantic_review/metrics_regression.rs` | 857 |
| `crates/cairn-core/src/verified/policy.rs` | 538 |
| `crates/cairn-cli/src/steel_orchestration.rs` | 493 |
| `crates/cairn-cli/tests/cli.rs` | 493 |
| `crates/cairn-core/src/verified/semantic_review/mod.rs` | 445 |
| `crates/cairn-cli/src/semantic_review.rs` | 423 |
| `crates/cairn-core/src/verified/developer_experience.rs` | 395 |
| `crates/cairn-core/src/verified/content_addressed/mod.rs` | 387 |
| `crates/cairn-cli/tests/semantic_review.rs` | 384 |
| `crates/cairn-cli/src/determinism/verify.rs` | 375 |
| `crates/cairn-cli/src/release_evidence/retention.rs` | 372 |
| `crates/cairn-cli/src/mcp/read_tools.rs` | 369 |
| `crates/cairn-cli/src/preserves_corpus.rs` | 357 |
| `crates/cairn-cli/src/mcp/schema.rs` | 355 |
| `crates/cairn-cli/tests/content_addressed_evidence.rs` | 350 |
| `crates/cairn-cli/tests/release_receipts/manifest.rs` | 336 |
| `crates/cairn-cli/src/policy.rs` | 334 |
| `crates/cairn-cli/src/evidence/import.rs` | 328 |
| `crates/cairn-cli/src/release_evidence/check_index.rs` | 319 |
| `crates/cairn-core/src/verified/content_addressed/name_pointer.rs` | 318 |

## Candidate future seams

1. `crates/cairn-core/src/verified/semantic_review/metrics_regression.rs` — extract deterministic fixture/category helpers behind focused semantic-review tests.
2. `crates/cairn-cli/src/steel_orchestration.rs` — split command parsing/report rendering from pure orchestration data preparation.
3. `crates/cairn-cli/src/semantic_review.rs` and `crates/cairn-cli/tests/semantic_review.rs` — separate ledger/profile/revision surfaces if new behavior lands.
4. `release_evidence` retention/check-index modules — keep generator/checker/publisher seams narrow before adding remote publication behavior.
5. Large integration tests such as `tests/cli.rs`, `content_addressed_evidence.rs`, and release receipt tests — split only with focused behavior-preserving checks.

## Focused checks for future refactors

```bash
nix develop path:$PWD -c bash -lc 'CARGO_TARGET_DIR=/tmp/cairn-target-maint RUSTC_WRAPPER= cargo fmt --check'
nix develop path:$PWD -c bash -lc 'CARGO_TARGET_DIR=/tmp/cairn-target-maint RUSTC_WRAPPER= cargo test -p cairn-cli -- --nocapture'
nix develop path:$PWD -c bash -lc 'CARGO_TARGET_DIR=/tmp/cairn-target-maint RUSTC_WRAPPER= cargo test -p cairn-core -- --nocapture'
nix develop path:$PWD -c bash -lc 'CARGO_TARGET_DIR=/tmp/cairn-target-maint RUSTC_WRAPPER= cargo run -p cairn-cli --bin cairn -- validate --root .'
```

r[verify developer_experience.cli_maintainability_hotspot_inventory.scan]
r[verify developer_experience.cli_maintainability_hotspot_inventory.docs]
r[verify developer_experience.cli_maintainability_hotspot_inventory.boundary]
r[verify developer_experience.cli_maintainability_hotspot_inventory.validation]
