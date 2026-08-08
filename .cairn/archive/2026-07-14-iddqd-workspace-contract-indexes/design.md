## Context

The workspace contract matrix validates declared edges, artifact identities, fixture coverage, readiness modes, and non-claim boundaries. Edge records and observed artifact records both carry `edge_id`, so a plain `BTreeMap<String, WorkspaceArtifactObservation>` duplicates identity and makes overwrite behavior too easy to reintroduce.

## Design

### Identity-map core

Introduce small pure helper types or trait implementations so workspace contract records can be indexed by ids borrowed from the values:

- declared `WorkspaceContractEdge` values keyed by `edge_id`;
- observed `WorkspaceArtifactObservation` values keyed by `edge_id`.

Use `IdOrdMap` rather than hash maps to preserve deterministic iteration and avoid fast-hasher tradeoffs for externally supplied lifecycle inputs. All insertion must use unique semantics so duplicate ids produce diagnostics, not silent replacement.

### Validation flow

Validation should parse the matrix and observations into normal DTOs, then build identity maps before edge checks. Duplicate declared edges continue to emit the existing ambiguous-edge class. Duplicate observations should emit a deterministic invalid-input or ambiguous-observation diagnostic before stale-identity comparison. Stale and missing observations should retain existing stale-edge behavior.

### Serialization and receipts

The public matrix JSON, validation receipt, release-readiness receipt, and non-claim text remain unchanged. iddqd is an implementation detail of the pure validation core.

### Fixtures

Positive fixtures cover one declaration per edge id and one matching observation per edge id. Negative fixtures cover duplicate declared edge ids, duplicate observations, missing observations in required modes, stale artifact identities, unsafe paths, and weakened non-claims.

## Non-goals

- Do not change workspace contract matrix schema version or release-readiness mode semantics.
- Do not make Cairn prove provider correctness, consumer behavioral correctness, verifier soundness, or release eligibility.
- Do not modify the active requirement-marker-core work in this slice.
