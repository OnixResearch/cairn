# Full gate

Run from the change worktree on 2026-11-20. Rust commands ran inside `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix --option min-free 0 develop -c ...`. The Cairn commands used the packaged CLI built from `c8b38c4` (`/nix/store/52fkm6qw21c7n230rlj9ygy43fgpnx94-cairn-0.2.1`).

| Command | Transcript | Exit | Result |
| --- | --- | --- | --- |
| `cargo fmt --check` | `01-fmt.txt` | 0 | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | `02-clippy.txt` | 0 | clean |
| `cargo test --workspace` | `03-test-workspace.txt` | 101 | 17 `test result: ok` sections; one pre-existing failure, below |
| `cairn validate --root .` and the proposal, design and tasks gates | `04-lifecycle-gates.txt` | 0 | `valid: true`, three gates `PASS` |
| `cairn policy export --check` | `05-policy-export-check.txt` | 0 | policy fresh |
| `cairn sync`, `cairn archive` (with `CAIRN_ARCHIVE_DATE=2026-11-20`), then `validate` and `traceability coverage` | `06-post-archive.txt` | 0 | archived; `valid: true`; coverage 2490/2490 |
| `cargo test --workspace --no-fail-fast` | `03b-test-workspace-no-fail-fast.txt` | 101 | 96 `test result: ok` sections, including both convergence test binaries; the only failing target is the pre-existing `buoy_comparison`, below |
| `nix flake check path:$PWD -L --keep-going` | `07-flake-check.summary.txt` | 1 | 39 of 40 `checks.x86_64-linux` attributes build; `rust-tests` fails only on the pre-existing `buoy_comparison` test |

The pre-existing failure is `cairn-cli --test buoy_comparison comparison_identity_binds_source_mapping_fixture_and_policy`, which reports "native semantic stale_item does not match declared unsupported" for `cases.schema_version_drift.semantic`. It fails identically at Cairn `9655914`, the `origin/main` before this change's first commit, and at `8f6cd0b` and `1fef210`. None of the commits in `9655914..8f6cd0b` touches the trace-index code, the Buoy fixtures or that test. Cause: the test appends a newline to the `clean` case's source, and `schema_version_drift` reuses `clean` as its base index. The index-verify run against the drift case's unchanged source then reports `stale_item` ahead of `schema_unsupported`. This change does not alter that behavior and does not fix it.
