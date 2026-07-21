# Design: Backend-Agnostic Change Collaboration

## Goal and Success Evidence

Cairn collaboration is backend-agnostic when the pure semantic state uses no mandatory Radicle context, an operator can import/status/checkout through a non-Radicle backend, existing Radicle v1 objects retain their historical content/state identities, and deterministic positive and negative tests exercise both provider paths.

False completion includes renaming Radicle symbols without changing schema semantics, adding a trait with only one implementation, requiring `--rad-home` or `--rid` on the neutral CLI, trusting caller-declared authority as cryptographic proof, or rehashing historical Radicle v1 state under a new interpretation.

## Portfolio Decision

Three mechanisms were evaluated:

1. **Neutral v2 reducer plus provider adapters and a portable backend — selected.** This moves workspace and base-revision identity into the semantic contract, proves independence with a second executable backend, and isolates legacy interpretation.
2. **Backend-neutral status/checkout projection over the existing v1 reducer — rejected.** Import state, content identity, and future updates remain Radicle/Git-shaped.
3. **Generic provider trait over existing v1 types — rejected.** Every implementation must still fabricate Radicle schema and Git OID inputs, so the abstraction is nominal rather than semantic.

An external helper/plugin protocol is deferred because process deadlines, executable authorization, protocol versioning, and credential transfer would create a new trust boundary. Built-in adapters establish the contract first.

## Semantic Core

The current reducer is generalized to a neutral schema, `cairn.change-collaboration.v1`. Initial state carries:

- stable change ID;
- opaque bounded workspace ID;
- typed base revision with a stable kind and bounded value;
- policy hash;
- creator identity; and
- exact supported active-change artifacts.

Operation IDs and actor IDs remain bounded opaque identifiers. The reducer continues to receive explicit `is_delegate` facts; it never opens a repository, reads an authority document, verifies a signature, or infers authority from identifier spelling.

BLAKE3 content identity includes the neutral schema, workspace, typed base revision, policy, creator, and exact artifacts. State identity additionally includes revision history, lifecycle, evidence, and operations. Conflict, evidence-staleness, and lifecycle behavior are unchanged.

## Radicle v1 Compatibility

`cairn.radicle-change-cob.v1` remains a recognized legacy schema. Legacy `repository_id` and `base_git_oid` fields decode into the neutral in-memory context, but content-root and state-hash functions use the exact legacy canonical field names and values for that schema. This preserves historical identities rather than silently reinterpreting them.

New Radicle imports use the neutral semantic schema. The Radicle adapter still owns Git OID validation, related-object checks, current canonical identity lookup, signer possession, COB object IDs, and signed store mutation. Existing `cairn radicle ...` commands remain compatibility entry points.

## Provider Boundary

The public neutral command is:

```text
cairn collaboration <import|status|checkout> <change> --backend BACKEND ...
```

`radicle.v1` dispatches to the existing adapter with explicit Radicle options. `portable-json.v1` uses an explicit store file, workspace ID, typed base revision, and actor for import. Status and checkout need only the store and repository root/policy observations.

The portable store is a canonical JSON envelope containing the neutral reduced state, semantic content root, state hash, and backend/schema labels. Import is preview-first. Execution serializes through an exclusive sibling lock, refuses an existing store, rechecks active files and policy, writes an exclusive prepared file, atomically renames it, reloads it, and validates all hashes. Status is bounded and read-only. Checkout rechecks store state under a lock and reuses the exact no-overlay materialization shell.

Portable actor and workspace inputs are caller declarations, not signature or remote-authority proof. Filesystem mutation relies on the operator's local capability. Receipts state this explicitly.

## Functional Core and Imperative Shell

Pure code owns schema validation, legacy-compatible hashing, portable envelope construction/validation, and checkout planning. Shell code owns backend selection, argument parsing, files, locks, exact reads/writes, atomic rename, policy loading, Radicle profiles/signers, and JSON output.

## Adversarial Cases

Tests cover malformed revision kinds/values, legacy hash drift, unsupported schemas, tampered portable envelopes, duplicate/existing stores, symlink stores, input drift, stale locks/prepared files, divergent destinations, conflict retention, and backend argument mismatch. A parity fixture proves the same neutral state loaded through portable serialization retains the same state/content identities.

## Claim Boundary

Backend selection and successful local materialization do not prove network publication, peer synchronization, remote trust, caller identity, signature validity for unsigned providers, gate validity, evidence truth, dependency readiness, archive execution, source correctness, or release eligibility. Provider-specific authority observations remain labeled and cannot be promoted into a universal authority claim.
