# Nominal evidence identities

Cairn uses private Rust types for selected lifecycle and evidence identities. These types prevent accidental role substitution inside the pure core.

Stable JSON remains the wire format. Cairn uses this boundary flow:

```text
wire DTO -> checked adapter -> typed core -> wire projection
```

## Ownership

Nickel owns the reviewed identity catalog in `cairn-policy/nominal-evidence-identities.ncl`. The generated policy contains runtime and Octet projections from one domain list.

Rust owns identity parsing, typed admission, comparisons, and deterministic diagnostics. The CLI owns file reads, JSON presentation, and mutation.

## Selected inventory

| Raw value | Classification | Typed role | Core scope | Wire compatibility |
|---|---|---|---|---|
| change, workspace, archive | domain ID | `ChangeId`, `WorkspaceId`, `ArchiveId` | lifecycle and collaboration plans | stable CLI and collaboration fields |
| actor, operation | domain ID or reference | `ActorId`, `OperationId` | collaboration transitions | portable and COB envelopes |
| policy, input, plan | BLAKE3 role | `PolicyDigest`, `InputDigest`, `PlanDigest` | gates, replay, and mutation plans | stable receipt hash fields |
| receipt, chain, ledger | BLAKE3 role | `ReceiptDigest`, `ChainDigest`, `LedgerDigest` | evidence and operation chains | stable receipt and ledger fields |
| state, content root | BLAKE3 role | `StateDigest`, `ContentRootDigest` | collaboration and mutation checks | stable state envelopes |
| artifact, source, binary | BLAKE3 role | `ArtifactDigest`, `SourceDigest`, `BinaryDigest` | readiness and evidence linkage | producer receipt fields |
| Valence, Mantle, graph, sidecar | BLAKE3 role | exact producer role | stack readiness | imported producer JSON |
| archive, pack | BLAKE3 role | `ArchiveDigest`, `PackDigest` | compaction | archive pointers and manifests |
| SHA-256, Git OID | imported digest | dedicated interoperability type | adapter validation only | external protocol spelling |
| paths, commands, schemas, diagnostics | safe path, wire, or display text | not a nominal identity | shell or display logic | unchanged |

The catalog keeps display text and shell paths outside nominal enforcement. A later reviewed policy can add a new role.

## Construction rules

`DomainId<Role>` checks the size, prefix, and canonical spelling. `Blake3Digest<Role>` checks the selected BLAKE3 spelling and normalized hexadecimal value.

Imported algorithms never enter `Blake3Digest<Role>`. Cairn uses separate types for SHA-256 and Git object identifiers.

The private fields prevent unchecked construction. The types do not implement `Deref` or unchecked primitive conversions.

## Compatibility

Public DTO fields remain strings. Typed values exist only after admission, so valid JSON and receipt hash material remain unchanged.

Compatibility adapters can accept an existing uppercase digest where the old boundary permits it. The typed core always stores lowercase hexadecimal text.

## Octet enforcement

The generated Octet projection uses `deny` mode for the selected symbols and core scope. Compatibility scopes remain explicit.

A passing Octet check proves configured source-shape conformance only. It does not prove evidence truth or runtime behavior.

## Claim boundary

Nominal identities prove configured local syntax and role separation only. They do not prove evidence truth, authority, policy correctness, or verifier soundness.

They do not prove mutation safety outside checked plans or release eligibility.

<!--
r[impl cairn.nominal_evidence_identities.inventory]
r[impl cairn.nominal_evidence_identities.wire_boundary]
r[impl cairn.nominal_evidence_identities.compatibility]
r[impl cairn.nominal_evidence_identities.octet]
r[impl cairn.nominal_evidence_identities.docs]
r[impl cairn.nominal_evidence_identities.final_checks]
r[verify cairn.nominal_evidence_identities.inventory.exclusions]
r[verify cairn.nominal_evidence_identities.docs.pure]
r[verify cairn.nominal_evidence_identities.final_checks.boundary]
-->
