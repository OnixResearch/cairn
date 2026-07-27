# Requirement ID Registry

Cairn has two versioned requirement registry contracts. The legacy contract preserves curated cross-repository aliases and consumer rules.

The content-bound contract records each accepted requirement from the selected repository root. It binds source bytes, policy bytes, repository metadata, and safe source spans.

## Export a content-bound registry

Run this command from the repository that owns the selected Cairn lifecycle:

```bash
cairn requirement-registry export \
  --root . \
  --repository-id OnixResearch/example \
  --revision-kind git \
  --revision 0123456789abcdef0123456789abcdef01234567 \
  --out target/requirement-registry.json
```

The export command reads only accepted specifications under `cairn/specs/`. It does not include active change deltas.

The generated policy defaults to `cairn-policy/generated/cairn-policy.json` under the selected root. Use `--policy RELATIVE_PATH` for another reviewed file under that root.

The output path is an explicit file capability. Cairn creates a new file only after the complete registry passes core validation.

## Verify the current root

Use the explicit expected repository identity during verification:

```bash
cairn requirement-registry verify \
  --root . \
  --registry target/requirement-registry.json \
  --repository-id OnixResearch/example
```

Verification recalculates the specification, policy, row, registry, and receipt identities. It rejects stale requirements, changed policy, duplicate IDs, unsafe paths, malformed revisions, and wrong repositories.

Diagnostics contain stable issue codes and safe field paths. They do not contain requirement bodies or private absolute repository paths.

## Content-bound contract

Content-bound registries use schema `cairn.content-bound-requirement-registry.v1`. The registry includes these identity groups:

- The repository ID and typed declared revision.
- Sorted accepted specification paths and BLAKE3 content identities.
- The BLAKE3 identity of the generated policy bytes.
- Sorted requirement rows with safe source spans and body identities.
- Domain-separated row, registry, and receipt BLAKE3 identities.
- Required non-claims and the identity-only boundary.

Revision kinds are `git`, `jujutsu`, and `opaque`. Git revisions must use a complete lowercase hexadecimal object identity.

Jujutsu revisions use a bounded lowercase hexadecimal commit identity. Opaque revisions are bounded reviewed labels.

A revision is caller-supplied metadata. Cairn does not inspect VCS state or prove revision authenticity.

## Evidence manifests

Add an optional content-bound registry to an evidence manifest with this flag:

```bash
cairn evidence-manifest generate \
  --bundle bundle.json \
  --agent-receipt agent.json \
  --replay replay.json \
  --replay-verification replay-verification.json \
  --requirement-registry target/requirement-registry.json \
  --out target/evidence-manifest.json
```

The `requirement_registry` role binds the exact registry file bytes. It also binds the nested registry receipt identity.

The role is identity evidence only. It cannot satisfy an implementation, test, proof, or release role.

## Legacy compatibility contract

Legacy registry inputs use schema `cairn.requirement-id-registry.v1`. Keep this command for existing curated registry inputs:

```bash
cairn requirement-registry verify \
  --input fixtures/requirement-id-registry/positive/owned-id.json
```

Each legacy row records a requirement ID, namespace, owner, accepted specification, status, aliases, and allowed consumers.

Cairn does not silently convert a legacy registry into a content-bound registry. A producer must export the new contract from accepted source bytes.

## Non-claims

A valid content-bound registry proves canonical identity, current-root freshness, and declared linkage for the supplied files and metadata.

It does not prove revision authenticity, requirement satisfaction, source correctness, implementation correctness, test coverage, or release eligibility.
