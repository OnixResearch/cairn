# Cross-repository adoption design

## Context

The workspace root already owns `stack-deps` audit and reconciliation plus `stack-doctor` observations.
Dependency planes are separate from transport kinds. Audit-only edges grant no update authority, and their cycles can be valid observations.
Cairn's cross-repository dependencies admit explicit receipt links. Its global registry is a derived discovery projection.
The new report connects these contracts without duplicating dependency discovery or update logic.

## Goals and non-goals

Cairn dependency maintainers own the report. Workspace maintainers retain producer contracts and catalogs.
Cairn, Octet, and Lattice maintainers are the initial consumer-chain operators.
The goal is an exact account of observed adoption gaps, not a workspace-wide green status or automated update plan.
No sibling scan, VCS transport, live check execution, credential lookup, or dependency mutation occurs in Cairn.

## Decisions

### Explicit inputs and adapters

The command accepts saved dependency audit, reconciliation, optional health, upstream lifecycle, and consumer-check receipts under explicit safe roots.
Each adapter declares supported schema, source revision, tool identity, policy identity, and observation scope.
The first workspace cohort uses catalog and reconciliation schema version 2 and dependency receipt schema version 1.
Those versions are producer-owned compatibility fields, not new Cairn policy constants.

The implementation consumes a published producer admission contract or explicit current validation receipt with checked conformance vectors.
It must not copy the root tools' source or reimplement their reconciliation semantics.
A missing compatible producer contract blocks that adapter until the producer owner publishes one through a separate change.
No ambient workspace import substitutes for that publication.

The exact root tool revision in `docs/cairn-utility-program.md` is an input provenance reference, not a runtime path dependency.
Normal report commands never run `stack-deps`, `stack-doctor`, or producer validators as hidden effects.
The operator obtains those observations outside Cairn.

### Pure adoption model

The shell loads bounded bytes and translates admitted source contracts into domain observations.
The pure core joins observations by explicit repository, relationship, change, consumer, revision, subject, and policy identities.
It preserves each producer's dependency plane and status vocabulary.

Each row has independent upstream-completion, consumer-pin, compatibility, and integration-evidence states.
A completed upstream archive cannot establish a matching pin.
A matching pin cannot establish compatibility or current consumer checks.
A complete local integration observation cannot establish release eligibility or current remote state.

Unknown, stale, missing, unavailable, contradictory, and partial inputs stay visible.
Absence from the supplied scope is not evidence that a consumer does not exist.
Audit-only cycles remain reportable facts. They never become a bulk update order or an invalid lifecycle DAG by coercion.

### Registry, actions, and views

Reuse existing registry node and edge identities as optional derived indexes over the exact input snapshot.
Registry membership cannot satisfy an evidence requirement. A discarded registry can be rebuilt without loss of source authority.
Queries require explicit scope and record selected and omitted coverage.
Bounds that truncate observations produce a partial report, never complete workspace health.

The package follows unified readiness and uses its typed next-action vocabulary.
Actions can identify an external stack-deps plan, a fresh audit, a consumer pin review, or a consumer check requirement.
They record the responsible repository and exact prerequisites but grant no effect authority.
CLI JSON, MCP, and concise human output share the same report identity.
Site or Subwayrat views remain optional later projections with no additional domain meaning.

### Evidence and source ownership

Valence admission preserves roles, spans, provenance, and non-claims for imported evidence where its contract applies.
Cairn lifecycle validators retain lifecycle meaning. Consumer-owned completion contracts retain required-check meaning.
Published pinned sources and consumer conformance evidence are mandatory for any new shared crate or schema dependency.
The workspace catalog remains outside Cairn. No generic shared platform package is created.

## Risks and trade-offs

Snapshot freshness is bounded to supplied observations. A report cannot claim the latest remote state without a new external observation.
Different checkouts can describe different consumer pins. Exact source identities prevent label-based reconciliation.
The workspace tools are operator infrastructure rather than a guaranteed reusable Rust API.
The adapter must publish its exact admitted boundary or report a blocker, not invent a library contract.

## Validation strategy

Establish cross-repository receipt, registry, and unified-readiness baselines before implementation.
Retain exact saved producer cohorts for Cairn-to-Octet and Octet-to-Lattice.
Positive cases cover complete staged adoption and deterministic reordering.
Negative cases cover an upstream archive with an old pin, matching pins without checks, wrong subjects, stale policy, conflicting rows, unsupported schemas, and missing evidence.
Include audit-only cycles and partial query bounds without accidental update authority.
Replay the same bundle from a directory without sibling checkouts. Assert zero network, process, and mutation effects.
Run direct producer-conformance comparisons, CLI/MCP parity, Nickel fixtures, focused tests, strict Clippy, Slotcar, Cairn gates, and full Nix checks.

## Migration and completion

This is an additive saved-input report. Existing registry and dependency decisions remain unchanged.
Publish the supported producer cohort, input recipe, and positive and negative portable fixture set together.
Completion requires fresh-directory report reproduction and consumer-owned evidence for every claimed stage.
A matching label, dependency pin, archive, or task count does not establish adoption.
