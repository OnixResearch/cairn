Fixture packaging; no proof executed.

# WP3 — Mantle verification-bridge fixture rehearsal

Recorded fixture bytes were packaged and run through ordinary Mantle
derivations. No external proof, verifier, theorem admission, tool-closure
construction, or proof acceptance was executed. Every artifact below is
**fixture packaging; no proof executed**.

- Mantle worktree: `/home/brittonr/wt/mantle-convergence`
- Mantle revision: `cd4ad3f43f6ce533a686d204168e8bae96688d85` (detached `origin/main`, clean)
- All commands ran from the worktree root inside `nix develop`; cargo steps used
  `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target`.
- Host context (revision, kernel, nproc, toolchain, b3sum, bwrap):
  `context.txt`
- One transcript per step under `transcripts/`, each holding the exact command
  line, stdout, stderr and exit code.

## Steps and exit codes

| # | Command | Exit | Note |
|---|---|---|---|
| 1 | context gathering (`context.txt`) | 0 | host + dev-shell tool versions |
| 2 | `cargo build -p mantle -p crunch-verification-bridge` (CARGO_TARGET_DIR=target) | 0 | `Finished dev profile ... in 6m 29s` |
| 3 | `target/debug/mantle doctor` | 3 | **failed**: default `--store /nix/store` is not writable by the current user |
| 3b | `target/debug/mantle doctor --store <staging>/doctor-preflight/store --state-dir <staging>/doctor-preflight/state` | 0 | supplementary run with an explicit writable store; `status: ok` |
| 4 | `cargo test -p crunch-verification-bridge --test boundaries` | 0 | `test result: ok. 6 passed; 0 failed` |
| 5 | `target/debug/mantle-verification-bridge octet tests/fixtures/verification-bridge/octet profile.json receipt.json 4a540dff99aa2040e267e42bbafe09144f276572 <staging>/octet-handoff` | 0 | `externally_reported_outcome: supported`, `proof_acceptance: not-evaluated-by-mantle` |
| 6 | `python3 scripts/prove-verification-bridge.py --mantle target/debug/mantle --bridge target/debug/mantle-verification-bridge --b3sum ... --out <staging>/bridge-packaging-proof` | 1 | **failed as documented**: the driver never creates its `--store` directory; Mantle rejects a missing `--store` (`crates/crunch-pipeline/src/lib.rs:1281`). Details in `bridge-packaging-proof/octet-cold.stderr` |
| 6b | same driver copied to `/tmp/wp3/prove-verification-bridge-storefix.py` with exactly one added line `(args.out / "store").mkdir()` (`transcripts/06b-driver.diff`), `--out <staging>/bridge-packaging-proof-storefix` | 0 | cold build then warm reuse for both fixtures; no tracked file was modified |

Step 6 is reported at its true result: the checked-in driver cannot complete on
this revision. Step 6b is the same driver logic with the missing directory
created, and is labelled accordingly — it is not the unmodified documented
command.

## Cold/warm evidence (step 6b)

| fixture | cold `counts` | warm `counts` |
|---|---|---|
| octet | `built_total: 1`, `cached_total: 0` | `built_total: 0`, `cached_total: 1` |
| trellis | `built_total: 1`, `cached_total: 0` | `built_total: 0`, `cached_total: 1` |

`octet-consume.stdout` is byte-identical to `octet-replay.stdout`, and
`trellis-consume.stdout` to `trellis-replay.stdout` (the replay re-consumes the
exported store output with `PATH=/no-ambient-tools`).

## Inventory

Sizes are `du -sh` per top-level entry.

| Size | Entry |
|---|---|
| 4.0K | `context.txt` |
| 64K | `transcripts/` |
| 8.0K | `doctor-preflight/` (store + state for the supplementary doctor run) |
| 84K | `octet-handoff/` (bridge `octet` mode output, step 5) |
| 112K | `bridge-packaging-proof/` (step 6 as documented — aborted at the cold build, no store created) |
| 2.2M | `bridge-packaging-proof-storefix/` (step 6b: consume/cold/warm/replay logs, `result.json`, generated `.ncl` plans, store, state) |
| **2.5M** | **staging total** |

`bridge-packaging-proof-storefix/state/build-work/sandbox-remote/*/overlay_workdir/work`
are empty scratch directories left mode `000` by the sandbox; they contribute
no bytes.
