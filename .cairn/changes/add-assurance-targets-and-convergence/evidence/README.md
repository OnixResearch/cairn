# Evidence: assurance targets and convergence

This directory holds the change's lifecycle gates (`gates/`), the Mantle fixture rehearsal (`rehearsal/`, which is fixture packaging and runs no proof), the real pilot (`pilot/`), and the full gate (`full-gate/`, with a pre-existing `buoy_comparison` test failure documented in `full-gate/README.md`). `pilot/BLAKE3SUMS` lists every retained pilot file.

A transcript larger than 1 MiB was not committed. In its place is `<name>.summary.txt`, which records the full transcript's byte size, BLAKE3, and first and last lines. The full transcripts stay under `/home/brittonr/wt/convergence-evidence/` on britton-desktop.

## Revisions

| Repository | Revision | Role |
| --- | --- | --- |
| Octet | `d5b34c2d64b1227417a46df8ed8427f2d10ed3c8` | Frozen pilot revision (REV) |
| Octet | `e5086dca9b7feabfad37df3769390cc5005811f5` | Pilot contract, evidence and decision. Archived in `d50a7536`; README note in `29417562`, which is branch `add-cairn-convergence-pilot`, [PR #33](https://github.com/OnixResearch/octet/pull/33) |
| Mantle | `b99ed8637` (branch `raise-foreign-import-graph-bounds`) | Foreign-import bounds raised to the producer contract (ADR 0166). Used for plan and realization |
| Mantle | `61f06cc59` (branch `admit-octet-producer-layout-in-verification-bridge`) | Bridge admits Octet's current producer layout and v4 graphs (ADR 0167). `b14db98cb` wrote the pilot handoff, and `7fdb6b861`/`61f06cc59` reproduce it byte for byte |
| Cairn | `c8b38c4` | Retained pilot corpus, producer record bounds, closed proof-acceptance relay set |

The pilot signing key is `convergence-pilot-20261119-1:sRZV4nbjXTCkhRjkEIVe5Pg4G2ajYaCobLqS0JJVu2A=`. It is the case's only trusted signer, and no shared cache is configured. Route label: prebuilt closure preserved (Mantle `preserve-cache-paths-v1` / `cache-only-preserve-v1`). Nix executed the producer job and its proof. Mantle verified the pilot-key signature and preserved the output. Mantle did not build the tool graph.

## Design steps 1-6

Steps 1-5 were first run as the pilot (`pilot/realization/`), then replayed from clean detached checkouts (`pilot/e2e/`). Every replayed output is byte-identical to the retained record.

| Step | Pilot run | Clean replay (`pilot/e2e/transcripts/`) | Result |
| --- | --- | --- | --- |
| 1 Author | Octet `targets.ncl`, `generated.ncl` and `contract.ncl`, exported to JSON at `e5086dca` | `01`, `02`: Nickel 1.17.0 re-export | identical (`56c80346…`, `a3c483b1…`, `56d7f459…`) |
| 2 Freeze | `export.py` at REV: 11,565 derivations, 677 sources | `03`, `04` | identical tree: 37,860 files and 44 symlinks |
| 3 Plan and realize | `realization/transcripts/34`-`41`: plan accepted, `cache-only-preserve-v1`. Realization `complete`/`realized`: 11,565 units (366 already present, 125 substituted, 11,074 not required); closure of 526 (131 substituted, 395 local reuse) | `05`, `10`-`15`, `18`: same `plan_identity` `edd387e5…`, same `manifest_blake3` `8ed1103b…`; reuse realization imports 0 and makes 0 NAR requests | identical plan and receipt |
| 4 Observe | `realization/transcripts/54`: `octet-job`, `proof_acceptance` `externally-reported-by-octet` | `16`, `17` | handoff tree identical |
| 5 Judge | Octet `requirements check`: aggregate `passed`, clause `formal_proof` `passed`; `verify` recomputes `906cc950…` | `06`-`09` | `decision.json` identical (`ca047f54…`) |
| 6 Inspect | `fixtures/convergence/pilot` via `tests/convergence_pilot.rs`: pilot row `current`, 739 rows `untargeted` | `20`: the retained case from a clean checkout at `c8b38c4` with the packaged binary. `21`, `22`: a case assembled from the replayed outputs and the **reuse** receipt | `20`: same receipt, `39ce3c51…`. `22`: `current`, `9757e63b…` (different bound records) |

Observation identities: `receipt_identity` `b3:84d2e777…`, `producer_job_identity` `b3:a14ec403…`, `tool_graph_identity` `b3:d0ebfcba…`, `external_report_identity` `b3:bfc3e95c…`. The root output is `/nix/store/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1`, with NAR SHA-256 `2a12b198…` and NAR size 580,056.

## Required negative controls

| Handoff control | Where | Observed |
| --- | --- | --- |
| Realize the unchanged job again | `realization/transcripts/48`, `e2e/transcripts/15`, `22` | Mantle imports 0 paths, makes 0 NAR requests and starts no build; Cairn reports `current` over the reuse receipt |
| One source byte changed, re-frozen | `negatives/` N1 | root derivation `vl7bqxyz…` becomes `lhwv6ahi…`; the plan's root unit is new, and the pilot cache has no output for it (`is not valid`), so it cannot be reused. Cairn's `unrealized` is covered by `no-mantle-records` |
| Lock change alters a tool | `negatives/` N2; corpus `tool-graph-changed` | `tool_graph_identity` `d0ebfcba…` becomes `a5e8b4da…` (Mantle's plan also rejects the new graph); Cairn reports `stale` with `convergence.tool_graph_identity_mismatch` |
| Edit a byte of the reviewed `spec.md` | corpus `spec-edited` | `stale`: `registry_unverified`, `reviewed_specification_stale` |
| Observation from another revision | corpus `observation-other-revision` | `stale`: `revision_mismatch` |
| Tamper with one payload in the realized output | `realization/transcripts/55` | the bridge refuses (`llbc.llbc mismatch`) and writes no handoff |
| Producer report not valid | `negatives/` N3 | the bridge refuses verified mode (`expected valid, observed invalid`) and writes no handoff |
| Output signed by an untrusted key | `realization/transcripts/50`; corpus `untrusted-signer` | Mantle refuses (`http-closure-untrusted-narinfo`) and writes no receipt; Cairn reports `unrealized` with `untrusted_signer` |
| Weaker clause disposition | corpus `weakened-contract` | `stale`: `projection_mismatch` |
| Decision for another contract | corpus `decision-other-contract` | `stale`: `contract_blake3_mismatch` |
| Empirical evidence for a `checked_proof` target | `negatives/N4-empirical/` | Octet exits 1 with aggregate `failed`, and the clause is `missing`: "has no retained formal-proof card among allowed evidence ids". Cairn reports the row `incomplete` (`decision_incomplete`, `proof_card_receipt_mismatch`), not `current` |
| Mantle records omitted | corpus `no-mantle-records` | `unrealized` |
| Unadmitted schema | corpus `unknown-schema` | rejected: `unknown_schema` |
| One requirement targeted in a multi-requirement spec | positive case | `coverage_complete` false, `complete` false |
| Two runs | positive test, `e2e/transcripts/20` | byte-identical receipts |
| Root before and after | positive test | unchanged |

The corpus (`fixtures/convergence/pilot-negatives/`, twenty records) adds one negative for each remaining binding check. The tampered NAR in a copy of the pilot cache (`realization/transcripts/52`, `53`) is rejected by Mantle's content admission.

## Deviations recorded during the pilot

- **Mantle bridge layout.** The bridge refused Octet's current producer layout. Mantle change `admit-octet-producer-layout-in-verification-bridge` fixes it; see design.md.
- **Proof-acceptance relay.** The observation from verified mode reports `externally-reported-by-octet`, so Cairn admits the closed set {`not-evaluated-by-mantle`, `externally-reported-by-octet`} and reads neither value as acceptance.
- **Record bounds.** The tool graph is 38,785,189 bytes and the realization receipt 6,469,512 bytes, so both are read within their producers' bounds.
- **Host store defect and cache repair.** Two pilot-cache NARs were taken from cache.nixos.org and re-signed with the pilot key, and the cache was served over loopback HTTP; see design.md and `realization/FINDINGS.md`.

## Claim boundary

A `current` pilot row shows only this: for Octet REV `d5b34c2d`, the supplied Mantle records and Octet decision bind to the declared target. The proof is Octet's external-consumer fixture theorem, and it ran under Nix; Mantle preserved and verified the pilot-key-signed output and did not build the tool graph. Nothing here shows that production code is proved, that a verifier, builder or sandbox is sound, or that any release is eligible.
