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
- `replay_verdict`: must be `pass`
- `replay_artifact_path`: optional display-only path for local operators

`agent-receipt validate` compares `replay_policy_hash` with the current generated
policy hash and requires the `agent_receipt_validate`, `mcp_describe`, `status`,
and `validate` replay cases. Missing, stale, failed, or insufficient replay
evidence produces deterministic issue codes such as `replay_evidence.missing`,
`replay_evidence.policy_hash_stale`, and
`replay_evidence.insufficient_coverage`. r[verify cairn.supersede.agent_receipt_replay_evidence.docs]

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
