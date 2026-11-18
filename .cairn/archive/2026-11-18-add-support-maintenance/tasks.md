# Support maintenance tasks

## Contract and implementation

- [x] [serial] [task:policy] Author the optional Nickel support-maintenance policy with the admitted change vocabulary and bounds. r[cairn.support_maintenance.bounds]
- [x] [serial] [task:records] [after:policy] Add versioned claims, branches, evidence, ordered changes, obligations and report types with explicit admission errors. r[cairn.support_maintenance.alternatives] r[cairn.support_maintenance.scope]
- [x] [serial] [task:changes] [after:records] Apply the ordered change log for evidence invalidation, assumption withdrawal, admitted counterevidence, revision, policy and context changes. r[cairn.support_maintenance.invalidation] r[cairn.support_maintenance.assumptions] r[cairn.support_maintenance.revisions]
- [x] [serial] [task:recompute] [after:changes] Implement deterministic full recomputation with alternative survival, counterevidence retention, dependent propagation and historical branch causes. r[cairn.support_maintenance.dispute] r[cairn.support_maintenance.determinism]
- [x] [serial] [task:cli] [after:recompute] Add the read-only `support-maintenance explain` CLI with bounded reads and an explicit advisory boundary. r[cairn.support_maintenance.inspection] r[cairn.support_maintenance.authority]

## Acceptance and handoff

- [x] [serial] [task:positive] [after:cli] Test alternative survival after unique-premise withdrawal, shared-assumption invalidation, stale evidence, admitted counterevidence, explicit revision relations and unaffected new scopes. r[cairn.support_maintenance.alternatives] r[cairn.support_maintenance.invalidation] r[cairn.support_maintenance.assumptions] r[cairn.support_maintenance.revisions]
- [x] [serial] [task:negative] [after:positive] Test missing cross-revision relations, unknown change targets, unsupported schemas, bound exhaustion, cycles and zero lifecycle mutation. r[cairn.support_maintenance.scope] r[cairn.support_maintenance.bounds] r[cairn.support_maintenance.inspection]
- [x] [serial] [task:determinism] [after:negative] Replay equivalent input permutations and explicit event orderings, retaining the same report identity for equivalent facts. r[cairn.support_maintenance.determinism]
- [x] [serial] [task:docs] [after:determinism] Document alternative support, historical withdrawal reasons, admissible changes, bounds and the advisory boundary. r[cairn.support_maintenance.authority]
- [x] [serial] [task:verify] [after:docs] Run focused core and CLI tests, policy freshness, formatting and lifecycle validation against current-source inputs. r[cairn.support_maintenance.inspection]
- [x] [serial] [task:handoff] [after:verify] Prepare the specification sync and archive preview after implementation evidence passes. r[cairn.support_maintenance.determinism]

Executed evidence: 10 core `support_maintenance` tests and 2 CLI
`support_maintenance` tests pass. `cairn policy export --check` is fresh,
`cairn validate --root .` reports 0 issues, and
`cairn traceability coverage --root .` is 2390/2390 after sync.

Repairs made during acceptance: three container/type mismatches
(`BTreeMap<&str, Vec<&str>>` versus `Vec<String>`) and one map lookup
(`state.cache.get(*premise)`) were corrected before the suites could run.

Scope: recomputation is deterministic and replayable over declared inputs; it
adds no truth-maintenance or lifecycle authority beyond the existing gate and
readiness meanings. A withdrawn branch retains its historical reason, and
classifications never transfer support across revisions.
