# Design: Evidence Manifest Generation

## Context

`cairn evidence-manifest verify` established a deterministic consumer contract for role-indexed evidence artifacts. The producer contract is still implicit: wrappers must create JSON, compute artifact hashes, choose role names, and decide how to include signer/anchor metadata.

## Approach

Add an `evidence-manifest generate` subcommand in the CLI shell. It will:

1. Parse role-specific artifact flags.
2. Require the core evidence chain roles:
   - `agent_evidence_bundle`
   - `agent_receipt`
   - `replay_receipt`
   - `replay_verification`
3. Include `release_readiness` when provided.
4. Read each artifact and compute a stable BLAKE3 content hash.
5. Emit manifest JSON with deterministic role order and stable field order.
6. Optionally include public signer and anchor metadata objects read from JSON files.
7. Write the manifest to stdout or an output file.

## Failure model

Generation is a producing command with file output side effects only when inputs validate. On failure it prints a deterministic JSON receipt with:

- `command`
- `root`
- `valid=false`
- `verdict="fail"`
- stable `issues`
- no raw absolute paths, credentials, environment values, or host-specific values

Unsupported roles are impossible through the public flag surface; missing required role flags produce deterministic issue codes.

## Manifest shape

Generated manifests use the verifier's accepted shape:

```json
{
  "artifacts": [
    {"role": "agent_evidence_bundle", "path": "bundle.json", "hash": "..."},
    {"role": "agent_receipt", "path": "agent.json", "hash": "..."},
    {"role": "replay_receipt", "path": "replay.json", "hash": "..."},
    {"role": "replay_verification", "path": "replay-verify.json", "hash": "..."}
  ],
  "signer": {"...": "..."},
  "anchor": {"...": "..."}
}
```

Paths are wrapper-provided artifact locations. Verification remains authoritative and must be run after generation.

## Determinism

- Artifact order is fixed by role order, not argument order.
- JSON output is pretty, stable, and produced by Cairn.
- Metadata must be object-shaped JSON and is serialized canonically by the manifest object.
- Failure receipts use stable issue codes/classes and do not echo raw path values.
