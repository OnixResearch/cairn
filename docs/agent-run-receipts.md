# Agent-run receipts

Agent wrappers can use Cairn agent-run receipts to make completion claims deterministic and gateable instead of prose-only. r[verify cairn.supersede.agent_run_receipts.schema]

## Canonical envelope

A receipt is canonical JSON with these required fields:

- `schema_version`: currently `cairn.agent_run_receipt.v1`
- `task_id`: stable task/run identifier chosen by the wrapper
- `agent_identity`: bounded model/provider/wrapper metadata
- `input_hash`: BLAKE3 hash of the captured prompt/input manifest
- `plan_hash`: BLAKE3 hash of the pre-mutation plan, or the explicit no-mutation plan constant for read-only runs
- `tool_manifest`: bounded list of tools available or used by the wrapper
- `mutation_manifest`: files, lifecycle actions, network writes, secret accesses, deploys, or other side effects
- `verification_manifest`: checks and artifact evidence proving the verdict
- `nondeterminism_manifest`: captured and forbidden entropy sources
- `replay_evidence`: deterministic replay preflight evidence for Cairn tool surfaces
- `verdict`: wrapper verdict such as `success`
- `receipt_hash`: BLAKE3 hash of the canonical receipt with `receipt_hash` excluded

Receipts should store bounded summaries, manifests, and hashes rather than raw transcripts or full tool logs. Secret-bearing values should be redacted and represented by metadata or content hashes. r[verify cairn.supersede.agent_run_receipts.schema.bounded_content]

## Plan before mutation

Before a wrapper performs side effects, it should write or construct a canonical plan that states intended files, lifecycle mutations, external writes, deploys, secret access, verification commands, and rollback boundary. The final receipt must cite the plan with `plan_hash`, and the mutation manifest must stay inside that approved scope. r[verify cairn.supersede.agent_run_receipts.plan_link]

Read-only runs may cite the explicit no-mutation plan constant, but any non-empty side-effect manifest with that constant fails the completion gate.

## Nondeterminism manifest

Wrappers should classify entropy sources as one of:

- `clock`
- `network`
- `model_provider`
- `environment`
- `random`
- `remote_api`
- `external_artifact`

Each captured entry needs bounded metadata or a content hash. Forbidden or uncaptured entropy belongs in `forbidden_uncaptured` and causes the completion gate to fail deterministically. r[verify cairn.supersede.agent_run_receipts.nondeterminism]

## Replay evidence

Agent receipts must bind completion claims to replay-checked Cairn surfaces. The
`replay_evidence` object contains:

- `replay_receipt_hash`: BLAKE3 hash from a passing `cairn determinism replay` receipt
- `replay_policy_hash`: BLAKE3 hash of the generated policy used by replay
- `replay_selected_cases`: replay case ids covered by the evidence
- `replay_selected_groups`: replay group ids covered by the evidence; wrappers should include `agent`
- `replay_verdict`: must be `pass`
- `replay_environment_attestation_hash`: optional BLAKE3 hash from the replay receipt's `environment_attestation.attestation_hash`
- `replay_artifact_path`: optional display-only path for local operators

## First-class evidence export

Agents and CI that only need the canonical local evidence directory should prefer
`cairn evidence export` over hand-assembling every artifact:

```sh
nix run .#cairn -- evidence export --root . --out target/cairn-evidence/
```

The export command materializes `coverage-audit.json`, `replay-agent.json`,
`replay-verification.json`, `release-readiness.json`, `agent-receipt.json`,
`agent-evidence-bundle.json`, `agent-evidence-bundle-verification.json`,
`evidence-manifest.json`, `evidence-manifest-verification.json`, and
`evidence-export-receipt.json`. The export receipt uses relative artifact paths,
BLAKE3 file hashes, the manifest hash, the manifest-verification receipt hash,
and deterministic issue codes for failed nested evidence. Consumers can verify a
received export without reconstructing the chain by hand:

```sh
nix run .#cairn -- evidence verify --root . --bundle target/cairn-evidence/
```

The verify command locates `evidence-export-receipt.json`, rehashes declared
relative artifact paths, rejects unsafe or missing paths, checks canonical roles,
re-runs the replay, bundle, and manifest receipt validators, and emits a
path-safe receipt with `export_receipt_hash`, `manifest_hash`,
`manifest_verification_receipt_hash`, per-artifact hashes, deterministic issue
codes, and its own `receipt_hash`. Consumers that need a safe local evidence copy
can import only declared artifacts after verification:

```sh
nix run .#cairn -- evidence import --root . --bundle target/cairn-evidence/ --into target/cairn-imported-evidence/
```

The import command fails before trusting the bundle if verification fails, rejects
undeclared bundle files and unsafe paths, preserves declared relative artifact
paths, writes `evidence-import-receipt.json`, and emits stable `bundle =
"<bundle>"`, `into = "<into>"`, `verification_receipt_hash`, artifact hashes,
deterministic issue codes, and its own `receipt_hash`. Imported evidence can be
republished for downstream caches without regenerating local replay evidence:

```sh
nix run .#cairn -- evidence republish --root . --imported target/cairn-imported-evidence/ --out target/cairn-republished-evidence/
```

The republish command verifies `evidence-import-receipt.json`, rehashes declared
imported artifacts, copies only those artifacts into the output directory, writes
`evidence-republish-receipt.json`, preserves roles and relative paths, and emits
stable `imported = "<imported>"`, `out = "<out>"`, `import_receipt_hash`,
artifact hashes, deterministic issue codes, and its own `receipt_hash`. Publish a
small deterministic catalog before downstream artifact transfer with:

```sh
nix run .#cairn -- evidence index --root . --bundle target/cairn-republished-evidence/ --out target/cairn-evidence-index.json
nix run .#cairn -- evidence index verify --root . --index target/cairn-evidence-index.json
```

The index command detects exported, imported, or republished bundle receipts,
rehashes declared artifacts, records bundle kind, receipt-chain hashes, policy
hash when available, and artifact roles/relative paths/hashes, then emits stable
`bundle = "<bundle>"`, `out = "<out>"`, deterministic issue codes, and its own
`receipt_hash`. Index verification recomputes the catalog from the referenced
bundle without leaking raw host paths. Aggregate one or more index files into a
release/channel registry with:

```sh
nix run .#cairn -- evidence registry build --root . \
  --index target/cairn-evidence-index.json \
  --out target/cairn-evidence-registry.json \
  --channel stable \
  --release 2026-05-19 \
  --platform x86_64-linux \
  --compatibility-class current
nix run .#cairn -- evidence registry verify --root . --registry target/cairn-evidence-registry.json
nix run .#cairn -- evidence registry mirror-verify --root . \
  --registry target/cairn-evidence-registry.json \
  --mirror target
```

Registry build verifies each index, sorts entries by stable content, records
policy-validated stable labels (`channel`, `release`, `platform`, and
`compatibility_class`), index receipt hashes, bundle kinds, bundle receipt
hashes, receipt chains, policy hashes, artifact roles/paths/hashes, and writes a
portable registry receipt. Registry verification recomputes referenced indexes
and rejects missing, duplicate, stale, tampered, unsafe, label-invalid, or
label-conflicting entries with deterministic issue codes. Mirror verification
checks an offline/mirrored directory using the registry's safe `index_relative`
refs, recomputes the mirrored indexes, preserves registry labels and receipt
chains, and emits a portable receipt with `registry`, `mirror`, `registry_hash`,
`registry_verification_receipt_hash`, `entry_count`, `entries`, `issues`, and
`receipt_hash` without copying files or performing network I/O.
Consumers can then choose one entry or produce a metadata-only download plan:

```sh
nix run .#cairn -- evidence registry select --root . \
  --registry target/cairn-evidence-registry.json \
  --channel stable \
  --release 2026-05-19 \
  --platform x86_64-linux \
  --bundle-kind republished \
  --out target/selected-cairn-evidence-index.json
nix run .#cairn -- evidence registry fetch-plan --root . \
  --registry target/cairn-evidence-registry.json \
  --channel stable \
  --platform x86_64-linux \
  --out target/cairn-evidence-fetch-plan.json
nix run .#cairn -- evidence registry fetch-execute --root . \
  --plan target/cairn-evidence-fetch-plan.json \
  --source target/cairn-republished-evidence/ \
  --out target/cairn-fetch-cache/
```

Selection verifies the registry first and fails on zero or multiple matches
instead of guessing. Selectors are stable metadata such as bundle kind, policy
hash, index receipt hash, bundle receipt hash, artifact role/hash,
coverage/release receipt hashes, and stable labels (`channel`, `release`,
`platform`, `compatibility_class`). Fetch plans also verify the registry first and
emit sorted safe `index_relative` refs plus artifact roles/paths/hashes and
receipt-chain hashes without doing network I/O or exposing raw paths, env values,
credentials, or secrets. Fetch execution consumes a verified fetch-plan receipt,
materializes artifacts from a local/offline source directory into an output cache,
rejects unsafe relative paths and unverified plans, checks each artifact's BLAKE3
hash before and after copying, and emits deterministic success/failure receipts
with linked fetch-plan and registry receipt hashes. It does not perform network
transport, credential handling, or remote trust validation.
r[cairn.supersede.fetch_execute_policy_docs.docs.flow]
r[cairn.supersede.fetch_execute_policy_docs.docs.non_claims]
r[verify cairn.supersede.evidence_registry_consume.select]
r[verify cairn.supersede.evidence_registry_consume.fetch_plan]
r[verify cairn.supersede.evidence_registry_consume.policy]
r[verify cairn.supersede.evidence_registry_consume.docs]
r[verify cairn.supersede.evidence_registry_consume.verify]
r[verify cairn.supersede.evidence_registry_rail.build]
r[verify cairn.supersede.evidence_registry_rail.verify]
r[verify cairn.supersede.registry_mirror_verification.command]
r[verify cairn.supersede.registry_mirror_verification.receipt]
r[verify cairn.supersede.registry_mirror_verification.failures]
r[verify cairn.supersede.registry_mirror_verification.docs]
r[verify cairn.supersede.evidence_registry_rail.failures]
r[verify cairn.supersede.evidence_registry_rail.gates]
r[verify cairn.supersede.evidence_registry_rail.archive]
r[verify cairn.supersede.evidence_export_bundle.docs]
r[verify cairn.supersede.evidence_bundle_verify_command.docs]
r[verify cairn.supersede.evidence_export_bundle.archive]

## Preserves receipt inspection

Cairn also provides an internal Preserves hash lens for receipt-shaped material. Operators should keep using JSON `receipt_hash` for public/stored receipt compatibility, and use Preserves only for read-only inspection and canonicalization drift checks. See [`preserves-receipts.md`](preserves-receipts.md) for the `agent-receipt preserves-hash` command, corpus update rail, and CI guard workflow.

r[impl cairn.determinism.preserves_operator_docs.discoverability]

## Manual replay and publication flow

`agent-receipt validate` compares `replay_policy_hash` with the current generated
policy hash and derives required cases from the policy-owned `agent` replay
group. Operators should collect the evidence with `cairn determinism replay --group agent`; validation checks both `replay_selected_groups` and the expanded
`replay_selected_cases` from that group. Missing, stale, failed, or insufficient
replay evidence produces deterministic issue codes such as
`replay_evidence.missing`, `replay_evidence.policy_hash_stale`,
`replay_evidence.missing_group_coverage`, and
`replay_evidence.insufficient_coverage`. Wrappers should save the replay artifact
and verify it before copying hashes into agent evidence:

```sh
nix run .#cairn -- determinism coverage-audit --root . > target/coverage-audit.json
nix run .#cairn -- determinism replay --root . --group agent --format summary --out target/replay.json
nix run .#cairn -- determinism verify-receipt target/replay.json --root .
```

`determinism coverage-audit` checks the policy-owned `determinism_surface_inventory`
against replay cases and groups before operators trust replay evidence. Each
surface names the public command, receipt command, required replay cases, required
replay groups, and determinism claim. The audit emits `covered_surfaces`,
`missing_surfaces`, `surface_inventory_hash`, `current_replay_policy_hash`, and
stable issue records with `surface_id`, `field_path`, `expected`, and `actual`;
missing cases or group coverage make the command fail deterministically.
r[verify cairn.supersede.determinism_surface_coverage_audit.docs]

Replay receipt verification recomputes `replay_receipt_hash`, compares the
current generated policy hash, validates `environment_attestation.attestation_hash`,
and checks selected cases/groups against policy. It emits deterministic issue
codes for malformed, stale, or tampered evidence without storing raw environment
values. Wrappers that persist a full evidence chain should then verify a bundle:

```sh
nix run .#cairn -- agent-receipt verify-bundle target/bundle.json --root .
```

The bundle JSON names `agent_receipt_path`, `replay_receipt_path`,
`replay_verification_path`, optional `release_readiness_path`, and optional
`coverage_audit_path`; it may also
pin expected hashes with `agent_receipt_hash`, `replay_receipt_hash`,
`replay_verification_receipt_hash`, `release_readiness_receipt_hash`, and
`coverage_audit_receipt_hash`. The
bundle verifier composes agent receipt validation and saved replay verification,
checks the policy-owned `agent` replay group, links the environment attestation
hash through agent replay evidence, and emits a single deterministic pass/fail
receipt.

The outer publication artifact should be an evidence manifest. Generate it from
saved artifacts and then verify the generated manifest before publication:

```sh
nix run .#cairn -- evidence-manifest generate \
  --bundle target/bundle.json \
  --agent-receipt target/agent-receipt.json \
  --replay target/replay.json \
  --replay-verification target/replay-verify.json \
  --release-readiness target/release-readiness.json \
  --coverage-audit target/coverage-audit.json \
  --registry target/cairn-evidence-registry.json \
  --registry-verification target/cairn-evidence-registry-verify.json \
  --registry-selection target/cairn-evidence-registry-selection.json \
  --registry-fetch-plan target/cairn-evidence-fetch-plan.json \
  --out target/manifest.json \
  --root .
nix run .#cairn -- evidence-manifest verify target/manifest.json --root .
```

A manifest contains an `artifacts` array of objects with `role`, `path`, `hash`,
and optional `schema` fields. Supported roles include `agent_evidence_bundle`,
`agent_receipt`, `replay_receipt`, `replay_verification`, `release_readiness`,
`coverage_audit`, `evidence_registry`, `evidence_registry_verification`,
`evidence_registry_selection`, and `evidence_registry_fetch_plan`. The verifier composes the existing role validators and
checks declared hashes before emitting one deterministic manifest verification
receipt. Optional `signer` and `anchor` objects are treated as public metadata
and hashed into the receipt, but Cairn does not validate private keys, remote
anchors, timestamp authorities, or cryptographic signature claims in this
slice. Generation writes canonical role ordering and artifact hashes so wrappers
avoid custom JSON/hashing glue. r[verify cairn.supersede.signed_evidence_manifest.docs]
r[verify cairn.supersede.evidence_manifest_generate.docs]
r[verify cairn.supersede.agent_evidence_bundle_verification.docs]
r[verify cairn.supersede.registry_manifest_binding.policy]
r[verify cairn.supersede.agent_receipt_replay_evidence.docs]
r[verify cairn.supersede.agent_receipt_replay_group_coverage.docs]
r[verify cairn.supersede.determinism_replay.groups.docs]
r[verify cairn.supersede.determinism_coverage_release_evidence.docs]
r[verify cairn.supersede.determinism_coverage_release_evidence.archive]
r[verify cairn.supersede.replay_environment_attestation.docs]
r[verify cairn.supersede.replay_receipt_verification.docs]

## Completion gate

`cairn-core` exposes pure helpers for `agent_run_receipt_hash`, `agent_run_receipt_issues`, and `evaluate_agent_run_completion`. A wrapper should run the completion gate before claiming success and cite the resulting receipt hash in its final response. The gate checks required fields, receipt hash consistency, side-effect plan linkage, verification evidence for successful verdicts, replay evidence shape, and nondeterminism hygiene. r[verify cairn.supersede.agent_run_receipts.completion_gate]

## CLI and MCP validation

Use the read-only CLI validation surface to replay a stored receipt without linking to `cairn-core`:

```sh
nix run .#cairn -- agent-receipt validate fixtures/agent-receipts/positive.json
```

The command emits deterministic JSON with `valid`, `verdict`, `issues`, `agent_receipt_hash`, `stated_agent_receipt_hash`, `input_hash`, and a wrapper `receipt_hash`. It returns process success for both passing and failing receipts; malformed JSON, missing files, or oversized files are command errors. r[verify cairn.supersede.agent_receipt_validation_surface.cli] r[verify cairn.supersede.agent_receipt_validation_surface.docs]

MCP clients can call `cairn_agent_receipt_validate` with either a bounded repository-relative `path` plus optional `root`, or an inline `receipt` object. The tool is listed in the default read-only catalog and returns the same structured validation payload under `structuredContent`. r[verify cairn.supersede.agent_receipt_validation_surface.mcp]

## Replay fixtures

Maintained replay fixtures should pair captured inputs, tool manifests, plans, mutations, verification evidence, and nondeterminism manifests with expected receipts. Positive fixtures must generate byte-identical receipts across repeated construction. Negative fixtures should cover missing plan linkage, missing verification evidence, uncaptured nondeterminism, and mutation outside the approved plan. r[verify cairn.supersede.agent_run_receipts.replay]
