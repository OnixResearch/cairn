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

**Choice:** `project` takes the targets contract and a generated source record keyed by `requirement_id`. Each entry carries `sources` (the `source_bindings` locator and BLAKE3 list) and optionally the four `required_bindings` lists (`source_ids`, `policy_ids`, `tool_identity_hashes`, `artifact_hashes`). Missing lists default to empty; `required_bindings.checked_scope` is always empty and `required_bindings.evidence_tier` is the clause tier. The pilot fills `sources` from an owner-produced file listing the realized output's identities and leaves the four lists empty, because phase 1 does not use `requirements freshness`. The golden fixture supplies a generated record copied from Octet's example clause, labeled as copied fixture data.

**Rationale:** Octet's example clause 0 has non-empty `required_bindings` and hashed `source_bindings`; the handoff sketch reproduces it only by taking those values as inputs. Keeping them in a separate generated record satisfies both the no-authored-hash rule and the byte-equal golden. Octet's `contract/validation.rs` requires non-empty `source_bindings`, `allowed_evidence_ids` and `checked_scope` but reads `required_bindings` only in `freshness.rs` and `related.rs`, so empty lists are admissible to `requirements check`.

### Decision: Fixed projection mapping, verified in Rust

**Choice:** For each projected target, `clause_id` and the single `allowed_source_requirement_ids` entry are `requirement.requirement_id`; `statement` is `claim`; `required` is true; `disposition` and `expected_evidence_tier` are the mapped tier; `checked_scope`, `allowed_evidence_ids`, `required_evidence_classes` and `required_negative_controls` copy `checked_scope`, `evidence_ids`, `evidence_classes` and `negative_controls`; `required_provider_id` is `provider` when present; `source_bindings` and `required_bindings` come from the generated record as above. The contract-level `reviewed_specification` is the contract's `specification`, and `schema_version` is `octet-requirement-contract/v1`. `cairn_core::assurance_targets` implements the same mapping, and convergence recomputes it and compares it with the supplied contract file after JSON parsing.

**Rationale:** Nickel projects; Rust verifies. A contract whose clause was weakened without changing the targets is `stale` even when Octet passes the weakened contract. The handoff sketch evaluates under the pinned Nickel 1.17.0 and reproduces Octet's clause 0 exactly.

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
- The BLAKE3 of the contract file equals the decision's `contract_blake3`; the decision's `reviewed_specification` equals the contract's.
- The BLAKE3 of the evidence bundle equals the decision's `evidence_blake3`, and its `contract_blake3` equals the contract's.
- The target's `evidence_job.job_id` equals `/job_id` in the handoff `job.json`, and the observation's `producer_job_identity` equals `b3:` plus the BLAKE3 of those bytes.
- The observation's `receipt_identity` equals `b3:` plus the BLAKE3 of the handoff `external-receipt.json`, and the producer output's `external-receipt.json` has the same bytes.
- The proof card that Octet accepted names that receipt (design question 3).
- The realization receipt shows the job's root output admitted and that output is the directory the bridge consumed (design questions 1 and 2).
- Every schema identifier is admitted by the policy.

**Rationale:** These are the handoff's binding checks restated against the exact field names found in the schemas below.

### Decision: Acceptance is Octet's decision only

**Choice:** A target is accepted only when the decision's `aggregate_result` is passing and its clause for the target has status `passed`. Cairn never reads the observation's `proof_acceptance` or `externally_reported_outcome` as acceptance; it only requires `proof_acceptance` to equal `not-evaluated-by-mantle`.

**Rationale:** ADR 0034 keeps proof authority in Octet; a Mantle success, a cache hit or a relayed outcome is never acceptance.

### Decision: Design question 1, the admitted-output record

**Choice:** The record is `mantle-foreign-realization-receipt-v1` together with `mantle-store-info-v3` for the root output. The realization receipt must have `status` `complete`, list the root in `selected_root_paths`, and contain a unit whose `target_derivation` equals the handoff `root-derivation` and whose output `target_path` is that root path; its `nar_sha256` and `nar_size` are retained. The store-info record's `paths[]` entry for the same `store_path` must carry the same `nar_sha256` and a `signatures` entry whose key name is in the case's trusted signer set. The `crunch-build-report-v1` is retained as evidence and bound through the receipt's `build_report_blake3`; neither it nor `mantle.build-observation.v1` names a signer, so neither alone shows trusted admission.

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

## Risks / Trade-offs

- The real producer job builds the Aeneas, Charon and Lean closure. If Mantle cannot realize that graph without importing a prebuilt closure, the route is recorded as weaker and work stops for a decision rather than reporting `current`.
- Signer admission is checked by key name in saved records, not by re-verifying signatures; a forged store-info record is outside the claim and stated as a non-claim.
- The registry row inside each target is hand-copied; a stale copy is caught as `stale` but not regenerated automatically.
- Phase 1 proves the pipeline for Octet's external-consumer fixture theorem only, not that any production code is proved.
