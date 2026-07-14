## Context

The existing readiness core receives deserialized values whose digest fields may have been supplied by the same untrusted input being evaluated. Lexical shape checks and equality between caller-controlled fields do not establish that a digest names the referenced bytes. Cairn also has policy defaults containing placeholder hash material and release profiles where high-value evidence gates may remain optional.

## Decisions

### Measure bytes in the shell, compare typed identities in the core

The CLI shell reads explicitly supplied artifact, sidecar, receipt, graph, and policy-export files. It computes BLAKE3 over the correct canonical byte domain and produces typed measured-input records. The pure core compares those records with declared roles, schemas, subject identities, parent links, and policy constraints.

The core does not read files, resolve sibling repositories, fetch network resources, inspect clocks, or verify signatures through ambient key discovery. The shell supplies bounded signature-verification observations to pure policy evaluation.

### Recompute receipt identity

Receipt identity is derived from the canonical receipt bytes with only the schema-declared self-hash field structurally excluded. A supplied receipt hash is an assertion to compare, never the source of truth. The corrected Valence Preserves profile is the accepted canonicalization dependency for imported Valence receipts.

### Require configured producer authorization

The Onix release profile requires evidence counted as passing to carry a verified producer signature under a reviewed trust-root and purpose policy. Signature verification binds producer, key identity, purpose, subject identity, canonical receipt bytes, and parent identities. Unknown, missing, revoked, stale, malformed, or wrong-purpose signatures fail closed.

Cairn does not discover trust roots or grant runtime authority. Trust-root configuration is explicit policy input.

### Derive policy identities from Nickel

Accepted policy/profile hashes are generated from canonical exports of reviewed Nickel policy material. Placeholder values are invalid. Policy freshness checks compare measured policy identity with the configured accepted identity and report stale or missing policy deterministically.

### Separate release and diagnostic modes

The Onix release profile requires authenticated stack-provenance and function-address evidence. Disabled or optional modes may remain available only in profiles that explicitly cannot produce Onix release-ready status. A skipped optional gate cannot be promoted by another summary field.

### Dependency ordering

Local package metadata blocks this change on `evidence-chain-readiness-gates`, `function-address-preserves-readiness`, and `preserves-evidence-envelope-ingest`. Cross-repository implementation waits for the archived Valence `harden-preserves-integrity-boundaries` receipt. The cross-repo prerequisite remains documented until Cairn's cross-repository dependency feature is implemented.

### CI scope

The checked-in workflow runs only `nix flake check`. Focused core, CLI, signature, and fixture tests remain required implementation evidence outside the minimal CI command scope.

## Risks / Trade-offs

- Signature policy introduces trust-root operations, but unsigned caller-controlled JSON cannot authenticate a producer.
- Mandatory release gates may block current release flows; that is preferable to issuing release-ready receipts over unauthenticated evidence.
- Correct canonical receipt measurement may invalidate existing fixtures and receipts; migrations must be explicit.
