# WP7 negative controls N1–N4 (Cairn `add-assurance-targets-and-convergence`)

Everything here was produced under the binding rules of
`local://freeze-negatives-spec.md`: clones only (no commit, checkout or write in
`/home/brittonr/git/OnixResearch/octet`), no `foreign-import realize`, no signing,
no pushes, read-only pilot-cache queries, `--option min-free 0` on every `nix` call
(via `realization/nix-minfree0`), no `path:` flake refs, no `--trust-unsigned`.
N4 is the exception in scope, not in rules: it was produced under
`local://empirical-negative-spec.md` and touches no Nix path at all.
No write of any kind under `/datapool/mantle-store/convergence-pilot-20261119`.

## Revisions and binaries

| item | value |
|---|---|
| `REV` (original freeze) | `d5b34c2d64b1227417a46df8ed8427f2d10ed3c8` |
| `NEWREV_SRC` (N1) | `ad455c1b9cc41f5f576dbcca0f2cbc7246052eb2` |
| `NEWREV_LOCK` (N2) | `a86d6e31f449cbc8b2515394d2175749c95210a4` |
| Mantle worktree head (observed) | `b99ed8637d95cbecb961b776aedc19a315147a89` |
| Mantle head named by the spec | `80ab420bc` |
| Mantle binaries | `/home/brittonr/wt/mantle-convergence/target/release/{mantle,mantle-verification-bridge}` (mtime 2026-11-19 19:43 / 20:37), used unmodified |
| pilot cache | `file:///datapool/mantle-store/convergence-pilot-20261119/cache`, public key `convergence-pilot-20261119-1:sRZV4nbjXTCkhRjkEIVe5Pg4G2ajYaCobLqS0JJVu2A=` |
| N3 bridge binary | `/home/brittonr/wt/mantle-bridge/target/release/mantle-verification-bridge`, BLAKE3 `ae41e077e8d9024626b8b6826620069fbf56829b97be469b06930d9533743282` (checked before use, not rebuilt), build commit `b14db98cb473c4bad449ce6dd68e33d96519d605` “borrow the receipt in the provider-profile locator and mark the layout tests as verification” |

Both scratch commits carry author and committer `Britton <b@robitzs.ch>` and
subjects beginning `negative control (not for push):`
(transcripts `04-commit-src-change.txt`, `14a-commit-lock-change.txt`).

Tool-graph identity, as required: `b3:` + BLAKE3 of the exact bytes the bridge
digests as `binding.graph`. Confirmed from
`crates/crunch-verification-bridge-core/src/producer.rs:57`
(`let tool_graph_identity = digest(&binding.graph);`) together with
`crates/crunch-verification-bridge/src/lib.rs:377-384`, which reads
`tool-closure.derivation-json.json` from the producer export into
`ProducerBinding.graph` (`lib.rs:320-325`) and writes it back under that same
name (`lib.rs:379`). So the identity is the BLAKE3 of
`tool-closure.derivation-json.json`.

## Result table

| control | change | transcripts | observed (old → new) | required result (quoted) | verdict |
|---|---|---|---|---|---|
| **N1** | one byte inside a comment in the declared fixture source, re-frozen | `01`,`02`,`03`,`04`,`05`,`06`,`07`,`08`,`09`,`10` | root derivation `/nix/store/vl7bqxyz2f86x0vlvxipcvryavckb9wj-octet-mantle-aeneas-consumer-job-v1.drv` → `/nix/store/lhwv6ahi2zbixms37ah5mppkc0jl12mf-octet-mantle-aeneas-consumer-job-v1.drv`; `job.json` blake3 `a14ec403…b318` (unchanged, bytes identical); changed source blake3 `776e9f85…c77f` → `98b8143d…f223`; tool-graph identity `b3:d0ebfcba…97808` → `b3:d5340b47…3a67b`; plan verdict `accepted` → `accepted`; `plan_identity` `5232099f…b0d3` → `fa8711cf…de0f`; root `node_id` `nix:vl7bqxyz…drv` → `nix:lhwv6ahi…drv`; root output `/nix/store/zjq6whhf2c641vkz89kgwwpbajjq5cbq-…` → `/nix/store/yayvxnadi7pqjrfgv7rjnij3y38g0zsn-…`; cache query for the new root output: `path '…' is not valid` (exit 1), control query for the old output: present (exit 0) | “the root derivation changes, and the plan shows a run, not a reuse” | **matches** — the root derivation, the plan identity, the root `node_id` and the root target output all change, and the cached output exists only for the old root, so the preserve route cannot reuse it |
| **N2** | `flake.lock` only: `aeneas` `bc2eb6f9…f0de` → `ab34bd28…8138` (tag `nightly-2026.11.01-ab34bd2`); root `nixpkgs`/`rust-overlay` pins deliberately preserved; re-frozen | `11`,`12`,`13a`,`13b`,`13c`,`13d`,`14a`,`15a`,`16a`,`16b` | root derivation `vl7bqxyz…drv` → `/nix/store/a7x94nyds465r2w0385g91s8fp7r60w5-octet-mantle-aeneas-consumer-job-v1.drv`; tool-graph identity `b3:d0ebfcba…97808` → `b3:a5e8b4da…5bb47`; graph derivations 11565 → 11489; `job.json` blake3 unchanged; `produce-nix` `accepted` (exit 0); `plan` **`rejected`** (exit 1) with `{"class":"foreign-compiler-leftover-reference","node_id":"nix:xdb17mqsnr2347mmdpx67hgzbafgs4g1-patchelf-static-0.18.0.drv","message":"rewritten field retains an unmapped foreign store object"}`; cache query: not applicable, the plan produced no root unit and no target output path | “`tool_graph_identity` changes, and the old observation no longer binds” | **matches** on both required points (identity differs; the old observation cannot bind a graph with a different identity) — and the plan additionally refuses the translated graph outright |
| **N3** | producer output whose `producer-artifact-verification.json` `status` is `invalid` instead of `valid` | `14-copy-n3-outputs`,`15-n3-report-diff`,`17-n3-bridge-identity`,`18-bridge-valid-fixed`,`19-bridge-invalid-fixed` | report differs only in `status` `valid` → `invalid`; positive control `n3-output-valid` → handoff written, exit 0, `observation` carries `external_report_identity b3:bfc3e95c…b444`, `producer_job_identity b3:a14ec403…b318`, `tool_graph_identity b3:d0ebfcba…97808` (the identity computed independently in N1); negative `n3-output-invalid` → `verification bridge: external artifact verification mismatch: expected valid, observed invalid`, exit 1, `n3-handoff-invalid` never created | “the bridge refuses verified mode, so the result is never `current`” | **matches** — the valid report is admitted, and the single invalid `status` is refused verbatim with no handoff directory written, so no `current` result can exist for it |
| **N4** | Octet evidence variant (N4-01…N4-09): `proof_decks` emptied; one well-formed `empirical_checks` record claiming the clause's allowed evidence id `aeneas.consumer.increment-matches-model` (`evidence_class` `empirical_check`, `status` `passed`, `checker_id` `lean-proof-runner`, real `command_argv`, the clause's allowed checked scope, the clause's three source bindings, and the realized job's Lean log as `artifact` with its BLAKE3 `20fe247f…9589`) | `N4-01`,`N4-02`,`N4-03`,`N4-04`,`N4-05`,`N4-06`,`N4-07`,`N4-08`,`N4-09` | Octet `requirements check`: exit 1; `aggregate_result` `passed` → `failed`; clause status `passed` → `missing`; `evidence_ids` `["aeneas.consumer.increment-matches-model"]` → `[]`; clause diagnostics, verbatim, ``clause `octet.aeneas_consumer_contract.compatibility` has no retained formal-proof card among allowed evidence ids`` and ``required clause `octet.aeneas_consumer_contract.compatibility` did not pass``; the `empirical_checks` record is never read under a `formal_proof` disposition. Cairn `convergence inspect` on the materialized case: pilot row `current` → `incomplete`; `issues` `[]` → `["convergence.decision_incomplete","convergence.proof_card_receipt_mismatch"]`; `bound` present → `null`; `receipt_blake3` `39ce3c51…5482` → `0d88a232…7283`; `targets_current` `true` → `false`; `complete` false in both because untargeted rows remain | “Offer empirical evidence for a `checked_proof` target → Octet rejects it; Cairn does not report `current`.” | **matches** — Octet refuses the clause (status `missing`, aggregate `failed`, exit 1) and Cairn's pilot row is `incomplete` with no bound block, so no `current` result exists for the offered evidence |

### N1 notes

* The named file `…/fixtures/external-consumer/src/lib.rs` at `d5b34c2d` has no
  comment at all — it is exactly three lines (`pub fn increment(value: u32) -> u32 {`,
  `    value + 1`, `}`), so “change one byte inside a comment of `src/lib.rs`”
  cannot be executed literally. The one-byte edit was made instead in the only
  comment among the job's declared source paths:
  `…/fixtures/external-consumer/Theorem.lean` line 3, `theorem` → `theorEm`.
  `cmp -l` is exactly one line (`49 145 105`, transcript `03-cmp-one-byte.txt`),
  and Lean semantics are untouched (doc-comment text only).
* The first freeze attempt (`05-freeze-src-change.txt`, exit 1) failed the same way
  the original WP4a attempts did: `nix flake metadata` resolves the revision but
  never materialises the flake source, so `export.py` reports
  `immutable owner source lacks nix/mantle-aeneas-job.nix`. The freeze recipe's own
  remedy — `nix flake prefetch` followed by a check of the materialised source — was
  added verbatim (`06-freeze-src-change.txt`, exit 0).
* The tool-graph identity changes too, not only the root derivation. The digests
  cover `nix derivation show --recursive` of the **root**, so the closure includes
  the owner source store path and Octet's own source-built tools. Four nodes move:
  the root and `cargo-octet`, `octet-standards`, `octet-policy-artifacts`
  (`07-compare-src-change.txt`); every other derivation is byte-identical.
* Cache queries are read-only `nix path-info --store file://…/cache` calls;
  the new path yields `error: path '…' is not valid` (exit 1) while the original
  output `zjq6whhf…` resolves (exit 0), so the query path itself is sound.

### N2 notes

* `nix flake lock --override-input aeneas …` also re-locks the root's branch
  inputs (`nixpkgs`, `rust-overlay`). The first attempt (`13b`) let them move;
  attempt `13c` pinned them to the wrong (older) revisions; `13d` pins them to the
  original lock revisions (`nixpkgs 9cf7092b…2512`, `rust-overlay e598b378…d72f`),
  so the only substantive change is `aeneas` and its subtree (`charon`,
  `minirust-src`, `jail-nix`, `nixpkgs-hol4`). `13a` records a cwd error: the
  command must run inside the clone.
* `plan` refusing the translated graph is a stronger outcome than reuse: with a
  different tool closure the graph no longer translates at all
  (`foreign-compiler-leftover-reference` at `patchelf-static-0.18.0.drv`).

### N3 notes

* The verification report the bridge requires is
  `producer-artifact-verification.json` read by `verified_report()`
  (`crates/crunch-verification-bridge/src/lib.rs:253-273`); the exact valid field
  is `/status == "valid"` (`crates/crunch-verification-bridge-core/src/lib.rs:311`,
  `equal(text(verification, "/status")?, Expected("valid"), "external artifact verification")`),
  with `schema_version == 1`, an empty `diagnostics` array, and a non-empty bounded
  `artifacts` array. The copy differs only in that one value
  (`15-n3-report-diff.txt`).
* The superseded binary
  (`/home/brittonr/wt/mantle-convergence/target/release/mantle-verification-bridge`)
  refuses **every** real producer output before reading the report, at its
  hard-coded `proofs/aeneas/provider-profile.json`
  (`crates/crunch-verification-bridge-core/src/lib.rs:42`,
  `crates/crunch-verification-bridge/src/lib.rs:211`), while Octet writes
  `proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/…`.
  Transcript `16-bridge-valid.txt` is that defect and is **not** N3 evidence.
* N3 was run with the fixed binary at
  `/home/brittonr/wt/mantle-bridge/target/release/mantle-verification-bridge`,
  BLAKE3 `ae41e077e8d9024626b8b6826620069fbf56829b97be469b06930d9533743282`,
  build commit `b14db98cb473c4bad449ce6dd68e33d96519d605`
  (`17-n3-bridge-identity.txt`). Hash checked before use; the binary was not
  rebuilt. It derives the provider-profile locator from the receipt instead of the
  stale constant, so the positive control admits the real producer output
  (`18-bridge-valid-fixed.txt`, exit 0) and the negative is refused
  (`19-bridge-invalid-fixed.txt`, exit 1) with
  `external artifact verification mismatch: expected valid, observed invalid`.
  No `n3-handoff-invalid` directory exists afterwards; no non-verified or fixture
  mode was used.

### N4 notes

* N4 was produced under `local://empirical-negative-spec.md`, not
  `freeze-negatives-spec.md`. Everything lives in `N4-empirical/` plus one `/tmp`
  case copy; no worktree, retained record, store path or commit was touched, and
  the case is a copy (hard-link/copy rule) of the retained pilot case with only
  `evidence.json` and `decision.json` replaced (transcript `N4-08`: those two
  files are the only differences). Transcripts are `N4/transcripts/NN-name.txt`,
  written by a copy of `realization/run-transcript.sh` whose `dir` points there.
* Octet binaries and inputs (`N4-01`): `cargo-octet` BLAKE3
  `6e576a4ee08b2d829fa4a130b69b2a40fa7f54e95df56a547303672a2535647b`, version
  `cargo-octet 0.1.1-rc.1`, built from `e5086dca9b7feabfad37df3769390cc5005811f5`
  (tree `v0.1.0-1103-ge5086dca`); source root `/home/brittonr/wt/e2e/octet-rev` at
  `d5b34c2d`; artifact log BLAKE3
  `20fe247f23727146eb84dd18b4e3923958c3d2197c96e0687f77c165beca9589`; the three
  source bindings hash to the contract's declared values.
* `cairn` was rebuilt at 11:00:10 during this session (an earlier build was
  replaced, so only this build ran any N4 command): BLAKE3
  `af873b99e2391460e309f3ca22804c34f0faee0d4438fbbd992f8db54f0a7763`,
  mtime 2026-11-20 11:00:10 -0500, from the `c8b38c4e02928afd5e2f746293553b4929b27508`
  tree with no `crates/` changes (`git status --short` shows only docs plus
  untracked evidence files). Re-checked after the runs (`N4-09`), unchanged.
* The evidence variant differs from the pilot `evidence.json` only in the two
  intended ways (`N4-02`): `proof_decks` is `[]` and one `empirical_checks` record
  is added. The record is otherwise valid by the same code Octet applies to
  empirical evidence (`validate_empirical_check`: class `empirical_check`, status
  `passed`, non-empty `command_argv` and `checked_scope`, readable artifact whose
  BLAKE3 matches, unchanged source bindings), and it claims exactly the clause's
  allowed evidence id and checked scope. Its only defect against the clause is
  class and tier: the clause is `formal_proof` with
  `required_evidence_classes: ["proof_card"]` and `expected_evidence_tier:
  formal_proof`.
* True strength of the Octet result: under `disposition: formal_proof` Octet
  reads only `proof_decks` (`evaluation/formal.rs:1-59`); `empirical_checks` is
  never consulted, so the clause is `missing` rather than `failed`, and the
  diagnostics say only that no formal-proof card remains. Octet therefore refuses
  the offer, but by absence of a proof card, not by an explicit "empirical
  evidence cannot satisfy formal_proof" message. The code path that would say
  that (`evaluation/clause.rs:57-95`, "empirical evidence cannot satisfy the
  formal-proof requirement") is only reachable for an `empirical_check`
  disposition.
* Cairn: `convergence inspect` exits 0 regardless of row state, so the row state,
  not the exit code, is the result. Policy maps Octet clause `missing` →
  `incomplete` (`cairn-policy/generated/assurance-targets.json`, `octet.clause_states`),
  giving `convergence.decision_incomplete`; the second issue
  `convergence.proof_card_receipt_mismatch` follows from emptying `proof_decks`
  (the case no longer carries the proof card the handoff receipt binds). The
  decision's `evidence_blake3` (`9d36a9df…9862`) equals the BLAKE3 of the swapped
  evidence file, and `contract_blake3` is unchanged, so `incomplete` is not a
  decision-binding/staleness artifact.
* Positive control (`N4-05`, `N4-07`): the untouched retained pilot case through
  the same binary and root gives row `current`, `issues: []`, `bound` populated,
  `receipt_blake3 39ce3c51b8c619091653d6c4f90bcca238187ca544c3477399051bee57535482`,
  `targets_current: true`, `complete: false` (untargeted rows).
* Reproduce: copy the retained case, then
  `cargo-octet requirements check --contract …/contract.json --evidence N4/evidence-empirical.json --source-root /home/brittonr/wt/e2e/octet-rev --output N4/decision-empirical.json --output-format json`
  (exit 1), and
  `cairn convergence inspect --case /tmp/n4-empirical-case/case.json --root /tmp/n4-empirical-case/root`.

### Housekeeping

`export-src-change/source-payloads` and `export-lock-change/source-payloads`
(the 0.8 GB raw source copies) were moved to `/tmp/neg-payloads/` because the root
filesystem was at 100 %. Bytes are unchanged and no identity above depends on
them: every digest in this file was computed from `job.json`,
`tool-closure.derivation-json.json`, `Theorem.lean` and the plan files, all of
which remain in place. Tree sizes: 45604 entries (src change, matching the
pre-move count) and 46524 entries (lock change).

## Every command and its exit code

| # | transcript | command (abridged) | exit |
|---|---|---|---|
| 01 | `01-clone-src-change.txt` | `git clone …/octet NEG/octet-src-change` | 0 |
| 02 | `02-checkout-src-rev.txt` | `git -C … checkout --detach d5b34c2d…` | 0 |
| 03 | `03-cmp-one-byte.txt` | `cmp -l /tmp/n1-old-Theorem.lean …/Theorem.lean` | 1 (files differ; one line) |
| 04 | `04-commit-src-change.txt` | `git -C … commit -a -m "negative control (not for push): one byte inside a doc comment"` | 0 |
| 05 | `05-freeze-src-change.txt` | `./freeze-src-change.sh` (first attempt, no prefetch) | 1 |
| 06 | `06-freeze-src-change.txt` | `./freeze-src-change.sh` (prefetch + `export.py`, `--graph-format nix-derivation-show-v4`) | 0 |
| 07 | `07-compare-src-change.txt` | `./compare-src-change.sh` (root drv, b3sums, graph diff) | 0 |
| 08 | `08-mantle-src-change.txt` | `mantle --json foreign-import produce-nix …` then `… plan …` into `mantle-src-change/` | 0 |
| 09 | `09-cache-query-src-change.txt` | `nix --option min-free 0 path-info --store file://…/cache /nix/store/yayvxna…-…` | 1 (`is not valid`) |
| 10 | `10-cache-query-control.txt` | same query for `/nix/store/zjq6whhf…-…` | 0 |
| 11 | `11-clone-lock-change.txt` | `git clone …/octet NEG/octet-lock-change` | 0 |
| 12 | `12-checkout-lock-rev.txt` | `git -C … checkout --detach d5b34c2d…` | 0 |
| 13a | `13a-lock-override-cwd-error.txt` | `nix flake lock --override-input aeneas …` run in `NEG/` (no flake there) | 1 |
| 13b | `13b-lock-override-aeneas.txt` | same, in the clone; branch inputs moved as a side effect | 0 |
| 13c | `13c-lock-override-aeneas.txt` | same with `nixpkgs`/`rust-overlay` pinned to the wrong (older) revisions | 0 |
| 13d | `13d-lock-override-aeneas.txt` | same with `nixpkgs 9cf7092b…` and `rust-overlay e598b378…` restored | 0 |
| 14 | `14-copy-n3-outputs.txt` | `cp -a` the producer output twice into `n3-output-valid` / `n3-output-invalid`, `chmod -R u+w` | 0 |
| 14a | `14a-commit-lock-change.txt` | `git -C … commit -m "negative control (not for push): aeneas flake input lock override" flake.lock` | 0 |
| 15 | `15-n3-report-diff.txt` | `diff -u` the two verification reports | 1 (one line differs) |
| 15a | `15a-freeze-lock-change.txt` | `./freeze-lock-change.sh` (prefetch + `export.py`) | 0 |
| 16 | `16-bridge-valid.txt` | `mantle-verification-bridge octet-job n3-output-valid …` — **superseded binary** | 1 (`proofs/aeneas: No such file or directory`) |
| 16a | `16a-produce-nix-lock-change.txt` | `mantle --json foreign-import produce-nix …` into `mantle-lock-change/` | 0 |
| 16b | `16b-plan-lock-change.txt` | `mantle --json foreign-import plan …` into `mantle-lock-change/` | 1 (`rejected`, `foreign-compiler-leftover-reference`) |
| 17 | `17-n3-bridge-identity.txt` | `ls -l`, `b3sum` and `git log -1` on the fixed `mantle-verification-bridge` | 0 |
| 18 | `18-bridge-valid-fixed.txt` | `mantle-bridge/…/mantle-verification-bridge octet-job n3-output-valid R/producer-export REV n3-handoff-valid` | 0 (handoff written) |
| 19 | `19-bridge-invalid-fixed.txt` | same command on `n3-output-invalid` into `n3-handoff-invalid` | 1 (`external artifact verification mismatch: expected valid, observed invalid`; no directory written) |
| N4-01 | `N4/transcripts/01-identities.txt` | `b3sum`, `--version`, `git log -1` on `cargo-octet` and `cairn`; `b3sum` of the artifact log and the three source bindings | 0 |
| N4-02 | `N4/transcripts/02-evidence-diff.txt` | `diff -u` pilot `evidence.json` → `N4/evidence-empirical.json` | 1 (proof deck removed, empirical check added) |
| N4-03 | `N4/transcripts/03-octet-check-empirical.txt` | `cargo-octet requirements check --contract …/contract.json --evidence N4/evidence-empirical.json --source-root /home/brittonr/wt/e2e/octet-rev --output N4/decision-empirical.json --output-format json` | 1 (`aggregate_result: failed`, clause `missing`) |
| N4-04 | `N4/transcripts/04-cairn-inspect-empirical.txt` | `cairn convergence inspect --case /tmp/n4-empirical-case/case.json --root /tmp/n4-empirical-case/root` | 0 (report seals `receipt_blake3 0d88a232…7283`) |
| N4-05 | `N4/transcripts/05-cairn-inspect-positive-control.txt` | same command on the untouched retained pilot case and root | 0 (row `current`, `receipt_blake3 39ce3c51…5482`) |
| N4-06 | `N4/transcripts/06-cairn-row-empirical.txt` | `cairn … inspect … \| N4/extract-row.py` (pilot row, receipt, completeness flags) | 0 (row `incomplete`, issues `decision_incomplete`, `proof_card_receipt_mismatch`) |
| N4-07 | `N4/transcripts/07-cairn-row-positive-control.txt` | same extraction on the retained pilot case | 0 (row `current`) |
| N4-08 | `N4/transcripts/08-case-diff.txt` | `diff -rq` retained pilot case vs `/tmp/n4-empirical-case` | 1 (only `evidence.json`, `decision.json` differ) |
| N4-09 | `N4/transcripts/09-identities-after.txt` | `b3sum` both binaries, `ls -l`, `git status --short` and `git diff --stat -- crates` on `cairn-convergence` after the runs | 0 (binary hash unchanged; no `crates/` changes) |

Fixed inputs used: `R/producer-export` (root derivation `vl7bqxyz2f86x0vlvxipcvryavckb9wj-…drv`,
`job.json` blake3 `a14ec403025db0330379bad1e1904fae23835ccd1ab95017a80e3e5ca715b318`),
`R/mantle-graph-preserve/plan.json` (plan identity `5232099fe9900dc116bee565fcb6155744f222871127b2f5605154588564b0d3`,
root output `/nix/store/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1`).
