# Design: Evidence Export Bundle

## Context

The current evidence chain is complete but operator assembled:

1. `determinism coverage-audit`
2. `determinism replay --group agent`
3. `determinism verify-receipt`
4. `release-readiness`
5. `agent-receipt verify-bundle`
6. `evidence-manifest generate`
7. `evidence-manifest verify`

This is deterministic when done correctly, but wrappers can forget a step, choose inconsistent paths, or publish an incomplete set.

## Goals

- Provide one command that materializes the canonical local evidence set.
- Keep all paths in receipts relative to the export directory or stable role names.
- Reuse existing validators rather than duplicating evidence semantics.
- Make repeated export into distinct roots produce the same logical hashes.

## Non-Goals

- Cryptographic signing or remote publication.
- Running expensive external CI or network checks.
- Changing the semantics of existing lower-level receipt commands.

## Decisions

### 1. Artifact directory with canonical filenames

`cairn evidence export --out <dir> --root .` writes stable filenames:

- `coverage-audit.json`
- `replay-agent.json`
- `replay-verification.json`
- `release-readiness.json`
- `agent-receipt.json`
- `agent-evidence-bundle.json`
- `agent-evidence-bundle-verification.json`
- `evidence-manifest.json`
- `evidence-manifest-verification.json`
- `evidence-export-receipt.json`

Rationale: stable filenames make archive/publish steps reviewable without teaching every wrapper the full flow.

### 2. Compose existing receipt builders in-process

The CLI should call the same Rust helpers used by the lower-level commands, write their JSON outputs, then verify the generated bundle and manifest.

Rationale: this prevents divergent shell-wrapper behavior while keeping lower-level validators authoritative.

### 3. Stable export receipt

The final receipt includes:

- `command`
- `root`
- `out_dir`
- `valid`
- `verdict`
- `issues`
- `artifacts` sorted by role
- `manifest_hash`
- `manifest_verification_receipt_hash`
- `receipt_hash`

Artifact entries include stable `role`, `path` relative to the export directory, `hash`, and `required`.

### 4. Deterministic failures

Failures use stable issue codes/classes and do not include raw absolute paths, raw environment values, credentials, or secrets. The command may include display `root`/`out_dir` but hashes and issue semantics must not depend on host-specific prefixes.

## Risks / Mitigations

- **Recursive receipt hashing**: write receipts with internal hashes already computed and compute export receipt hash over artifact metadata, not volatile command output.
- **Dirty-repo release readiness in tests**: use temp fixtures or assert structural behavior without relying on live repo cleanliness when testing failure modes.
- **Path leakage**: keep artifact metadata relative to output directory and normalize root/out path prefixes in export receipt hashing.
