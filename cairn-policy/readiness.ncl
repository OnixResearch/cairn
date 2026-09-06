# r[impl cairn.unified_readiness.snapshot]
# r[impl cairn.unified_readiness.actions]
let Positive = std.contract.from_predicate (fun n => std.is_number n && n > 0 && std.number.floor n == n) in
let State = [| 'ready, 'blocked, 'unavailable, 'not_applicable |] in
let StageKind = [| 'artifact, 'proposal, 'design, 'tasks, 'completion, 'sync, 'archive |] in
let ActionKind = [| 'artifact_repair, 'evidence_refresh, 'dependency_wait, 'diagnostic_inspection, 'sync_preview, 'archive_preview, 'unresolved |] in
{
  contracts = {
    Scope = { change | String, policy_hash | String, source_hash | String, evidence_hash | String },
    Stage = { stage | StageKind, state | State, decision_hash | String },
    Blocker = { id | String, stage | StageKind, code | String, target | String },
    Action = { id | String, kind | ActionKind, causes | Array String, target | String,
      prerequisites | Array String, authority | [| 'advisory |], expected_observation | String,
      argv | Array String },
    Snapshot = { scope | { change | String, policy_hash | String, source_hash | String, evidence_hash | String },
      stages | Array { stage | StageKind, state | State, decision_hash | String } },
  },
  policy = {
    schema = "cairn.readiness.v1",
    max_blockers | Positive = 512,
    max_observation_bytes | Positive = 16777216,
    max_target_bytes | Positive = 4096,
    rules = [
      { code = "artifact.incomplete", kind = "artifact_repair" },
      { code = "cairn.workflow.profile.review", kind = "evidence_refresh" },
      { code = "dependency.blocked", kind = "dependency_wait" },
      { code = "verification_result.missing", kind = "evidence_refresh" },
      { code = "verification_result.subject_stale", kind = "evidence_refresh" },
      { code = "probe.saved_evidence.invalid", kind = "evidence_refresh" },
      { code = "verification_contract.subject_stale", kind = "evidence_refresh" },
    ],
  },
}
