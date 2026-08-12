# Content-bound trace index

Cairn can export declared requirement links as one deterministic JSON artifact.
The index composes with the content-bound requirement registry.
It does not parse requirement meaning a second time.

## Export an index

First, export a current requirement registry:

```bash
cairn requirement-registry export \
  --root . \
  --repository-id onixresearch/cairn \
  --revision-kind git \
  --revision REVISION \
  --out target/requirement-registry.json
```

Then export the trace index:

```bash
cairn traceability export \
  --root . \
  --registry target/requirement-registry.json \
  --out target/trace-index.json
```

The command first verifies the registry against the current accepted specifications and policy bytes.
The command then scans the evidence roots from the selected traceability profile.
The output path must not exist.

The canonical index contains:

- the complete selected traceability profile and its BLAKE3.
- the policy and requirement-registry BLAKE3 values.
- the sorted content-bound requirement rows.
- each safe source path and exact source-file BLAKE3.
- each marker role, exact marker BLAKE3, and half-open byte span.
- display line bounds for operator navigation.
- each domain-separated link BLAKE3.
- the canonical index BLAKE3 and explicit non-claims.

Line numbers do not define a link identity.
The link identity includes the source path, source-file BLAKE3, and byte span.
Thus, a moved or copied marker changes the current link set.

## Verify current content

Use the same registry and selected profile when you verify the index:

```bash
cairn traceability verify \
  --root . \
  --registry target/requirement-registry.json \
  --index target/trace-index.json
```

Verification rebuilds the index from current bounded inputs.
It rejects policy drift, requirement-registry drift, changed files, moved markers, copied markers, and changed links.
It also rejects unsafe paths, duplicate source paths, dangling markers, invalid hashes, and noncanonical order.

## Query one requirement

Query a saved index with an exact requirement ID:

```bash
cairn traceability show cairn.trace_index.query \
  --index target/trace-index.json
```

The result contains one content-bound requirement row and its sorted declared links.
An unknown requirement ID produces a deterministic issue and a failing exit status.

`show` validates the saved artifact, but it does not compare the artifact with a repository.
Run `traceability verify` when current-root freshness is required.

## Assurance and anchors

Nickel owns the selected assurance level and anchor kind.
This version supports `assurance_level = "declared"` and `anchor_kind = "exact_marker_bytes"`.

The declared assurance level records marker linkage only.
It does not provide an AST-resolved symbol, executed test result, or proof result.
A future adapter can add structural or executed evidence through a new versioned contract.

## Claim boundary

A valid index proves canonical identity, current-input freshness, and declared marker linkage for the supplied inputs.
It does not prove:

- requirement satisfaction.
- implementation correctness.
- test adequacy.
- proof soundness.
- release eligibility.
- lifecycle readiness.
- behavioral correctness.

## Design references

Ambient separates stable identities from exact source occurrences.
See `../ambient/crates/ambient-analysis/src/occurrences.rs` and `../ambient/crates/ambient-engine/src/module_interface/ast_hash.rs`.
Ambient also provides canonical snapshot and deterministic diff precedents in `disk_store/snapshot.rs` and `disk_store/diff.rs`.

Unison separates mutable names from immutable definition hashes.
Its recursive components use a component hash and a canonical member position.
See `unison-core/src/Unison/Reference.hs` and `unison-hashing-v2/src/Unison/Hashing/V2/Term.hs`.

Cairn keeps BLAKE3 because this is a stack-owned content identity.
It does not copy Unison's SHA3-512 hash choice or codebase-manager model.
