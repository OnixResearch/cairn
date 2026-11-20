# WP4a findings — freeze Octet's producer job and assess Mantle realization feasibility

Date: 2026-11-19. Scope: freeze `octet.aeneas.external-consumer.mantle.v1` at Octet
`d5b34c2d64b1227417a46df8ed8427f2d10ed3c8` and decide whether/how Mantle can plan its
graph. No realization build was started. Nothing in Octet or Mantle was edited.

## 1. Inputs read

- Octet `docs/aeneas-consumer-contract.md` §"Mantle producer handoff" — the job
  (`octet-mantle-producer-job/v1`), the two wire formats, and the fact that
  `packages.x86_64-linux.octet-mantle-aeneas-consumer-job` runs the real
  `cargo-octet prove` + `artifact verify` in the sandbox with network denied.
- Octet `standards/proof-rails/aeneas-lean/mantle-job.ncl` (68 lines) — the job record.
- Octet `nix/mantle-aeneas-job.nix` (131 lines) — the ordinary `runCommand` producer.
- Octet `tools/mantle-producer/export.py` (176 lines) — acquisition-only exporter.
- Mantle `docs/foreign-realization-operator-guide.md`,
  `docs/verification-build-bridge.md`, `tests/fixtures/foreign-import/*`,
  `config/foreign-execution-profiles/generated/nix.json`.

`python3 tools/mantle-producer/export.py --help` (exit 0) requires exactly:
`--nix --nickel --flake --revision --graph-format {nix-derivation-show-legacy,nix-derivation-show-v4} --out`.

## 2. Host constraints found before the freeze

- `/nix` was below `min-free`; plain `nix` blocked for minutes on
  `running auto-GC ... waiting for the big garbage collector lock` and the auto-GC
  deleted the freshly added flake source between steps. Fix used: every `nix`
  invocation went through a wrapper recorded verbatim below.
- The host was running several unrelated `nix flake check` builds; load average peaked
  at 111 on 32 cores, so daemon round-trips for a 6,259-file tree took minutes.

Wrapper `realization/nix-minfree0` (mode 0755), full text:

```sh
#!/bin/sh
# Records and neutralises the host min-free policy: /nix is below min-free, so an
# unmodified nix blocks for many minutes on "running auto-GC ... waiting for the big
# garbage collector lock". This wrapper only disables that automatic GC trigger; it
# never deletes store paths. All arguments are passed through.
exec /run/current-system/sw/bin/nix --option min-free 0 "$@"
```

Disk at time of writing: `/nix` 548G total, 395G used, 153G free (73%);
`/home` 1.8T total, 1.7T used, 55G free (97%). The real realization builds the whole
Aeneas/Charon/Lean/Rust closure, so `/nix` (or the `--store` directory) needs
tens of GB free and `/home` is close to full.

Tool paths used:

- nix: `/run/current-system/sw/bin/nix` (Nix 2.36.0), via the wrapper.
- nickel: `/nix/store/ck3a4k9az87z5rjwgy8nmpcs34xp9cc5-nickel-lang-rev-fixup-1.17.0/bin/nickel`
  (`1.17.0 (rev 1320a98)`), which is the lock's nickel rev
  `1320a983e6c3d1e2fb53dd2464b084b4903b1426`. `nix develop -c …` was not usable:
  the dev-shell command did not finish within 300 s.

## 3. Freeze attempt

Command (transcripts `01-export.attempt*.txt`):

```
cd /home/brittonr/wt/octet-convergence            # git rev = REV, tree clean
python3 tools/mantle-producer/export.py \
  --nix  <wrapper nix-minfree0> \
  --nickel /nix/store/…-nickel-lang-rev-fixup-1.17.0/bin/nickel \
  --flake 'git+file:///home/brittonr/git/OnixResearch/octet?rev=d5b34c2d64b1227417a46df8ed8427f2d10ed3c8' \
  --revision d5b34c2d64b1227417a46df8ed8427f2d10ed3c8 \
  --graph-format nix-derivation-show-v4 \
  --out  /home/brittonr/wt/convergence-evidence/realization/producer-export
```

Result: **exit 1, no export.** Exact error:

```
ValueError: immutable owner source lacks nix/mantle-aeneas-job.nix; recorded fixture pins cannot substitute
```

Cause, established with `strace` and by direct probe: `nix flake metadata --json` on a
`git+file` flake resolves the revision correctly (it returns
`git+file:///home/brittonr/git/OnixResearch/octet?ref=refs/heads/release/public-0.1.1-rc.1-20261112&rev=d5b34c2…`,
path `/nix/store/6s4aph8fm1wq5q9rz5ly7bjq0d5fxh79-source`) but **does not materialise that
store path on disk**. `nix path-info` on it answers `path … is not valid`. export.py then
does `(locked_root / required).is_file()` on a non-existent directory and fails closed —
which is the exporter behaving correctly, not a defect in the freeze procedure.

The path only appears once the source is added to the store (`nix flake prefetch`,
`nix flake archive`, or any evaluation of the flake). That add is the bottleneck on this
host: the client streams the whole NAR (43 MB written) in about one second and then blocks
on the daemon; `nix store add-path` of the same 6,259-file tree needed 90 s+ and was not
finishing while unrelated builds saturated the machine. Sub-adds of every top-level
directory of the tree completed in ~1 s each, so the tree itself is valid.

A third attempt, `transcripts/01-export.attempt3.txt`, chains
`nix flake prefetch --json <flake>` (1200 s), then `nix build --out-link
realization/source-root <path>`, then the identical `export.py` command, under the
wrapper. It was still waiting on the daemon at hand-off; that transcript is the record of
whatever it produced. Final outcome of that attempt: `prefetch EXIT=124` (the 1200 s
`timeout` killed it), `materialized=` empty, so the export step was skipped. The Octet
source at REV was therefore **never materialised** on this host during this task.

State of the output directory at hand-off:

| Path | Meaning |
| --- | --- |
| `realization/nix-minfree0` | the recorded nix wrapper (text above) |
| `realization/translation-policy.json` | Mantle translation policy prepared for the plan step |
| `realization/transcripts/01-export.attempt*.txt` | exact commands, stdout, stderr, exit codes of the freeze attempts |
| `realization/transcripts/06-store-verify.txt` | `mantle store verify` transcripts |
| `realization/source-root` | GC root for the octet source, if the prefetch succeeded |

**Export summary: not produced.** No `job.json`, no `tool-closure.derivation-json.json`,
no `root-derivation`; therefore no derivation count, no fixed-output count, and no
lean/charon/aeneas/rustc/root derivation list can be reported. Nothing was substituted.

## 4. What the export would contain (from the exporter's own code, not observed output)

Not a substitute for the numbers: this is what `export.py` writes once the source is real.

- `job.json` (Nickel export of `mantle-job.ncl`), `tool-closure.derivation-json.json`
  (`nix derivation show --recursive <root drv>`, raw v4 bytes), `flake-metadata.json`,
  `flake.lock`, `root-derivation`, `source-revision`, and `source-payloads/` with a copy of
  every `inputSrcs` object in the closure (symlinks preserved, directories copied).
- Bounds enforced: graph ≤ 256 MiB, ≤ 32,768 derivations, ≤ 16,384 source objects,
  ≤ 32 GiB of source bytes; every source must be a direct child of `/nix/store`.
- `producer_graph()` fails closed on mixed legacy/v4, unknown version, dynamic outputs,
  incomplete recursion, or a root that does not bind `str(locked_root)` in `inputSrcs`.

Job record fields, from `mantle-job.ncl` (these are source facts, not export output):
`job_id = octet.aeneas.external-consumer.mantle.v1`,
`package_attribute = packages.x86_64-linux.octet-mantle-aeneas-consumer-job`,
`owner = Octet`, `system = x86_64-linux`,
`limits.cores = 32`, `limits.timeout_ms/stdout/stderr` inherited from the consumer profile,
`execution.network_allowed = false`, `source.network_allowed = false`,
`tools.graph_formats = [nix-derivation-show-legacy, nix-derivation-show-v4]`,
`tools.solver = "none: …"`,
`tools.dependencies = [cargo-octet, octet-policy-artifacts, octet-aeneas-lean-provider-closure,
<aeneas rev>, <charon rev>, <lean toolchain>, flake.lock:mathlib-src, flake.lock:nixpkgs,
flake.lock:nixpkgs:stdenv.cc, flake.lock:aeneas]`.

## 5. Mantle planning feasibility (no build)

Mantle cannot consume Octet's v4 `nix derivation show` bytes directly: `foreign-import plan
--graph` takes `foreign-derivation-graph-v1`. The bridge is a real, shipped subcommand:

```
mantle foreign-import produce-nix \
  --derivation-json <export>/tool-closure.derivation-json.json \
  --root-derivation "$(cat <export>/root-derivation | tr -d '\n')" \
  --producer-identity octet@<REV> --producer-revision <REV> \
  --package octet-mantle-aeneas-consumer-job --system x86_64-linux \
  --out-dir <out>/mantle-graph
```

`produce-nix` accepts `--derivation-json` alone (no `--drv`/`--drv-dir`) and writes
`nixpkgs.graph.json` (`foreign-derivation-graph-v1`) plus `nixpkgs.index.json`
(`foreign-package-index-v1`). Its v4 normaliser
(`normalize_versioned_nix_derivations`) is strict: every output must carry a concrete
`path`, and nodes must not mix `version`/`inputs` with `inputDrvs`/`inputSrcs`. Octet's
closure is evaluated with `ca-derivations` enabled on this host, so floating
content-addressed outputs without a concrete path are the most likely rejection class —
this cannot be confirmed without the export.

Then:

```
mantle --json --state-dir <out>/plan-state --store <out>/plan-store \
  foreign-import plan \
  --graph <out>/mantle-graph/nixpkgs.graph.json \
  --package-index <out>/mantle-graph/nixpkgs.index.json \
  --policy <out>/translation-policy.json \
  --package octet-mantle-aeneas-consumer-job --system x86_64-linux \
  --execution-profile config/foreign-execution-profiles/generated/nix.json \
  --plan-out <out>/plan.json --receipt-out <out>/import-receipt.json
```

`plan` is pure with respect to the store: it reads the three JSON inputs, translates,
and atomically publishes only the plan and import receipt (verified by
`run_plan`). It builds nothing.

Policy: `realization/translation-policy.json` is the fixture policy's field set
(`source_prefixes ["/nix/store"]`, `target_prefix "/mantle/store"`,
`builtin_mappings {fixed-output-fetch: mantle.fetch, nix.derivation:
mantle.foreign.nix.derivation}`, `output_path_recompute_mode recompute-blake3-v1`,
`trusted_cache_scopes ["trusted-binary-cache"]`,
`allow_embedded_source_payload_rewrite false`, `allowed_sandbox_capabilities []`).

**Plan result: blocked by a hard Mantle bound, proved on a real Nix v4 graph.**

Octet's own graph could not be frozen (see §3), so the lower/plan half was exercised
against a *real* Nix 2.36 v4 closure that this host already has, to separate "Mantle cannot
plan" from "Octet's graph was not exported":

```
nix derivation show --recursive /nix/store/36qrksi…-tmpfiles.d.drv > nix-graph.json   # exit 0, version 4
mantle --json foreign-import produce-nix --derivation-json nix-graph.json \
  --root-derivation /nix/store/36qrksi12r9c2hmi5bc454kqr6cynrgp-tmpfiles.d.drv \
  --producer-identity nix-2.36.0 --producer-revision smoke \
  --package tmpfiles --system x86_64-linux --out-dir plan-smoke/mantle
```

Exit **1**, verdict `rejected`:

```json
{"schema":"mantle-foreign-import-cli-v1","command":"produce-nix","verdict":"rejected",
 "diagnostics":[{"class":"nix-derivation-count-out-of-range","node_id":null,
 "message":"Nix derivation export size is outside supported limits"}],"receipt":null,"plan":null}
```

Transcript `transcripts/07-produce-nix-smoke.txt`. Cause:
`MAX_GRAPH_NODES = 2048` in `src/foreign_derivation_import.rs:82`, enforced at lines
1203/1234 (legacy/ATerm), 1478 (v4), and in the `foreign-derivation-graph-v1` validator. The smoke graph has **7,157** derivations — a
single trivial stdenv-based derivation on this host is already 3.5× the bound. Two larger
real closures measured here: 23,257 and 45,586 derivations.

So the size limit is not a property of Octet's job; **no useful Nix closure imports at all**
under the current bound. `plan` shares the same module and the same constant, so it would
reject the translated graph for the same reason.

Consequence: Octet's producer closure (Aeneas + Charon + Lean + Mathlib + Rust + Nixpkgs
bootstrap; the exported graph is bounded by `export.py` at 32,768 derivations) cannot be
imported or planned by Mantle as shipped. This is exactly the "unsupported construct that
needs a Mantle source change" case, i.e. **stop and ask**, not a workaround.

## 6. If planning succeeds: source supply and scale

`prepare-sources` needs one `--source <payload-id>=<path>` per non-derivation source
requirement in the plan, or a Nario v2 archive plus a trusted public key. The Nario route
requires a pinned Determinate-Nix `nario` archive and a `--nario-trusted-public-key`; this
host has no Nario archive and the route demands original Nix trust signatures, so it is
not available offline here without producing that archive first. The realistic offline
route is `--source` bindings taken from the export's own `source-payloads/` (the exporter
copies every `inputSrcs` object), which needs no network and no `/nix/store` reads at
realization time (Mantle refuses ambient `/nix/store` reads).

Scale: measured lower bounds from this host — a trivial stdenv `tmpfiles.d` closure is
7,157 derivations, a system `user-units` 23,257 and a system `etc` 45,586, against Mantle's
2,048-node import bound (§5). Octet's own count is unknown without the export, but the job
record shows the closure necessarily contains
the Aeneas and Charon toolchains, the Lean toolchain plus Mathlib, `rustToolchain`,
`cargo-octet`, `octet-policy-artifacts`, `octet-aeneas-lean-provider-closure`, `stdenv.cc`,
and Nixpkgs bootstrap (the graph "includes declared bootstrap/binary acquisition inputs").
That is a multi-thousand-derivation graph, most of it compile work, and the job itself
self-caps at 32 cores, with an OCaml-domain warning already recorded upstream.

Realization prerequisites observed on this host: `mantle doctor --json` reports
`bwrap: failed (bubblewrap executable not found)` and `store-dir: failed (/nix/store is not
writable)`. bwrap binaries exist in the store
(`/nix/store/g7svy17fhkg2cq3q4lfzzc0mmsl3d8hq-bubblewrap-0.11.2/bin/bwrap` and others), so
`SNIX_BUILD_BWRAP` or PATH must supply one; a private `--store` avoids the writability
failure (WP3 already did this).

## 7. Signing-key mechanics (pilot-only key)

- Key path: `${CRUNCH_CONFIG_DIR:-<state-dir>}/signing-key` — a fresh `--state-dir`
  therefore owns its own key at `<state-dir>/signing-key`.
- Creation: `load_or_generate_signing_keypair` generates on first use (first build/realize
  in that state dir), writes 0600 through a `.signing-key.*` temp file with
  `persist_noclobber`, so a concurrent first run has exactly one winner. Readers reject
  files that are symlinks, non-regular, >4,096 bytes, or not `0600`.
- Key file format: one line `name:base64(secret32||public32)`, i.e. the Nix
  `nix key generate-secret` format.
- Key name: `crunch-<hostname>-1`. The WP3 state on this host holds
  `crunch-britton-desktop-1`.
- Public key extraction: `nix key convert-secret-to-public < <state-dir>/signing-key`
  returns `crunch-britton-desktop-1:V1QkLi6cBA6owBScwU9VUqoW0NtGEHcxXBqlvNwVB8U=`
  (exit 0). That string is what goes in `--trusted-public-keys` and in the case file.
  Mantle has no dedicated key subcommand.
- `mantle store info <fragment> --json` (schema `mantle-store-info-v3`) reports, per path,
  `signatures: ["crunch-britton-desktop-1:mXshOr5q2yEFnAKc9omevTUXLgjzZAGeo15lVvKdSntyG7We1LYLWft1A+8hjekCA68tyZKRGGAfh2zmTLDPCA=="]`
  for `glnhkn9a41rc31jqjj0hillcvr8alf5c-verification-bridge-octet-handoff` in the WP3 store
  (run read-only against
  `/home/brittonr/wt/convergence-evidence/rehearsal/bridge-packaging-proof-storefix/{state,store}`;
  exit 0).
- `mantle store verify --trusted-public-keys 'crunch-britton-desktop-1:…'` prints
  `OK <path>  trusted_signatures=1/1` for the two WP3 handoff outputs (exit 0 with the
  correct key). With `--trusted-public-keys 'cache.nixos.org-1:…'` the same paths report
  `MISSING`/mismatch, `25 checked, 23 mismatches`, exit 1 and
  `{"error":"23 path(s) failed verification","code":1,"kind":"build"}`. Transcript:
  `transcripts/06-store-verify.txt`.

## 8. Recommendation

**(b) needs a Mantle change — stop and ask.** Two independent blockers, in order:

1. *Hard, decisive:* Mantle's foreign-import bound `MAX_GRAPH_NODES = 2048`
   (`src/foreign_derivation_import.rs:82`) rejects every real Nix closure on this host,
   proved with a trivial 7,157-node closure (`transcripts/07-produce-nix-smoke.txt`).
   Octet's producer closure is far larger (Aeneas + Charon + Lean + Mathlib + Rust +
   Nixpkgs bootstrap). Planning cannot succeed until that bound is raised or the job's
   graph is scoped per module — both are Mantle source/product decisions, and the second
   would contradict WP4's "whole graph" requirement.
2. *Operational:* the Octet flake source at REV never reached `/nix/store` on this host
   (`transcripts/01-export.attempt*.txt`), so no `job.json` / `root-derivation` /
   derivation list exists. The store add of that 6,259-file tree streams in ~1 s and then
   waits on the daemon indefinitely while other builds hold the machine at load 70-110.
   This must be redone on a quiet host; it is not a Mantle problem.

(a) is therefore **not** currently feasible — not because of source supply or signing, but
because of the graph-size bound. (c) preserving a prebuilt closure does not help either:
it still needs an admitted executable plan, hence the same import path and the same bound.

## 9. Resume

`resume-freeze.sh` (0755, same directory) performs the whole sequence unattended on a
quiet host: verify the worktree is at REV, `flake prefetch` through the wrapper, assert
`<source>/nix/mantle-aeneas-job.nix`, root the source at `source-root`, then run the exact
export command. Its log lands in `transcripts/09-resume.txt`. Nothing in it builds or
realizes.

## 10. Freeze completed (orchestrator, 2026-11-19 16:56 EST)

The nix-daemon was restarted at 16:54:43 EST by another party, which cleared the
stale workers and path locks that had blocked the source add. Then:

- `transcripts/11-eval-warmup-retry.txt`: `nix eval ...#packages.x86_64-linux.octet-mantle-aeneas-consumer-job.drvPath`
  -> `/nix/store/vl7bqxyz2f86x0vlvxipcvryavckb9wj-octet-mantle-aeneas-consumer-job-v1.drv` in 44 s (exit 0).
- `transcripts/12-export.txt`: the exact export.py command of section 3 -> exit 0 in 47 s, `producer-export/`
  with job.json, tool-closure.derivation-json.json (38,785,189 bytes), root-derivation, flake-metadata.json,
  flake.lock, source-revision and source-payloads/. Exporter summary: graph_format nix-derivation-show-v4,
  11,565 derivations, 677 source objects, 848,923,562 source bytes, proof_executed false.

Measured against Mantle foreign-import bounds (jq over the v4 graph):

| Dimension | Graph | Mantle bound at cd4ad3f43 |
| --- | --- | --- |
| derivations | 11,565 | MAX_GRAPH_NODES 2,048 |
| source objects | 677 | MAX_SOURCE_PAYLOADS 512 |
| max input derivations per node | 611 (two cargo vendor-registry drvs: 346, 611) | MAX_GRAPH_EDGES 256 |
| max env value bytes | 102,597 (three buildCommand values over 32 KiB) | MAX_FIELD_BYTES 32,768 |
| fixed-output derivations | 5,231 | - |
| structured-attrs derivations | 1,652 | MAX_STRUCTURED_ATTRS_BYTES 1 MiB |
| max outputs / inputSrcs / args / env entries / urls | 10 / 23 / 39 / 70 / 3 | - |

## 9. Preserve-route realization: two operator-guide defects found 2026-11-19

The pilot preserved output is a 526-path, 19,500,072,416-NAR-byte runtime closure
(`nix path-info -r --json` of
`/nix/store/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1`).
Running the guide's "Realize a preserved Nix cache path" text verbatim failed twice.

1. **The snippet omits `--remote-binding-config`.** `StoreHandle::open` calls
   `admit_source` for *every* plan-bound substituter URL, and `admit_source` fails
   with `store-source-admission-required` when no operator-owned admission authority
   is configured (`crates/crunch-store/src/handle.rs:admit_sources`,
   `crates/crunch-store/src/source_admission.rs:78`). Evidence: transcript 33.
   The binding authority must name the exact cache URL; the store-open call uses the
   URL *with* its `?trusted_public_keys[...]` query, while hydration strips the query
   (`cache_only_http_cache_url` calls `url.set_query(None)`), so an operator must bind
   **both** canonical strings (`.../` and `.../?trusted_public_keys[0]=...`).
   Evidence: transcript 38 (`source endpoint admission denied`) and 39.

2. **Cache-only hydration is HTTP(S)-only.** `validate_http_cache_url` rejects any
   other scheme (`crates/crunch-store/src/pull.rs:200`), so a `file://` receipt-bound
   cache URL can never hydrate, although `foreign-import produce-nix --cache-url`
   accepts one and `foreign-import plan` accepts the resulting plan. The pilot plan
   was therefore regenerated with a loopback URL (`transcripts/34`, `35`) and the
   cache served read-only over `python3 -m http.server 18471 --bind 127.0.0.1`.

3. **The shipped `config/foreign-cache-closure/generated/default.json` is
   hello-scale.** Its `max_nar_bytes` is 8 GiB, below this closure's 19.5 GB, so it
   rejects the pilot before importing anything. An operator policy sized for the case
   is the precedent (the archived GuixPkgs proof used a case-specific
   `cache-policy-one-member.json`); the pilot uses
   `realization/pilot-cache-closure-policy.json` (only `max_nar_bytes` changed).
   The Mantle default policy file is untouched.

## 10. Realize 39: two /nix/store paths disagree with their own NAR identity (2026-11-19)

Realize over the loopback cache (transcript `39-realize-cache-only.txt`, exit 3)
imported ~14 G and then refused:

```text
{"error":"hydrating preflighted foreign cache closure: store: http-closure-content-admission-failed: q2vpbpf8vqzizz6836vlw3n83cjr8bma-mold-unwrapped-wrapper-2.40.4","code":3,"kind":"internal"}
```

An audit of all 526 cached NARs (decompress + SHA-256 against each narinfo
`NarHash`, Nix base32 decoded little-endian) found 524 consistent, 2 not:

| path | narinfo NarHash / NarSize | `nix hash path` of the store path | cached NAR bytes |
| --- | --- | --- | --- |
| `q2vpbpf8vqzizz6836vlw3n83cjr8bma-mold-unwrapped-wrapper-2.40.4` | `sha256:0h5b8pgm…` / 41448 | `sha256-SACFMvI7Uo0NzY/w7HzweH1pNK6tVKh1lVoO3SZUfvc=` | 41480 |
| `y8xj70yyzsml4z3aa5gsn9pvpx4za1py-clang-wrapper-21.1.8` | `sha256:0n7jm9bi…` / 73504 | `sha256-WXv71qaPkVdlUJFisrYIpnVNkAQE1fvtyMH9jQi9wfo=` | 73568 |

The cache copy is not the defect: its compressed bytes match the narinfo
`FileHash`/`FileSize`, and `nix store dump-path` of each `/nix/store` path equals the
cached NAR. `nix store verify --store file://P0/cache --trusted-public-keys <pilot>`
also reports `path '…' was modified!`. Both are Nixpkgs wrapper outputs and both are
reachable runtime references of the root, so the closure cannot omit them.

Consequences: no realization receipt (`realization-receipt.json` absent), so WP4
steps 7-10 and the WP5 Part B judgment (deck at absolute paths inside P) cannot
run. Repair requires either fixing the two store paths (`nix store repair`, which
writes `/nix/store`; brittonr is not a store owner, the daemon is) or authorizing a
different source for exactly those two NARs. Re-signing the divergent bytes with
the pilot key was not done: it would assert a content identity Nix never attested.

### 10a. Resolution (operator decision, 2026-11-19/20)

Britton chose the cache-only repair: transcript `40-cache-replace-corrupt-paths.txt`
replaced the two NARs in `P0/cache/nar/` with the original `cache.nixos.org` NARs and
re-signed each narinfo with the pilot key (`added 2 signatures`). The narinfo identity
is unchanged (`StorePath`, `NarHash`, `NarSize`, `References`, `Deriver`), the
decompressed NAR sha256 now equals both the narinfo `NarHash` and the hash the local
Nix DB expects (`0h5b8pgm…` mold, `0n7jm9bi…` clang), and
`nix store verify --store file://P0/cache --trusted-public-keys <pilot pub only>
--recursive <job output>` exited 0 over the whole closure. Superseded NAR files remain
unreferenced under `P0/cache/nar` by instruction.

**Provenance deviation (recorded here and in the realization notes):** those two
members' NAR bytes now come from `cache.nixos.org` instead of `nix copy` from the
local store. Trust is unchanged: the pilot key remains the only trusted key, and the
narinfo identity Mantle verifies is the same one the local store records.

**Root cause of the mismatch:** the host `/nix/store` content is still modified because
Britton's root `nix store repair` did not take effect. The bytes are identical; only
mode bits differ. Three empty `nix-support` files are 0755 where the NAR says 0444:
`q2vpbpf8…-mold-unwrapped-wrapper-2.40.4/nix-support/ld-set-dynamic-linker`, and in
`y8xj70yy…-clang-wrapper-21.1.8/nix-support/` both `libc-ldflags` and
`libcxx-ldflags`. Their ctimes are 2026-04-27, so this is an old host defect and not
part of this run.

## 11. Bridge layout defect: `octet-job` cannot observe the real producer output (2026-11-20)

Transcript `47-bridge-octet-job.txt`, exit 1:

```text
verification bridge: /datapool/mantle-store/convergence-pilot-20261119/store/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/aeneas: No such file or directory (os error 2)
```

`crunch-verification-bridge-core/src/lib.rs:42` hard-codes
`proofs/aeneas/provider-profile.json` (and `crates/crunch-verification-bridge/src/lib.rs:211`
names the same path), which matches only the recorded fixture layout. Octet's
producer at `d5b34c2d` writes
`proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/provider-profile.json`,
and the receipt already carries that locator under
`/provider/payload/stage_chain/provider_profile/locator`. Every other bridge
material path is taken from the receipt's `artifact_identities`, so the stale
constant is the only mismatch. Resolving it needs a Mantle source change (derive
the provider-profile locator from the receipt) or an Octet layout change; per the
orchestrator, P and Mantle source were left untouched, so `realization/handoff`
does not exist and bridge negatives (c)/(d) were not run.

### 11a. Resolution (2026-11-20)

Britton authorized the Mantle change `admit-octet-producer-layout-in-verification-bridge`
in worktree `/home/brittonr/wt/mantle-bridge`, branch of the same name, commit
`b14db98cb`. It derives the provider-profile locator from the receipt and admits the v4
tool graph. The release binary
`/home/brittonr/wt/mantle-bridge/target/release/mantle-verification-bridge` has BLAKE3
`ae41e077e8d9024626b8b6826620069fbf56829b97be469b06930d9533743282` (verified before
use; not rebuilt, Mantle source not edited here).

With that binary, WP4 step 8 succeeds (transcript `54-bridge-octet-job.txt`, exit 0):
`proof_acceptance=externally-reported-by-octet`, `externally_reported_outcome=supported`,
observation written to `realization/handoff/mantle-observation.json`. Bridge negatives
(c) and (d) now run and are refused:
(c) `55-negative-c-tampered-output.txt` exit 1, one byte changed in
`proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/llbc.llbc` of a
writable copy, `llbc.llbc mismatch: expected a4d7eb68…, observed b9f7af2f…`, no handoff;
(d) `56-negative-d-wrong-revision.txt` exit 1,
`producer source revision mismatch: expected 01234567…, observed d5b34c2d…`, no handoff.
The /tmp copies and destinations were deleted afterwards.
