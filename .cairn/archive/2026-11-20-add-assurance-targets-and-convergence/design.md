## Context

Cairn specifications are Markdown `spec.md` files. `cairn requirement-registry export` binds each requirement to a `cairn.content-bound-requirement-registry.v1` row carrying `requirement_id`, `specification_path`, `specification_blake3`, `source_span`, `requirement_body_blake3` and `row_blake3`, and the registry carries `revision {kind, value}`. No row declares an assurance target.

Mantle realizes a foreign Nix derivation graph with `mantle foreign-import plan`, `prepare-sources` and `realize`, signs output PathInfo with Ed25519 keys, rechecks signatures before admitting reuse, and writes `mantle-foreign-realization-receipt-v1`. Its verification bridge (`mantle-verification-bridge octet-job`) consumes a realized Octet producer output plus the frozen export from Octet's `tools/mantle-producer/export.py` and writes a handoff directory containing `mantle-observation.json` (schema `mantle-external-evidence-observation-v1`), `external-receipt.json`, `external-profile.json`, `job.json`, `tool-closure.derivation-json.json`, `root-derivation`, `flake-metadata.json`, `flake.lock`, `source-revision` and the checked materials. ADR 0034 keeps proof authority in Octet.

Octet's `cargo octet requirements check` evaluates an `octet-requirement-contract/v1` over an `octet-requirement-evidence/v1` bundle and a `--source-root`, and writes an `octet-requirement-decision/v1` with `contract_blake3`, `evidence_blake3`, `source_snapshot_blake3`, `reviewed_specification`, `aggregate_result` and per-clause `status`. `requirements verify` recomputes it. Octet's producer job `octet.aeneas.external-consumer.mantle.v1` (`octet-mantle-producer-job/v1`) runs `cargo-octet prove` and `artifact verify` and copies the rail receipt to `external-receipt.json`.

This change adds the Declare stage (targets) and the Diff stage (convergence) for one committed revision, read-only and advisory.

## Decisions

### Decision: Target vocabulary reuses Cairn's ladder (D1)

**Choice:** A target's `assurance` is `reviewed`, `bounded_check` or `checked_proof`, the serialized values of `Assurance` in `refinement_typed_dependencies.rs`. `unassessed` is rejected as a target. `checked_proof` maps to Octet's `formal_proof` disposition and evidence tier; `bounded_check` maps to `empirical_check`. `reviewed` produces no clause and is reported `not_projected`. Ordinary tests count as `bounded_check`.

**Rationale:** Britton confirmed the recommended default on 2026-11-19. Reusing the existing ladder avoids a second assurance vocabulary, and Octet has no review disposition.

### Decision: Targets file shape and no authored hashes

**Choice:** `contracts/assurance-targets.ncl` defines a targets file of contracts, one per specification file. A contract carries `contract_id`, `specification {path, blake3}`, `targets`, and Octet `assumptions`, `exclusions` and `non_claims` passed through unchanged. A target carries `requirement` (the six registry-row fields copied from `cairn requirement-registry export`), `assurance`, `claim`, `informal_remainder`, `evidence_job {job_id, definition}`, `checked_scope`, `evidence_ids`, `evidence_classes`, optional `provider` and `negative_controls`. The contract closes every record, so a target cannot carry `sources`, `source_ids`, `policy_ids`, `tool_identity_hashes`, `artifact_hashes` or any other unlisted field. The registry row is the one hand-maintained lock; generating it from the export is a later change.

**Rationale:** Source and tool identity come from the frozen graph and its lock, so nothing is typed by hand and nothing drifts silently. `spec.md` stays authoritative and keeps its identity, the way `cairn-policy/typed-requirement-contracts.ncl` binds exact rows.

### Decision: Generated source record is a separate projection input

**Choice:** `project` takes the targets contract and a generated source record keyed by `requirement_id`. Each entry carries `sources` (the `source_bindings` locator and BLAKE3 list) and optionally the four `required_bindings` lists (`source_ids`, `policy_ids`, `tool_identity_hashes`, `artifact_hashes`). Missing lists default to empty; `required_bindings.checked_scope` is always empty and `required_bindings.evidence_tier` is the clause tier. The pilot fills `sources` from an owner-produced file listing the realized output's identities and leaves the four lists empty, because phase 1 does not use `requirements freshness`. The golden fixture supplies a generated record copied from Octet's example clause, labeled as copied fixture data. The record also carries `specifications`: for each reviewed specification file, its path and the plain BLAKE3 of that file's bytes at the revision, produced by a tool (for example `b3sum`). `project` copies the single entry naming the contract's specification path into the contract's `reviewed_specification` and fails when it is absent or duplicated.

**Rationale:** Octet's example clause 0 has non-empty `required_bindings` and hashed `source_bindings`; the handoff sketch reproduces it only by taking those values as inputs. Keeping them in a separate generated record satisfies both the no-authored-hash rule and the byte-equal golden. Octet's `contract/validation.rs` requires non-empty `source_bindings`, `allowed_evidence_ids` and `checked_scope` but reads `required_bindings` only in `freshness.rs` and `related.rs`, so empty lists are admissible to `requirements check`.

### Decision: Fixed projection mapping, verified in Rust

**Choice:** For each projected target, `clause_id` and the single `allowed_source_requirement_ids` entry are `requirement.requirement_id`; `statement` is `claim`; `required` is true; `disposition` and `expected_evidence_tier` are the mapped tier; `checked_scope`, `allowed_evidence_ids`, `required_evidence_classes` and `required_negative_controls` copy `checked_scope`, `evidence_ids`, `evidence_classes` and `negative_controls`; `required_provider_id` is `provider` when present; `source_bindings` and `required_bindings` come from the generated record as above. The contract-level `reviewed_specification` is the generated `specifications` entry for the contract's specification path (its path and plain file digest), never the registry's domain-separated identity, and `schema_version` is `octet-requirement-contract/v1`. `cairn_core::assurance_targets` implements the same mapping, and convergence recomputes it and compares it with the supplied contract file after JSON parsing.

**Rationale:** Nickel projects; Rust verifies. A contract whose clause was weakened without changing the targets is `stale` even when Octet passes the weakened contract. The handoff sketch evaluates under the pinned Nickel 1.17.0 and reproduces Octet's clause 0 exactly.

### Decision: Reviewed specification identity

**Choice:** The registry identifies a specification by a domain-separated hash, `specification_blake3 = domain_hash_bytes("cairn.requirement-registry.specification.v1", markdown)`. Octet's `requirements check` instead verifies `reviewed_specification.blake3` against the plain `blake3::hash` of `resolve_path(source_root, reviewed_specification.path)`. The projection therefore never copies the registry hash: `contract.specification` keeps the registry identity and binds the targets' rows, while the generated record's `specifications` entry supplies the plain file digest that becomes `reviewed_specification`. Octet runs with the consumer checkout at the revision as `--source-root`, where the specification file lives, and names the proof deck and artifact root by absolute paths inside the realized producer output; convergence accepts an absolute deck locator only when it lexically starts with the case's resolved `producer_output` directory.

**Rationale:** The Octet pilot registry identity is `7529c7f5…`, while the plain `b3sum` of the same file is `7b6bbfda…`; Octet's own example uses the plain digest `78afa0f6…`. Copying the domain hash would make every contract fail Octet's specification check, and editing the specification to match would break the registry. Keeping the two identities separate binds the registry rows and Octet's reviewed bytes without weakening either.

### Decision: Projection home and vendoring (D3)

**Choice:** The projection library lives in Cairn `contracts/assurance-targets.ncl`. Consumers vendor it at a recorded Cairn revision through their own repository-owned vendoring tool. Octet does this through `vendor/dependencies.ncl` and `tools/import-public-dependencies.py`, with a separate source entry for the exact Cairn revision and a projection to `cairn-policy/assurance-targets.ncl`, rather than a hand copy. Octet stays unaware of Cairn semantics.

**Rationale:** Britton confirmed the default. Octet already vendors Cairn files through that tool with archive digests and drift checks, and workspace rules require repository-owned sync tools for vendored siblings.

### Decision: Policy export

**Choice:** `cairn-policy/assurance-targets.ncl` is a standalone root exported to `cairn-policy/generated/assurance-targets.json` and wired into `policy export --check` like `scoped-claim-derivations`. It holds the admitted assurance values and their mapping; the admitted schema identifiers (`cairn.content-bound-requirement-registry.v1`, `octet-requirement-contract/v1`, `octet-requirement-evidence/v1`, `octet-requirement-decision/v1`, `octet-proof-cards/v1`, `octet-formal-proof-envelope/v1`, `octet-mantle-producer-job/v1`, `mantle-external-evidence-observation-v1`, `mantle-foreign-realization-receipt-v1`, `mantle-store-info-v3`, `crunch-build-report-v1`); the admitted Octet clause statuses and aggregate results; the admitted realization states; and bounds on contracts, targets per contract, rows, text bytes per field and input bytes per file. The CLI embeds the checked export, and `--policy FILE` substitutes another checked export.

**Rationale:** Nickel owns every bound and vocabulary; runtime commands read generated JSON only and never start the evaluator. A separate policy keeps the default policy and existing receipt hashes unchanged.

### Decision: Convergence case and receipt (D4)

**Choice:** `cairn convergence inspect --case FILE [--policy FILE] [--root DIR]` is CLI only; an MCP tool is a later change. The case file (`cairn.convergence-case.v1`, typed and closed) names the registry, the exported targets, the generated source record, the trusted signer key names, and per contract: the contract, the Octet decision and evidence bundle, the bridge handoff directory, the producer output directory, the Mantle realization receipt and the Mantle store-info record for the root output. The receipt (`cairn.convergence.v1`) lists every registry row in registry order with its state and the identities it bound, then `targets_current`, `coverage_complete`, `complete`, the input identities, the non-claims and a BLAKE3 receipt identity over the canonical receipt without that field.

**Rationale:** Britton confirmed the CLI-only default. The shape follows `cairn footprints inspect`: bounded capability-relative reads, a pure evaluator and one printed advisory receipt.

### Decision: States and completeness

**Choice:** Each row is `untargeted` (no target), `not_projected` (target is `reviewed`), `current`, `unrealized` (no admitted Mantle realization for the job at this revision), `stale` (any binding mismatch), `failed` (Octet producer verification, the bridge observation or the decision clause reports failure), `unsupported` (Octet reports `unsupported`, or a value outside the admitted vocabulary) or `incomplete` (an input is missing or a Cairn bound was reached). `targets_current` holds when every targeted row is `current` or `not_projected`; `coverage_complete` holds when no row is `untargeted`; `complete` requires both. Precedence for one target is `incomplete`, then `unsupported`, then `unrealized`, then `stale`, then `failed`, then `current`.

**Rationale:** A closed vocabulary with a fixed precedence fails closed and keeps a repository from looking converged by declaring two targets.

### Decision: Binding checks

**Choice:** Every check fails closed and has a stable issue code:

- The registry verifies against `--root`, its `revision.kind` is `git`, and its `revision.value` equals `packaging.source_revision` in `mantle-observation.json` and the handoff `source-revision` bytes.
- Each target's `requirement` equals its registry row in all six fields.
- Each contract's `specification` equals the path and BLAKE3 on its targets' rows.
- The recomputed clause mapping equals the contract file's clauses after JSON parsing.
- The contract's `reviewed_specification` names an accepted specification under `--root` and its `blake3` equals the plain BLAKE3 of that file's bytes; a missing or changed digest is `stale` (`convergence.reviewed_specification_stale`).
- The BLAKE3 of the contract file equals the decision's `contract_blake3`; the decision's `reviewed_specification` equals the contract's.
- The BLAKE3 of the evidence bundle equals the decision's `evidence_blake3`, and its `contract_blake3` equals the contract's.
- The target's `evidence_job.job_id` equals `/job_id` in the handoff `job.json`, and the observation's `producer_job_identity` equals `b3:` plus the BLAKE3 of those bytes.
- The observation's `receipt_identity` equals `b3:` plus the BLAKE3 of the handoff `external-receipt.json`, and the producer output's `external-receipt.json` has the same bytes.
- The proof card that Octet accepted names that receipt (design question 3). A proof-deck locator is relative to `producer_output` or absolute inside it, starting with the case's resolved directory path followed by `/`; any other locator is rejected before evaluation (`convergence.proof_deck_outside_output`).
- The realization receipt shows the job's root output admitted and that output is the directory the bridge consumed (design questions 1 and 2).
- Every schema identifier is admitted by the policy.

**Rationale:** These are the handoff's binding checks restated against the exact field names found in the schemas below.

### Decision: Acceptance is Octet's decision only

**Choice:** A target is accepted only when the decision's `aggregate_result` is passing and its clause for the target has status `passed`. Cairn never reads the observation's `proof_acceptance` or `externally_reported_outcome` as acceptance; it only requires `proof_acceptance` to be one of the closed set `["not-evaluated-by-mantle", "externally-reported-by-octet"]`. The first is what the bridge's fixture `octet` mode emits; the second is what its verified mode emits, and `octet-job` requires verified mode. Any other value, including an acceptance-shaped one such as `accepted-by-mantle`, is `unsupported`.

**Rationale:** ADR 0034 keeps proof authority in Octet; a Mantle success, a cache hit or a relayed outcome is never acceptance.

### Decision: Design question 1, the admitted-output record

**Choice:** The record is `mantle-foreign-realization-receipt-v1` together with `mantle-store-info-v3` for the root output. The realization receipt must have `status` `complete`, list the root in `selected_root_paths`, and contain a unit whose `node_id` equals `nix:` plus the final path component of the handoff `root-derivation` and whose output `target_path` is that root path; the unit's `target_derivation` is Mantle's translated derivation and is retained rather than compared, because it differs from the frozen root even on the preserve route. The receipt's `cache_policy.realization_route` (absent or null for the build route, `cache-only-preserve-v1` when Mantle preserved a prebuilt output without building or executing it) is carried into the receipt's binding. Its `nar_sha256` and `nar_size` are retained. The store-info record's `paths[]` entry for the same `store_path` must carry the same `nar_sha256` and a `signatures` entry whose key name is in the case's trusted signer set. The `crunch-build-report-v1` is retained as evidence and bound through the receipt's `build_report_blake3`; neither it nor `mantle.build-observation.v1` names a signer, so neither alone shows trusted admission.

**Rationale:** These are the only saved Mantle records that name both the realized root path and its signatures. Cairn checks the signer name and the NAR facts it is given; it does not verify an Ed25519 signature itself. Mantle verified signatures when it admitted the output, and the receipt states that non-claim.

### Decision: Design question 2, tying the record to the consumed directory

**Choice:** The case names the producer output directory that `mantle-verification-bridge octet-job` consumed. Its final path component must equal the final component of the realization receipt's root `target_path`, and the bytes of `external-receipt.json` and `external-profile.json` read from it must equal the handoff copies.

**Rationale:** The bridge records exact-byte copies and identities but not the directory it read. Name equality with the signed root path, plus exact-byte equality of the receipt and profile, binds the observation to that output without Cairn reading the store or recomputing a NAR hash.

### Decision: Design question 3, the receipt in Octet's evidence

**Choice:** Octet's evidence bundle has no receipt field. The tie is the proof deck it names: the bundle's `proof_decks[]` entry whose `evidence_id` the clause allows names a deck (resolved under the producer output) whose card for the target has a `receipt_path` and an `artifact_blake3_identities` entry for that same locator. That entry's `blake3` must equal the observation's `receipt_identity` without its `b3:` prefix. Octet's `validate_artifacts` recomputes each listed artifact identity against `--source-root`, so a passing decision means the receipt bytes it judged are the bytes Mantle observed.

**Rationale:** Octet's evidence, contract and decision schemas must not change in phase 1. `cargo-octet/src/requirements/evaluation/formal.rs` and Octet's `octet-proof-cards/v1` cards already carry `receipt_path` and the receipt's BLAKE3.

### Decision: Design question 4, `producer_job_identity`

**Choice:** `crunch-verification-bridge-core` computes `producer_job_identity` as `digest(job)`, where `digest` is `b3:` followed by the lowercase BLAKE3 hex of the exact `job.json` bytes read from the producer export, and the bridge writes the same bytes to the handoff `job.json`. Cairn recomputes it from the handoff file. `receipt_identity` uses the same `digest` over the receipt bytes.

**Rationale:** `crates/crunch-verification-bridge-core/src/lib.rs` (`fn digest`) and `src/producer.rs` (`job_identity = digest(&binding.job)`) define it; the bridge's `export` writes `binding.job` unchanged.

### Decision: Design question 5, parsing strictness

**Choice:** Cairn-owned inputs (the case, the exported targets and the policy) are typed DTOs with `deny_unknown_fields`. Foreign Mantle and Octet records follow the Kani receipt pattern in `kani_receipt.rs`: a bounded read, parsing to `serde_json::Value`, an exact admitted schema identifier, explicit type checks on every field Cairn uses, and stable issue codes for missing or mistyped fields. Unknown extra fields in foreign records are ignored, because their owners may extend them, but an unknown schema identifier is rejected. Every read is bounded, refuses symbolic links and stays inside its named root.

**Rationale:** Cairn validates saved metadata and never re-runs producers. Owners keep schema authority; Cairn keeps admission strict where it reads.

### Decision: Pilot execution (D2, D5, D6, D7)

**Choice:** The pilot targets `octet.aeneas_consumer_contract.compatibility` in Octet with `checked_proof` and job `octet.aeneas.external-consumer.mantle.v1`. The real job runs on britton-desktop (32 cores, user namespaces enabled). A pilot-only Mantle signing key is generated there; its public key name is the case's only trusted signer, and no shared cache is configured. The Terrane pilot is deferred until this change lands. The agent fast-forwards and pushes `main` after gates pass, with author `Britton <b@robitzs.ch>`.

**Rationale:** Britton confirmed each default on 2026-11-19. One named key keeps the trusted set explicit; nothing widens it to make a result current.

### Decision: Mantle foreign-import graph bound

**Choice:** The real pilot waits on Mantle change `raise-foreign-import-graph-bounds` (Mantle ADR 0166), which raises Mantle's foreign-import node bound from 2,048 to the verification bridge's `MAX_TOOL_DERIVATIONS` (32,768), its source-payload bound from 512 to Octet's exporter bound (16,384), its per-node input-edge bound from 256 to 4,096 and its field bound from 32 KiB to 256 KiB. Cairn's convergence code is unchanged by it.

**Rationale:** `mantle foreign-import produce-nix` rejected a measured 7,157-derivation nixpkgs closure with `nix-derivation-count-out-of-range` (`MAX_GRAPH_NODES = 2048` in Mantle's `src/foreign_derivation_import.rs`), while the bridge already admits 32,768-derivation producer tool graphs, so no real producer graph could be planned. The frozen Octet job at `d5b34c2d64b1227417a46df8ed8427f2d10ed3c8` has 11,565 derivations, 677 source objects, up to 611 input derivations on one node and one 102,597-byte `buildCommand`. This met the handoff's stop-and-ask condition; Britton authorized the Mantle change on 2026-11-19.

### Decision: Mantle bridge producer layout

**Choice:** The real observation depends on Mantle change `admit-octet-producer-layout-in-verification-bridge` (Mantle ADR 0167). In it, the bridge derives the provider-profile locator from the receipt's `artifact_identities` instead of the fixture path `proofs/aeneas/provider-profile.json`, and admits the versioned (`version: 4`) `nix derivation show` graph alongside the legacy map. Mixed or unknown shapes fail closed. Cairn's convergence code is unchanged by it.

**Rationale:** The bridge refused the realized output of Octet `d5b34c2d`, whose provider profile sits under the rail-named directory `proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/`, and the job's Nix 2.36 graph uses the versioned form. Observing the real job without changing Mantle was therefore impossible, which met the handoff's stop-and-ask condition. Britton authorized the Mantle change on 2026-11-20. Mantle gains no proof authority: the bridge only locates bytes and parses graphs.

### Decision: Pilot cache repair and transport

**Choice:** Two pilot-cache NARs (`mold-unwrapped-wrapper-2.40.4`, `clang-wrapper-21.1.8`) are cache.nixos.org's original NARs, re-signed with the pilot key, and not `nix copy` output from the host store. The pilot cache is served to Mantle over a loopback-only HTTP endpoint, `127.0.0.1:18471`, because cache-only hydration admits HTTP closures only. The pilot key is still the only trusted key.

**Rationale:** On the host, empty `nix-support` files in those two paths had mode 0755 where their NARs record 0444; the ctimes date from 2026-04-27. As a result, `nix copy` wrote content that did not match the recorded `NarHash`, and Mantle's content admission rejected it, failing closed as it should. The original NARs' SHA-256 equals the recorded `NarHash`, so the narinfo identity the pilot key signs is unchanged. Britton chose this repair on 2026-11-19 and approved the loopback transport.

### Decision: Producer record bounds in the inspect shell

**Choice:** `cairn convergence inspect` reads the handoff tool graph within `max_tool_graph_bytes` (268,435,456 bytes) and the Mantle realization receipt within `max_realization_receipt_bytes` (16,777,216 bytes), both named in `cairn-policy/assurance-targets.ncl`, exported to `cairn-policy/generated/assurance-targets.json` and carried in the bounds DTO. `max_document_bytes` stays at 1,048,576 bytes for every other Mantle and Octet document. An over-bound read keeps its existing `convergence.unreadable_input` rejection.

**Rationale:** The measured pilot records exceed 1 MiB: the frozen Nix tool graph (`tool-closure.derivation-json.json`, 11,565 derivations) is 38,785,189 bytes, and `realization-receipt.json` (`mantle-foreign-realization-receipt-v1`) is 6,469,512 bytes for 11,565 units, so with fixture-sized bounds the pilot could never be `current`. Each new bound equals the bound its producer applies, rather than a size guessed from the corpus: `max_tool_graph_bytes` is Mantle's `crunch_verification_bridge_core::producer::MAX_TOOL_GRAPH_BYTES`, the largest graph the verification bridge binds and digests into `tool_graph_identity`, and `max_realization_receipt_bytes` is `MAX_FOREIGN_REALIZATION_RECEIPT_BYTES` in Mantle's `src/store_cmd.rs`, the bound Mantle's own store commands apply when they read a realization receipt. Both stay far below `max_input_bytes` for aggregate inputs, and every read remains bounded and link-refusing.

### Decision: Realization route for the pilot

**Choice:** The pilot uses Mantle's preserve route (`preserve-cache-paths-v1` planning and `cache-only-preserve-v1` realization). Nix builds Octet's producer job on top of prebuilt toolchains and runs its proof in Nix's sandbox; the output closure is signed with the pilot-only key into a local file cache; Mantle plans the frozen graph with paths preserved, hydrates and verifies the root output under that key only, and exports it for the bridge. Every receipt and document labels the route as a preserved prebuilt closure whose proof Nix executed.

**Rationale:** The frozen graph needs 6,334 non-fixed-output builds, including six nixpkgs bootstrap chains, so a from-source realization through Mantle is out of proportion to the pilot. Britton chose the preserve route on 2026-11-19, which answers the handoff's stop-and-ask for importing a prebuilt closure. The route is weaker: it does not show that Mantle built the tool graph or executed the proof, and the convergence receipt's non-claims already say so unless the recorded route shows otherwise.

## Risks / Trade-offs

- The pilot's proof runs under Nix, and Mantle only preserves and verifies the signed output; a `current` pilot row therefore does not show that Mantle built the tool graph or executed the proof, and the receipt's recorded route says so.
- Signer admission is checked by key name in saved records, not by re-verifying signatures; a forged store-info record is outside the claim and stated as a non-claim.
- The registry row inside each target is hand-copied; a stale copy is caught as `stale` but not regenerated automatically.
- Phase 1 proves the pipeline for Octet's external-consumer fixture theorem only, not that any production code is proved.
