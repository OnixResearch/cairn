# Design: Export a content-bound requirement registry

## Goal

Produce one portable registry that lets downstream tools distinguish an accepted requirement from a same-named or stale requirement.

## Functional core

`cairn-core` receives already-loaded accepted specifications, generated policy bytes, and explicit repository metadata. It returns a registry or deterministic issues.

The core performs no file access, environment inspection, VCS lookup, process execution, clock access, or output rendering.

## Registry model

The versioned registry contains:

- a repository identifier and typed declared revision;
- accepted-specification and generated-policy BLAKE3 identities;
- sorted requirement rows with ID, safe relative path, source span, and requirement-body BLAKE3;
- required non-claims and producer schema identity;
- a canonical registry BLAKE3 over all semantic fields.

The declared revision uses a closed kind vocabulary. Initial kinds cover Git commits, Jujutsu commits, and opaque reviewed revisions.

The revision is caller-supplied metadata. The registry does not claim that it authenticates or discovers repository history.

## Canonicalization

The core sorts specification files and requirement rows by portable keys. It rejects duplicate IDs before registry hashing.

Display-only absolute roots never enter canonical bytes. Safe repository-relative paths and exact requirement spans remain hash-bound navigation data.

The registry hash uses BLAKE3 over canonical typed content. Row hashes use separate domain labels from the registry hash.

## Imperative shell

The CLI opens an explicit capability root, loads accepted specifications and generated policy, and passes bounded values to the core.

The proposed commands are:

```text
cairn requirement-registry export --root <ROOT> --repository-id <ID> --revision-kind <KIND> --revision <VALUE> --out <FILE>
cairn requirement-registry verify --root <ROOT> --registry <FILE> --repository-id <ID>
```

Export writes only the requested file after complete validation. Verification recomputes current specification and policy identities before comparing rows.

## Evidence publication

Cairn evidence manifests can carry a `requirement_registry` artifact role. The outer manifest binds the registry bytes and receipt identity.

Publication does not promote the registry into evidence of requirement satisfaction or source correctness.

## Failure behavior

Export or verification fails for duplicate IDs, unsafe paths, malformed revisions, malformed BLAKE3 values, stale requirement bodies, policy drift, or registry hash mismatch.

Issues use stable codes and safe field paths. Diagnostics must not include absolute private roots or unbounded requirement text.

## Verification

Positive fixtures cover deterministic export and current-root verification. Negative fixtures mutate one identity domain at a time.

Core tests use in-memory specifications. CLI tests cover bounded reads, explicit writes, no-write failures, and evidence-manifest integration.

## Rollout

Valence consumes this registry through a separate reviewed revision. Mantle, Octet, and OnixOS then bind their requirement references to that Valence contract.
