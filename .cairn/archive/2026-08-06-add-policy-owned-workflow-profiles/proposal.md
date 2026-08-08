## Why

Cairn policy declares one `spec-driven` artifact schema. Every active change therefore follows the same artifact and gate shape, even when assurance work needs more evidence or a bounded internal change has no accepted-spec effect.

A typed profile registry can vary workflow obligations without adding user-global schemas or weakening lifecycle authority. An explicit no-spec profile can also avoid false accepted requirements while retaining review, acceptance, task, and archive evidence.

## What Changes

- Add policy-owned workflow profiles with typed artifact graphs, stage mappings, apply prerequisites, and non-claims.
- Preserve `spec-driven` as the compatibility default for changes without profile metadata.
- Add an `assurance` profile with policy-required risk, verification, and review artifacts.
- Add a `no-spec-delta` profile with explicit `spec_effect = "none"`, a rationale, local acceptance criteria, and semantic review evidence.
- Add profile-aware scaffolding, workflow status, gates, task linkage, sync, archive, receipts, and MCP results.
- Reject user-global schemas, ambient profile selection, conflicting delta specs, and profile changes after lifecycle authoring begins.
- Add positive and negative profile, no-spec, compatibility, mutation, and claim-boundary fixtures.

## Impact

- **Policy**: Add typed profile, artifact, stage, acceptance, review, and non-claim contracts.
- **Core**: Add pure profile admission, selection, artifact obligation, task-link, no-spec sync, and archive planning.
- **CLI and MCP**: Add explicit profile selection to change creation and profile facts to lifecycle results.
- **Documentation**: Add profile selection, assurance, and no-spec guides.
- **Testing**: Add positive and negative policy, core, CLI, MCP, scaffold, gate, sync, archive, and compatibility coverage.

## Out of Scope

- Loading schemas from user home directories or remote packages.
- Letting a profile bypass policy, review, gate, evidence, or release requirements.
- Proving that a `no-spec-delta` declaration has no behavior effect.
- Adding a universal low-rigor profile without a reviewed risk-admission contract.
- Changing profiles after substantive lifecycle artifacts exist.
