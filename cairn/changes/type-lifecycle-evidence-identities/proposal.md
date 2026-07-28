# Proposal: Type lifecycle evidence identities

## Why

Cairn core models carry many distinct BLAKE3 roles and lifecycle identities as `String`. Examples include policy, input, plan, receipt, chain, state, content, artifact, Valence, Mantle, graph, archive, change, workspace, operation, and ledger identities.

Field names and runtime checks preserve serialized meaning. They do not stop a wrong-role value from entering another pure core API.

Cairn needs shared nominal identity primitives and a staged migration across its highest-risk lifecycle and evidence seams.

## What Changes

- Inventory semantic IDs, references, digest roles, paths, display text, and wire values across `cairn-core`, with priority on `verified` modules.
- Add private generic ID and BLAKE3 types with marker domains, checked construction, and explicit accessors.
- Separate serialized receipt and policy DTOs from admitted typed core models where roles are statically known.
- Migrate stack and function-address readiness first.
- Migrate agent, replay, release, registry, and evidence receipt chains next.
- Migrate sync, archive, compaction, collaboration, and lifecycle plan identities after compatibility evidence passes.
- Preserve stable CLI JSON fields, receipt hashes, policy hashes, fixture bytes, and legacy compatibility schemas.
- Add compile-fail, parser, receipt, replay, archive, collaboration, and tamper fixtures.
- Dogfood future Octet nominal-domain enforcement in migrated verified-core scopes.

## Impact

- **Core**: Shared identity module and staged changes across verified pure cores.
- **CLI**: Parsing and projection adapters. Filesystem and process authority remain unchanged.
- **Policy**: Nickel-authored domain catalog and deterministic generated projection.
- **Compatibility**: Stable documented receipt and command fields remain unchanged.
- **Traceability**: New requirements and tests for compiler-enforced role separation.

## Non-goals

- Do not wrap every path, message, schema string, command label, or diagnostic field.
- Do not change BLAKE3 or reinterpret imported protocol hashes.
- Do not treat a typed receipt or policy identity as proof that its claims are true.
- Do not move filesystem, process, clock, network, or JSON rendering effects into `cairn-core`.
- Do not change release eligibility policy through this migration.
