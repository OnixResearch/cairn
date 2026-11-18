# Opt-in finite workflow obligations.
# One admitted definition yields both agent guidance and a completion predicate.
# References pin existing native evidence predicates by requirement ID and exact
# accepted revision; nothing here duplicates native evaluator logic. Independent
# of the legacy default policy, so it changes no existing policy or receipt hash
# material and adds no executor, clock service or monitoring.
let NonClaims = [
  "guidance and the completion predicate describe one admitted finite definition",
  "presentation adapters change layout and prose only, never evidence requirements, command classes or authority",
  "activity counts, checked task boxes and agent success statements are not native evidence",
  "a finite trace ordering result implies no unbounded safety or liveness",
  "an accepted policy exception never proves the original unsatisfied condition",
  "no result here authorizes execution, approval, readiness or a gate verdict",
] in
let EvidenceContract = {
  id = "agent-receipt.verification-contract",
  requirement_id = "cairn.supersede.agent_run_receipts.verification_contract",
  owner = "cairn_core::verified::agent_receipt::authoritative",
  accepted_revision_blake3 = "cab928f04ff3d61d2ea9e2a2640262e2022dc3ba820d1949fad73d8d6dc977af",
  kind = "evidence",
  description = "The repository-owned verification contract is present and independently supplied.",
} in
let EvidencePassOnly = {
  id = "agent-receipt.pass-only",
  requirement_id = "cairn.supersede.agent_run_receipts.pass_only",
  owner = "cairn_core::verified::agent_receipt::authoritative",
  accepted_revision_blake3 = "34abbfe64cae739b75554f48d89d0e0bc19ca8e55d54cf51a9363ffe84a6f58d",
  kind = "evidence",
  description = "Exactly one eligible passed result per selected required check; no unknown rows substitute.",
} in
let EvidenceFreshness = {
  id = "agent-receipt.freshness",
  requirement_id = "cairn.supersede.agent_run_receipts.freshness",
  owner = "cairn_core::verified::agent_receipt::authoritative",
  accepted_revision_blake3 = "653914140db582ef1145f36f87c233a9ab57a310f715cadf8a23efa5f5333e5e",
  kind = "evidence",
  description = "Every completion-eligible result subject equals the current final subject.",
} in
let EvidenceAuthority = {
  id = "agent-receipt.completion-authority",
  requirement_id = "cairn.supersede.agent_run_receipts.completion_authority",
  owner = "cairn_core::verified::agent_receipt::authoritative",
  accepted_revision_blake3 = "5427af464b1ebc00db599a9ec90cdee48d0ce7b4a0b2652c6dc1d134bd89451b",
  kind = "evidence",
  description = "Repository-owned authoritative completion admits the selected candidate.",
} in
let EvidenceClaimRevision = {
  id = "verification-obligation.claim-revision",
  requirement_id = "cairn.verification_obligation_cob.claim_revision",
  owner = "cairn_core::verified::workflow_collaboration",
  accepted_revision_blake3 = "1de40130cc0a88bb88288b16daa621de599b1508511648332e9e23ef11a3b666",
  kind = "evidence",
  description = "The verification obligation COB claim revision is current for the selected candidate.",
} in
let EvidenceSandbox = {
  id = "exception.sandbox-revalidation",
  requirement_id = "cairn.supersede.agent_run_receipts.freshness",
  owner = "cairn_core::verified::agent_receipt::authoritative",
  accepted_revision_blake3 = "653914140db582ef1145f36f87c233a9ab57a310f715cadf8a23efa5f5333e5e",
  kind = "evidence",
  description = "The compensating control revalidated the freshness obligation in an isolated sandbox and retained that observation identity.",
} in
let ApplicabilityScope = {
  id = "verification-scope.applicable",
  requirement_id = "cairn.supersede.agent_run_receipts.verification_contract",
  owner = "cairn_core::verified::agent_receipt::authoritative",
  accepted_revision_blake3 = "cab928f04ff3d61d2ea9e2a2640262e2022dc3ba820d1949fad73d8d6dc977af",
  kind = "applicability",
  description = "Native observation that the selected candidate is inside the repository contract verification scope.",
} in
let evidence = fun node predicate => { kind = "evidence", id = node, predicate = predicate } in
let all_of = fun node children => { kind = "all_of", id = node, obligations = children } in
let any_of = fun node alternatives => { kind = "any_of", id = node, alternatives = alternatives } in
let alternative = fun node children => { id = node, obligations = children } in
let applicable = fun node condition child => { kind = "applicable", id = node, condition = condition, "then" = child } in
let Completion = all_of "verify.completion" [
  evidence "verify.evidence.contract" "agent-receipt.verification-contract",
  evidence "verify.evidence.pass-only" "agent-receipt.pass-only",
  evidence "verify.evidence.freshness" "agent-receipt.freshness",
  evidence "verify.evidence.authority" "agent-receipt.completion-authority",
  any_of "verify.traceability" [
    alternative "receipt-chain" [
      evidence "verify.traceability.contract" "agent-receipt.verification-contract",
      evidence "verify.traceability.freshness" "agent-receipt.freshness",
    ],
    alternative "claim-revision" [
      evidence "verify.traceability.claim" "verification-obligation.claim-revision",
    ],
  ],
  applicable "verify.conditional.claim" "verification-scope.applicable"
    (evidence "verify.conditional.claim.revision" "verification-obligation.claim-revision"),
] in
# Intent identity preimage, canonical JSON sorted keys and compact separators:
# {"goal":"Check requirement coverage, gates, tests, and receipts without changing lifecycle artifacts.","intent":"finite completion over native verification evidence","version":1,"workflow":"verify"}
let VerifyDefinition = {
  id = "verify",
  workflow_id = "verify",
  workflow_version = 1,
  intent_blake3 = "a1b776a9b77c138f9a14f8d2e380d48824c9b994bf342cf3f1721a8804514f32",
  action_preconditions = [
    {
      id = "verify.precondition.status",
      action = "workflow-inspect",
      required_predicates = ["agent-receipt.verification-contract"],
      authority = "The native workflow status and policy retain authority for the selected change.",
    },
    {
      id = "verify.precondition.gate",
      action = "gate",
      required_predicates = ["agent-receipt.verification-contract", "agent-receipt.pass-only"],
      authority = "The native Pass | Block gate retains authority; this precondition never replaces it.",
    },
  ],
  expected_observations = [
    {
      id = "verify.observation.receipt",
      command_class = "receipt-inspect",
      predicate = "agent-receipt.freshness",
      description = "inspect current receipt freshness for the selected candidate",
    },
    {
      id = "verify.observation.claim",
      command_class = "traceability",
      predicate = "verification-obligation.claim-revision",
      description = "read the current verification obligation claim revision",
    },
  ],
  progress_facts = [
    {
      id = "verify.progress.inspected",
      description = "The agent inspected the selected change receipts and traceability output.",
    },
    {
      id = "verify.progress.reported",
      description = "The agent reported checked evidence and exact blockers.",
    },
  ],
  completion = Completion,
  exceptions = [],
  temporal = null,
  non_claims = NonClaims,
} in
# Intent identity preimage, canonical JSON sorted keys and compact separators:
# {"goal":"Check requirement coverage, gates, tests, and receipts without changing lifecycle artifacts.","intent":"finite completion plus declared trace ordering and a native policy-exception path","variant":"bounded-temporal-exception","version":1,"workflow":"verify"}
let BoundedDefinition = {
  id = "verify.bounded-exception",
  workflow_id = VerifyDefinition.workflow_id,
  workflow_version = VerifyDefinition.workflow_version,
  intent_blake3 = "ef968625ac8c9ac745d81fbb90c9f76feeaefb904dc9e9c8aa4bd39c8dae77e0",
  action_preconditions = VerifyDefinition.action_preconditions,
  expected_observations = VerifyDefinition.expected_observations,
  progress_facts = VerifyDefinition.progress_facts,
  completion = all_of "verify.bounded.completion" [
    Completion,
    { kind = "temporal", id = "verify.bounded.ordering", check = "verify.receipt-order" },
  ],
  exceptions = [
    {
      id = "verify.exception.freshness",
      node = "verify.evidence.freshness",
      exception_id = "policy-exception:verify-freshness",
      request_revision = 3,
      policy_identity = "9f463020b564b252f6322a6f4ecfa8094a28a78ff5bc32e6b9ad1b2e01541542",
      scope_rule_id = "allow-fs-write",
      control = {
        id = "sandboxed-revalidation",
        description = "Revalidate the freshness obligation inside an isolated sandbox and retain the observation identity.",
        evidence_predicate = "exception.sandbox-revalidation",
      },
    },
  ],
  temporal = {
    id = "verify.receipt-order",
    trace_id = "verify:selected-change",
    first_event = "receipt-recorded",
    second_event = "candidate-frozen",
    observer = "cairn-lifecycle",
    clock = "supplied-u64-epoch.v1",
    max_span_epochs = 86400,
    completeness = "complete",
  },
  non_claims = VerifyDefinition.non_claims,
} in
{
  schema = "cairn.workflow-obligation-policy.v1",
  profile = "cairn.workflow-obligation.finite.v1",
  interpretation = "finite all_of/any_of/conditional applicability over exact native evidence revisions, with declared finite trace ordering and native policy-exception applicability",
  evidence_predicates = [
    EvidenceContract,
    EvidencePassOnly,
    EvidenceFreshness,
    EvidenceAuthority,
    EvidenceClaimRevision,
    EvidenceSandbox,
    ApplicabilityScope,
  ],
  observation_model = {
    id = "cairn.observations.explicit-supplied.v1",
    evidence_kinds = ["evidence", "applicability", "progress", "exception", "temporal"],
    clock = "supplied-u64-epoch.v1",
    completeness = "declared-finite-trace-interval.v1",
    unknown_rule = "missing or partial observations remain unknown and never satisfy an obligation",
  },
  definitions = [VerifyDefinition, BoundedDefinition],
  bounds = {
    max_definitions = 8,
    max_obligations = 128,
    max_alternatives = 8,
    max_predicates = 32,
    max_exceptions = 8,
    max_temporal_events = 16,
    max_depth = 12,
    max_text_bytes = 4096,
  },
  non_claims = NonClaims,
} | {
  schema | String,
  profile | String,
  interpretation | String,
  evidence_predicates | Array {
    id | String,
    requirement_id | String,
    owner | String,
    accepted_revision_blake3 | String,
    kind | String,
    description | String,
  },
  observation_model | {
    id | String,
    evidence_kinds | Array String,
    clock | String,
    completeness | String,
    unknown_rule | String,
  },
  definitions | Array Dyn,
  bounds | {
    max_definitions | Number,
    max_obligations | Number,
    max_alternatives | Number,
    max_predicates | Number,
    max_exceptions | Number,
    max_temporal_events | Number,
    max_depth | Number,
    max_text_bytes | Number,
  },
  non_claims | Array String,
}
