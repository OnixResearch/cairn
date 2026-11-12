# Source provenance

<!-- r[impl durable_publication.repository.identity] -->

## Reviewed source

- Repository: `https://github.com/OnixResearch/onix-artifact`
- Imported revision: `c932138d880ddf4c2967f4c024b489b5c0022bf1`
- Reviewed candidate revision: `67f14770a82e24eb39d3a751285b908232566e6b`
- Source path: `fixtures/publication-candidate`
- Decision: `docs/publication/DECISION.md`
- License: `MIT OR Apache-2.0`

The Artifact candidate was a clean-room implementation of its Cairn contract. This extraction uses Artifact’s owner-granted source expression. It does not copy Mantle or OnixOS implementation text.

The typed source manifest is [`source.ncl`](source.ncl). Its deterministic JSON projection is [`source.json`](source.json). The JSON BLAKE3 is `bcb84e58099fbc11cc2d0e6c3ce8e3f0013cfeb3618094379891f4d9d3eee267`.

## Reviewed source files

| File | BLAKE3 | Role |
|---|---|---|
| `src/core.rs` | `ae4a3ee7940b6b782082e9370587f219913e545a3cdbfaeaa9d7eca856b95fec` | Pure request and observation classifier |
| `src/corpus.rs` | `d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da` | Shared conformance corpus |
| `src/shell.rs` | `a9a4fcf75f555d2b2dd9bdac0f2363a359c86d1b496cd556d5dd1edd2ed97ea7` | Capability-relative filesystem shell |
| `src/pilot.rs` | `4e8fbd66b02e0d3306aa9483e383c579a4287e87a9537e49e32c62d6ed576193` | Consumer-shaped extraction evidence |

## Promoted source files

| File | BLAKE3 | Role |
|---|---|---|
| `src/lib.rs` | `4bcb234a0d2ffb1c202a19ae41ef5a6b9529124abb9bbe002c2263049391d129` | Independent public crate boundary |
| `src/core.rs` | `4d040eb172f0fc518f5fa662c1ffa0e4457319921e39fb7a6c1f76d994665469` | Promoted pure classifier |
| `src/corpus.rs` | `d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da` | Promoted conformance corpus |
| `src/shell.rs` | `3fcf209f905c47aac62c30d97e7ae0dfb276949d2eebe68b1298c76d5b0bc05e` | Shell with synchronizable-parent preflight |

## Extraction changes

- The package has an independent release name.
- Product-specific pilot mappings moved under `tests/`.
- Requirement markers now use repository-owned identities.
- Tests now check operation order, preflight non-mutation, symlink stage collisions, replacement, and invalid parent handles.
- Typed Nickel, Nix, Cairn, release, and provenance contracts now govern the repository.

## Non-claims

Source linkage does not prove behavior correctness. Pilot passage does not prove permanent consumer adoption. License recording does not import product-owned implementation expression.
