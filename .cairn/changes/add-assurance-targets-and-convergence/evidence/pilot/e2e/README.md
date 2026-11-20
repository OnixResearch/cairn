# WP7: design steps 1-5 replayed from clean checkouts

Every step below was rerun from new detached worktrees at the recorded revisions,
with outputs written under `/home/brittonr/wt/convergence-evidence/e2e/`, and each
output compared byte-for-byte with the record the pilot retained. Transcripts are in
`transcripts/NN-name.txt`, written by `run-transcript.sh` (copied from
`realization/` with `dir` changed to this directory); each records the command,
stdout, stderr, exit code and wall time.

Nothing was edited, committed or checked out in the existing worktrees
(`octet-convergence`, `octet-rev`, `mantle-convergence`, `mantle-bridge`,
`cairn-convergence`); `/home/brittonr/wt/cairn-convergence` was not touched
at all. No retained record was modified. Nothing was signed,
`--trust-unsigned` was never used, and nothing was pushed. The only store writes
under `/datapool/mantle-store/convergence-pilot-20261119` are Mantle's own
`realize` reuse into its existing `state`/`store` in step 3.

## Clean checkouts

Created with exactly these commands (each exited 0); `git status --porcelain`
was empty and `rev-parse HEAD` returned the revision for all three
(transcript `00-checkouts`, exit 0, and again in `19-final-status`):

```text
git -C /home/brittonr/git/OnixResearch/octet  worktree add --detach /home/brittonr/wt/e2e/octet-pilot e5086dca9b7feabfad37df3769390cc5005811f5
git -C /home/brittonr/git/OnixResearch/octet  worktree add --detach /home/brittonr/wt/e2e/octet-rev   d5b34c2d64b1227417a46df8ed8427f2d10ed3c8
git -C /home/brittonr/git/OnixResearch/mantle worktree add --detach /home/brittonr/wt/e2e/mantle      61f06cc59ebb3bc78f66b2d516abdc615cdbdf2a
```

| worktree | repository | revision | HEAD |
| --- | --- | --- | --- |
| `/home/brittonr/wt/e2e/octet-pilot` | `/home/brittonr/git/OnixResearch/octet` (branch `add-cairn-convergence-pilot`) | `e5086dca9b7feabfad37df3769390cc5005811f5` | `e5086dca9b7feabfad37df3769390cc5005811f5` |
| `/home/brittonr/wt/e2e/octet-rev` | `/home/brittonr/git/OnixResearch/octet` | `d5b34c2d64b1227417a46df8ed8427f2d10ed3c8` | `d5b34c2d64b1227417a46df8ed8427f2d10ed3c8` |
| `/home/brittonr/wt/e2e/mantle` | `/home/brittonr/git/OnixResearch/mantle` (branch `admit-octet-producer-layout-in-verification-bridge`) | `61f06cc59` | `61f06cc59ebb3bc78f66b2d516abdc615cdbdf2a` |

Binary provenance: everything in step 3 came from `e2e/mantle` built at
`61f06cc59` inside its dev shell (transcript `05-step3-build`, exit 0, 589.05 s:
`mantle` and `mantle-verification-bridge`); step 5's judge came from
`e2e/octet-pilot` built at `e5086dca` inside its dev shell (transcript
`06-step5-build`, exit 0, 150.69 s, isolated `CARGO_TARGET_DIR` because the
dev shell otherwise redirects to a shared `/home/brittonr/.cargo-target`); the
generator in step 1 is the pinned Nickel `1.17.0 (rev 1320a98)` from
`/nix/store/ck3a4k9az87z5rjwgy8nmpcs34xp9cc5-nickel-lang-rev-fixup-1.17.0/bin/nickel`.

## Results

| step | command (transcript) | revision | exit | compared with | result |
| --- | --- | --- | --- | --- | --- |
| 1 targets | `nickel export --format json standards/requirements/cairn-pilot/targets.ncl \| jq -S .` → `step1/targets.json` (`01-step1-export`, `02-step1-compare`) | `e5086dca` | 0 | `standards/requirements/cairn-pilot/targets.json` (worktree) | **identical** (BLAKE3 `56c80346…f54dcd`) |
| 1 generated | `nickel export --format json standards/requirements/cairn-pilot/generated.ncl \| jq -S .` → `step1/generated.json` (`01-step1-export`, `02-step1-compare`) | `e5086dca` | 0 | `standards/requirements/cairn-pilot/generated.json` (worktree) | **identical** (BLAKE3 `a3c483b1…d82ca7`) |
| 1 contract | `nickel export --format json standards/requirements/cairn-pilot/contract.ncl \| jq -S .` → `step1/contract.json` (`01-step1-export`, `02-step1-compare`) | `e5086dca` | 0 | `standards/requirements/cairn-pilot/contract.json` (worktree) | **identical** (BLAKE3 `56d7f459…e8eb3e`) |
| 2 freeze | `nix flake prefetch --json git+file://…octet?rev=d5b34c2d…` then `tools/mantle-producer/export.py --nix realization/nix-minfree0 --nickel <1.17.0> --flake … --revision d5b34c2d… --graph-format nix-derivation-show-v4 --out step2/producer-export` (`03-step2-freeze`) | `d5b34c2d` | 0 | `realization/producer-export` (`04-step2-compare`) | **identical**: 45 631 entries, 37 860 regular files, 44 symlinks; names/hashes/links listings differ not at all; aggregate BLAKE3 of the hash listing `be55e9f8…5ecb9` on both sides |
| 3 produce-nix | `mantle --json foreign-import produce-nix --derivation-json step2/producer-export/tool-closure.derivation-json.json --root-derivation /nix/store/vl7bqxyz2f86x0vlvxipcvryavckb9wj-…drv … --cache-url http://127.0.0.1:18471?trusted_public_keys[0]=convergence-pilot-20261119-1:… --out-dir step3/mantle-graph` (`10-step3-produce-nix`) | `61f06cc59` (Mantle), export from `d5b34c2d` | 0 | `realization/mantle-graph-preserve-http/{nixpkgs.graph.json,nixpkgs.index.json}` (`12-step3-plan-compare`) | **identical** (`8ed10e2d…` / `64375405…`) |
| 3 plan | `mantle --json foreign-import plan --graph step3/…/nixpkgs.graph.json --package-index … --policy config/foreign-cache-closure/generated/preserve-nix.json --execution-profile config/foreign-execution-profiles/generated/nix.json --package octet-mantle-aeneas-consumer-job --system x86_64-linux --nix-compat --plan-out step3/…/plan.json --receipt-out step3/…/import-receipt.json --state-dir step3/plan-state` (`11-step3-plan`) | `61f06cc59` | 0 | `realization/mantle-graph-preserve-http/plan.json` + `import-receipt.json` (`12-step3-plan-compare`) | **identical**; `plan_identity` `edd387e5e529e1d4b277d726edc93901889413dc0dc393c7c7fd715981afb08a78` on both, plan file BLAKE3 `d4efeaa6…f65ae45` on both |
| 3 prepare-sources | `mantle --json foreign-import prepare-sources --plan step3/…/plan.json --out step3/source-bundle-http.json` (`13-step3-prepare-sources`) | `61f06cc59` | 0 | `/datapool/…/convergence-pilot-20261119/source-bundle-http.json` (`14-step3-bundle-compare`) | **identical**; `manifest_blake3` `8ed1103b5de3054ee13ea391af805e276e3cc3b6ceaa50b2149193adb1ff1777`, 0 records, file BLAKE3 `b8abb366…ab8030` on both |
| 3 realize (reuse) | `env PATH=/var/empty mantle --json --state-dir <P0>/state --store <P0>/store --store-prefix /nix/store --remote-binding-config <P0>/binding/remote-endpoint-binding.json foreign-import realize --plan step3/…/plan.json --import-receipt step3/…/import-receipt.json --source-bundle step3/source-bundle-http.json --source-bundle-blake3 8ed1103b… --execution-profile config/foreign-execution-profiles/generated/nix.json --cache-closure-policy realization/pilot-cache-closure-policy.json --receipt-out step3/realization-receipt-reuse.json --substitute` (`15-step3-realize-reuse`, wall 495.92 s, loopback server log `15-cache-server.log`) | `61f06cc59` | 0 | `realization/realization-receipt-reuse.json` (`18-step3-receipt-compare`) | **identical**: receipt file BLAKE3 `d96e36da…b03c6` and `receipt_blake3` `004e36388689a67eca3d8c1d0edf6a6e665d61208e302cea2eda8bf790baf829` on both; `status=complete`, `strongest_state=realized`, `cache_policy.realization_route=cache-only-preserve-v1`; 11 565 units (`already-present` 491, `not-required-cache-only` 11 074); **imported 0**, **NAR requests 0** (526 `narinfo` GETs, 0 `.nar` GETs in the loopback log); root unit `node_id` `nix:vl7bqxyz2f86x0vlvxipcvryavckb9wj-octet-mantle-aeneas-consumer-job-v1.drv`, `target_path` `/nix/store/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1` — the same values the pilot recorded in `48-realize-reuse.txt` |
| 4 observe | `mantle-verification-bridge octet-job <P0>/store/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1 step2/producer-export d5b34c2d64b1227417a46df8ed8427f2d10ed3c8 handoff` (`16-step4-bridge`) | `61f06cc59` | 0 | `realization/handoff` (`17-step4-handoff-diff`) | **identical**: `diff -r` reports no differences, per-file BLAKE3 listing identical (aggregate `80adc658…319480`) |
| 5 judge check | `cargo-octet requirements check --contract standards/requirements/cairn-pilot/contract.json --evidence standards/requirements/fixtures/cairn-pilot/evidence.json --source-root /home/brittonr/wt/e2e/octet-rev --output step5/decision.json --output-format json` (`07-step5-check`) | `e5086dca` | 0 | `standards/requirements/fixtures/cairn-pilot/decision.json` (`09-step5-compare`) | **identical**; `decision_blake3` `906cc950af8318c420b9c8188f325aede254fe2c54360b4881c5c87f2d4f9a45`, file BLAKE3 `ca047f54…91c7` on both |
| 5 judge verify | `cargo-octet requirements verify --contract … --evidence … --decision step5/decision.json --source-root /home/brittonr/wt/e2e/octet-rev` (`08-step5-verify`) | `e5086dca` | 0 | recorded vs recomputed decision identity (`09-step5-compare`) | **passed**; `recorded=recomputed=906cc950…`, `status=passed`, `aggregate=passed`, `diagnostics=0` |

## Transcripts

| file | what it records |
| --- | --- |
| `00-checkouts` | `rev-parse HEAD` and empty `status --porcelain` for the three new worktrees |
| `01-step1-export`, `02-step1-compare` | Nickel 1.17.0 exports and `cmp`/BLAKE3 for targets, generated, contract |
| `03-step2-freeze`, `04-step2-compare` | `flake prefetch` + `export.py`, then the full-tree BLAKE3 comparison |
| `05-step3-build` | Mantle + bridge release build in `e2e/mantle` (589.05 s) |
| `10-step3-produce-nix`, `11-step3-plan`, `13-step3-prepare-sources`, `15-step3-realize-reuse` | step 3 commands, in the pilot's order |
| `12-step3-plan-compare`, `14-step3-bundle-compare`, `18-step3-receipt-compare` | step 3 comparisons (plan/receipt/graph/index, bundle, realization receipt) |
| `15-cache-server.log` | loopback `python3 -m http.server 18471` log for the realize (started in the same call, stopped after) |
| `16-step4-bridge`, `17-step4-handoff-diff` | bridge observation and `diff -r` + per-file BLAKE3 against `realization/handoff` |
| `06-step5-build`, `07-step5-check`, `08-step5-verify`, `09-step5-compare` | `cargo-octet` build, check, verify, `cmp` against the committed decision |
| `19-final-status` | worktree HEADs and still-empty `status --porcelain` after all runs; port 18471 free |

## Deviations from the earlier transcripts

Only output paths changed, as the task required:

- step 2 export target: `realization/producer-export` → `e2e/producer-export`;
- step 3 produce-nix `--out-dir`, plan `--plan-out`/`--receipt-out`, `prepare-sources --out`,
  realize `--receipt-out`: all under `e2e/step3/`;
- plan `--state-dir`: `/datapool/…/plan-http` → `e2e/step3/plan-state`, because the
  `/datapool` store may be written only by realize in step 3 (the plan state
  directory is a plan output, and the plan bytes are unaffected: `plan_identity`
  and the file BLAKE3 equal the retained ones);
- realize keeps `--state-dir <P0>/state` and `--store <P0>/store`, because this is a
  reuse into the pilot's existing state;
- step 5 built `cargo-octet` with `CARGO_TARGET_DIR` pinned inside the worktree
  (the dev shell default points at a shared target dir) and invoked the built
  binary directly (`target/release/cargo-octet requirements …`), which is the
  documented fallback when `cargo octet` is not wired.

Step 2 produced a byte-identical export, so the conditional second bridge run
(`realization/producer-export` → `handoff-retained-export`) was not needed.

## Differences found

None. Every comparison above is byte-identical.

## Step 6: inspect (clean Cairn checkout `c8b38c4`, packaged binary from `nix build path:/home/brittonr/wt/e2e/cairn#cairn`)

| step | command | exit | result |
| --- | --- | --- | --- |
| 6a | `cairn convergence inspect --case fixtures/convergence/pilot/case.json --root fixtures/convergence/pilot/root`, the retained case (transcript `20`) | 0 | pilot row `current`, route `cache-only-preserve-v1`, 739 rows `untargeted`, `targets_current` true, `coverage_complete`/`complete` false; `receipt_blake3` `39ce3c51b8c619091653d6c4f90bcca238187ca544c3477399051bee57535482`, which matches the worktree build; the clean checkout is unchanged afterwards |
| 6b prerequisite | `mantle store info` with the clean Mantle build (transcript `21`) | 0 | equal to the retained `store-info.json` except the store's root-record bookkeeping, `retention_interests.interests[0].{reason,record_id}`, which the later reuse realizations re-registered |
| 6b | `cairn convergence inspect` over a case built from the reproduced step 1, 4 and 5 outputs, the **reuse** realization receipt from step 3, the step-6b store info and the live realized output, with root set to the clean Octet checkout at REV (transcript `22`) | 0 | pilot row `current` with no issues, route `cache-only-preserve-v1`; `receipt_blake3` `9757e63beea05d61c00b09d4d6d0be610d4a7ac8916b038b301ef6b1ba8262af`. It differs from 6a only through the bound input records (the reuse receipt and the store info). This answers the handoff control "Realize the unchanged job a second time: Cairn still reports `current`" |
