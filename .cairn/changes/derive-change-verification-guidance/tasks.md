# Verification guidance tasks

## Contract and core

- [ ] [serial] [task:baseline] Record trace-index, requirement-registry, developer-loop, and applicability baselines with valid and stale inputs. r[cairn.verification_impact.conformance]
- [ ] [serial] [task:contracts] [after:baseline] Define typed source-comparison, check-map, scope, uncertainty, and explanation contracts with named bounds. r[cairn.verification_impact.inputs] r[cairn.verification_impact.uncertainty]
- [ ] [serial] [task:core] [after:contracts] Implement pure delta admission and conservative requirement-to-check selection with exact causal paths and mandatory-check preservation. r[cairn.verification_impact.inputs] r[cairn.verification_impact.selection]
- [ ] [serial] [task:evidence] [after:core] Compose current applicability validators without granting whole-subject evidence reuse after a source change. r[cairn.verification_impact.applicability]

## Stack adapters and interfaces

- [ ] [serial] [task:adapters] [after:evidence] Admit compatible Octet structural and Valence span evidence while retaining declared-link fallback for unavailable stronger contracts. r[cairn.verification_impact.uncertainty]
- [ ] [serial] [task:interfaces] [after:adapters] Add CLI and MCP guidance through unified readiness actions and exact producer-plan requests without execution. r[cairn.verification_impact.interfaces]
- [ ] [serial] [task:controls] [after:interfaces] Add deletion, rename, copied-marker, missing-edge, stale-index, cycle, macro, generated-source, target, and feature controls. r[cairn.verification_impact.selection] r[cairn.verification_impact.uncertainty] r[cairn.verification_impact.conformance]

## Consumer evidence and handoff

- [ ] [serial] [task:mutation] [after:controls] Retain a failing unsafe-omission mutant and direct positive controls for required-check preservation. r[cairn.verification_impact.conformance]
- [ ] [serial] [task:consumer] [after:mutation] Replay fresh Cairn source comparisons and document selections, refresh needs, fallback scope, and outstanding full gates. r[cairn.verification_impact.applicability] r[cairn.verification_impact.interfaces]
- [ ] [serial] [task:verification] [after:consumer] Run focused and full checks, retain exact comparison receipts, and prepare reviewed sync and archive plans. r[cairn.verification_impact.conformance]

All tasks describe future implementation. Planning gates do not complete these tasks.
Archive execution is not a prerequisite of any task. Use the normal lifecycle after implementation evidence passes.
